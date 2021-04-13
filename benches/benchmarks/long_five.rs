#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_multiple_broadcast_role_short,
    create_multiple_normal_role_short, create_recv_mpst_session, create_send_mpst_session,
};

use std::error::Error;
use std::thread::JoinHandle;
use std::time::Duration;

// Create the new SessionMpst for five participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstFive, 5);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E);
// broadcast
create_multiple_broadcast_role_short!(A, B, C, D, E);

// Create new send functions
// A
create_send_mpst_session!(send_mpst_b_to_a, RoleA, RoleB, SessionMpstFive, 5, 1);
create_send_mpst_session!(send_mpst_c_to_a, RoleA, RoleC, SessionMpstFive, 5, 1);
create_send_mpst_session!(send_mpst_d_to_a, RoleA, RoleD, SessionMpstFive, 5, 1);
create_send_mpst_session!(send_mpst_e_to_a, RoleA, RoleE, SessionMpstFive, 5, 1);
// B
create_send_mpst_session!(send_mpst_a_to_b, RoleB, RoleA, SessionMpstFive, 5, 1);
create_send_mpst_session!(send_mpst_c_to_b, RoleB, RoleC, SessionMpstFive, 5, 2);
create_send_mpst_session!(send_mpst_d_to_b, RoleB, RoleD, SessionMpstFive, 5, 2);
create_send_mpst_session!(send_mpst_e_to_b, RoleB, RoleE, SessionMpstFive, 5, 2);
// C
create_send_mpst_session!(send_mpst_a_to_c, RoleC, RoleA, SessionMpstFive, 5, 2);
create_send_mpst_session!(send_mpst_b_to_c, RoleC, RoleB, SessionMpstFive, 5, 2);
create_send_mpst_session!(send_mpst_d_to_c, RoleC, RoleD, SessionMpstFive, 5, 3);
create_send_mpst_session!(send_mpst_e_to_c, RoleC, RoleE, SessionMpstFive, 5, 3);
// D
create_send_mpst_session!(send_mpst_a_to_d, RoleD, RoleA, SessionMpstFive, 5, 3);
create_send_mpst_session!(send_mpst_b_to_d, RoleD, RoleB, SessionMpstFive, 5, 3);
create_send_mpst_session!(send_mpst_c_to_d, RoleD, RoleC, SessionMpstFive, 5, 3);
create_send_mpst_session!(send_mpst_e_to_d, RoleD, RoleE, SessionMpstFive, 5, 4);
// E
create_send_mpst_session!(send_mpst_a_to_e, RoleE, RoleA, SessionMpstFive, 5, 4);
create_send_mpst_session!(send_mpst_b_to_e, RoleE, RoleB, SessionMpstFive, 5, 4);
create_send_mpst_session!(send_mpst_c_to_e, RoleE, RoleC, SessionMpstFive, 5, 4);
create_send_mpst_session!(send_mpst_d_to_e, RoleE, RoleD, SessionMpstFive, 5, 4);

