use either::Either;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use std::error::Error;

use rand::{thread_rng, Rng};

use mpstthree::generate;

// Create new roles
generate!("basic", MeshedChannels, A, B, C);

// Those types will be code generated
type OfferMpst<S0, S1, S2, S3, R0, R1, N0> =
    Recv<Either<MeshedChannels<S0, S1, R0, N0>, MeshedChannels<S2, S3, R1, N0>>, End>;

type ChooseMpst<S0, S1, S2, S3, R0, R1, N0> = Send<
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

// Creating the MP sessions
// For C
type ChooseCtoA<N> =
    ChooseMpst<BtoAVideo<N>, CtoAVideo<N>, BtoAClose, CtoAClose, StackAVideoDual, StackAEnd, NameA>;
type ChooseCtoB<N> =
    ChooseMpst<AtoBVideo<N>, CtoBClose, AtoBClose, CtoBClose, StackBVideoDual, StackBEnd, NameB>;
type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
type EndpointCFull<N> = MeshedChannels<InitC<N>, ChooseCtoB<N>, StackCFull, NameC>;

// For A
type EndpointAVideo<N> = MeshedChannels<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, NameA>;
type EndpointAEnd = MeshedChannels<AtoBClose, AtoCClose, StackAEnd, NameA>;

type OfferA<N> =
    OfferMpst<AtoBVideo<N>, AtoCVideo<N>, AtoBClose, AtoCClose, StackAVideo, StackAEnd, NameA>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAFull, NameA>;

// For B
type EndpointBVideo<N> = MeshedChannels<BtoAVideo<N>, BtoCClose, StackBVideo, NameB>;
type EndpointBEnd = MeshedChannels<BtoAClose, BtoCClose, StackBEnd, NameB>;

type OfferB<N> =
    OfferMpst<BtoAVideo<N>, BtoCClose, BtoAClose, BtoCClose, StackBVideo, StackBEnd, NameB>;
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

/////////////////////////////////////////

pub fn run_usecase_right() {
    // Test end branch.
    let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_close);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}

pub fn run_usecase_left() {
    // Test video branch.
    let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_video);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}
