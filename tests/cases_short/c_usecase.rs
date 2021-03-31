use rand::{thread_rng, Rng};

use mpstthree::checking::checker;

use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;

use std::boxed::Box;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::all_to_c::RoleAlltoC;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_dual::RoleBDual;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_to_all::RoleCtoAll;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// Test our usecase
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

type AtoCClose = End;
type AtoBClose = End;
type AtoCVideo<N> = Recv<N, Send<N, End>>;
type AtoBVideo<N> = Send<N, Recv<N, End>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type CtoBClose = <BtoCClose as Session>::Dual;
type CtoAClose = <AtoCClose as Session>::Dual;
type CtoAVideo<N> = <AtoCVideo<N> as Session>::Dual;

/// Queues
type QueueAEnd = RoleEnd;
type QueueAVideo = RoleC<RoleB<RoleB<RoleC<RoleEnd>>>>;
type QueueAVideoDual = <QueueAVideo as Role>::Dual;
type QueueAFull = RoleC<RoleC<RoleAlltoC<RoleEnd, RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleA<RoleA<RoleEnd>>;
type QueueBVideoDual = <QueueBVideo as Role>::Dual;
type QueueBFull = RoleAlltoC<RoleEnd, RoleEnd>;

type QueueCEnd = RoleEnd;
type QueueCVideo = RoleA<RoleA<RoleEnd>>;
type QueueCChoice = RoleCtoAll<QueueCVideo, QueueCEnd>;
type QueueCFull = RoleA<RoleA<QueueCChoice>>;

/// Creating the MP sessions
/// For C
type ChooseCtoA<N> = ChooseMpst<
    BtoAVideo<N>,
    CtoAVideo<N>,
    BtoAClose,
    CtoAClose,
    QueueAVideoDual,
    QueueAEnd,
    RoleADual<RoleEnd>,
>;
type ChooseCtoB<N> = ChooseMpst<
    AtoBVideo<N>,
    CtoBClose,
    AtoBClose,
    CtoBClose,
    QueueBVideoDual,
    QueueBEnd,
    RoleBDual<RoleEnd>,
>;
type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCtoB<N>, QueueCFull, RoleC<RoleEnd>>;

/// For A
type EndpointAVideo<N> = SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo, RoleA<RoleEnd>>;
type EndpointAEnd = SessionMpst<AtoBClose, AtoCClose, QueueAEnd, RoleA<RoleEnd>>;

type OfferA<N> = OfferMpst<
    AtoBVideo<N>,
    AtoCVideo<N>,
    AtoBClose,
    AtoCClose,
    QueueAVideo,
    QueueAEnd,
    RoleA<RoleEnd>,
>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAFull, RoleA<RoleEnd>>;

/// For B
type EndpointBVideo<N> = SessionMpst<BtoAVideo<N>, BtoCClose, QueueBVideo, RoleB<RoleEnd>>;
type EndpointBEnd = SessionMpst<BtoAClose, BtoCClose, QueueBEnd, RoleB<RoleEnd>>;

type OfferB<N> = OfferMpst<
    BtoAVideo<N>,
    BtoCClose,
    BtoAClose,
    BtoCClose,
    QueueBVideo,
    QueueBEnd,
    RoleB<RoleEnd>,
>;
type EndpointBFull<N> = SessionMpst<End, OfferB<N>, QueueBFull, RoleB<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointBVideo<i32>| {
            let (request, s) = s.recv()?;
            s.send(request + 1).close()
        },
        |s: EndpointBEnd| s.close(),
    )
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv()?;
    let s = s.send(id + 1);

    s.offer(
        |s: EndpointAVideo<i32>| {
            let (request, s) = s.recv()?;
            let (video, s) = s.send(request + 1).recv()?;
            let s = s.send(video + 1);

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            s.close()
        },
        |s: EndpointAEnd| s.close(),
    )
}

fn client_video(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let id: i32 = thread_rng().gen();

    let (accept, s) = s.send(id).recv()?;

    assert_eq!(accept, id + 1);

    let (result, s) = s.choose_left().send(accept).recv()?;

    assert_eq!(result, accept + 3);

    s.close()
}

fn client_close(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let id: i32 = thread_rng().gen();

    let (accept, s) = s.send(id).recv()?;

    assert_eq!(accept, id + 1);

    s.choose_right().close()
}

/////////////////////////////////////////

pub fn run_c_usecase_left() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_video);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn run_c_usecase_right() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test end branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_close);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn run_c_usecase_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let s = RandomState::new();
            let hm: HashMap<String, &Vec<String>> = HashMap::with_hasher(s);

            let (s1, _): (EndpointAFull<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointBFull<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointCFull<i32>, _) = SessionMpst::new();

            let (a, b, c) = checker(s1, s2, s3, &hm, &HashMap::new())?;

            assert_eq!(a, "A: A?C.A!C.( A?C.A!B.A?B.A!C.0 & 0 )");
            assert_eq!(b, "B: ( B?A.B!A.0 & 0 )");
            assert_eq!(c, "C: C!A.C?A.( C!A.C?A.0 + 0 )");
        }
        Ok(())
    }()
    .is_ok());
}