// Create new recv functions and related types
// normal
// A
create_recv_mpst_session!(recv_mpst_b_from_a, RoleA, RoleB, SessionMpstFive, 5, 1);
create_recv_mpst_session!(recv_mpst_c_from_a, RoleA, RoleC, SessionMpstFive, 5, 1);
create_recv_mpst_session!(recv_mpst_d_from_a, RoleA, RoleD, SessionMpstFive, 5, 1);
create_recv_mpst_session!(recv_mpst_e_from_a, RoleA, RoleE, SessionMpstFive, 5, 1);
// B
create_recv_mpst_session!(recv_mpst_a_from_b, RoleB, RoleA, SessionMpstFive, 5, 1);
create_recv_mpst_session!(recv_mpst_c_from_b, RoleB, RoleC, SessionMpstFive, 5, 2);
create_recv_mpst_session!(recv_mpst_d_from_b, RoleB, RoleD, SessionMpstFive, 5, 2);
create_recv_mpst_session!(recv_mpst_e_from_b, RoleB, RoleE, SessionMpstFive, 5, 2);
// C
create_recv_mpst_session!(recv_mpst_a_from_c, RoleC, RoleA, SessionMpstFive, 5, 2);
create_recv_mpst_session!(recv_mpst_b_from_c, RoleC, RoleB, SessionMpstFive, 5, 2);
create_recv_mpst_session!(recv_mpst_d_from_c, RoleC, RoleD, SessionMpstFive, 5, 3);
create_recv_mpst_session!(recv_mpst_e_from_c, RoleC, RoleE, SessionMpstFive, 5, 3);
// D
create_recv_mpst_session!(recv_mpst_a_from_d, RoleD, RoleA, SessionMpstFive, 5, 3);
create_recv_mpst_session!(recv_mpst_b_from_d, RoleD, RoleB, SessionMpstFive, 5, 3);
create_recv_mpst_session!(recv_mpst_c_from_d, RoleD, RoleC, SessionMpstFive, 5, 3);
create_recv_mpst_session!(recv_mpst_e_from_d, RoleD, RoleE, SessionMpstFive, 5, 4);
// E
create_recv_mpst_session!(recv_mpst_a_from_e, RoleE, RoleA, SessionMpstFive, 5, 4);
create_recv_mpst_session!(recv_mpst_b_from_e, RoleE, RoleB, SessionMpstFive, 5, 4);
create_recv_mpst_session!(recv_mpst_c_from_e, RoleE, RoleC, SessionMpstFive, 5, 4);
create_recv_mpst_session!(recv_mpst_d_from_e, RoleE, RoleD, SessionMpstFive, 5, 4);

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

// Stacks
type StackA = RoleB<RoleC<RoleD<RoleE<RoleB<RoleC<RoleD<RoleE<RoleEnd>>>>>>>>;
type StackB = RoleA<RoleC<RoleD<RoleE<RoleA<RoleC<RoleD<RoleE<RoleEnd>>>>>>>>;
type StackC = RoleA<RoleB<RoleD<RoleE<RoleA<RoleB<RoleD<RoleE<RoleEnd>>>>>>>>;
type StackD = RoleA<RoleB<RoleC<RoleE<RoleA<RoleB<RoleC<RoleE<RoleEnd>>>>>>>>;
type StackE = RoleA<RoleB<RoleC<RoleD<RoleA<RoleB<RoleC<RoleD<RoleEnd>>>>>>>>;

// Creating the MP sessions
type EndpointA = SessionMpstFive<AtoB, AtoC, AtoD, AtoE, StackA, NameA>;
type EndpointB = SessionMpstFive<BtoA, BtoC, BtoD, BtoE, StackB, NameB>;
type EndpointC = SessionMpstFive<CtoA, CtoB, CtoD, CtoE, StackC, NameC>;
type EndpointD = SessionMpstFive<DtoA, DtoB, DtoC, DtoE, StackD, NameD>;
type EndpointE = SessionMpstFive<EtoA, EtoB, EtoC, EtoD, StackE, NameE>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_b((), s);
    let s = send_mpst_a_to_c((), s);
    let s = send_mpst_a_to_d((), s);
    let s = send_mpst_a_to_e((), s);
    let (_, s) = recv_mpst_a_from_b(s)?;
    let (_, s) = recv_mpst_a_from_c(s)?;
    let (_, s) = recv_mpst_a_from_d(s)?;
    let (_, s) = recv_mpst_a_from_e(s)?;

    close_mpst_multi(s)
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_b_from_a(s)?;
    let s = send_mpst_b_to_c((), s);
    let s = send_mpst_b_to_d((), s);
    let s = send_mpst_b_to_e((), s);
    let s = send_mpst_b_to_a((), s);
    let (_, s) = recv_mpst_b_from_c(s)?;
    let (_, s) = recv_mpst_b_from_d(s)?;
    let (_, s) = recv_mpst_b_from_e(s)?;

    close_mpst_multi(s)
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_c_from_a(s)?;
    let (_, s) = recv_mpst_c_from_b(s)?;
    let s = send_mpst_c_to_d((), s);
    let s = send_mpst_c_to_e((), s);
    let s = send_mpst_c_to_a((), s);
    let s = send_mpst_c_to_b((), s);
    let (_, s) = recv_mpst_c_from_d(s)?;
    let (_, s) = recv_mpst_c_from_e(s)?;

    close_mpst_multi(s)
}

