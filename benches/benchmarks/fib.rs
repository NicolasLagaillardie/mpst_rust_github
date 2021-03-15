#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, choose_mpst_multi_to_all, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer, offer_mpst,
};

use std::error::Error;
use std::marker;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

// global protocol Fibonacci(role A, role B)
// {
//     rec Fib
//     {
//         choice at A
//         {
//             fibonacci(Long) from A to B;
//             fibonacci(Long) from B to A;
//             continue Fib;
//         }
//         or
//         {
//             stop() from A to B;
//         }
//     }
// }

// Create the new SessionMpst for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstThree, 3);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, next_a, RoleADual, next_a_dual |
    RoleB, next_b, RoleBDual, next_b_dual |
    RoleC, next_c, RoleCDual, next_c_dual |
);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b,
    RoleB,
    next_b,
    1 |
    send_mpst_a_to_c,
    RoleC,
    next_c,
    2 | =>
    RoleA,
    SessionMpstThree,
    3
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a,
    RoleA,
    next_a,
    1 | =>
    RoleB,
    SessionMpstThree,
    3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b,
    RoleB,
    next_b,
    1 | =>
    RoleA,
    SessionMpstThree,
    3
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a,
    RoleA,
    next_a,
    1 | =>
    RoleB,
    SessionMpstThree,
    3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a,
    RoleA,
    next_a,
    1 | =>
    RoleC,
    SessionMpstThree,
    3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
// A
type Choose0fromAtoB<N> = Send<Branching0fromAtoB<N>, End>;
type Choose0fromAtoC = Send<Branching0fromAtoC, End>;
// B
enum Branching0fromAtoB<N: marker::Send> {
    More(
        SessionMpstThree<Recv<N, Send<N, RecursBtoA<N>>>, End, RoleA<RoleA<RoleA<RoleEnd>>>, NameB>,
    ),
    Done(SessionMpstThree<End, End, RoleEnd, NameB>),
}
type RecursBtoA<N> = Recv<Branching0fromAtoB<N>, End>;
// C
enum Branching0fromAtoC {
    More(SessionMpstThree<RecursCtoA, End, RoleA<RoleEnd>, NameC>),
    Done(SessionMpstThree<End, End, RoleEnd, NameC>),
}
type RecursCtoA = Recv<Branching0fromAtoC, End>;

// Creating the MP sessions
type EndpointA<N> =
    SessionMpstThree<Choose0fromAtoB<N>, Choose0fromAtoC, RoleB<RoleC<RoleEnd>>, NameA>;
type EndpointB<N> = SessionMpstThree<RecursBtoA<N>, End, RoleA<RoleEnd>, NameB>;
type EndpointC = SessionMpstThree<RecursCtoA, End, RoleA<RoleEnd>, NameC>;

// Functions
fn simple_five_endpoint_a(s: EndpointA<i64>) -> Result<(), Box<dyn Error>> {
    recurs_a(s, SIZE, 1)
}

fn recurs_a(s: EndpointA<i64>, index: i64, old: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_a_to_b,
                send_mpst_a_to_c, =>
                Branching0fromAtoB::<i64>::Done,
                Branching0fromAtoC::Done, =>
                RoleB,
                RoleC, =>
                RoleA,
                SessionMpstThree,
                3,
                1
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_mpst_a_to_b,
                send_mpst_a_to_c, =>
                Branching0fromAtoB::<i64>::More,
                Branching0fromAtoC::More, =>
                RoleB,
                RoleC, =>
                RoleA,
                SessionMpstThree,
                3,
                1
            );

            let s = send_mpst_a_to_b(old, s);
            let (new, s) = recv_mpst_a_from_b(s)?;

            recurs_a(s, i - 1, new)
        }
    }
}

fn simple_five_endpoint_b(s: EndpointB<i64>) -> Result<(), Box<dyn Error>> {
    recurs_b(s, 0)
}

fn recurs_b(s: EndpointB<i64>, old: i64) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_a, {
        Branching0fromAtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromAtoB::More(s) => {
            let (new, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a(new + old, s);
            recurs_b(s, new + old)
        },
    })
}

fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_a, {
        Branching0fromAtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromAtoC::More(s) => {
            simple_five_endpoint_c(s)
        },
    })
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
enum BinaryA<N: marker::Send> {
    More(Recv<N, Send<N, RecursA<N>>>),
    Done(End),
}
type RecursA<N> = Recv<BinaryA<N>, End>;

fn binary_a_to_b(s: RecursA<i64>) -> Result<(), Box<dyn Error>> {
    recurs_a_binary(s, 1)
}
fn recurs_a_binary(s: RecursA<i64>, old: i64) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::More(s) => {
            let (new, s) = recv(s)?;
            let s = send(old + new, s);
            recurs_a_binary(s, old + new)
        },
    })
}

// B
type RecursB<N> = <RecursA<N> as Session>::Dual;
fn binary_b_to_a(s: Send<i64, Recv<i64, RecursB<i64>>>) -> Result<RecursB<i64>, Box<dyn Error>> {
    recurs_b_binary(s, 0)
}
fn recurs_b_binary(
    s: Send<i64, Recv<i64, RecursB<i64>>>,
    old: i64,
) -> Result<RecursB<i64>, Box<dyn Error>> {
    let s = send(old, s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() -> Result<(), Box<dyn Error>> {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..3 {
        let (thread, s): (JoinHandle<()>, RecursB<i64>) =
            fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..SIZE {
            sessions = sessions
                .into_iter()
                .map(|s| binary_b_to_a(choose!(BinaryA::<i64>::More, s)).unwrap())
                .collect::<Vec<_>>();
        }

        sessions
            .into_iter()
            .for_each(|s| close(choose!(BinaryA::<i64>::Done, s)).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();

    Ok(())
}

/////////////////////////

static SIZE: i64 = 10;

fn fibo_mpst(c: &mut Criterion) {
    c.bench_function(&format!("Fibo MPST {}", SIZE), |b| b.iter(|| all_mpst()));
}

fn fibo_binary(c: &mut Criterion) {
    c.bench_function(&format!("Fibo binary {}", SIZE), |b| {
        b.iter(|| all_binaries())
    });
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(30, 0))
}

criterion_group! {
    name = fibo;
    config = long_warmup();
    targets = fibo_mpst, fibo_binary
}

criterion_main!(fibo);
