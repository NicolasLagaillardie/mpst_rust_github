use either::Either;
use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use std::error::Error;

use rand::{thread_rng, Rng};

use mpstthree::bundle_impl;

// Create new roles
bundle_impl!(SessionMpst => A, B, C => fork_mpst);

type OfferMpst<S0, S1, S2, S3, R0, R1, N0> =
    Recv<Either<SessionMpst<S0, S1, R0, N0>, SessionMpst<S2, S3, R1, N0>>, End>;

type ChooseMpst<S0, S1, S2, S3, R0, R1, N0> = Send<
    Either<
        SessionMpst<
            <S0 as Session>::Dual,
            <S1 as Session>::Dual,
            <R0 as Role>::Dual,
            <N0 as Role>::Dual,
        >,
        SessionMpst<
            <S2 as Session>::Dual,
            <S3 as Session>::Dual,
            <R1 as Role>::Dual,
            <N0 as Role>::Dual,
        >,
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

/// Stacks
type StackAEnd = RoleEnd;
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleEnd>>>>;
type StackAVideoDual = <StackAVideo as Role>::Dual;
type StackAFull = RoleC<RoleC<RoleAlltoC<RoleEnd, RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleA<RoleA<RoleEnd>>;
type StackBVideoDual = <StackBVideo as Role>::Dual;
type StackBFull = RoleAlltoC<RoleEnd, RoleEnd>;

type StackCEnd = RoleEnd;
type StackCVideo = RoleA<RoleA<RoleEnd>>;
type StackCChoice = RoleCtoAll<StackCVideo, StackCEnd>;
type StackCFull = RoleA<RoleA<StackCChoice>>;

/// Creating the MP sessions
/// For C
type ChooseCtoA<N> = ChooseMpst<
    BtoAVideo<N>,
    CtoAVideo<N>,
    BtoAClose,
    CtoAClose,
    StackAVideoDual,
    StackAEnd,
    RoleADual<RoleEnd>,
>;
type ChooseCtoB<N> = ChooseMpst<
    AtoBVideo<N>,
    CtoBClose,
    AtoBClose,
    CtoBClose,
    StackBVideoDual,
    StackBEnd,
    RoleBDual<RoleEnd>,
>;
type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCtoB<N>, StackCFull, RoleC<RoleEnd>>;

/// For A
type EndpointAVideo<N> = SessionMpst<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, RoleA<RoleEnd>>;
type EndpointAEnd = SessionMpst<AtoBClose, AtoCClose, StackAEnd, RoleA<RoleEnd>>;

type OfferA<N> = OfferMpst<
    AtoBVideo<N>,
    AtoCVideo<N>,
    AtoBClose,
    AtoCClose,
    StackAVideo,
    StackAEnd,
    RoleA<RoleEnd>,
>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, StackAFull, RoleA<RoleEnd>>;

/// For B
type EndpointBVideo<N> = SessionMpst<BtoAVideo<N>, BtoCClose, StackBVideo, RoleB<RoleEnd>>;
type EndpointBEnd = SessionMpst<BtoAClose, BtoCClose, StackBEnd, RoleB<RoleEnd>>;

type OfferB<N> = OfferMpst<
    BtoAVideo<N>,
    BtoCClose,
    BtoAClose,
    BtoCClose,
    StackBVideo,
    StackBEnd,
    RoleB<RoleEnd>,
>;
type EndpointBFull<N> = SessionMpst<End, OfferB<N>, StackBFull, RoleB<RoleEnd>>;

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

    s.send(id + 1).offer(
        |s: EndpointAVideo<i32>| {
            let (request, s) = s.recv()?;
            let (video, s) = s.send(request + 1).recv()?;

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

    let (accept, s) = s.send(id).recv()?;

    assert_eq!(accept, id + 1);

    let (result, s) = s.choose_left().send(accept).recv()?;

    assert_eq!(result, accept + 3);

    s.close()
}

fn client_close(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let (accept, s) = s.send(id).recv()?;

    assert_eq!(accept, id + 1);

    s.choose_right().close()
}

/////////////////////////////////////////

pub fn run_usecase_right() {
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

pub fn run_usecase_left() {
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
