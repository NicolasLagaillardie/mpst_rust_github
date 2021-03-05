// Test for parametrisation on the number of roles

use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    close_mpst, create_multiple_normal_role, create_recv_mpst_session, create_send_mpst_session,
    create_sessionmpst, fork_mpst_multi,
};
use std::error::Error;

// Create new SessionMpst for five participants
create_sessionmpst!(SessionMpst, 5);

// Create new roles
create_multiple_normal_role!(
    RoleA, next_a, RoleADual, next_a_dual |
    RoleB, next_b, RoleBDual, next_b_dual |
    RoleC, next_c, RoleCDual, next_c_dual |
    RoleD, next_d, RoleDDual, next_d_dual |
    RoleE, next_e, RoleEDual, next_e_dual |
);

// Create new send functions
create_send_mpst_session!(send_mpst_d_to_b, RoleB, next_b, RoleD, SessionMpst, 5, 2);

// Create new recv functions and related types
create_recv_mpst_session!(recv_mpst_b_to_d, RoleD, next_d, RoleB, SessionMpst, 5, 3);

close_mpst!(close_mpst_multi, SessionMpst, 5);

type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;

type SendSessionMPSTD<N> = SessionMpst<End, Send<N, End>, End, End, NameB, NameD>;

type RecvSessionMPSTB<N> = SessionMpst<End, End, Recv<N, End>, End, NameD, NameB>;

type PawnA = SessionMpst<End, End, End, End, RoleEnd, NameA>;
type PawnC = SessionMpst<End, End, End, End, RoleEnd, NameC>;
type PawnE = SessionMpst<End, End, End, End, RoleEnd, NameE>;

fn send_d_to_b(s: SendSessionMPSTD<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_d_to_b(0, s);
    close_mpst_multi(s)
}

fn recv_b_to_d(s: RecvSessionMPSTB<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_b_to_d(s)?;
    close_mpst_multi(s)
}

fn pawn_a(s: PawnA) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn pawn_c(s: PawnC) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn pawn_e(s: PawnE) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fork_mpst_multi!(fork_mpst, SessionMpst, 5);

////////////////////////////////////////

pub fn test_new_send() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c, thread_d, thread_e) =
                fork_mpst(pawn_a, recv_b_to_d, pawn_c, send_d_to_b, pawn_e);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
            assert!(thread_d.join().is_ok());
            assert!(thread_e.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
