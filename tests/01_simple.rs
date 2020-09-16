extern crate mpstthree;
use mpstthree::checking::checker;

use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::fork_mpst;
use mpstthree::sessionmpst::SessionMpst;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
use mpstthree::functionmpst::recv::recv_mpst_b_to_a;
use mpstthree::functionmpst::recv::recv_mpst_c_to_b;

use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_a;

/// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

type BtoA<N> = <AtoB<N> as Session>::Dual;
type BtoC<N> = Send<N, End>;

type CtoA<N> = <AtoC<N> as Session>::Dual;
type CtoB<N> = <BtoC<N> as Session>::Dual;

/// Queues
type QueueA = RoleB<RoleC<RoleEnd>>;
type QueueB = RoleA<RoleC<RoleEnd>>;
type QueueC = RoleA<RoleB<RoleEnd>>;

/// Creating the MP sessions
type EndpointA<N> = SessionMpst<AtoB<N>, AtoC<N>, QueueA, RoleA<RoleEnd>>;
type EndpointB<N> = SessionMpst<BtoA<N>, BtoC<N>, QueueB, RoleB<RoleEnd>>;
type EndpointC<N> = SessionMpst<CtoA<N>, CtoB<N>, QueueC, RoleC<RoleEnd>>;

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
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_triple_endpoint_a,
                simple_triple_endpoint_b,
                simple_triple_endpoint_c,
            );

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
#[test]
fn simple_triple_endpoints_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let hm: HashMap<String, &Vec<String>> = HashMap::new();

            let (s1, _): (EndpointA<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointB<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointC<i32>, _) = SessionMpst::new();

            let (a, b, c) = checker(s1, s2, s3, &hm)?;

            assert_eq!(a, "A: A!B.A?C.0");
            assert_eq!(b, "B: B?A.B!C.0");
            assert_eq!(c, "C: C!A.C?B.0");
        }
        Ok(())
    }()
    .is_ok());
}
