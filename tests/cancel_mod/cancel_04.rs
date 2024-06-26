use mpstthree::binary::cancel::cancel;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    close_mpst_cancel, create_meshedchannels, create_multiple_normal_name,
    create_multiple_normal_role, create_recv_mpst_session_bundle, create_send_mpst_cancel,
    create_send_mpst_session_bundle, fork_mpst_multi,
};

use rand::random;
use std::error::Error;

// A --> B canceled
// A --> B.C--> B

// Create new MeshedChannels for three participants
create_meshedchannels!(MeshedChannels, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
);

// Create new names
create_multiple_normal_name!(NameA, NameB, NameC, NameD);

// Create new send functions
// A
create_send_mpst_cancel!(send_cancel_a_to_b, RoleB, NameA, MeshedChannels, 3, 1);

// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b,
    RoleB,
    2 | =>
    NameC,
    MeshedChannels,
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
    NameB,
    MeshedChannels,
    3
);

// Create close function
close_mpst_cancel!(close_mpst_multi, MeshedChannels, 3);

// Create fork function
fork_mpst_multi!(fork_mpst, MeshedChannels, 3);

// Types
type EndpointA = MeshedChannels<Send<i32, End>, End, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<Recv<i32, End>, Recv<i32, End>, RoleA<RoleC<RoleEnd>>, NameB>;
type EndpointC = MeshedChannels<End, Send<i32, End>, RoleB<RoleEnd>, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = send_cancel_a_to_b(random(), s)?;
    close_mpst_multi(s)
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    cancel(s);

    // let (_, s) = recv_mpst_b_from_a(s)?;
    // let (_, s) = recv_mpst_b_from_c(s)?;
    // close_mpst_multi(s)

    panic!("B canceled")
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_b(random(), s);
    close_mpst_multi(s)
}

pub fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
}
