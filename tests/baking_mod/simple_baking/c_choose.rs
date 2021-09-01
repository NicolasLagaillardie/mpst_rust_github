use std::boxed::Box;
use std::error::Error;

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

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

use petgraph::dot::Dot;

/// Test a simple storage server, implemented using binary
/// choice. Simple types
type BtoCNeg<N> = Recv<N, End>;
type BtoCAdd<N> = Recv<N, End>;

type BtoANeg<N> = Send<N, End>;
type BtoAAdd<N> = Send<N, End>;

type CtoBNeg<N> = <BtoCNeg<N> as Session>::Dual;
type CtoBAdd<N> = <BtoCAdd<N> as Session>::Dual;

type AtoBNeg<N> = <BtoANeg<N> as Session>::Dual;
type AtoBAdd<N> = <BtoAAdd<N> as Session>::Dual;

/// Stacks
type StackOfferB = RoleC<RoleA<RoleEnd>>;
type StackOfferBDual = <StackOfferB as Role>::Dual;
type StackFullB = RoleAlltoC<RoleEnd, RoleEnd>;

type StackChoiceC = RoleB<RoleEnd>;
type StackFullC = RoleCtoAll<StackChoiceC, StackChoiceC>;

type StackOfferA = RoleB<RoleEnd>;
type StackOfferADual = <StackOfferA as Role>::Dual;
type StackFullA = RoleAlltoC<RoleEnd, RoleEnd>;

/// Creating the MP sessions
/// For B
type EndpointBAdd<N> = MeshedChannels<BtoAAdd<N>, BtoCAdd<N>, StackOfferB, RoleB<RoleEnd>>;
type EndpointBNeg<N> = MeshedChannels<BtoANeg<N>, BtoCNeg<N>, StackOfferB, RoleB<RoleEnd>>;

type OfferB<N> = OfferMpst<
    BtoAAdd<N>,
    BtoCAdd<N>,
    BtoANeg<N>,
    BtoCNeg<N>,
    StackOfferB,
    StackOfferB,
    RoleB<RoleEnd>,
>;
type EndpointChoiceB<N> = MeshedChannels<End, OfferB<N>, StackFullB, RoleB<RoleEnd>>;

/// For C
type ChooseCtoB<N> = ChooseMpst<
    AtoBAdd<N>,
    CtoBAdd<N>,
    AtoBNeg<N>,
    CtoBNeg<N>,
    StackOfferBDual,
    StackOfferBDual,
    RoleBDual<RoleEnd>,
>;
type ChooseCtoA<N> = ChooseMpst<
    BtoAAdd<N>,
    End,
    BtoANeg<N>,
    End,
    StackOfferADual,
    StackOfferADual,
    RoleADual<RoleEnd>,
>;
type EndpointChoiceC<N> = MeshedChannels<ChooseCtoA<N>, ChooseCtoB<N>, StackFullC, RoleC<RoleEnd>>;

/// For A
type EndpointAAdd<N> = MeshedChannels<AtoBAdd<N>, End, StackOfferA, RoleA<RoleEnd>>;
type EndpointANeg<N> = MeshedChannels<AtoBNeg<N>, End, StackOfferA, RoleA<RoleEnd>>;

type OfferA<N> =
    OfferMpst<AtoBAdd<N>, End, AtoBNeg<N>, End, StackOfferA, StackOfferA, RoleA<RoleEnd>>;
type EndpointChoiceA<N> = MeshedChannels<End, OfferA<N>, StackFullA, RoleA<RoleEnd>>;

/// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointBAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 1);

            s.send(x + 1).close()
        },
        |s: EndpointBNeg<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.send(x + 1).close()
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_left().send(1).close()
}

fn simple_store_client_right(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_right().send(2).close()
}

fn simple_store_pawn(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointAAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.close()
        },
        |s: EndpointANeg<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 3);

            s.close()
        },
    )
}

/////////////////////////////////////////

pub fn double_choice() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test the left branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_pawn,
                simple_store_server,
                simple_store_client_left,
            );

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        // Test the right branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_pawn,
                simple_store_server,
                simple_store_client_right,
            );

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn double_choice_checker() {
    let graphs = mpstthree::checker_concat!(
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
            0 -> 1 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
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
            3 [ label = \"\\\"0.3\\\"\" ]\n    \
            4 [ label = \"\\\"0.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.2\\\"\" ]\n    \
            6 [ label = \"\\\"0.3\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 4 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"0\\\"\" ]\n\
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
            0 -> 1 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            0 -> 3 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );
}
