use rand::{thread_rng, Rng};

use mpstthree::checking::checker;

use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use std::boxed::Box;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_to_all::RoleAtoAll;
use mpstthree::role::all_to_a::RoleAlltoA;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_dual::RoleBDual;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// Test our usecase
/// Simple types
/// Client = A
/// Authenticator = B
/// Server = C

type BtoAClose = End;
type BtoCClose = End;
type BtoAVideo<N> = Recv<N, Send<N, End>>;
type BtoCVideo<N> = Send<N, Recv<N, End>>;

type CtoBClose = <BtoCClose as Session>::Dual;
type CtoAClose = End;
type CtoBVideo<N> = <BtoCVideo<N> as Session>::Dual;

type AtoCClose = <CtoAClose as Session>::Dual;
type AtoBClose = <BtoAClose as Session>::Dual;
type AtoBVideo<N> = <BtoAVideo<N> as Session>::Dual;

/// Stacks
type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleC<RoleC<RoleA<RoleEnd>>>>;
type StackBVideoDual = <StackBVideo as Role>::Dual;
type StackBFull = RoleA<RoleA<RoleAlltoA<RoleEnd, RoleEnd>>>;

type StackCEnd = RoleEnd;
type StackCVideo = RoleB<RoleB<RoleEnd>>;
type StackCVideoDual = <StackCVideo as Role>::Dual;
type StackCFull = RoleAlltoA<RoleEnd, RoleEnd>;

type StackAEnd = RoleEnd;
type StackAVideo = RoleB<RoleB<RoleEnd>>;
type StackAChoice = RoleAtoAll<StackAVideo, StackAEnd>;
type StackAFull = RoleB<RoleB<StackAChoice>>;

/// Creating the MP sessions
/// For A
type ChooseAtoB<N> = ChooseMpst<
    AtoBVideo<N>,
    CtoBVideo<N>,
    AtoBClose,
    CtoBClose,
    StackBVideoDual,
    StackBEnd,
    RoleBDual<RoleEnd>,
>;
type ChooseAtoC<N> = ChooseMpst<
    AtoCClose,
    BtoCVideo<N>,
    BtoCClose,
    AtoCClose,
    StackCVideoDual,
    StackCEnd,
    RoleCDual<RoleEnd>,
>;
type InitA<N> = Send<N, Recv<N, ChooseAtoB<N>>>;
type EndpointAFull<N> = MeshedChannels<InitA<N>, ChooseAtoC<N>, StackAFull, RoleA<RoleEnd>>;

/// For B
type EndpointBVideo<N> = MeshedChannels<BtoAVideo<N>, BtoCVideo<N>, StackBVideo, RoleB<RoleEnd>>;
type EndpointBEnd = MeshedChannels<BtoAClose, BtoCClose, StackBEnd, RoleB<RoleEnd>>;

type OfferB<N> = OfferMpst<
    BtoAVideo<N>,
    BtoCVideo<N>,
    BtoAClose,
    BtoCClose,
    StackBVideo,
    StackBEnd,
    RoleB<RoleEnd>,
>;
type InitB<N> = Recv<N, Send<N, OfferB<N>>>;
type EndpointBFull<N> = MeshedChannels<InitB<N>, End, StackBFull, RoleB<RoleEnd>>;

/// For C
type EndpointCVideo<N> = MeshedChannels<CtoAClose, CtoBVideo<N>, StackCVideo, RoleC<RoleEnd>>;
type EndpointCEnd = MeshedChannels<CtoAClose, CtoBClose, StackCEnd, RoleC<RoleEnd>>;

type OfferC<N> = OfferMpst<
    CtoAClose,
    CtoBVideo<N>,
    CtoAClose,
    CtoBClose,
    StackCVideo,
    StackCEnd,
    RoleC<RoleEnd>,
>;
type EndpointCFull<N> = MeshedChannels<OfferC<N>, End, StackCFull, RoleC<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointCVideo<i32>| {
            let (request, s) = s.recv()?;
            s.send(request + 1).close()
        },
        |s: EndpointCEnd| s.close(),
    )
}

fn authenticator(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv()?;

    s.send(id + 1).offer(
        |s: EndpointBVideo<i32>| {
            let (request, s) = s.recv()?;
            let (video, s) = s.send(request + 1).recv()?;
            let s = s.send(video + 1);

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            s.close()
        },
        |s: EndpointBEnd| s.close(),
    )
}

fn client_video(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let (accept, s) = s.send(id).recv()?;
    let (result, s) = s.choose_left().send(accept).recv()?;

    assert_eq!(accept, id + 1);
    assert_eq!(result, accept + 3);

    s.close()
}

fn client_close(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let (accept, s) = s.send(id).recv()?;

    assert_eq!(accept, id + 1);

    s.choose_right().close()
}

/////////////////////////////////////////

pub fn run_a_usecase_left() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(client_video, authenticator, server);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn run_a_usecase_right() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test end branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(client_close, authenticator, server);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn run_a_usecase_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let s = RandomState::new();
            let hm: HashMap<String, &Vec<String>> = HashMap::with_hasher(s);

            let (s1, _): (EndpointAFull<i32>, _) = MeshedChannels::new();
            let (s2, _): (EndpointBFull<i32>, _) = MeshedChannels::new();
            let (s3, _): (EndpointCFull<i32>, _) = MeshedChannels::new();

            let (a, b, c) = checker(s1, s2, s3, &hm, &HashMap::new())?;

            assert_eq!(a, "A: A!B.A?B.( A!B.A?B.0 + 0 )");
            assert_eq!(b, "B: B?A.B!A.( B?A.B!C.B?C.B!A.0 & 0 )");
            assert_eq!(c, "C: ( C?B.C!B.0 & 0 )");
        }
        Ok(())
    }()
    .is_ok());
}
