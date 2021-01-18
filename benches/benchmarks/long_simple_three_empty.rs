#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::{close, fork_with_thread_id, recv, send, End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose, choose_mpst_multi_to_all, close_mpst, create_normal_role,
    create_recv_mpst_session, create_send_mpst_session, create_sessionmpst, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstThree, 3);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);

// Create new send functions
// A
create_send_mpst_session!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstThree,
    3,
    1
);
create_send_mpst_session!(
    send_mpst_a_to_c,
    RoleC,
    next_c,
    RoleA,
    SessionMpstThree,
    3,
    2
);
// B
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
    send_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstThree,
    3,
    2
);
// C
create_send_mpst_session!(
    send_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstThree,
    3,
    1
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

// Create new recv functions and related types
// A
create_recv_mpst_session!(
    recv_mpst_a_to_b,
    RoleB,
    next_b,
    RoleA,
    SessionMpstThree,
    3,
    1
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
// B
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
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    RoleB,
    SessionMpstThree,
    3,
    2
);
// C
create_recv_mpst_session!(
    recv_mpst_c_to_a,
    RoleA,
    next_a,
    RoleC,
    SessionMpstThree,
    3,
    1
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
    More(
        SessionMpstThree<
            Recv<(), Send<(), End>>,
            Recv<(), Send<(), RecursAtoC>>,
            RoleC<RoleC<RoleB<RoleB<RoleC<RoleEnd>>>>>,
            NameA,
        >,
    ),
    Done(SessionMpstThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = Recv<BranchingCforA, End>;
// B
enum BranchingCforB {
    More(
        SessionMpstThree<
            Send<(), Recv<(), End>>,
            Recv<(), Send<(), RecursBtoC>>,
            RoleC<RoleC<RoleA<RoleA<RoleC<RoleEnd>>>>>,
            NameB,
        >,
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
            let (_, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_c((), s);
            let (_, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_b((), s);
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
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_to_a(s)?;
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

            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_to_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_to_b(s)?;

            recurs_c(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        black_box(simple_five_endpoint_a),
        black_box(simple_five_endpoint_b),
        black_box(simple_five_endpoint_c),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();

    Ok(())
}

/////////////////////////
// A
enum BinaryA {
    More(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_a_to_b(s: RecursA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::More(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            binary_a_to_b(s)
        },
    })
}

// B
type RecursB = <RecursA as Session>::Dual;
fn binary_b_to_a(s: Send<(), Recv<(), RecursB>>) -> Result<RecursB, Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..3 {
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..SIZE {
            sessions = sessions
                .into_iter()
                .map(|s| binary_b_to_a(choose!(BinaryA::More, s)).unwrap())
                .collect::<Vec<_>>();
        }

        sessions
            .into_iter()
            .for_each(|s| close(choose!(BinaryA::Done, s)).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();

    Ok(())
}

/////////////////////////

static SIZE: i64 = 0;

fn long_simple_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("long three empty simple protocol MPST {}", SIZE),
        |b| b.iter(|| all_mpst()),
    );
}

fn long_simple_protocol_binary(c: &mut Criterion) {
    c.bench_function(
        &format!("long three empty simple protocol binary {}", SIZE),
        |b| b.iter(|| all_binaries()),
    );
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(20, 0))
}

criterion_group! {
    name = long_three_empty_simple_protocols;
    config = long_warmup();
    targets = long_simple_protocol_mpst, long_simple_protocol_binary
}
criterion_main!(long_three_empty_simple_protocols);
