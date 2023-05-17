use rand::{thread_rng, Rng};

use either::Either;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use std::error::Error;

use mpstthree::baker;

// Create new roles
baker!("basic", MeshedChannels, A, B, D);

// Those types will be code generated
type OfferMpstThree<S0, S1, S2, S3, R0, R1, N0> =
    Recv<Either<MeshedChannels<S0, S1, R0, N0>, MeshedChannels<S2, S3, R1, N0>>, End>;

type ChooseMpstThree<S0, S1, S2, S3, R0, R1, N0> = Send<
    Either<
        MeshedChannels<<S0 as Session>::Dual, <S1 as Session>::Dual, <R0 as Role>::Dual, N0>,
        MeshedChannels<<S2 as Session>::Dual, <S3 as Session>::Dual, <R1 as Role>::Dual, N0>,
    >,
    End,
>;

// Types
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

// Stacks
type StackAEnd = RoleEnd;
type StackAVideo = RoleD<RoleB<RoleB<RoleD<RoleEnd>>>>;
type StackAVideoDual = <StackAVideo as Role>::Dual;
type StackAFull = RoleD<RoleD<RoleAlltoD<RoleEnd, RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleA<RoleEnd>>;
type StackBVideoDual = <StackBVideo as Role>::Dual;
type StackBFull = RoleAlltoD<RoleEnd, RoleEnd>;

type StackCEnd = RoleEnd;
type StackCVideo = RoleA<RoleA<RoleEnd>>;
type StackCChoice = RoleDtoAll<StackCVideo, StackCEnd>;
type StackCFull = RoleA<RoleA<StackCChoice>>;

// Creating the MP sessions
// For C
type ChooseCtoA<N> = ChooseMpstThree<
    BtoAVideo<N>,
    CtoAVideo<N>,
    BtoAClose,
    CtoAClose,
    StackAVideoDual,
    StackAEnd,
    NameA,
>;
type ChooseCtoB<N> = ChooseMpstThree<
    AtoBVideo<N>,
    CtoBClose,
    AtoBClose,
    CtoBClose,
    StackBVideoDual,
    StackBEnd,
    NameB,
>;
type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
type EndpointCFull<N> = MeshedChannels<InitC<N>, ChooseCtoB<N>, StackCFull, NameD>;

// For A
type EndpointAVideo<N> = MeshedChannels<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, NameA>;
type EndpointAEnd = MeshedChannels<AtoBClose, AtoCClose, StackAEnd, NameA>;

type OfferA<N> =
    OfferMpstThree<AtoBVideo<N>, AtoCVideo<N>, AtoBClose, AtoCClose, StackAVideo, StackAEnd, NameA>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAFull, NameA>;

// For B
type EndpointBVideo<N> = MeshedChannels<BtoAVideo<N>, BtoCClose, StackBVideo, NameB>;
type EndpointBEnd = MeshedChannels<BtoAClose, BtoCClose, StackBEnd, NameB>;

type OfferB<N> =
    OfferMpstThree<BtoAVideo<N>, BtoCClose, BtoAClose, BtoCClose, StackBVideo, StackBEnd, NameB>;
type EndpointBFull<N> = MeshedChannels<End, OfferB<N>, StackBFull, NameB>;

// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointBVideo<i32>| {
            let (request, s) = s.recv();
            s.send(request + 1).close()
        },
        |s: EndpointBEnd| s.close(),
    )
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv();

    s.send(id + 1).offer(
        |s: EndpointAVideo<i32>| {
            let (request, s) = s.recv();
            let (video, s) = s.send(request + 1).recv();

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            s.send(video + 1).close()
        },
        |s: EndpointAEnd| s.close(),
    )
}

fn client_video(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let (accept, s) = s.send(id).recv();

    assert_eq!(accept, id + 1);

    let (result, s) = s.choose_left().send(accept).recv();

    assert_eq!(result, accept + 3);

    s.close()
}

fn client_close(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let (accept, s) = s.send(id).recv();

    assert_eq!(accept, id + 1);

    s.choose_right().close()
}

////////////////////////////////////////

pub fn test_new_choice_full() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(authenticator, server, client_video);

            assert!(thread_a.join().is_ok());
            assert!(thread_pawn.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

pub fn test_new_choice_close() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test end branch.
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(authenticator, server, client_close);

            assert!(thread_a.join().is_ok());
            assert!(thread_pawn.join().is_ok());
            assert!(thread_d.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}
