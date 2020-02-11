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

/// Creating the MP sessions
type EndpointADual<N> = SessionMpst<<AtoB<N> as Session>::Dual, <AtoC<N> as Session>::Dual>;

type EndpointBDual<N> = SessionMpst<<BtoA<N> as Session>::Dual, <BtoC<N> as Session>::Dual>;

type EndpointCDual<N> = SessionMpst<<CtoA<N> as Session>::Dual, <CtoB<N> as Session>::Dual>;

/// Creating the endoint's functions
/// Here for A
fn endpoint_a_for_b(s: AtoB<i32>) -> Result<(), Box<dyn Error>> {
    let s = send(1, s);
    close(s)?;
    Ok(())
}

fn endpoint_a_for_c(s: AtoC<i32>) -> Result<(), Box<dyn Error>> {
    let (_x, s) = recv(s)?;
    close(s)?;
    Ok(())
}

/// Here for B
fn endpoint_b_for_a(s: BtoA<i32>) -> Result<(), Box<dyn Error>> {
    let (_x, s) = recv(s)?;
    close(s)?;
    Ok(())
}

fn endpoint_b_for_c(s: BtoC<i32>) -> Result<(), Box<dyn Error>> {
    let s = send(2, s);
    close(s)?;
    Ok(())
}

/// Here for C
fn endpoint_c_for_a(s: CtoA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send(1, s);
    close(s)?;
    Ok(())
}

fn endpoint_c_for_b(s: CtoB<i32>) -> Result<(), Box<dyn Error>> {
    let (_x, s) = recv(s)?;
    close(s)?;
    Ok(())
}

/// Single test for A
#[test]
fn simple_triple_endpoint_a() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test endpoint A
        {
            let s: EndpointADual<i32> = fork_mpst(endpoint_a_for_b, endpoint_a_for_c);

            let (x, s) = recv_mpst_session_one(s)?;
            let s = send_mpst_session_two(1, s);
            close_mpst(s)?;

            assert_eq!(x, 1);
        }

        Ok(())
    }()
    .is_ok());
}

/// Single test for B
#[test]
fn simple_triple_endpoint_b() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test endpoint B
        {
            let s: EndpointBDual<i32> = fork_mpst(endpoint_b_for_a, endpoint_b_for_c);

            let s = send_mpst_session_one(1, s);
            let (x, s) = recv_mpst_session_two(s)?;
            close_mpst(s)?;

            assert_eq!(x, 2);
        }

        Ok(())
    }()
    .is_ok());
}
/// Single test for C
#[test]
fn simple_triple_endpoint_c() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test endpoint C
        {
            let s: EndpointCDual<i32> = fork_mpst(endpoint_c_for_a, endpoint_c_for_b);

            let (x, s) = recv_mpst_session_one(s)?;
            let s = send_mpst_session_two(1, s);
            close_mpst(s)?;

            assert_eq!(x, 1);
        }

        Ok(())
    }()
    .is_ok());
}
