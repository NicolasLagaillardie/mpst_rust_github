#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_fork_multi, close_mpst, create_multiple_broadcast_role, create_multiple_normal_role,
    create_recv_mpst_session, create_send_mpst_session, create_sessionmpst,
};

use std::error::Error;
use std::thread::JoinHandle;
use std::time::Duration;

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstFive, 5);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, next_a, RoleADual, next_a_dual |
    RoleB, next_b, RoleBDual, next_b_dual |
    RoleC, next_c, RoleCDual, next_c_dual |
    RoleD, next_d, RoleDDual, next_d_dual |
    RoleE, next_e, RoleEDual, next_e_dual |
);
// broadcast
create_multiple_broadcast_role!(
    RoleAlltoA, next_all_to_a, RoleAtoAll, next_a_to_all |
    RoleAlltoB, next_all_to_b, RoleBtoAll, next_b_to_all |
    RoleAlltoC, next_all_to_c, RoleCtoAll, next_c_to_all |
    RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all |
    RoleAlltoE, next_all_to_e, RoleEtoAll, next_e_to_all |
);

// Create new send functions
// A
create_send_mpst_session!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstFive,
    5,
    1
);
create_send_mpst_session!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstFive,
    5,
    1
);
create_send_mpst_session!(
    send_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstFive,
    5,
    1
);
create_send_mpst_session!(
    send_mpst_e_to_a,
    RoleA,
    next_a,
    RoleE,
    SessionMpstFive,
    5,
    1
);
// B
create_send_mpst_session!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstFive,
    5,
    1
);
create_send_mpst_session!(
    send_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstFive,
    5,
    2
);
create_send_mpst_session!(
    send_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstFive,
    5,
    2
);
create_send_mpst_session!(
    send_mpst_e_to_b,
    RoleB,
    next_b,
    RoleE,
    SessionMpstFive,
    5,
    2
);
// C
create_send_mpst_session!(
    send_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstFive,
    5,
    2
);
create_send_mpst_session!(
    send_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstFive,
    5,
    2
);
create_send_mpst_session!(
    send_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstFive,
    5,
    3
);
create_send_mpst_session!(
    send_mpst_e_to_c,
    RoleC,
    next_c,
    RoleE,
    SessionMpstFive,
    5,
    3
);
// D
create_send_mpst_session!(
    send_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstFive,
    5,
    3
);
create_send_mpst_session!(
    send_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstFive,
    5,
    3
);
create_send_mpst_session!(
    send_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstFive,
    5,
    3
);
create_send_mpst_session!(
    send_mpst_e_to_d,
    RoleD,
    next_d,
    RoleE,
    SessionMpstFive,
    5,
    4
);
// E
create_send_mpst_session!(
    send_mpst_a_to_e,
    RoleE,
    next_e,
    RoleA,
    SessionMpstFive,
    5,
    4
);
create_send_mpst_session!(
    send_mpst_b_to_e,
    RoleE,
    next_e,
    RoleB,
    SessionMpstFive,
    5,
    4
);
create_send_mpst_session!(
    send_mpst_c_to_e,
    RoleE,
    next_e,
    RoleC,
    SessionMpstFive,
    5,
    4
);
create_send_mpst_session!(
    send_mpst_d_to_e,
    RoleE,
    next_e,
    RoleD,
    SessionMpstFive,
    5,
    4
);

