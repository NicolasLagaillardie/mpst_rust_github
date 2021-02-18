use mpstthree::binary::{cancel, End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_fork_multi, close_mpst, create_normal_role, create_recv_mpst_session,
    create_recv_mpst_session_bundle, create_send_mpst_session, create_send_mpst_session_bundle,
    create_sessionmpst,
};

use rand::random;
use std::error::Error;

// A --> B canceled
// A --> B.C--> B

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstThree, 3);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    1, | =>
    RoleA,
    SessionMpstThree,
    3
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b,
    RoleB,
    next_b,
    2, | =>
    RoleC,
    SessionMpstThree,
    3
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    1, |
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    2, | =>
    RoleB,
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
type EndpointA = SessionMpstThree<Send<i32, End>, End, RoleB<RoleEnd>, NameA>;
type EndpointB = SessionMpstThree<Recv<i32, End>, Recv<i32, End>, RoleA<RoleC<RoleEnd>>, NameB>;
type EndpointC = SessionMpstThree<End, Send<i32, End>, RoleB<RoleEnd>, NameC>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    cancel(s.session1);
    let s = send_mpst_a_to_b(random(), s);
    close_mpst_multi(s)
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    cancel(s.session1);

    let (_, s) = recv_mpst_b_to_a(s)?;
    let (_, s) = recv_mpst_b_to_c(s)?;
    close_mpst_multi(s)
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_b(random(), s);
    close_mpst_multi(s)
}

fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}
