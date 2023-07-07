use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use std::boxed::Box;
use std::error::Error;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_to_all::RoleAtoAll;
use mpstthree::role::all_to_a::RoleAlltoA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

use mpstthree::checker_concat;

use petgraph::dot::Dot;

// Test our usecase
// Simple types
// Client = A
// Authenticator = B
// Server = C

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

// Stacks
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

// Creating the MP sessions
// For A
type ChooseAtoB<N> =
    ChooseMpst<AtoBVideo<N>, CtoBVideo<N>, AtoBClose, CtoBClose, StackBVideoDual, StackBEnd, NameB>;
type ChooseAtoC<N> =
    ChooseMpst<AtoCClose, BtoCVideo<N>, BtoCClose, AtoCClose, StackCVideoDual, StackCEnd, NameC>;
type InitA<N> = Send<N, Recv<N, ChooseAtoB<N>>>;
type EndpointAFull<N> = MeshedChannels<InitA<N>, ChooseAtoC<N>, StackAFull, NameA>;

// For B
type EndpointBVideo<N> = MeshedChannels<BtoAVideo<N>, BtoCVideo<N>, StackBVideo, NameB>;
type EndpointBEnd = MeshedChannels<BtoAClose, BtoCClose, StackBEnd, NameB>;

type OfferB<N> =
    OfferMpst<BtoAVideo<N>, BtoCVideo<N>, BtoAClose, BtoCClose, StackBVideo, StackBEnd, NameB>;
type InitB<N> = Recv<N, Send<N, OfferB<N>>>;
type EndpointBFull<N> = MeshedChannels<InitB<N>, End, StackBFull, NameB>;

// For C
type EndpointCVideo<N> = MeshedChannels<CtoAClose, CtoBVideo<N>, StackCVideo, NameC>;
type EndpointCEnd = MeshedChannels<CtoAClose, CtoBClose, StackCEnd, NameC>;

type OfferC<N> =
    OfferMpst<CtoAClose, CtoBVideo<N>, CtoAClose, CtoBClose, StackCVideo, StackCEnd, NameC>;
type EndpointCFull<N> = MeshedChannels<OfferC<N>, End, StackCFull, NameC>;

// Functions related to endpoints
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
    assert!({
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(client_video, authenticator, server);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok::<(), Box<dyn Error>>(())
    }
    .is_ok());
}

pub fn run_a_usecase_right() {
    assert!({
        // Test end branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(client_close, authenticator, server);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok::<(), Box<dyn Error>>(())
    }
    .is_ok());
}

pub fn run_a_usecase_checker() {
    let (graphs, kmc) =
        checker_concat!(EndpointAFull<i32>, EndpointBFull<i32>, EndpointCFull<i32>).unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_a)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"1\\\"\" ]\n    \
            2 [ label = \"\\\"2\\\"\" ]\n    \
            3 [ label = \"\\\"2.1\\\"\" ]\n    \
            4 [ label = \"\\\"2.2\\\"\" ]\n    \
            5 [ label = \"\\\"2.3\\\"\" ]\n    \
            6 [ label = \"\\\"2.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleA!RoleB: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleA!RoleB: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 6 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_b)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"1\\\"\" ]\n    \
            2 [ label = \"\\\"2\\\"\" ]\n    \
            3 [ label = \"\\\"2.1\\\"\" ]\n    \
            4 [ label = \"\\\"2.2\\\"\" ]\n    \
            5 [ label = \"\\\"2.3\\\"\" ]\n    \
            6 [ label = \"\\\"2.4\\\"\" ]\n    \
            7 [ label = \"\\\"2.5\\\"\" ]\n    \
            8 [ label = \"\\\"2.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleB?RoleA: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleB?RoleA: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleB!RoleC: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 8 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_c)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.2\\\"\" ]\n    \
            3 [ label = \"\\\"0.3\\\"\" ]\n    \
            4 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleC?RoleB: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(kmc, None);
}
