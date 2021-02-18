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
// C-->D.A-->B

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstFour, 4);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    1, | =>
    RoleA,
    SessionMpstFour,
    4
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_d,
    RoleD,
    next_d,
    3, | =>
    RoleC,
    SessionMpstFour,
    4
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    1, | =>
    RoleB,
    SessionMpstFour,
    4
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    3, | =>
    RoleD,
    SessionMpstFour,
    4
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstFour, 4);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstFour, 4);

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

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    cancel(s.session1);

    let s = send_mpst_a_to_b(random(), s);
    close_mpst_multi(s)
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    cancel(s.session1);

    let (_, s) = recv_mpst_b_to_a(s)?;
    close_mpst_multi(s)
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_d(random(), s);
    close_mpst_multi(s)
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_d_to_c(s)?;
    close_mpst_multi(s)
}

fn main() {
    let (thread_a, thread_b, thread_c, thread_d) = fork_mpst(
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
        simple_five_endpoint_d,
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
}