use std::boxed::Box;
use std::error::Error;

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use mpstthree::role::a::RoleA;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

use mpstthree::checker_concat;

use petgraph::dot::Dot;

// Test a simple storage server, implemented using binary
// choice. Simple types
type AtoBNeg<N> = Recv<N, End>;
type AtoBAdd<N> = Recv<N, End>;

type AtoCNeg<N> = Send<N, End>;
type AtoCAdd<N> = Send<N, End>;

type BtoANeg<N> = <AtoBNeg<N> as Session>::Dual;
type BtoAAdd<N> = <AtoBAdd<N> as Session>::Dual;

type CtoANeg<N> = <AtoCNeg<N> as Session>::Dual;
type CtoAAdd<N> = <AtoCAdd<N> as Session>::Dual;

// Stacks
type StackOfferA = RoleB<RoleC<RoleEnd>>;
type StackOfferADual = <StackOfferA as Role>::Dual;
type StackFullA = RoleAlltoB<RoleEnd, RoleEnd>;

type StackChoiceB = RoleA<RoleEnd>;
type StackFullB = RoleBtoAll<StackChoiceB, StackChoiceB>;

type StackOfferC = RoleA<RoleEnd>;
type StackOfferCDual = <StackOfferC as Role>::Dual;
type StackFullC = RoleAlltoB<RoleEnd, RoleEnd>;

// Creating the MP sessions
// For A
type EndpointAAdd<N> = MeshedChannels<AtoBAdd<N>, AtoCAdd<N>, StackOfferA, NameA>;
type EndpointANeg<N> = MeshedChannels<AtoBNeg<N>, AtoCNeg<N>, StackOfferA, NameA>;

type OfferA<N> =
    OfferMpst<AtoBAdd<N>, AtoCAdd<N>, AtoBNeg<N>, AtoCNeg<N>, StackOfferA, StackOfferA, NameA>;
type EndpointChoiceA<N> = MeshedChannels<OfferA<N>, End, StackFullA, NameA>;

// For B
type ChooseBtoA<N> = ChooseMpst<
    BtoAAdd<N>,
    CtoAAdd<N>,
    BtoANeg<N>,
    CtoANeg<N>,
    StackOfferADual,
    StackOfferADual,
    NameA,
>;
type ChooseBtoC<N> =
    ChooseMpst<AtoCAdd<N>, End, AtoCNeg<N>, End, StackOfferCDual, StackOfferCDual, NameC>;
type EndpointChoiceB<N> = MeshedChannels<ChooseBtoA<N>, ChooseBtoC<N>, StackFullB, NameB>;

// For C
type EndpointCAdd<N> = MeshedChannels<CtoAAdd<N>, End, StackOfferC, NameC>;
type EndpointCNeg<N> = MeshedChannels<CtoANeg<N>, End, StackOfferC, NameC>;

type OfferC<N> = OfferMpst<CtoAAdd<N>, End, CtoANeg<N>, End, StackOfferC, StackOfferC, NameC>;
type EndpointChoiceC<N> = MeshedChannels<End, OfferC<N>, StackFullC, NameC>;

// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointAAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 1);

            s.send(x + 1).close()
        },
        |s: EndpointANeg<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.send(x + 1).close()
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_left().send(1).close()
}

fn simple_store_client_right(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_right().send(2).close()
}

fn simple_store_pawn(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointCAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.close()
        },
        |s: EndpointCNeg<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 3);

            s.close()
        },
    )
}

/////////////////////////////////////////

pub fn double_choice_left() {
    // Test the left branch.
    let (thread_a, thread_b, thread_c) = fork_mpst(
        simple_store_server,
        simple_store_client_left,
        simple_store_pawn,
    );

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}

pub fn double_choice_right() {
    // Test the right branch.
    let (thread_a, thread_b, thread_c) = fork_mpst(
        simple_store_server,
        simple_store_client_right,
        simple_store_pawn,
    );

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}

pub fn double_choice_checker() {
    let (graphs, kmc) = checker_concat!(
        EndpointChoiceB<i32>,
        EndpointChoiceC<i32>,
        EndpointChoiceA<i32>
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
            5 [ label = \"\\\"0.2\\\"\" ]\n    \
            6 [ label = \"\\\"0.3\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"0\\\"\" ]\n\
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
            0 -> 1 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
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
            3 [ label = \"\\\"0.1\\\"\" ]\n    \
            4 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(kmc, None);
}
