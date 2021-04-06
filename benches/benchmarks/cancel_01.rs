#![allow(dead_code)]

use criterion::{criterion_group, criterion_main, Criterion};

use mpstthree::binary::cancel::cancel;
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_cancel, create_send_mpst_session_bundle,
};

use rand::random;
use std::error::Error;
use std::time::Duration;

// A --> B canceled
// A --> B.B--> C

// Create new SessionMpst for three participants
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstThree, 3);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C);

// Create new send functions
// A
create_send_mpst_cancel!(send_cancel_a_to_b, RoleB, RoleA, SessionMpstThree, 3, 1);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_c,
    RoleC,
    2 | =>
    RoleB,
    SessionMpstThree,
    3
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a,
    RoleA,
    1 | =>
    RoleB,
    SessionMpstThree,
    3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b,
    RoleB,
    2 | =>
    RoleC,
    SessionMpstThree,
    3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
type EndpointA = SessionMpstThree<Send<i32, End>, End, RoleB<RoleEnd>, NameA>;
type EndpointB = SessionMpstThree<Recv<i32, End>, Send<i32, End>, RoleA<RoleC<RoleEnd>>, NameB>;
type EndpointC = SessionMpstThree<End, Recv<i32, End>, RoleB<RoleEnd>, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = send_cancel_a_to_b(random(), s)?;
    close_mpst_multi(s)
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    cancel(s);

    // let (_, s) = recv_mpst_b_from_a(s)?;
    // let s = send_mpst_b_to_c(random(), s);
    // close_mpst_multi(s)

    panic!("Session dropped");
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_c_from_b(s)?;
    close_mpst_multi(s)
}

fn cancel_main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_err());
    assert!(thread_c.join().is_err());
}

fn cancel_1(c: &mut Criterion) {
    c.bench_function(&format!("Cancel_1"), |b| b.iter(|| cancel_main()));
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(1800, 0))
}

criterion_group! {
    name = cancel_bench;
    config = long_warmup();
    targets = cancel_1
}

criterion_main!(cancel_bench);
