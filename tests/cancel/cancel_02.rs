use mpstthree::binary::cancel::cancel;
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi_cancel, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_cancel, create_send_mpst_session_bundle,
};

use rand::random;
use std::error::Error;

// A --> B canceled
// C--> A.A-->B

// Create new SessionMpst for three participants
bundle_struct_fork_close_multi_cancel!(close_mpst_multi, fork_mpst, SessionMpstThree, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
);

// Create new send functions
// A
create_send_mpst_cancel!(send_cancel_a_to_b, RoleB, RoleA, SessionMpstThree, 3, 1);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a,
    RoleA,
    1 | =>
    RoleC,
    SessionMpstThree,
    3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_c,
    RoleC,
    2 | =>
    RoleA,
    SessionMpstThree,
    3
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a,
    RoleA,
    1 | =>
    RoleB,
    SessionMpstThree,
    3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
type EndpointA = SessionMpstThree<Send<i32, End>, Recv<i32, End>, RoleC<RoleB<RoleEnd>>, NameA>;
type EndpointB = SessionMpstThree<Recv<i32, End>, End, RoleA<RoleEnd>, NameB>;
type EndpointC = SessionMpstThree<Send<i32, End>, End, RoleA<RoleEnd>, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_a_from_c(s)?;
    let s = send_cancel_a_to_b(random(), s)?;
    close_mpst_multi(s)
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    cancel(s);

    // let (_, s) = recv_mpst_b_from_a(s)?;
    // close_mpst_multi(s)

    panic!("B canceled")
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_a(random(), s);
    close_mpst_multi(s)
}

pub fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
}
