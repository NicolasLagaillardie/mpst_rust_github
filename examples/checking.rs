use std::boxed::Box;
use std::error::Error;

use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
use mpstthree::functionmpst::recv::recv_mpst_c_from_b;

use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_a;

// use mpstthree::checking;

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

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Point {
//     fn new() -> Self {
//         Point { x: 0, y: 0 }
//     }
// }

/// Single test for A
fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_b(1, s);
    let (_, s) = recv_mpst_a_from_c(s)?;

    // let (test_struct, _) = EndpointA::<i32>::new();

    // println!("{:?}", test_struct);
    // checking!(s);
    // checking!(test_struct);
    // mpst_seq::checking!(test_struct);
    // mpst_seq::checking!(Point { x: 5, y: 7 });

    close_mpst(s)
}

/// Single test for B
fn endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_b_from_a(s)?;
    let s = send_mpst_b_to_c(2, s);

    close_mpst(s)
}

/// Single test for C
fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_a(3, s);
    let (_, s) = recv_mpst_c_from_b(s)?;

    close_mpst(s)
}

/////////////////////////////////////////

fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}
