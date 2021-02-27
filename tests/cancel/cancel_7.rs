use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, close_mpst_check_cancel, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, create_sessionmpst,
    fork_mpst_multi, send_cancel,
};

use rand::random;
use std::error::Error;

// C-->B.C-->D

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstFour, 4);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, next_a, RoleADual, next_a_dual |
    RoleB, next_b, RoleBDual, next_b_dual |
    RoleC, next_c, RoleCDual, next_c_dual |
    RoleD, next_d, RoleDDual, next_d_dual |
);

// Create new send functions
// C
create_send_check_cancel_bundle!(
    send_check_c_to_b,
    RoleB,
    next_b,
    2 |
    send_check_c_to_d,
    RoleD,
    next_d,
    3 | =>
    RoleC,
    SessionMpstFour,
    4
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    2 | =>
    RoleB,
    SessionMpstFour,
    4
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    3 | =>
    RoleD,
    SessionMpstFour,
    4
);

send_cancel!(cancel_mpst, RoleB, SessionMpstFour, 4);

// Create close function
close_mpst_check_cancel!(close_check_cancel, SessionMpstFour, 4);

// Create fork function
fork_mpst_multi!(fork_mpst, SessionMpstFour, 4);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;

// Types
type EndpointA = SessionMpstFour<End, End, End, RoleEnd, NameA>;
type EndpointB = SessionMpstFour<End, Recv<i32, End>, End, RoleC<RoleEnd>, NameB>;
type EndpointC = SessionMpstFour<End, Send<i32, End>, Send<i32, End>, RoleB<RoleD<RoleEnd>>, NameC>;
type EndpointD = SessionMpstFour<End, End, Recv<i32, End>, RoleC<RoleEnd>, NameD>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 4);
    Ok(())
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_b_to_c(s)?;
    close_check_cancel(s)
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = send_check_c_to_b(random(), s)?;
    let s = send_check_c_to_d(random(), s)?;
    close_check_cancel(s)
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_d_to_c(s)?;
    close_check_cancel(s)
}

pub fn main() {
    let (thread_a, thread_b, thread_c, thread_d) =
        fork_mpst(endpoint_a, endpoint_b, endpoint_c, endpoint_d);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_d.join().is_ok());
}
