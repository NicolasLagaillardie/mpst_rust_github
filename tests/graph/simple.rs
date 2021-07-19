use std::boxed::Box;
use std::error::Error;

use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use petgraph::dot::Dot;
use petgraph::Graph;

/// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

type BtoA<N> = <AtoB<N> as Session>::Dual;
type BtoC<N> = Send<N, End>;

type CtoA<N> = <AtoC<N> as Session>::Dual;
type CtoB<N> = <BtoC<N> as Session>::Dual;

/// Stacks
type StackA = RoleB<RoleC<RoleEnd>>;
type StackB = RoleA<RoleC<RoleEnd>>;
type StackC = RoleA<RoleB<RoleEnd>>;

/// Creating the MP sessions
type EndpointA<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, RoleA<RoleEnd>>;
type EndpointB<N> = MeshedChannels<BtoA<N>, BtoC<N>, StackB, RoleB<RoleEnd>>;
type EndpointC<N> = MeshedChannels<CtoA<N>, CtoB<N>, StackC, RoleC<RoleEnd>>;

/// Single test for A
fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let mut g = Graph::new();
    let first_node = g.add_node(String::from("0"));

    let (s, g, new_node) = s.dot_send(1, g, first_node);

    println!("G 1 A: {:?}", Dot::new(&g));

    let (x, s, g, _new_node) = s.dot_recv(g, new_node)?;

    println!("G 2 A: {:?}", Dot::new(&g));

    assert_eq!(x, 3);

    s.close()
}

/// Single test for B
fn endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    let mut g = Graph::new();
    let first_node = g.add_node(String::from("0"));

    let (x, s, g, new_node) = s.dot_recv(g, first_node)?;

    println!("G 1 B: {:?}", Dot::new(&g));

    assert_eq!(x, 1);

    let (s, g, _new_node) = s.dot_send(2, g, new_node);

    println!("G 2 B: {:?}", Dot::new(&g));

    s.close()
}

/// Single test for C
fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let mut g = Graph::new();
    let first_node = g.add_node(String::from("0"));

    let (s, g, new_node) = s.dot_send(3, g, first_node);

    println!("G 1 C: {:?}", Dot::new(&g));

    let (x, s, g, _new_node) = s.dot_recv(g, new_node)?;

    println!("G 2 C: {:?}", Dot::new(&g));

    assert_eq!(x, 2);

    s.close()
}

/////////////////////////////////////////

pub fn simple_triple_endpoints() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
