use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use mpstthree::checker_concat;

use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
use mpstthree::functionmpst::recv::recv_mpst_b_from_c;
use mpstthree::functionmpst::recv::recv_mpst_c_from_a;
use mpstthree::functionmpst::recv::recv_mpst_c_from_b;

use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::functionmpst::offer::offer_mpst_session_to_a_from_b;
use mpstthree::functionmpst::offer::offer_mpst_session_to_c_from_b;

use mpstthree::functionmpst::choose::choose_left_mpst_session_b_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_b_to_all;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

use petgraph::dot::Dot;

// Test our usecase
// Simple types
// Authenticator = C
// Server = A
// Client = B

type CtoBClose = End;
type CtoAClose = End;
type CtoBVideo<N> = Recv<N, Send<N, End>>;
type CtoAVideo<N> = Send<N, Recv<N, End>>;

type AtoCClose = <CtoAClose as Session>::Dual;
type AtoBClose = End;
type AtoCVideo<N> = <CtoAVideo<N> as Session>::Dual;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = <CtoBClose as Session>::Dual;
type BtoCVideo<N> = <CtoBVideo<N> as Session>::Dual;

// Stacks
type StackCEnd = RoleEnd;
type StackCEndDual = <StackCEnd as Role>::Dual;
type StackCVideo = RoleB<RoleA<RoleA<RoleB<RoleEnd>>>>;
type StackCVideoDual = <StackCVideo as Role>::Dual;
type StackCFull = RoleB<RoleB<RoleAlltoB<RoleEnd, RoleEnd>>>;

type StackAEnd = RoleEnd;
type StackAEndDual = <StackAEnd as Role>::Dual;
type StackAVideo = RoleC<RoleC<RoleEnd>>;
type StackAVideoDual = <StackAVideo as Role>::Dual;
type StackAFull = RoleAlltoB<RoleEnd, RoleEnd>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleC<RoleC<RoleEnd>>;
type StackBChoice = RoleBtoAll<StackBVideo, StackBEnd>;
type StackBFull = RoleC<RoleC<StackBChoice>>;

// Creating the MP sessions
// For C
type ChooseBtoC<N> =
    ChooseMpst<AtoCVideo<N>, BtoCVideo<N>, AtoCClose, BtoCClose, StackCVideoDual, StackCEnd, NameC>;
type ChooseBtoA<N> =
    ChooseMpst<BtoAClose, CtoAVideo<N>, BtoAClose, CtoAClose, StackAVideoDual, StackAEnd, NameA>;
type InitB<N> = Send<N, Recv<N, ChooseBtoC<N>>>;
type EndpointBFull<N> = MeshedChannels<ChooseBtoA<N>, InitB<N>, StackBFull, NameB>;

// For A
type EndpointCVideo<N> = MeshedChannels<CtoAVideo<N>, CtoBVideo<N>, StackCVideo, NameC>;

type OfferC<N> =
    OfferMpst<CtoAVideo<N>, CtoBVideo<N>, CtoAClose, CtoBClose, StackCVideo, StackCEnd, NameC>;
type InitC<N> = Recv<N, Send<N, OfferC<N>>>;
type EndpointCFull<N> = MeshedChannels<End, InitC<N>, StackCFull, NameC>;

// For B
type EndpointAVideo<N> = MeshedChannels<AtoBClose, AtoCVideo<N>, StackAVideo, NameA>;

type OfferA<N> =
    OfferMpst<AtoBClose, AtoCVideo<N>, AtoBClose, AtoCClose, StackAVideo, StackAEnd, NameA>;
type EndpointAFull<N> = MeshedChannels<OfferA<N>, End, StackAFull, NameA>;

// Functions related to endpoints
fn server(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_a_from_b(
        s,
        |s: EndpointAVideo<i32>| {
            let (request, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c(request + 1, s);

            close_mpst(s)
        },
        close_mpst,
    )
}

fn authenticator(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_c_from_b(s)?;
    let s = send_mpst_c_to_b(id + 1, s);

    offer_mpst_session_to_c_from_b(
        s,
        |s: EndpointCVideo<i32>| {
            let (request, s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_a(request + 1, s);
            let (video, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b(video + 1, s);

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            close_mpst(s)
        },
        close_mpst,
    )
}

fn client_video(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_b_to_c(id, s);
    let (accept, s) = recv_mpst_b_from_c(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_left_mpst_session_b_to_all::<
        CtoAVideo<i32>,
        CtoAClose,
        BtoAClose,
        BtoCVideo<i32>,
        BtoAClose,
        CtoBClose,
        StackAVideoDual,
        StackAEndDual,
        StackCVideoDual,
        StackCEndDual,
        StackBVideo,
        StackBEnd,
    >(s);

    let s = send_mpst_b_to_c(accept, s);
    let (result, s) = recv_mpst_b_from_c(s)?;

    assert_eq!(result, accept + 3);

    close_mpst(s)
}

fn client_close(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_b_to_c(id, s);
    let (accept, s) = recv_mpst_b_from_c(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_right_mpst_session_b_to_all::<
        CtoAVideo<i32>,
        CtoAClose,
        BtoAClose,
        BtoCVideo<i32>,
        BtoAClose,
        CtoBClose,
        StackAVideoDual,
        StackAEndDual,
        StackCVideoDual,
        StackCEndDual,
        StackBVideo,
        StackBEnd,
    >(s);

    close_mpst(s)
}

/////////////////////////////////////////

pub fn run_b_usecase_left() {
    // Test video branch.
    let (thread_a, thread_b, thread_c) = fork_mpst(server, client_video, authenticator);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}

pub fn run_b_usecase_right() {
    // Test end branch.
    let (thread_a, thread_b, thread_c) = fork_mpst(server, client_close, authenticator);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}

pub fn run_b_usecase_checker() {
    let (graphs, kmc) = checker_concat!(
        "",
        EndpointAFull<i32>,
        EndpointBFull<i32>,
        EndpointCFull<i32>
    )
    .unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_a)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.2\\\"\" ]\n    \
            3 [ label = \"\\\"0.3\\\"\" ]\n    \
            4 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleA?RoleC: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"0\\\"\" ]\n\
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
            6 [ label = \"\\\"2.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleB!RoleC: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleB!RoleC: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 6 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_c)),
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
            0 -> 1 [ label = \"\\\"RoleC?RoleB: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleC?RoleB: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleC!RoleA: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 8 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(kmc, None);
}
