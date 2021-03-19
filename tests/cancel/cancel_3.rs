use mpstthree::binary::cancel::cancel;
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    close_mpst, create_multiple_normal_role, create_recv_mpst_session_bundle,
    create_send_mpst_cancel, create_send_mpst_session_bundle, create_sessionmpst, fork_mpst_multi,
};

use rand::random;
use std::error::Error;

// A --> B canceled
// C-->D.A-->B

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
// A
create_send_mpst_cancel!(send_cancel_a_to_b, RoleB, RoleA, SessionMpstFour, 4, 1);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_d,
    RoleD,
    3 | =>
    RoleC,
    SessionMpstFour,
    4
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a,
    RoleA,
    1 | =>
    RoleB,
    SessionMpstFour,
    4
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c,
    RoleC,
    3 | =>
    RoleD,
    SessionMpstFour,
    4
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstFour, 4);

// Create fork function
fork_mpst_multi!(fork_mpst, SessionMpstFour, 4);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;

// Types
type EndpointA = SessionMpstFour<Send<i32, End>, End, End, RoleB<RoleEnd>, NameA>;
type EndpointB = SessionMpstFour<Recv<i32, End>, End, End, RoleA<RoleEnd>, NameB>;
type EndpointC = SessionMpstFour<End, End, Send<i32, End>, RoleD<RoleEnd>, NameC>;
type EndpointD = SessionMpstFour<End, End, Recv<i32, End>, RoleC<RoleEnd>, NameD>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = send_cancel_a_to_b(random(), s)?;
    close_mpst_multi(s)
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    cancel(s);

    // let (_, s) = recv_mpst_b_from_a(s)?;
    // close_mpst_multi(s)

    Ok(())
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_d(random(), s);
    close_mpst_multi(s)
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_d_from_c(s)?;
    close_mpst_multi(s)
}

pub fn main() {
    let (thread_a, thread_b, thread_c, thread_d) =
        fork_mpst(endpoint_a, endpoint_b, endpoint_c, endpoint_d);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_err());
    assert!(thread_d.join().is_err());
}
