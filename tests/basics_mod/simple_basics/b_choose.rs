use std::boxed::Box;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use mpstthree::checker_concat;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_from_b;
use mpstthree::functionmpst::recv::recv_mpst_c_from_a;

use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_a;

use mpstthree::functionmpst::offer::offer_mpst_session_to_a_from_b;
use mpstthree::functionmpst::offer::offer_mpst_session_to_c_from_b;

use mpstthree::functionmpst::choose::choose_left_mpst_session_b_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_b_to_all;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

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
type EndpointAAdd<N> = MeshedChannels<AtoBAdd<N>, AtoCAdd<N>, StackOfferA, RoleA<RoleEnd>>;
type EndpointANeg<N> = MeshedChannels<AtoBNeg<N>, AtoCNeg<N>, StackOfferA, RoleA<RoleEnd>>;

type OfferA<N> = OfferMpst<
    AtoBAdd<N>,
    AtoCAdd<N>,
    AtoBNeg<N>,
    AtoCNeg<N>,
    StackOfferA,
    StackOfferA,
    RoleA<RoleEnd>,
>;
type EndpointChoiceA<N> = MeshedChannels<OfferA<N>, End, StackFullA, RoleA<RoleEnd>>;

// For B
type ChooseBtoA<N> = ChooseMpst<
    BtoAAdd<N>,
    CtoAAdd<N>,
    BtoANeg<N>,
    CtoANeg<N>,
    StackOfferADual,
    StackOfferADual,
    RoleADual<RoleEnd>,
>;
type ChooseBtoC<N> = ChooseMpst<
    AtoCAdd<N>,
    End,
    AtoCNeg<N>,
    End,
    StackOfferCDual,
    StackOfferCDual,
    RoleCDual<RoleEnd>,
>;
type EndpointChoiceB<N> = MeshedChannels<ChooseBtoA<N>, ChooseBtoC<N>, StackFullB, RoleB<RoleEnd>>;

// For C
type EndpointCAdd<N> = MeshedChannels<CtoAAdd<N>, End, StackOfferC, RoleC<RoleEnd>>;
type EndpointCNeg<N> = MeshedChannels<CtoANeg<N>, End, StackOfferC, RoleC<RoleEnd>>;

type OfferC<N> =
    OfferMpst<CtoAAdd<N>, End, CtoANeg<N>, End, StackOfferC, StackOfferC, RoleC<RoleEnd>>;
type EndpointChoiceC<N> = MeshedChannels<End, OfferC<N>, StackFullC, RoleC<RoleEnd>>;

// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_a_from_b(
        s,
        |s: EndpointAAdd<i32>| {
            let (x, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_c(x + 1, s);

            assert_eq!(x, 1);

            close_mpst(s)
        },
        |s: EndpointANeg<i32>| {
            let (x, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_c(x + 1, s);

            assert_eq!(x, 2);

            close_mpst(s)
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    let s = choose_left_mpst_session_b_to_all::<
        CtoAAdd<i32>,
        CtoANeg<i32>,
        BtoAAdd<i32>,
        End,
        BtoANeg<i32>,
        End,
        StackOfferADual,
        StackOfferADual,
        StackOfferCDual,
        StackOfferCDual,
        StackChoiceB,
        StackChoiceB,
    >(s);
    let s = send_mpst_b_to_a(1, s);
    close_mpst(s)
}

fn simple_store_client_right(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    let s = choose_right_mpst_session_b_to_all::<
        CtoAAdd<i32>,
        CtoANeg<i32>,
        BtoAAdd<i32>,
        End,
        BtoANeg<i32>,
        End,
        StackOfferADual,
        StackOfferADual,
        StackOfferCDual,
        StackOfferCDual,
        StackChoiceB,
        StackChoiceB,
    >(s);
    let s = send_mpst_b_to_a(2, s);
    close_mpst(s)
}

fn simple_store_pawn(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_c_from_b(
        s,
        |s: EndpointCAdd<i32>| {
            let (x, s) = recv_mpst_c_from_a(s)?;

            assert_eq!(x, 2);

            close_mpst(s)
        },
        |s: EndpointCNeg<i32>| {
            let (x, s) = recv_mpst_c_from_a(s)?;

            assert_eq!(x, 3);

            close_mpst(s)
        },
    )
}

/////////////////////////////////////////

pub fn double_choice() {
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

pub fn double_choice_checker() {
    let graphs = checker_concat!(
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
}
