extern crate mpst;

use mpst::*;
use std::boxed::Box;
use std::error::Error;

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
        let s = send_mpst_session_one_A_to_B(1, s);
        let (x, s) = recv_mpst_session_two_A_to_C(s)?;
        // close_mpst(s)?;
    }
    Ok(())
}

/// Single test for B
fn simple_triple_endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    {
        let (x, s) = recv_mpst_session_one_B_to_A(s)?;
        let s = send_mpst_session_two_B_to_C(1, s);
        // close_mpst(s)?;
    }
    Ok(())
}

/// Single test for C
fn simple_triple_endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    {
        let s = send_mpst_session_one_C_to_A(1, s);
        let (x, s) = recv_mpst_session_two_C_to_B(s)?;
        // close_mpst(s)?;
    }
    Ok(())
}

#[test]
fn main() {
    let (channel_ab, channel_ba) = Session::new();
    let (channel_ac, channel_ca) = Session::new();
    let (channel_bc, channel_cb) = Session::new();

    let (roleA, _) = Role::new();
    let (roleB, _) = Role::new();
    let (roleC, _) = Role::new();

    let a: EndpointA<i32> = SessionMpst {
        session1: channel_ab,
        session2: channel_ac,
        queue: roleA,
    };
    let b: EndpointB<i32> = SessionMpst {
        session1: channel_ba,
        session2: channel_bc,
        queue: roleB,
    };
    let c: EndpointC<i32> = SessionMpst {
        session1: channel_ca,
        session2: channel_cb,
        queue: roleC,
    };

    let thread_a = fork_simple(simple_triple_endpoint_a, a);
    let thread_b = fork_simple(simple_triple_endpoint_b, b);
    let thread_c = fork_simple(simple_triple_endpoint_c, c);

    thread_a.join();
    thread_b.join();
    thread_c.join();
}

// Test a simple calculator server, implemented using binary choice.
/// Simple types
type AtoBNeg<N> = Recv<N, End>;
type BtoANeg<N> = Send<N, End>; 

type AtoBAdd<N> = Recv<N, End>;
type BtoAAdd<N> = Send<N, End>; 

type SimpleCalcServer<N> = Offer<AtoBNeg<N>, AtoBAdd<N>>;
type SimpleCalcClient<N> = <SimpleCalcServer<N> as Session>::Dual;

/// Queues

type QueueA = RoleAtoB<RoleAtoB<RoleEnd>>;
type QueueB = RoleBtoA<RoleBtoA<RoleEnd>>;
type QueueC = RoleEnd;

/// Creating the MP sessions
type EndpointA<N> = SessionMpst<SimpleCalcClient<N>, End, QueueA>;
type EndpointB<N> = SessionMpst<SimpleCalcServer<N>, End, QueueB>;
type EndpointC<N> = SessionMpst<End, End, QueueC>;

fn simple_calc_server(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    offer_either(s,
                 |s: AtoBNeg<i32>| {
                     let (x, s) = recv_mpst_session_one_B_to_A(s)?;
                     
                     close(s)?;
                     Ok(())
                 },
                 |s: AtoBAdd<i32>| {
                     let (x, s) = recv_mpst_session_one_B_to_A(s)?;                 
                     close(s)?;
                     Ok(())
                 })
}

#[test]
fn simple_calc_works() {
    assert!(|| -> Result<(), Box<dyn Error>> {

        let mut rng = thread_rng();

        // Test the negation function.
        {
            let s: SimpleCalcClient<i32> = fork(simple_calc_server);
            let x: i32 = rng.gen();
            let s = choose_left::<_, AddClient<i32>>(s);
            let s = send(x, s);
            let (y, s) = recv(s)?;
            close(s)?;
            assert_eq!(-x, y);
        }

        // Test the addition function.
        {
            let s: SimpleCalcClient<i32> = fork(simple_calc_server);
            let x: i32 = rng.gen();
            let y: i32 = rng.gen();
            let s = choose_right::<NegClient<i32>, _>(s);
            let s = send(x, s);
            let s = send(y, s);
            let (z, s) = recv(s)?;
            close(s)?;
            assert_eq!(x.wrapping_add(y), z);
        }

        Ok(())

    }().is_ok());
}


// Test a nice calculator server, implemented using variant types.

enum CalcOp<N: marker::Send> {
    Neg(NegServer<N>),
    Add(AddServer<N>),
}
type NiceCalcServer<N> = Recv<CalcOp<N>, End>;
type NiceCalcClient<N> = <NiceCalcServer<N> as Session>::Dual;

fn nice_calc_server(s: NiceCalcServer<i32>) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        CalcOp::Neg(s) => {
            let (x, s) = recv(s)?;
            let s = send(-x, s);
            close(s)?;
            Ok(())
        },
        CalcOp::Add(s) => {
            let (x, s) = recv(s)?;
            let (y, s) = recv(s)?;
            let s = send(x.wrapping_add(y), s);
            close(s)?;
            Ok(())
        },
    })
}

#[test]
fn nice_calc_works() {
    assert!(|| -> Result<(), Box<dyn Error>> {

        // Pick some random numbers.
        let mut rng = thread_rng();

        // Test the negation function.
        {
            let s: NiceCalcClient<i32> = fork(nice_calc_server);
            let x: i32 = rng.gen();
            let s = choose!(CalcOp::Neg, s);
            let s = send(x, s);
            let (y, s) = recv(s)?;
            close(s)?;
            assert_eq!(-x, y);
        }

        // Test the addition function.
        {
            let s: NiceCalcClient<i32> = fork(nice_calc_server);
            let x: i32 = rng.gen();
            let y: i32 = rng.gen();
            let s = choose!(CalcOp::Add, s);
            let s = send(x, s);
            let s = send(y, s);
            let (z, s) = recv(s)?;
            close(s)?;
            assert_eq!(x.wrapping_add(y), z);
        }

        Ok(())

    }().is_ok());
}



