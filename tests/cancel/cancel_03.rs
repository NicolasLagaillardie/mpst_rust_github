use mpstthree::binary::cancel::cancel;
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    close_mpst_cancel, create_meshedchannels, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_cancel, create_send_mpst_session_bundle,
    fork_mpst_multi,
};

use rand::random;
use std::error::Error;

// A --> B canceled
// C-->D.A-->B

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

// Create new send functions
// A
create_send_mpst_cancel!(send_cancel_a_to_b, RoleB, RoleA, MeshedChannelsFour, 4, 1);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_d,
    RoleD,
    3 | =>
    RoleC,
    MeshedChannelsFour,
    4
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a,
    RoleA,
    1 | =>
    RoleB,
    MeshedChannelsFour,
    4
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c,
    RoleC,
    3 | =>
    RoleD,
    MeshedChannelsFour,
    4
);

// Create close function
close_mpst_cancel!(close_mpst_multi, MeshedChannelsFour, 4);

// Create fork function
fork_mpst_multi!(fork_mpst, MeshedChannelsFour, 4);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;

// Types
type EndpointA = MeshedChannelsFour<Send<i32, End>, End, End, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFour<Recv<i32, End>, End, End, RoleA<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFour<End, End, Send<i32, End>, RoleD<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsFour<End, End, Recv<i32, End>, RoleC<RoleEnd>, NameD>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
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
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
    assert!(thread_d.join().is_err());
}