fn simple_five_endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_d_from_a(s)?;
    let (_, s) = recv_mpst_d_from_b(s)?;
    let (_, s) = recv_mpst_d_from_c(s)?;
    let s = send_mpst_d_to_e((), s);
    let s = send_mpst_d_to_a((), s);
    let s = send_mpst_d_to_b((), s);
    let s = send_mpst_d_to_c((), s);
    let (_, s) = recv_mpst_d_from_e(s)?;

    close_mpst_multi(s)
}

fn simple_five_endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_e_from_a(s)?;
    let (_, s) = recv_mpst_e_from_b(s)?;
    let (_, s) = recv_mpst_e_from_c(s)?;
    let (_, s) = recv_mpst_e_from_d(s)?;
    let s = send_mpst_e_to_a((), s);
    let s = send_mpst_e_to_b((), s);
    let s = send_mpst_e_to_c((), s);
    let s = send_mpst_e_to_d((), s);

    close_mpst_multi(s)
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_a, thread_b, thread_c, thread_d, thread_e) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
        black_box(simple_five_endpoint_d),
        black_box(simple_five_endpoint_e),
    );

    thread_a.join()?;
    thread_b.join()?;
    thread_c.join()?;
    thread_d.join()?;
    thread_e.join()?;

    Ok(())
}

/////////////////////////
// A
fn binary_a_to_b(s: AtoB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_a_to_c(s: AtoC) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_a_to_d(s: AtoD) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_a_to_e(s: AtoE) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}

// B
fn binary_b_to_a(s: BtoA) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_b_to_c(s: BtoC) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_b_to_d(s: BtoD) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_b_to_e(s: BtoE) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}

// C
fn binary_c_to_a(s: CtoA) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_c_to_b(s: CtoB) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_c_to_d(s: CtoD) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}
fn binary_c_to_e(s: CtoE) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}

// D
fn binary_d_to_a(s: DtoA) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_d_to_b(s: DtoB) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_d_to_c(s: DtoC) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_d_to_e(s: DtoE) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;

    close(s)
}

// E
fn binary_e_to_a(s: EtoA) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_e_to_b(s: EtoB) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_e_to_c(s: EtoC) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}
fn binary_e_to_d(s: EtoD) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;
    let s = send((), s);

    close(s)
}

fn all_binaries() -> Result<(), Box<dyn std::any::Any + std::marker::Send + 'static>> {
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

    thread_a_to_b.join()?;
    thread_a_to_c.join()?;
    thread_a_to_d.join()?;
    thread_a_to_e.join()?;

    thread_b_to_c.join()?;
    thread_b_to_d.join()?;
    thread_b_to_e.join()?;

    thread_c_to_d.join()?;
    thread_c_to_e.join()?;

    thread_d_to_e.join()?;

    Ok(())
}

/////////////////////////

fn long_protocol_mpst(c: &mut Criterion) {
    c.bench_function("long protocol MPST", |b| b.iter(|| all_mpst()));
}

fn long_protocol_binary(c: &mut Criterion) {
    c.bench_function("long protocol binary", |b| b.iter(|| all_binaries()));
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(180, 0))
}

criterion_group! {
    name = long_five_protocols;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = long_protocol_mpst, long_protocol_binary
}
criterion_main!(long_five_protocols);
