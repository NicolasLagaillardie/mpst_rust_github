use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_fork_multi, close_mpst, create_normal_role, create_recv_mpst_session,
    create_send_mpst_session, create_sessionmpst,
};

use std::error::Error;
use std::thread::JoinHandle;
use std::time::Duration;

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstThree, 3);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);

// Create new send functions
create_send_mpst_session!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstThree,
    3,
    1
);
create_send_mpst_session!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstThree,
    3,
    1
);

// Create new recv functions and related types
create_recv_mpst_session!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstThree,
    3,
    1
);
create_recv_mpst_session!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstThree,
    3,
    1
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
// Binary
// A
type AtoB = Send<
    (),
    Recv<(), Send<(), Recv<(), Send<(), Recv<(), Send<(), Recv<(), Send<(), Recv<(), End>>>>>>>>>,
>;
// B
type BtoA = Recv<
    (),
    Send<(), Recv<(), Send<(), Recv<(), Send<(), Recv<(), Send<(), Recv<(), Send<(), End>>>>>>>>>,
>;

// Queues
type QueueA = RoleB<RoleB<RoleB<RoleB<RoleB<RoleB<RoleB<RoleB<RoleB<RoleB<RoleEnd>>>>>>>>>>;
type QueueB = RoleA<RoleA<RoleA<RoleA<RoleA<RoleA<RoleA<RoleA<RoleA<RoleA<RoleEnd>>>>>>>>>>;

// Creating the MP sessions
type EndpointA = SessionMpstThree<AtoB, End, QueueA, NameA>;
type EndpointB = SessionMpstThree<BtoA, End, QueueB, NameB>;
type EndpointC = SessionMpstThree<End, End, RoleEnd, NameC>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_b((), s);
    let (_, s) = recv_mpst_a_to_b(s)?;
    let s = send_mpst_a_to_b((), s);
    let (_, s) = recv_mpst_a_to_b(s)?;
    let s = send_mpst_a_to_b((), s);
    let (_, s) = recv_mpst_a_to_b(s)?;
    let s = send_mpst_a_to_b((), s);
    let (_, s) = recv_mpst_a_to_b(s)?;
    let s = send_mpst_a_to_b((), s);
    let (_, s) = recv_mpst_a_to_b(s)?;

    close_mpst_multi(s)
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_b_to_a(s)?;
    let s = send_mpst_b_to_a((), s);
    let (_, s) = recv_mpst_b_to_a(s)?;
    let s = send_mpst_b_to_a((), s);
    let (_, s) = recv_mpst_b_to_a(s)?;
    let s = send_mpst_b_to_a((), s);
    let (_, s) = recv_mpst_b_to_a(s)?;
    let s = send_mpst_b_to_a((), s);
    let (_, s) = recv_mpst_b_to_a(s)?;
    let s = send_mpst_b_to_a((), s);

    close_mpst_multi(s)
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, _) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    ////////////////////
    let (thread_a, thread_b, _) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    Ok(())
}

/////////////////////////
// A
fn binary_a_to_b(s: AtoB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;
    let s = send((), s);
    let (_, s) = recv(s)?;
    let s = send((), s);
    let (_, s) = recv(s)?;
    let s = send((), s);
    let (_, s) = recv(s)?;
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}

// B
fn binary_b_to_a(s: BtoA) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);
    let (_, s) = recv(s)?;
    let s = send((), s);
    let (_, s) = recv(s)?;
    let s = send((), s);
    let (_, s) = recv(s)?;
    let s = send((), s);
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}

// B
fn binary_c(s: End) -> Result<(), Box<dyn Error>> {
    close(s)
}

fn all_binaries() -> Result<(), Box<dyn Error>> {
    let _ = fork_with_thread_id(black_box(binary_c));

    let (thread_a_to_b, s_a_to_b): (JoinHandle<()>, BtoA) =
        fork_with_thread_id(black_box(binary_a_to_b));

    binary_b_to_a(black_box(s_a_to_b)).unwrap();
    thread_a_to_b.join().unwrap();

    /////////////////////////
    let _ = fork_with_thread_id(black_box(binary_c));

    let (thread_b_to_a, s_b_to_a): (JoinHandle<()>, AtoB) =
        fork_with_thread_id(black_box(binary_b_to_a));

    binary_a_to_b(black_box(s_b_to_a)).unwrap();
    thread_b_to_a.join().unwrap();

    Ok(())
}

/////////////////////////

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function("long simple protocol MPST", |b| b.iter(|| all_mpst()));
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function("long simple protocol binary", |b| b.iter(|| all_binaries()));
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(30, 0))
}

criterion_group! {
    name = long_three_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary
}
criterion_main!(long_three_simple_protocols);
