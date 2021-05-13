// Test for parametrisation on the name of the roles
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::fork::fork_mpst;
use mpstthree::functionmpst::close::close_mpst;
use mpstthree::role::end::RoleEnd;
use mpstthree::sessionmpst::SessionMpst;
use mpstthree::{
    create_multiple_normal_role, create_recv_mpst_session_1, create_recv_mpst_session_2,
    create_send_mpst_session_1, create_send_mpst_session_2,
};
use std::error::Error;

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleD, RoleDDual |
);

type TestA = RoleA<RoleEnd>;
type TestB = RoleB<RoleEnd>;
type TestD = RoleD<RoleEnd>;

type SendSessionMPSTD<N> = SessionMpst<Send<N, End>, End, TestA, TestD>;

type SendSessionMPSTA<N> = SessionMpst<End, Send<N, End>, TestD, TestA>;

type RecvSessionMPSTD<N> = SessionMpst<Recv<N, End>, End, TestA, TestD>;

type RecvSessionMPSTA<N> = SessionMpst<End, Recv<N, End>, TestD, TestA>;

// Create an B pawn
type Pawn = SessionMpst<End, End, RoleEnd, TestB>;

// Create new send functions
create_send_mpst_session_1!(send_mpst_d_to_a, RoleA, RoleD);
create_send_mpst_session_2!(send_mpst_a_to_d, RoleD, RoleA);

// Create new recv functions
create_recv_mpst_session_1!(recv_mpst_d_from_a, RoleA, RoleD);
create_recv_mpst_session_2!(recv_mpst_a_from_d, RoleD, RoleA);

// The functions for the basic exchanges
fn send_a_to_d(s: SendSessionMPSTA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_d(0, s);
    close_mpst(s)
}

fn send_d_to_a(s: SendSessionMPSTD<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_d_to_a(0, s);
    close_mpst(s)
}

fn recv_a_to_d(s: RecvSessionMPSTA<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_a_from_d(s)?;
    close_mpst(s)
}

fn recv_d_to_a(s: RecvSessionMPSTD<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_d_from_a(s)?;
    close_mpst(s)
}

fn pawn(s: Pawn) -> Result<(), Box<dyn Error>> {
    close_mpst(s)
}

/////////////////////////////////////////

pub fn basic_macros_send() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(send_a_to_d, pawn, recv_d_to_a);

            assert!(thread_a.join().is_ok());
            assert!(thread_pawn.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

pub fn basic_macros_recv() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(recv_a_to_d, pawn, send_d_to_a);

            assert!(thread_a.join().is_ok());
            assert!(thread_pawn.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
