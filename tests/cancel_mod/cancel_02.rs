use mpstthree::binary::cancel::cancel;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi_cancel, create_multiple_normal_name,
    create_multiple_normal_role, create_recv_mpst_session_bundle, create_send_mpst_cancel,
    create_send_mpst_session_bundle,
};

use rand::random;
use std::error::Error;

// A --> B canceled
// C--> A.A-->B

// Create new MeshedChannels for three participants
bundle_struct_fork_close_multi_cancel!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
);

// Create new names
create_multiple_normal_name!(NameA, NameB, NameC);

// Create new send functions
// A
create_send_mpst_cancel!(send_cancel_a_to_b, RoleB, NameA, MeshedChannelsThree, 3, 1);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a,
    RoleA,
    1 | =>
    NameC,
    MeshedChannelsThree,
    3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_c,
    RoleC,
    2 | =>
    NameA,
    MeshedChannelsThree,
    3
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a,
    RoleA,
    1 | =>
    NameB,
    MeshedChannelsThree,
    3
);

// Types
type EndpointA = MeshedChannelsThree<Send<i32, End>, Recv<i32, End>, RoleC<RoleB<RoleEnd>>, NameA>;
type EndpointB = MeshedChannelsThree<Recv<i32, End>, End, RoleA<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsThree<Send<i32, End>, End, RoleA<RoleEnd>, NameC>;

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