// Create new recv functions and related types
// normal
// A
create_recv_mpst_session!(
    recv_mpst_b_to_a,
    RoleA,
    next_a,
    RoleB,
    SessionMpstFive,
    5,
    1
);
create_recv_mpst_session!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstFive,
    5,
    1
);
create_recv_mpst_session!(
    recv_mpst_d_to_a,
    RoleA,
    next_a,
    RoleD,
    SessionMpstFive,
    5,
    1
);
create_recv_mpst_session!(
    recv_mpst_e_to_a,
    RoleA,
    next_a,
    RoleE,
    SessionMpstFive,
    5,
    1
);
// B
create_recv_mpst_session!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstFive,
    5,
    1
);
create_recv_mpst_session!(
    recv_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstFive,
    5,
    2
);
create_recv_mpst_session!(
    recv_mpst_d_to_b,
    RoleB,
    next_b,
    RoleD,
    SessionMpstFive,
    5,
    2
);
create_recv_mpst_session!(
    recv_mpst_e_to_b,
    RoleB,
    next_b,
    RoleE,
    SessionMpstFive,
    5,
    2
);
// C
create_recv_mpst_session!(
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstFive,
    5,
    2
);
create_recv_mpst_session!(
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstFive,
    5,
    2
);
create_recv_mpst_session!(
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    RoleD,
    SessionMpstFive,
    5,
    3
);
create_recv_mpst_session!(
    recv_mpst_e_to_c,
    RoleC,
    next_c,
    RoleE,
    SessionMpstFive,
    5,
    3
);
// D
create_recv_mpst_session!(
    recv_mpst_a_to_d,
    RoleD,
    next_d,
    RoleA,
    SessionMpstFive,
    5,
    3
);
create_recv_mpst_session!(
    recv_mpst_b_to_d,
    RoleD,
    next_d,
    RoleB,
    SessionMpstFive,
    5,
    3
);
create_recv_mpst_session!(
    recv_mpst_c_to_d,
    RoleD,
    next_d,
    RoleC,
    SessionMpstFive,
    5,
    3
);
create_recv_mpst_session!(
    recv_mpst_e_to_d,
    RoleD,
    next_d,
    RoleE,
    SessionMpstFive,
    5,
    4
);
// E
create_recv_mpst_session!(
    recv_mpst_a_to_e,
    RoleE,
    next_e,
    RoleA,
    SessionMpstFive,
    5,
    4
);
create_recv_mpst_session!(
    recv_mpst_b_to_e,
    RoleE,
    next_e,
    RoleB,
    SessionMpstFive,
    5,
    4
);
create_recv_mpst_session!(
    recv_mpst_c_to_e,
    RoleE,
    next_e,
    RoleC,
    SessionMpstFive,
    5,
    4
);
create_recv_mpst_session!(
    recv_mpst_d_to_e,
    RoleE,
    next_e,
    RoleD,
    SessionMpstFive,
    5,
    4
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstFive, 5);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstFive, 5);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;

// Types
// Binary
// A
type AtoB = Send<(), Recv<(), End>>;
type AtoC = Send<(), Recv<(), End>>;
type AtoD = Send<(), Recv<(), End>>;
type AtoE = Send<(), Recv<(), End>>;
// B
type BtoA = Recv<(), Send<(), End>>;
type BtoC = Send<(), Recv<(), End>>;
type BtoD = Send<(), Recv<(), End>>;
type BtoE = Send<(), Recv<(), End>>;
// C
type CtoA = Recv<(), Send<(), End>>;
type CtoB = Recv<(), Send<(), End>>;
type CtoD = Send<(), Recv<(), End>>;
type CtoE = Send<(), Recv<(), End>>;
// D
type DtoA = Recv<(), Send<(), End>>;
type DtoB = Recv<(), Send<(), End>>;
type DtoC = Recv<(), Send<(), End>>;
type DtoE = Send<(), Recv<(), End>>;
// E
type EtoA = Recv<(), Send<(), End>>;
type EtoB = Recv<(), Send<(), End>>;
type EtoC = Recv<(), Send<(), End>>;
type EtoD = Recv<(), Send<(), End>>;

// Queues
type QueueA = RoleB<RoleC<RoleD<RoleE<RoleB<RoleC<RoleD<RoleE<RoleEnd>>>>>>>>;
type QueueB = RoleA<RoleC<RoleD<RoleE<RoleA<RoleC<RoleD<RoleE<RoleEnd>>>>>>>>;
type QueueC = RoleA<RoleB<RoleD<RoleE<RoleA<RoleB<RoleD<RoleE<RoleEnd>>>>>>>>;
type QueueD = RoleA<RoleB<RoleC<RoleE<RoleA<RoleB<RoleC<RoleE<RoleEnd>>>>>>>>;
type QueueE = RoleA<RoleB<RoleC<RoleD<RoleA<RoleB<RoleC<RoleD<RoleEnd>>>>>>>>;

