use std::boxed::Box;
use std::error::Error;

use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

use petgraph::dot::Dot;

/// Test a simple storage server, implemented using binary
/// choice. Simple types
type AtoBNeg<N> = Recv<N, End>;
type AtoBAdd<N> = Recv<N, End>;

// type BtoANeg<N> = Send<N, End>;
// type BtoAAdd<N> = Send<N, End>;
type BtoANeg<N> = <AtoBNeg<N> as Session>::Dual;
type BtoAAdd<N> = <AtoBAdd<N> as Session>::Dual;

/// Stacks
type StackOfferA = RoleB<RoleEnd>;
type StackFullA = RoleAlltoB<RoleEnd, RoleEnd>;

type StackChoiceB = RoleA<RoleEnd>;
type StackFullB = RoleBtoAll<StackChoiceB, StackChoiceB>;

type StackOfferC = RoleEnd;
type StackFullC = RoleAlltoB<StackOfferC, StackOfferC>;

/// Creating the MP sessions
/// For A
type EndpointAAdd<N> = MeshedChannels<AtoBAdd<N>, End, StackOfferA, RoleA<RoleEnd>>;
type EndpointANeg<N> = MeshedChannels<AtoBNeg<N>, End, StackOfferA, RoleA<RoleEnd>>;

type OfferAfromB<N> =
    OfferMpst<AtoBAdd<N>, End, AtoBNeg<N>, End, StackOfferA, StackOfferA, RoleA<RoleEnd>>;
type EndpointChoiceA<N> = MeshedChannels<OfferAfromB<N>, End, StackFullA, RoleA<RoleEnd>>;

/// For B
type ChooseBtoA<N> = ChooseMpst<
    BtoAAdd<N>,
    End,
    BtoANeg<N>,
    End,
    <StackOfferA as Role>::Dual,
    <StackOfferA as Role>::Dual,
    RoleADual<RoleEnd>,
>;
type ChooseBtoC = ChooseMpst<End, End, End, End, RoleEnd, RoleEnd, RoleCDual<RoleEnd>>;
type EndpointChoiceB<N> = MeshedChannels<ChooseBtoA<N>, ChooseBtoC, StackFullB, RoleB<RoleEnd>>;

/// For C
type EndpointCAdd = MeshedChannels<End, End, StackOfferC, RoleC<RoleEnd>>;
type EndpointCNeg = MeshedChannels<End, End, StackOfferC, RoleC<RoleEnd>>;

type OfferCfromB = OfferMpst<End, End, End, End, StackOfferC, StackOfferC, RoleC<RoleEnd>>;
type EndpointChoiceC = MeshedChannels<End, OfferCfromB, StackFullC, RoleC<RoleEnd>>;

/// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointAAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 1);

            s.close()
        },
        |s: EndpointANeg<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.close()
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_left().send(1).close()
}

fn simple_store_client_right(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_right().send(2).close()
}

fn simple_store_pawn(s: EndpointChoiceC) -> Result<(), Box<dyn Error>> {
    s.offer(|s: EndpointCAdd| s.close(), |s: EndpointCNeg| s.close())
}

/////////////////////////////////////////

pub fn simple_choice() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test the left branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_server,
                simple_store_client_left,
                simple_store_pawn,
            );

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        // Test the right branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_server,
                simple_store_client_right,
                simple_store_pawn,
            );

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn simple_choice_checker() {
    let graphs =
        mpstthree::checker_concat!(EndpointChoiceC, EndpointChoiceA<i32>, EndpointChoiceB<i32>)
            .unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_a)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.0\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.2\\\"\" ]\n    \
            4 [ label = \"\\\"0.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"& RoleB\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            1 -> 4 [ label = \"\\\"RoleA?RoleB: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_b)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.0\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.2\\\"\" ]\n    \
            4 [ label = \"\\\"0.1\\\"\" ]\n    \
            5 [ label = \"\\\"0.2\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"+ RoleB\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n    \
            1 -> 4 [ label = \"\\\"RoleB!RoleA: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_c)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.0\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"& RoleB\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"0\\\"\" ]\n    \
            1 -> 3 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );
}