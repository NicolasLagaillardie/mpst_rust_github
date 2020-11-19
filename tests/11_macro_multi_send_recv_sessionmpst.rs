// Test for parametrisation on the number of roles

use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    close_mpst, create_broadcast_role, create_normal_role, create_offer_mpst_session_multi,
    create_offer_type_multi, create_recv_mpst_all_session, create_recv_mpst_session,
    create_send_mpst_session, create_sessionmpst, fork_mpst_multi, fork_simple_multi,
};
use std::error::Error;

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpst, 3);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
// broadcast
create_broadcast_role!(RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);

// Create new send functions
create_send_mpst_session!(send_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
create_send_mpst_session!(send_mpst_a_to_d, RoleD, next_d, RoleD, SessionMpst, 3, 2);
create_send_mpst_session!(send_mpst_d_to_b, RoleB, next_b, RoleD, SessionMpst, 3, 2);
create_send_mpst_session!(send_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpst, 3, 1);
create_send_mpst_session!(send_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpst, 3, 1);

// Create new recv functions and related types
// normal
create_recv_mpst_session!(recv_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
create_recv_mpst_session!(recv_mpst_a_to_d, RoleD, next_d, RoleA, SessionMpst, 3, 2);
create_recv_mpst_session!(recv_mpst_b_to_d, RoleD, next_d, RoleB, SessionMpst, 3, 2);
create_recv_mpst_session!(recv_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpst, 3, 1);
create_recv_mpst_session!(recv_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpst, 3, 1);
// broadcast
create_recv_mpst_all_session!(
    recv_mpst_b_all_to_d,
    RoleAlltoD,
    next_all_to_d,
    RoleB,
    SessionMpst,
    3,
    2
);
create_recv_mpst_all_session!(
    recv_mpst_a_all_to_d,
    RoleAlltoD,
    next_all_to_d,
    RoleA,
    SessionMpst,
    3,
    2
);

close_mpst!(close_mpst_multi, SessionMpst, 3);

create_offer_type_multi!(OfferMpstMultiThree, SessionMpst, 3);

create_offer_mpst_session_multi!(
    offer_mpst_session_a_to_c,
    OfferMpstMultiThree,
    RoleAlltoD,
    recv_mpst_a_all_to_d,
    RoleA,
    SessionMpst,
    3,
    2
);

create_offer_mpst_session_multi!(
    offer_mpst_session_b_to_c,
    OfferMpstMultiThree,
    RoleAlltoD,
    recv_mpst_b_all_to_d,
    RoleB,
    SessionMpst,
    3,
    2
);

type TestA = RoleA<RoleEnd>;
type TestB = RoleB<RoleEnd>;
type TestD = RoleD<RoleEnd>;

type SendSessionMPSTD<N> = SessionMpst<Send<N, End>, End, TestA, TestD>;

type RecvSessionMPSTA<N> = SessionMpst<End, Recv<N, End>, TestD, TestA>;

type Pawn = SessionMpst<End, End, RoleEnd, TestB>;

fn send_d_to_a(s: SendSessionMPSTD<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_d_to_a(0, s);
    close_mpst_multi(s)?;
    Ok(())
}

fn recv_a_to_d(s: RecvSessionMPSTA<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_a_to_d(s)?;
    close_mpst_multi(s)?;
    Ok(())
}

fn pawn(s: Pawn) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)?;
    Ok(())
}

fork_simple_multi!(fork_simple, SessionMpst, 3);
fork_mpst_multi!(fork_mpst, fork_simple, SessionMpst, 3);

////////////////////////////////////////

#[test]
fn test_new_send() {
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
