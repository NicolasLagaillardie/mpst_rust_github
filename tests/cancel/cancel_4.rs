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
// A --> B.C--> B

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstThree, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, next_a, RoleADual, next_a_dual |
    RoleB, next_b, RoleBDual, next_b_dual |
    RoleC, next_c, RoleCDual, next_c_dual |
);

// Create new send functions
// A
create_send_mpst_cancel!(send_cancel_a_to_b, RoleB, RoleA, SessionMpstThree, 3, 1);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b,
    RoleB,
    2 | =>
    RoleC,
    SessionMpstThree,
    3
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a,
    RoleA,
    1 |
    recv_mpst_b_from_c,
    RoleC,
    2 | =>
    RoleB,
    SessionMpstThree,
    3
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstThree, 3);

// Create fork function
fork_mpst_multi!(fork_mpst, SessionMpstThree, 3);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
type EndpointA = SessionMpstThree<Send<i32, End>, End, RoleB<RoleEnd>, NameA>;
type EndpointB = SessionMpstThree<Recv<i32, End>, Recv<i32, End>, RoleA<RoleC<RoleEnd>>, NameB>;
type EndpointC = SessionMpstThree<End, Send<i32, End>, RoleB<RoleEnd>, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = send_cancel_a_to_b(random(), s)?;
    close_mpst_multi(s)
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    cancel(s);

    // let (_, s) = recv_mpst_b_from_a(s)?;
    // let (_, s) = recv_mpst_b_from_c(s)?;
    // close_mpst_multi(s)

    Ok(())
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_b(random(), s);
    close_mpst_multi(s)
}

pub fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_err());
}
