use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, close_mpst_check_cancel, create_meshedchannels, create_multiple_normal_name,
    create_multiple_normal_role, create_recv_mpst_session_bundle, create_send_check_cancel_bundle,
    fork_mpst_multi,
};

use rand::random;
use std::error::Error;

// C-->B.C-->D

// Create new MeshedChannels for three participants
create_meshedchannels!(MeshedChannelsFour, 4);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
    RoleD, RoleDDual |
);

// Create new names
create_multiple_normal_name!(NameA, NameB, NameC, NameD);

// Create new send functions
// C
create_send_check_cancel_bundle!(
    send_check_c_to_b, RoleB, 2 |
    send_check_c_to_d, RoleD, 3 | =>
    NameC,
    MeshedChannelsFour,
    4
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_c, RoleC, 2 | =>
    NameB,
    MeshedChannelsFour,
    4
);

// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 3 | =>
    NameD,
    MeshedChannelsFour,
    4
);

// Create close function
close_mpst_check_cancel!(close_check_cancel, MeshedChannelsFour, 4);

// Create fork function
fork_mpst_multi!(fork_mpst, MeshedChannelsFour, 4);

// Types
type EndpointA = MeshedChannelsFour<End, End, End, RoleEnd, NameA>;
type EndpointB = MeshedChannelsFour<End, Recv<i32, End>, End, RoleC<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannelsFour<End, Send<i32, End>, Send<i32, End>, RoleB<RoleD<RoleEnd>>, NameC>;
type EndpointD = MeshedChannelsFour<End, End, Recv<i32, End>, RoleC<RoleEnd>, NameD>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, 4)
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_b_from_c(s)?;
    close_check_cancel(s)
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = send_check_c_to_b(random(), s)?;
    let s = send_check_c_to_d(random(), s)?;
    close_check_cancel(s)
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_d_from_c(s)?;
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
