// A → X → B
// B → Y → C
// C → Z → A

use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use mpstthree::checker_concat;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_to_all::RoleAtoAll;
use mpstthree::role::all_to_a::RoleAlltoA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

use mpstthree::functionmpst::recv::recv_mpst_b_from_c;
use mpstthree::functionmpst::recv::recv_mpst_c_from_a;

use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::functionmpst::offer::offer_mpst_session_to_b_from_a;
use mpstthree::functionmpst::offer::offer_mpst_session_to_c_from_a;

use mpstthree::functionmpst::choose::choose_left_mpst_session_a_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_a_to_all;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

use petgraph::dot::Dot;

// Test a simple storage server, implemented using binary
// choice. Simple types
type CtoANeg<N> = Recv<N, End>;
type CtoAAdd<N> = Recv<N, End>;

type CtoBNeg<N> = Send<N, End>;
type CtoBAdd<N> = Send<N, End>;

type AtoCNeg<N> = <CtoANeg<N> as Session>::Dual;
type AtoCAdd<N> = <CtoAAdd<N> as Session>::Dual;

type BtoCNeg<N> = <CtoBNeg<N> as Session>::Dual;
type BtoCAdd<N> = <CtoBAdd<N> as Session>::Dual;

// Stacks
type StackOfferC = RoleA<RoleB<RoleEnd>>;
type StackOfferCDual = <StackOfferC as Role>::Dual;
type StackFullC = RoleAlltoA<RoleEnd, RoleEnd>;

type StackChoiceA = RoleC<RoleEnd>;
type StackFullA = RoleAtoAll<StackChoiceA, StackChoiceA>;

type StackOfferB = RoleC<RoleEnd>;
type StackOfferBDual = <StackOfferB as Role>::Dual;
type StackFullB = RoleAlltoA<RoleEnd, RoleEnd>;

// Creating the MP sessions
// For C
type EndpointCAdd<N> = MeshedChannels<CtoAAdd<N>, CtoBAdd<N>, StackOfferC, NameC>;
type EndpointCNeg<N> = MeshedChannels<CtoANeg<N>, CtoBNeg<N>, StackOfferC, NameC>;

type OfferC<N> =
    OfferMpst<CtoAAdd<N>, CtoBAdd<N>, CtoANeg<N>, CtoBNeg<N>, StackOfferC, StackOfferC, NameC>;
type EndpointChoiceC<N> = MeshedChannels<OfferC<N>, End, StackFullC, NameC>;

// For A
type ChooseAtoC<N> = ChooseMpst<
    AtoCAdd<N>,
    BtoCAdd<N>,
    AtoCNeg<N>,
    BtoCNeg<N>,
    StackOfferCDual,
    StackOfferCDual,
    NameC,
>;
type ChooseCtoA<N> =
    ChooseMpst<End, CtoBAdd<N>, End, CtoBNeg<N>, StackOfferBDual, StackOfferBDual, NameB>;
type EndpointChoiceA<N> = MeshedChannels<ChooseCtoA<N>, ChooseAtoC<N>, StackFullA, NameA>;

// For B
type EndpointBAdd<N> = MeshedChannels<End, BtoCAdd<N>, StackOfferB, NameB>;
type EndpointBNeg<N> = MeshedChannels<End, BtoCNeg<N>, StackOfferB, NameB>;

type OfferA<N> = OfferMpst<End, BtoCAdd<N>, End, BtoCNeg<N>, StackOfferB, StackOfferB, NameB>;
type EndpointChoiceB<N> = MeshedChannels<OfferA<N>, End, StackFullB, NameB>;

// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_c_from_a(
        s,
        |s: EndpointCAdd<i32>| {
            let (x, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b(x + 1, s);

            assert_eq!(x, 1);

            close_mpst(s)
        },
        |s: EndpointCNeg<i32>| {
            let (x, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b(x + 1, s);

            assert_eq!(x, 2);

            close_mpst(s)
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    let s = choose_left_mpst_session_a_to_all::<
        CtoBAdd<i32>,
        CtoBNeg<i32>,
        End,
        AtoCAdd<i32>,
        End,
        AtoCNeg<i32>,
        StackOfferBDual,
        StackOfferBDual,
        StackOfferCDual,
        StackOfferCDual,
        StackChoiceA,
        StackChoiceA,
    >(s);
    let s = send_mpst_a_to_c(1, s);
    close_mpst(s)
}

fn simple_store_client_right(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    let s = choose_right_mpst_session_a_to_all::<
        CtoBAdd<i32>,
        CtoBNeg<i32>,
        End,
        AtoCAdd<i32>,
        End,
        AtoCNeg<i32>,
        StackOfferBDual,
        StackOfferBDual,
        StackOfferCDual,
        StackOfferCDual,
        StackChoiceA,
        StackChoiceA,
    >(s);
    let s = send_mpst_a_to_c(2, s);
    close_mpst(s)
}

fn simple_store_pawn(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_b_from_a(
        s,
        |s: EndpointBAdd<i32>| {
            let (x, s) = recv_mpst_b_from_c(s)?;

            assert_eq!(x, 2);

            close_mpst(s)
        },
        |s: EndpointBNeg<i32>| {
            let (x, s) = recv_mpst_b_from_c(s)?;

            assert_eq!(x, 3);

            close_mpst(s)
        },
    )
}

/////////////////////////////////////////

pub fn double_choice_left() {
    // Test the left branch.
    let (thread_a, thread_b, thread_c) = fork_mpst(
        simple_store_client_left,
        simple_store_pawn,
        simple_store_server,
    );

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}

pub fn double_choice_right() {
    // Test the right branch.
    let (thread_a, thread_b, thread_c) = fork_mpst(
        simple_store_client_right,
        simple_store_pawn,
        simple_store_server,
    );

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}

pub fn double_choice_checker() {
    let (graphs, kmc) = checker_concat!(
        "",
        EndpointChoiceA<i32>,
        EndpointChoiceC<i32>,
        EndpointChoiceB<i32>
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
            3 [ label = \"\\\"0.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"0\\\"\" ]\n\
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
            3 [ label = \"\\\"0.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"0\\\"\" ]\n\
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
            5 [ label = \"\\\"0.2\\\"\" ]\n    \
            6 [ label = \"\\\"0.3\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(kmc, None);
}
