use std::error::Error;

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;

use mpstthree::checker_concat;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

use petgraph::dot::Dot;

// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

type BtoA<N> = <AtoB<N> as Session>::Dual;
type BtoC<N> = Send<N, End>;

type CtoA<N> = <AtoC<N> as Session>::Dual;
type CtoB<N> = <BtoC<N> as Session>::Dual;

// Stacks
type StackA = RoleB<RoleC<RoleEnd>>;
type StackB = RoleA<RoleC<RoleEnd>>;
type StackC = RoleA<RoleB<RoleEnd>>;

// Creating the MP sessions
type EndpointA<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, NameA>;
type EndpointB<N> = MeshedChannels<BtoA<N>, BtoC<N>, StackB, NameB>;
type EndpointC<N> = MeshedChannels<CtoA<N>, CtoB<N>, StackC, NameC>;

// Single test for A
fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let (x, s) = s.send(1).recv()?;

    assert_eq!(x, 3);

    s.close()
}

// Single test for B
fn endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    let (x, s) = s.recv()?;

    assert_eq!(x, 1);

    s.send(2).close()
}

// Single test for C
fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let (x, s) = s.send(3).recv()?;

    assert_eq!(x, 2);

    s.close()
}

/////////////////////////////////////////

pub fn simple_triple_endpoints() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
}

pub fn simple_triple_endpoints_checker() {
    let (graphs, kmc) =
        checker_concat!("", EndpointB<i32>, EndpointC<i32>, EndpointA<i32>).unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_a)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"1\\\"\" ]\n    \
            2 [ label = \"\\\"2\\\"\" ]\n    \
            3 [ label = \"\\\"3\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleA!RoleB: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA?RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n\
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
            3 [ label = \"\\\"3\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleB?RoleA: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB!RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n\
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
            3 [ label = \"\\\"3\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleC!RoleA: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleC?RoleB: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test KMC output
    assert_eq!(kmc, None);
}