// Creating the MP sessions
type EndpointA = SessionMpstFive<AtoB, AtoC, AtoD, AtoE, QueueA, NameA>;
type EndpointB = SessionMpstFive<BtoA, BtoC, BtoD, BtoE, QueueB, NameB>;
type EndpointC = SessionMpstFive<CtoA, CtoB, CtoD, CtoE, QueueC, NameC>;
type EndpointD = SessionMpstFive<DtoA, DtoB, DtoC, DtoE, QueueD, NameD>;
type EndpointE = SessionMpstFive<EtoA, EtoB, EtoC, EtoD, QueueE, NameE>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>>
{
    let s = send_mpst_a_to_b((), s);
    let s = send_mpst_a_to_c((), s);
    let s = send_mpst_a_to_d((), s);
    let s = send_mpst_a_to_e((), s);
    let (_, s) = recv_mpst_a_to_b(s)?;
    let (_, s) = recv_mpst_a_to_c(s)?;
    let (_, s) = recv_mpst_a_to_d(s)?;
    let (_, s) = recv_mpst_a_to_e(s)?;

    close_mpst_multi(s)
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv_mpst_b_to_a(s)?;
    let s = send_mpst_b_to_c((), s);
    let s = send_mpst_b_to_d((), s);
    let s = send_mpst_b_to_e((), s);
    let s = send_mpst_b_to_a((), s);
    let (_, s) = recv_mpst_b_to_c(s)?;
    let (_, s) = recv_mpst_b_to_d(s)?;
    let (_, s) = recv_mpst_b_to_e(s)?;

    close_mpst_multi(s)
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv_mpst_c_to_a(s)?;
    let (_, s) = recv_mpst_c_to_b(s)?;
    let s = send_mpst_c_to_d((), s);
    let s = send_mpst_c_to_e((), s);
    let s = send_mpst_c_to_a((), s);
    let s = send_mpst_c_to_b((), s);
    let (_, s) = recv_mpst_c_to_d(s)?;
    let (_, s) = recv_mpst_c_to_e(s)?;

    close_mpst_multi(s)
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv_mpst_d_to_a(s)?;
    let (_, s) = recv_mpst_d_to_b(s)?;
    let (_, s) = recv_mpst_d_to_c(s)?;
    let s = send_mpst_d_to_e((), s);
    let s = send_mpst_d_to_a((), s);
    let s = send_mpst_d_to_b((), s);
    let s = send_mpst_d_to_c((), s);
    let (_, s) = recv_mpst_d_to_e(s)?;

    close_mpst_multi(s)
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv_mpst_e_to_a(s)?;
    let (_, s) = recv_mpst_e_to_b(s)?;
    let (_, s) = recv_mpst_e_to_c(s)?;
    let (_, s) = recv_mpst_e_to_d(s)?;
    let s = send_mpst_e_to_a((), s);
    let s = send_mpst_e_to_b((), s);
    let s = send_mpst_e_to_c((), s);
    let s = send_mpst_e_to_d((), s);

    close_mpst_multi(s)
}

fn all_mpst() -> Result<(), Box<dyn Error>>
{
    let (thread_a, thread_b, thread_c, thread_d, thread_e) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
        black_box(simple_five_endpoint_d),
        black_box(simple_five_endpoint_e),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();

    Ok(())
}

/////////////////////////
// A
fn binary_a_to_b(s: AtoB) -> Result<(), Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_a_to_c(s: AtoC) -> Result<(), Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_a_to_d(s: AtoD) -> Result<(), Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_a_to_e(s: AtoE) -> Result<(), Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}

// B
fn binary_b_to_a(s: BtoA) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_b_to_c(s: BtoC) -> Result<(), Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_b_to_d(s: BtoD) -> Result<(), Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_b_to_e(s: BtoE) -> Result<(), Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}

// C
fn binary_c_to_a(s: CtoA) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_c_to_b(s: CtoB) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_c_to_d(s: CtoD) -> Result<(), Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_c_to_e(s: CtoE) -> Result<(), Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}

