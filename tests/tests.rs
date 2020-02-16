extern crate mpst;

use mpst::*;
use std::boxed::Box;
use std::error::Error;
use sessionmpst::SessionMpst;
use binary::*;
use role::*;

use mpst::functionmpst::close::close_mpst;

use mpst::role::a_to_b::RoleAtoB;
use mpst::role::a_to_c::RoleAtoC;
use mpst::role::b_to_a::RoleBtoA;
use mpst::role::b_to_c::RoleBtoC;
use mpst::role::c_to_a::RoleCtoA;
use mpst::role::c_to_b::RoleCtoB;
use mpst::role::end::RoleEnd;

use mpst::functionmpst::recv::recv_mpst_session_two_c_to_b;
use mpst::functionmpst::send::send_mpst_session_one_c_to_a;
use mpst::functionmpst::send::send_mpst_session_two_b_to_c;
use mpst::functionmpst::recv::recv_mpst_session_one_b_to_a;
use mpst::functionmpst::recv::recv_mpst_session_two_a_to_c;
use mpst::functionmpst::send::send_mpst_session_one_a_to_b;

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
    {
        let s = send_mpst_session_one_a_to_b(1, s);
        let (x, s) = recv_mpst_session_two_a_to_c(s)?;
        close_mpst(s)?;

	assert_eq!(x, 3);
    }
    Ok(())
}

/// Single test for B
fn simple_triple_endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    {
        let (x, s) = recv_mpst_session_one_b_to_a(s)?;
        let s = send_mpst_session_two_b_to_c(2, s);
        close_mpst(s)?;

	assert_eq!(x, 1);
    }
    Ok(())
}

/// Single test for C
fn simple_triple_endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    {
        let s = send_mpst_session_one_c_to_a(3, s);
        let (x, s) = recv_mpst_session_two_c_to_b(s)?;
        close_mpst(s)?;

	assert_eq!(x, 2);
    }
    Ok(())
}

#[test]
fn main() {
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

    thread_a.join();
    thread_b.join();
    thread_c.join();
}

//// Test a simple calculator server, implemented using binary choice.
///// Simple types
//type AtoBNeg<N> = Recv<N, End>;
//type BtoANeg<N> = Send<N, End>;
//
//type AtoBAdd<N> = Recv<N, End>;
//type BtoAAdd<N> = Send<N, End>;
//
//type SimpleCalcServer<N> = Offer<AtoBNeg<N>, AtoBAdd<N>>;
//type SimpleCalcClient<N> = <SimpleCalcServer<N> as Session>::Dual;
//
///// Queues
//
//type QueueA = RoleAtoB<RoleAtoB<RoleEnd>>;
//type QueueB = RoleBtoA<RoleBtoA<RoleEnd>>;
//type QueueC = RoleEnd;
//
///// Creating the MP sessions
//type EndpointAAdd<N> = SessionMpst<AtoBNeg<N>, End, QueueA>;
//type EndpointANeg<N> = SessionMpst<AtoBAdd<N>, End, QueueA>;
//type EndpointA<N> = Offer<EndpointAAdd<N>, EndpointANeg<N>>;
//
//type EndpointB<N> = SessionMpst<SimpleCalcServer<N>, End, QueueB>;
//type EndpointC<N> = SessionMpst<End, End, QueueC>;
//
//fn simple_calc_server(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
//    offer_either(
//        s,
//        |s: AtoBNeg<i32>| {
//            close(s)?;
//            Ok(())
//        },
//        |s: AtoBAdd<i32>| {
//            let (x, s) = recv_mpst_session_one_B_to_A(s)?;
//            close(s)?;
//            Ok(())
//        },
//    )
//}
//
//#[test]
//fn simple_calc_works() {
//    assert!(|| -> Result<(), Box<dyn Error>> {
//        let mut rng = thread_rng();
//
//        // Test the negation function.
//        {
//            let s: SimpleCalcClient<i32> = fork(simple_calc_server);
//            let x: i32 = rng.gen();
//            let s = choose_left::<_, AddClient<i32>>(s);
//            let s = send(x, s);
//            let (y, s) = recv(s)?;
//            close(s)?;
//            assert_eq!(-x, y);
//        }
//
//        // Test the addition function.
//        {
//            let s: SimpleCalcClient<i32> = fork(simple_calc_server);
//            let x: i32 = rng.gen();
//            let y: i32 = rng.gen();
//            let s = choose_right::<NegClient<i32>, _>(s);
//            let s = send(x, s);
//            let s = send(y, s);
//            let (z, s) = recv(s)?;
//            close(s)?;
//            assert_eq!(x.wrapping_add(y), z);
//        }
//
//        Ok(())
//    }()
//    .is_ok());
//}
//
//// Test a nice calculator server, implemented using variant types.
//
//enum CalcOp<N: marker::Send> {
//    Neg(NegServer<N>),
//    Add(AddServer<N>),
//}
//type NiceCalcServer<N> = Recv<CalcOp<N>, End>;
//type NiceCalcClient<N> = <NiceCalcServer<N> as Session>::Dual;
//
//fn nice_calc_server(s: NiceCalcServer<i32>) -> Result<(), Box<dyn Error>> {
//    offer!(s, {
//        CalcOp::Neg(s) => {
//            let (x, s) = recv(s)?;
//            let s = send(-x, s);
//            close(s)?;
//            Ok(())
//        },
//        CalcOp::Add(s) => {
//            let (x, s) = recv(s)?;
//            let (y, s) = recv(s)?;
//            let s = send(x.wrapping_add(y), s);
//            close(s)?;
//            Ok(())
//        },
//    })
//}
//
//#[test]
//fn nice_calc_works() {
//    assert!(|| -> Result<(), Box<dyn Error>> {
//        // Pick some random numbers.
//        let mut rng = thread_rng();
//
//        // Test the negation function.
//        {
//            let s: NiceCalcClient<i32> = fork(nice_calc_server);
//            let x: i32 = rng.gen();
//            let s = choose!(CalcOp::Neg, s);
//            let s = send(x, s);
//            let (y, s) = recv(s)?;
//            close(s)?;
//            assert_eq!(-x, y);
//        }
//
//        // Test the addition function.
//        {
//            let s: NiceCalcClient<i32> = fork(nice_calc_server);
//            let x: i32 = rng.gen();
//            let y: i32 = rng.gen();
//            let s = choose!(CalcOp::Add, s);
//            let s = send(x, s);
//            let s = send(y, s);
//            let (z, s) = recv(s)?;
//            close(s)?;
//            assert_eq!(x.wrapping_add(y), z);
//        }
//
//        Ok(())
//    }()
//    .is_ok());
//}
