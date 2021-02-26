use mpstthree::binary::{cancel, End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_fork_multi, close_mpst, create_multiple_normal_role, create_recv_mpst_session_bundle,
    create_send_mpst_cancel, create_send_mpst_session_bundle, create_sessionmpst,
};

use rand::random;
use std::error::Error;

// A --> B canceled
// A--> B.A-->C

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
create_send_mpst_session_bundle!(
    send_mpst_a_to_c,
    RoleC,
    next_c,
    2 | =>
    RoleA,
    SessionMpstThree,
    3
);
create_send_mpst_cancel!(
    send_cancel_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstThree,
    3,
    1
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    1 | =>
    RoleB,
    SessionMpstThree,
    3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    1 | =>
    RoleC,
    SessionMpstThree,
    3
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstThree, 3);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstThree, 3);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
type EndpointA = SessionMpstThree<Send<i32, End>, Send<i32, End>, RoleB<RoleC<RoleEnd>>, NameA>;
type EndpointB = SessionMpstThree<Recv<i32, End>, End, RoleA<RoleEnd>, NameB>;
type EndpointC = SessionMpstThree<Recv<i32, End>, End, RoleA<RoleEnd>, NameC>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = send_cancel_a_to_b(random(), s)?;
    let s = send_mpst_a_to_c(random(), s);
    close_mpst_multi(s)
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    cancel(s);

    // let (_, s) = recv_mpst_b_to_a(s)?;
    // close_mpst_multi(s)

    Ok(())
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_c_to_a(s)?;
    close_mpst_multi(s)
}

pub fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
    );

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_err());
}