// D
fn binary_d_to_a(s: DtoA) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_d_to_b(s: DtoB) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_d_to_c(s: DtoC) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_d_to_e(s: DtoE) -> Result<(), Box<dyn Error>>
{
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}

// E
fn binary_e_to_a(s: EtoA) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_e_to_b(s: EtoB) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_e_to_c(s: EtoC) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_e_to_d(s: EtoD) -> Result<(), Box<dyn Error>>
{
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}

fn all_binaries() -> Result<(), Box<dyn Error>>
{
    // A
    let (thread_a_to_b, s_a_to_b): (JoinHandle<()>, BtoA) =
        fork_with_thread_id(black_box(binary_a_to_b));
    let (thread_a_to_c, s_a_to_c): (JoinHandle<()>, CtoA) =
        fork_with_thread_id(black_box(binary_a_to_c));
    let (thread_a_to_d, s_a_to_d): (JoinHandle<()>, DtoA) =
        fork_with_thread_id(black_box(binary_a_to_d));
    let (thread_a_to_e, s_a_to_e): (JoinHandle<()>, EtoA) =
        fork_with_thread_id(black_box(binary_a_to_e));
    // B
    let (thread_b_to_c, s_b_to_c): (JoinHandle<()>, CtoB) =
        fork_with_thread_id(black_box(binary_b_to_c));
    let (thread_b_to_d, s_b_to_d): (JoinHandle<()>, DtoB) =
        fork_with_thread_id(black_box(binary_b_to_d));
    let (thread_b_to_e, s_b_to_e): (JoinHandle<()>, EtoB) =
        fork_with_thread_id(black_box(binary_b_to_e));
    // C
    let (thread_c_to_d, s_c_to_d): (JoinHandle<()>, CtoD) =
        fork_with_thread_id(black_box(binary_d_to_c));
    let (thread_c_to_e, s_c_to_e): (JoinHandle<()>, CtoE) =
        fork_with_thread_id(black_box(binary_e_to_c));
    // D
    let (thread_d_to_e, s_d_to_e): (JoinHandle<()>, DtoE) =
        fork_with_thread_id(black_box(binary_e_to_d));

    binary_b_to_a(black_box(s_a_to_b)).unwrap();
    binary_c_to_a(black_box(s_a_to_c)).unwrap();
    binary_d_to_a(black_box(s_a_to_d)).unwrap();
    binary_e_to_a(black_box(s_a_to_e)).unwrap();

    binary_c_to_b(black_box(s_b_to_c)).unwrap();
    binary_d_to_b(black_box(s_b_to_d)).unwrap();
    binary_e_to_b(black_box(s_b_to_e)).unwrap();

    binary_c_to_d(black_box(s_c_to_d)).unwrap();
    binary_c_to_e(black_box(s_c_to_e)).unwrap();

    binary_d_to_e(black_box(s_d_to_e)).unwrap();

    thread_a_to_b.join().unwrap();
    thread_a_to_c.join().unwrap();
    thread_a_to_d.join().unwrap();
    thread_a_to_e.join().unwrap();

    thread_b_to_c.join().unwrap();
    thread_b_to_d.join().unwrap();
    thread_b_to_e.join().unwrap();

    thread_c_to_d.join().unwrap();
    thread_c_to_e.join().unwrap();

    thread_d_to_e.join().unwrap();

    Ok(())
}

/////////////////////////

fn long_protocol_mpst(c: &mut Criterion)
{
    c.bench_function("long protocol MPST", |b| b.iter(|| all_mpst()));
}

fn long_protocol_binary(c: &mut Criterion)
{
    c.bench_function("long protocol binary", |b| b.iter(|| all_binaries()));
}

fn short_warmup() -> Criterion
{
    Criterion::default().measurement_time(Duration::new(180, 0))
}

criterion_group! {
    name = long_five_protocols;
    config = short_warmup();
    targets = long_protocol_mpst, long_protocol_binary
}
criterion_main!(long_five_protocols);
