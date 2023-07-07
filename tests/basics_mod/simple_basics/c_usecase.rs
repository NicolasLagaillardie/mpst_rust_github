use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use mpstthree::checker_concat;

use std::boxed::Box;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::all_to_c::RoleAlltoC;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_to_all::RoleCtoAll;
use mpstthree::role::end::RoleEnd;

use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
use mpstthree::functionmpst::recv::recv_mpst_c_from_a;

use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_a;

use mpstthree::functionmpst::offer::offer_mpst_session_to_a_from_c;
use mpstthree::functionmpst::offer::offer_mpst_session_to_b_from_c;

use mpstthree::functionmpst::choose::choose_left_mpst_session_c_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_c_to_all;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

use petgraph::dot::Dot;

// Test our usecase
// Simple types
// Client = C
// Authenticator = A
// Server = B

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
type StackAEndDual = <StackAEnd as Role>::Dual;
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleEnd>>>>;
type StackAVideoDual = <StackAVideo as Role>::Dual;
type StackAFull = RoleC<RoleC<RoleAlltoC<RoleEnd, RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBEndDual = <StackBEnd as Role>::Dual;
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

type OfferA<N> =
    OfferMpst<AtoBVideo<N>, AtoCVideo<N>, AtoBClose, AtoCClose, StackAVideo, StackAEnd, NameA>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAFull, NameA>;

// For B
type EndpointBVideo<N> = MeshedChannels<BtoAVideo<N>, BtoCClose, StackBVideo, NameB>;

type OfferB<N> =
    OfferMpst<BtoAVideo<N>, BtoCClose, BtoAClose, BtoCClose, StackBVideo, StackBEnd, NameB>;
type EndpointBFull<N> = MeshedChannels<End, OfferB<N>, StackBFull, NameB>;

// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_b_from_c(
        s,
        |s: EndpointBVideo<i32>| {
            let (request, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);

            close_mpst(s)
        },
        close_mpst,
    )
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_from_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    offer_mpst_session_to_a_from_c(
        s,
        |s: EndpointAVideo<i32>| {
            let (request, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            close_mpst(s)
        },
        close_mpst,
    )
}

fn client_video(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_c_to_a(id, s);
    let (accept, s) = recv_mpst_c_from_a(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_left_mpst_session_c_to_all::<
        BtoAVideo<i32>,
        BtoAClose,
        CtoAVideo<i32>,
        CtoAClose,
        BtoCClose,
        AtoCClose,
        StackAVideoDual,
        StackAEndDual,
        StackBVideoDual,
        StackBEndDual,
        StackCVideo,
        StackCEnd,
    >(s);

    let s = send_mpst_c_to_a(accept, s);
    let (result, s) = recv_mpst_c_from_a(s)?;

    assert_eq!(result, accept + 3);

    close_mpst(s)
}

fn client_close(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_c_to_a(id, s);
    let (accept, s) = recv_mpst_c_from_a(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_right_mpst_session_c_to_all::<
        BtoAVideo<i32>,
        BtoAClose,
        CtoAVideo<i32>,
        CtoAClose,
        BtoCClose,
        AtoCClose,
        StackAVideoDual,
        StackAEndDual,
        StackBVideoDual,
        StackBEndDual,
        StackCVideo,
        StackCEnd,
    >(s);

    close_mpst(s)
}

/////////////////////////////////////////

pub fn run_c_usecase_left() {
    assert!({
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_video);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok::<(), Box<dyn Error>>(())
    }
    .is_ok());
}

pub fn run_c_usecase_right() {
    assert!({
        // Test end branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_close);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok::<(), Box<dyn Error>>(())
    }
    .is_ok());
}

pub fn run_c_usecase_checker() {
    let (graphs, kmc) =
        checker_concat!(EndpointBFull<i32>, EndpointAFull<i32>, EndpointCFull<i32>).unwrap();

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
            6 [ label = \"\\\"2.4\\\"\" ]\n    \
            7 [ label = \"\\\"2.5\\\"\" ]\n    \
            8 [ label = \"\\\"2.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleA?RoleC: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleA?RoleC: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleA!RoleB: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 8 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_b)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.1\\\"\" ]\n    \
            2 [ label = \"\\\"0.2\\\"\" ]\n    \
            3 [ label = \"\\\"0.3\\\"\" ]\n    \
            4 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleB?RoleA: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"0\\\"\" ]\n\
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
            6 [ label = \"\\\"2.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleC!RoleA: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleC!RoleA: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"0\\\"\" ]\n    \
            2 -> 6 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(kmc, None);
}
