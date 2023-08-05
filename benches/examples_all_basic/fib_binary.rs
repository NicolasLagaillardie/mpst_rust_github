#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::{choose, offer};

use std::error::Error;
use std::marker;
use std::thread::{spawn, JoinHandle};

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

fn aux() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..3 {
        let (thread, s): (JoinHandle<()>, RecursB<i64>) =
            fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..LOOPS {
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
}

/////////////////////////

static LOOPS: i64 = 20;

pub fn fib(c: &mut Criterion) {
    c.bench_function(&format!("Fibo {LOOPS} binary"), |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(10000);
    targets = fib,
}

criterion_main! {
    bench
}
