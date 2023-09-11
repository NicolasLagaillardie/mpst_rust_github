#![allow(clippy::type_complexity)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
use mpstthree::{choose, offer};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::thread::spawn;

// S
enum BinaryA {
    Success(Recv<(), Recv<(), Recv<(), Recv<(), Recv<(), End>>>>>),
    Failure(Recv<(), Recv<(), Recv<(), End>>>),
}
type FullA = Recv<(), Recv<(), Recv<(), Recv<(), Recv<(), Recv<BinaryA, End>>>>>>;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    let (_start, s) = recv(s)?;
    let (_redirect, s) = recv(s)?;
    let (_login, s) = recv(s)?;
    let (_auth, s) = recv(s)?;
    let (_password, s) = recv(s)?;

    offer!(s, {
        BinaryA::Success(s) => {
            let (_success_c, s) = recv(s)?;
            let (_success_s, s) = recv(s)?;
            let (_get_token, s) = recv(s)?;
            let (_put_token_s, s) = recv(s)?;
            let (_put_token_c, s) = recv(s)?;
            close(s)
        },
        BinaryA::Failure(s) => {
            let (_failure_c, s) = recv(s)?;
            let (_failures, s) = recv(s)?;
            let (_received_c, s) = recv(s)?;
            close(s)
        },
    })
}

// C
type FullB = <FullA as Session>::Dual;

fn binary_success_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = choose!(BinaryA::Success, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    close(s)
}

fn binary_failure_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = choose!(BinaryA::Failure, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    close(s)
}

fn aux() {
    let (thread, session) = fork_with_thread_id(black_box(binary_a));

    let main = spawn(move || {
        let choice = thread_rng().gen_range(1..=2);

        if choice != 1 {
            binary_success_b(session).unwrap();
        } else {
            binary_failure_b(session).unwrap();
        }

        thread.join().unwrap();
    });

    main.join().unwrap();
}

/////////////////////////

pub fn o_auth_binary(c: &mut Criterion) {
    c.bench_function("oAuth binary", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = o_auth_binary,
}

criterion_main! {
    bench
}
