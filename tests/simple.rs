extern crate mpst;

use binary::*;
use mpst::*;
use role::*;
use sessionmpst::SessionMpst;
use std::boxed::Box;
use std::error::Error;

use mpst::functionmpst::close::close_mpst;

use mpst::role::a_to_b::RoleAtoB;
use mpst::role::a_to_c::RoleAtoC;
use mpst::role::b_to_a::RoleBtoA;
use mpst::role::b_to_c::RoleBtoC;
use mpst::role::c_to_a::RoleCtoA;
use mpst::role::c_to_b::RoleCtoB;
use mpst::role::end::RoleEnd;

use mpst::functionmpst::recv::recv_mpst_session_one_b_to_a;
use mpst::functionmpst::recv::recv_mpst_session_two_a_to_c;
use mpst::functionmpst::recv::recv_mpst_session_two_c_to_b;

use mpst::functionmpst::send::send_mpst_session_one_a_to_b;
use mpst::functionmpst::send::send_mpst_session_one_c_to_a;
use mpst::functionmpst::send::send_mpst_session_two_b_to_c;

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

/// Queueus
type QueueA = RoleAtoB<RoleAtoC<RoleEnd>>;
type QueueB = RoleBtoA<RoleBtoC<RoleEnd>>;
type QueueC = RoleCtoA<RoleCtoB<RoleEnd>>;

/// Creating the MP sessions
type EndpointA<N> = SessionMpst<AtoB<N>, AtoC<N>, QueueA>;
type EndpointB<N> = SessionMpst<BtoA<N>, BtoC<N>, QueueB>;
type EndpointC<N> = SessionMpst<CtoA<N>, CtoB<N>, QueueC>;

/// Single test for A
fn simple_triple_endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_session_one_a_to_b(1, s);
    let (x, s) = recv_mpst_session_two_a_to_c(s)?;

    assert_eq!(x, 3);

    close_mpst(s)?;

    Ok(())
}

/// Single test for B
fn simple_triple_endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    let (x, s) = recv_mpst_session_one_b_to_a(s)?;
    let s = send_mpst_session_two_b_to_c(2, s);

    assert_eq!(x, 1);

    close_mpst(s)?;

    Ok(())
}

/// Single test for C
fn simple_triple_endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_session_one_c_to_a(3, s);
    let (x, s) = recv_mpst_session_two_c_to_b(s)?;

    assert_eq!(x, 2);

    close_mpst(s)?;

    Ok(())
}

#[test]
fn simple_triple_endpoints() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (channel_ab, channel_ba) = Session::new();
            let (channel_ac, channel_ca) = Session::new();
            let (channel_bc, channel_cb) = Session::new();

            let (role_a, _) = Role::new();
            let (role_b, _) = Role::new();
            let (role_c, _) = Role::new();

            let a: EndpointA<i32> = SessionMpst {
                session1: channel_ab,
                session2: channel_ac,
                queue: role_a,
            };
            let b: EndpointB<i32> = SessionMpst {
                session1: channel_ba,
                session2: channel_bc,
                queue: role_b,
            };
            let c: EndpointC<i32> = SessionMpst {
                session1: channel_ca,
                session2: channel_cb,
                queue: role_c,
            };

            let thread_a = fork_simple(simple_triple_endpoint_a, a);
            let thread_b = fork_simple(simple_triple_endpoint_b, b);
            let thread_c = fork_simple(simple_triple_endpoint_c, c);

            thread_a.join().unwrap();
            thread_b.join().unwrap();
            thread_c.join().unwrap();
        }
        Ok(())
    }()
    .is_ok());
}
