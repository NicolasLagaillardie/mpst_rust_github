use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose, choose_mpst_multi_to_all, close_mpst, create_normal_role,
    create_recv_mpst_session, create_send_mpst_session, create_sessionmpst, offer, offer_mpst,
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
    send_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstThree,
    3,
    2
);
create_send_mpst_session!(
    send_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstThree,
    3,
    2
);
create_send_mpst_session!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstThree,
    3,
    1
);

// Create new recv functions and related types
create_recv_mpst_session!(
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstThree,
    3,
    2
);
create_recv_mpst_session!(
    recv_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstThree,
    3,
    2
);
create_recv_mpst_session!(
    recv_mpst_c_to_b,
    RoleB,
    next_b,
    RoleC,
    SessionMpstThree,
    3,
    2
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
enum BranchingCforA {
    More(SessionMpstThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>),
    Done(SessionMpstThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = Recv<BranchingCforA, End>;
// B
enum BranchingCforB {
    More(
        SessionMpstThree<End, Recv<(), Send<(), RecursBtoC>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameB>,
    ),
    Done(SessionMpstThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = Recv<BranchingCforB, End>;
// C
type ChooseCforAtoC = Send<BranchingCforA, End>;
type ChooseCforBtoC = Send<BranchingCforB, End>;

// Creating the MP sessions
type EndpointA = SessionMpstThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = SessionMpstThree<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = SessionMpstThree<ChooseCforAtoC, ChooseCforBtoC, RoleA<RoleB<RoleEnd>>, NameC>;

fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_to_c, {
        BranchingCforA::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingCforA::More(s) => {
            simple_five_endpoint_a(s)
        },
    })
}

fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_to_c, {
        BranchingCforB::Done(s) => {
            close_mpst_multi(s)
        },
        BranchingCforB::More(s) => {
            let (_, s) = recv_mpst_b_to_c(s)?;
            let s = send_mpst_b_to_c((), s);
            simple_five_endpoint_b(s)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, SIZE)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_c_to_a,
                send_mpst_c_to_b, =>
                BranchingCforA::Done,
                BranchingCforB::Done, =>
                RoleA,
                RoleB, =>
                RoleC,
                SessionMpstThree,
                3
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_c_to_a,
                send_mpst_c_to_b, =>
                BranchingCforA::More,
                BranchingCforB::More, =>
                RoleA,
                RoleB, =>
                RoleC,
                SessionMpstThree,
                3
            );

            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_to_b(s)?;

            recurs_c(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {

    let (_, thread_b, thread_c) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
    );

    thread_b.join().unwrap();
    thread_c.join().unwrap();

    Ok(())
}

/////////////////////////
// B
enum BinaryB {
    More(Recv<(), Send<(), RecursB>>),
    Done(End),
}
type RecursB = Recv<BinaryB, End>;
fn binary_b_to_c(s: RecursB) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryB::More(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            binary_b_to_c(s)
        },
        BinaryB::Done(s) => {
            close(s)
        },
    })
}

// C
type RecursC = <RecursB as Session>::Dual;
fn binary_c_to_b(s: RecursC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose!(BinaryB::Done, s);
            close(s)
        }
        i => {
            let s = choose!(BinaryB::More, s);
            let s = send((), s);
            let (_, s) = recv(s)?;
            binary_c_to_b(s, i - 1)
        }
    }
}

fn all_binaries(index: i64) -> Result<(), Box<dyn Error>> {
    let (thread_b_to_c, s_b_to_c): (JoinHandle<()>, RecursC) =
        fork_with_thread_id(black_box(binary_b_to_c));

    binary_c_to_b(black_box(s_b_to_c), index).unwrap();
    thread_b_to_c.join().unwrap();

    ////////////////////

    let (thread_b_to_c, s_b_to_c): (JoinHandle<()>, RecursC) =
        fork_with_thread_id(black_box(binary_b_to_c));

    binary_c_to_b(black_box(s_b_to_c), index).unwrap();
    thread_b_to_c.join().unwrap();

    Ok(())
}

/////////////////////////

static SIZE: i64 = 1000;

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function("long simple protocol MPST", |b| b.iter(|| all_mpst()));
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function("long simple protocol binary", |b| {
        b.iter(|| all_binaries(SIZE))
    });
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(180, 0))
}

criterion_group! {
    name = long_three_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary
}
criterion_main!(long_three_simple_protocols);
