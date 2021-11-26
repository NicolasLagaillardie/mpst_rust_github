// A → X → B
// B → Y → C
// C → Z → A

use std::boxed::Box;
use std::error::Error;

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_to_all::RoleAtoAll;
use mpstthree::role::all_to_a::RoleAlltoA;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_dual::RoleBDual;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::end::RoleEnd;

use mpstthree::checker_concat;

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
type EndpointCAdd<N> = MeshedChannels<CtoAAdd<N>, CtoBAdd<N>, StackOfferC, RoleC<RoleEnd>>;
type EndpointCNeg<N> = MeshedChannels<CtoANeg<N>, CtoBNeg<N>, StackOfferC, RoleC<RoleEnd>>;

type OfferC<N> = OfferMpst<
    CtoAAdd<N>,
    CtoBAdd<N>,
    CtoANeg<N>,
    CtoBNeg<N>,
    StackOfferC,
    StackOfferC,
    RoleC<RoleEnd>,
>;
type EndpointChoiceC<N> = MeshedChannels<OfferC<N>, End, StackFullC, RoleC<RoleEnd>>;

// For A
type ChooseAtoC<N> = ChooseMpst<
    AtoCAdd<N>,
    BtoCAdd<N>,
    AtoCNeg<N>,
    BtoCNeg<N>,
    StackOfferCDual,
    StackOfferCDual,
    RoleCDual<RoleEnd>,
>;
type ChooseCtoA<N> = ChooseMpst<
    End,
    CtoBAdd<N>,
    End,
    CtoBNeg<N>,
    StackOfferBDual,
    StackOfferBDual,
    RoleBDual<RoleEnd>,
>;
type EndpointChoiceA<N> = MeshedChannels<ChooseCtoA<N>, ChooseAtoC<N>, StackFullA, RoleA<RoleEnd>>;

// For B
type EndpointBAdd<N> = MeshedChannels<End, BtoCAdd<N>, StackOfferB, RoleB<RoleEnd>>;
type EndpointBNeg<N> = MeshedChannels<End, BtoCNeg<N>, StackOfferB, RoleB<RoleEnd>>;

type OfferA<N> =
    OfferMpst<End, BtoCAdd<N>, End, BtoCNeg<N>, StackOfferB, StackOfferB, RoleB<RoleEnd>>;
type EndpointChoiceB<N> = MeshedChannels<OfferA<N>, End, StackFullB, RoleB<RoleEnd>>;

// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointCAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 1);

            s.send(x + 1).close()
        },
        |s: EndpointCNeg<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.send(x + 1).close()
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_left().send(1).close()
}

fn simple_store_client_right(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_right().send(2).close()
}

fn simple_store_pawn(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointBAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.close()
        },
        |s: EndpointBNeg<i32>| {
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
                simple_store_client_left,
                simple_store_pawn,
                simple_store_server,
            );

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        // Test the right branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_client_right,
                simple_store_pawn,
                simple_store_server,
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
    let (graphs, kmc) = checker_concat!(
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
