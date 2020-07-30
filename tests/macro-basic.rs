// Test for Macro
extern crate crossbeam_channel;
extern crate either;
extern crate mpstthree;
use crossbeam_channel::{bounded, Sender};
use mpstthree::binary::{recv, send, End, Recv, Send, Session};
use mpstthree::fork_mpst;
use mpstthree::functionmpst::close::close_mpst;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;
use mpstthree::{
    create_normal_role, create_recv_mpst_session_1, create_recv_mpst_session_2,
    create_send_mpst_session_1, create_send_mpst_session_2,
};
use std::error::Error;
use std::marker;

// Create new roles
create_normal_role!(RoleAtoD, next_a_to_d, RoleDtoA, next_d_to_a);

type TestAtoD = RoleAtoD<RoleEnd>;

type TestDtoA = RoleDtoA<RoleEnd>;

create_normal_role!(RoleBtoD, next_b_to_d, RoleDtoB, next_d_to_b);

// Create new send functions
create_send_mpst_session_1!(send_mpst_d_to_a, RoleDtoA, next_d_to_a);

type SendSessionMPSTDtoA<N> = SessionMpst<Send<N, End>, End, TestDtoA>;

create_send_mpst_session_2!(send_mpst_a_to_d, RoleAtoD, next_a_to_d);

type SendSessionMPSTAtoD<N> = SessionMpst<End, Send<N, End>, TestAtoD>;

// Create new recv functions and related types
create_recv_mpst_session_1!(recv_mpst_d_to_a, RoleDtoA, next_d_to_a);

type RecvSessionMPSTDtoA<N> = SessionMpst<Recv<N, End>, End, TestDtoA>;

create_recv_mpst_session_2!(recv_mpst_a_to_d, RoleAtoD, next_a_to_d);

type RecvSessionMPSTAtoD<N> = SessionMpst<End, Recv<N, End>, TestAtoD>;

// Create an End pawn
type Pawn = SessionMpst<End, End, RoleEnd>;

// The functions for the basic exchanges
fn send_a_to_d(s: SendSessionMPSTAtoD<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_d(0, s);
    close_mpst(s)?;
    Ok(())
}

fn send_d_to_a(s: SendSessionMPSTDtoA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_d_to_a(0, s);
    close_mpst(s)?;
    Ok(())
}

fn recv_a_to_d(s: RecvSessionMPSTAtoD<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_a_to_d(s)?;
    close_mpst(s)?;
    Ok(())
}

fn recv_d_to_a(s: RecvSessionMPSTDtoA<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_d_to_a(s)?;
    close_mpst(s)?;
    Ok(())
}

fn pawn(s: Pawn) -> Result<(), Box<dyn Error>> {
    close_mpst(s)?;
    Ok(())
}

#[test]
fn basic_macros() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(send_a_to_d, pawn, recv_d_to_a);

            assert!(thread_a.is_ok());
            assert!(thread_pawn.is_ok());
            assert!(thread_d.is_ok());
        }
        Ok(())
    }()
    .is_ok());

    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(recv_a_to_d, pawn, send_d_to_a);

            assert!(thread_a.is_ok());
            assert!(thread_pawn.is_ok());
            assert!(thread_d.is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
