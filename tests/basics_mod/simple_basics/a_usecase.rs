use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use std::boxed::Box;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_to_all::RoleAtoAll;
use mpstthree::role::all_to_a::RoleAlltoA;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_dual::RoleBDual;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
use mpstthree::functionmpst::recv::recv_mpst_b_from_c;
use mpstthree::functionmpst::recv::recv_mpst_c_from_b;

use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::functionmpst::offer::offer_mpst_session_to_b_from_a;
use mpstthree::functionmpst::offer::offer_mpst_session_to_c_from_a;

use mpstthree::functionmpst::choose::choose_left_mpst_session_a_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_a_to_all;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

use petgraph::dot::Dot;

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

// Stacks
type StackBEnd = RoleEnd;
type StackBEndDual = <StackAEnd as Role>::Dual;
type StackBVideo = RoleA<RoleC<RoleC<RoleA<RoleEnd>>>>;
type StackBVideoDual = <StackBVideo as Role>::Dual;
type StackBFull = RoleA<RoleA<RoleAlltoA<RoleEnd, RoleEnd>>>;

type StackCEnd = RoleEnd;
type StackCEndDual = <StackCEnd as Role>::Dual;
type StackCVideo = RoleB<RoleB<RoleEnd>>;
type StackCVideoDual = <StackCVideo as Role>::Dual;
type StackCFull = RoleAlltoA<RoleEnd, RoleEnd>;

type StackAEnd = RoleEnd;
type StackAVideo = RoleB<RoleB<RoleEnd>>;
type StackAChoice = RoleAtoAll<StackAVideo, StackAEnd>;
type StackAFull = RoleB<RoleB<StackAChoice>>;

// Creating the MP sessions
// For A
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

// For B
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

// For C
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

// Functions related to endpoints
fn server(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_c_from_a(
        s,
        |s: EndpointCVideo<i32>| {
            let (request, s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_b(request + 1, s);

            close_mpst(s)
        },
        |s: EndpointCEnd| close_mpst(s),
    )
}

fn authenticator(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_b_from_a(s)?;
    let s = send_mpst_b_to_a(id + 1, s);

    offer_mpst_session_to_b_from_a(
        s,
        |s: EndpointBVideo<i32>| {
            let (request, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c(request + 1, s);
            let (video, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a(video + 1, s);

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            close_mpst(s)
        },
        |s: EndpointBEnd| close_mpst(s),
    )
}

fn client_video(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_a_to_b(id, s);
    let (accept, s) = recv_mpst_a_from_b(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_left_mpst_session_a_to_all::<
        CtoBVideo<i32>,
        CtoBClose,
        AtoBVideo<i32>,
        CtoAClose,
        AtoBClose,
        BtoAClose,
        StackBVideoDual,
        StackBEndDual,
        StackCVideoDual,
        StackCEndDual,
        StackAVideo,
        StackAEnd,
    >(s);

    let s = send_mpst_a_to_b(accept, s);
    let (result, s) = recv_mpst_a_from_b(s)?;

    assert_eq!(result, accept + 3);

    close_mpst(s)
}

fn client_close(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_a_to_b(id, s);
    let (accept, s) = recv_mpst_a_from_b(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_right_mpst_session_a_to_all::<
        CtoBVideo<i32>,
        CtoBClose,
        AtoBVideo<i32>,
        CtoAClose,
        AtoBClose,
        BtoAClose,
        StackBVideoDual,
        StackBEndDual,
        StackCVideoDual,
        StackCEndDual,
        StackAVideo,
        StackAEnd,
    >(s);

    close_mpst(s)
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
    let graphs =
        mpstthree::checker_concat!(EndpointAFull<i32>, EndpointBFull<i32>, EndpointCFull<i32>)
            .unwrap();

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
}
