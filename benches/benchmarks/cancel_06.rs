#![allow(dead_code)]

use criterion::{criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, close_mpst, create_multiple_normal_role, create_recv_mpst_session_bundle,
    create_send_check_cancel_bundle, create_sessionmpst, fork_mpst_multi, send_cancel,
};

use rand::random;
use std::error::Error;
use std::time::Duration;

// B --> C canceled
// C-->B.C-->D

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstFour, 4);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
    RoleD, RoleDDual |
);

// Create new send functions
// C
create_send_check_cancel_bundle!(
    send_check_c_to_b,
    RoleB,
    2 |
    send_check_c_to_d,
    RoleD,
    3 | =>
    RoleC,
    SessionMpstFour,
    4
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_c,
    RoleC,
    2 | =>
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

send_cancel!(cancel_mpst, RoleB, SessionMpstFour, 4, "Session dropped");

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
type EndpointA = SessionMpstFour<End, End, End, RoleEnd, NameA>;
type EndpointB = SessionMpstFour<End, Recv<i32, End>, End, RoleC<RoleEnd>, NameB>;
type EndpointC = SessionMpstFour<End, Send<i32, End>, Send<i32, End>, RoleB<RoleD<RoleEnd>>, NameC>;
type EndpointD = SessionMpstFour<End, End, Recv<i32, End>, RoleC<RoleEnd>, NameD>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, RoleA, SessionMpstFour, 4);
    Ok(())
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    cancel_mpst(s)

    // let (_, s) = recv_mpst_b_from_a(s)?;
    // close_mpst_multi(s)
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = send_check_c_to_b(random(), s)?;
    let s = send_check_c_to_d(random(), s)?;
    close_mpst_multi(s)
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_d_from_c(s)?;
    close_mpst_multi(s)
}

pub fn cancel_main() {
    let (thread_a, thread_b, thread_c, thread_d) =
        fork_mpst(endpoint_a, endpoint_b, endpoint_c, endpoint_d);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
    assert!(thread_d.join().is_err());
}

fn cancel_6(c: &mut Criterion) {
    c.bench_function(&format!("Cancel_6"), |b| b.iter(|| cancel_main()));
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(1800, 0))
}

criterion_group! {
    name = cancel_bench;
    config = long_warmup();
    targets = cancel_6
}

criterion_main!(cancel_bench);
