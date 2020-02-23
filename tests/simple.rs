extern crate mpst;

use std::boxed::Box;
use std::error::Error;

use mpst::binary::{End, Recv, Send};
use mpst::run_processes;
use mpst::sessionmpst::SessionMpst;

use mpst::functionmpst::close::close_mpst;

use mpst::role::a_to_b::RoleAtoB;
use mpst::role::a_to_c::RoleAtoC;
use mpst::role::b_to_a::RoleBtoA;
use mpst::role::b_to_c::RoleBtoC;
use mpst::role::c_to_a::RoleCtoA;
use mpst::role::c_to_b::RoleCtoB;
use mpst::role::end::RoleEnd;

use mpst::functionmpst::recv::recv_mpst_a_to_c;
use mpst::functionmpst::recv::recv_mpst_b_to_a;
use mpst::functionmpst::recv::recv_mpst_c_to_b;

use mpst::functionmpst::send::send_mpst_a_to_b;
use mpst::functionmpst::send::send_mpst_b_to_c;
use mpst::functionmpst::send::send_mpst_c_to_a;

/// A = !B.?C
/// B = ?A.!C
/// C = !A.?B

/// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

type BtoA<N> = Recv<N, End>;
type BtoC<N> = Send<N, End>;

type CtoA<N> = Send<N, End>;
type CtoB<N> = Recv<N, End>;

/// Queues
type QueueA = RoleAtoB<RoleAtoC<RoleEnd>>;
type QueueB = RoleBtoA<RoleBtoC<RoleEnd>>;
type QueueC = RoleCtoA<RoleCtoB<RoleEnd>>;

/// Creating the MP sessions
type EndpointA<N> = SessionMpst<AtoB<N>, AtoC<N>, QueueA>;
type EndpointB<N> = SessionMpst<BtoA<N>, BtoC<N>, QueueB>;
type EndpointC<N> = SessionMpst<CtoA<N>, CtoB<N>, QueueC>;

/// Single test for A
fn simple_triple_endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_b(1, s);
    let (x, s) = recv_mpst_a_to_c(s)?;

    assert_eq!(x, 3);

    close_mpst(s)?;

    Ok(())
}

/// Single test for B
fn simple_triple_endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    let (x, s) = recv_mpst_b_to_a(s)?;
    let s = send_mpst_b_to_c(2, s);

    assert_eq!(x, 1);

    close_mpst(s)?;

    Ok(())
}

/// Single test for C
fn simple_triple_endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_a(3, s);
    let (x, s) = recv_mpst_c_to_b(s)?;

    assert_eq!(x, 2);

    close_mpst(s)?;

    Ok(())
}

#[test]
fn simple_triple_endpoints() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c) = run_processes(
                simple_triple_endpoint_a,
                simple_triple_endpoint_b,
                simple_triple_endpoint_c,
            );

            thread_a.unwrap();
            thread_b.unwrap();
            thread_c.unwrap();
        }
        Ok(())
    }()
    .is_ok());
}
