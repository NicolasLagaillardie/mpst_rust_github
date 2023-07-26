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
    Up(Recv<(), Recv<(), Recv<(), Recv<(), OfferA>>>>),
    Failure(Recv<(), Recv<(), Recv<(), OfferA>>>),
    Stop(Recv<(), Recv<(), Recv<(), End>>>),
}
type RecA = Recv<BinaryA, End>;
type OfferA = Recv<(), Recv<(), RecA>>;
type FullA = Recv<(), Recv<(), Recv<(), OfferA>>>;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    let (_start_storage, s) = recv(s)?;
    let (_start_api, s) = recv(s)?;
    let (_hard_ping, s) = recv(s)?;

    recurs_a(s)
}

fn recurs_a(s: OfferA) -> Result<(), Box<dyn Error>> {
    let (_request, s) = recv(s)?;
    let (_get_mode, s) = recv(s)?;

    offer!(s, {
        BinaryA::Up(s) => {
            let (_up, s) = recv(s)?;
            let (_request_storage, s) = recv(s)?;
            let (_response_api, s) = recv(s)?;
            let (_response_user, s) = recv(s)?;
            recurs_a(s)
        },
        BinaryA::Failure(s) => {
            let (_failure_api, s) = recv(s)?;
            let (_restart_storage, s) = recv(s)?;
            let (_failure_user, s) = recv(s)?;
            recurs_a(s)
        },
        BinaryA::Stop(s) => {
            let (_stop_api, s) = recv(s)?;
            let (_stop_storage, s) = recv(s)?;
            let (_stop_user, s) = recv(s)?;
            close(s)
        },
    })
}

// C
type ChoiceB = <OfferA as Session>::Dual;

fn binary_up_b(s: ChoiceB) -> Result<ChoiceB, Box<dyn Error>> {
    let s = send((), s);
    let s = send((), s);
    let s = choose!(BinaryA::Up, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn binary_failure_b(s: ChoiceB) -> Result<ChoiceB, Box<dyn Error>> {
    let s = send((), s);
    let s = send((), s);
    let s = choose!(BinaryA::Failure, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn binary_close_b(s: ChoiceB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = send((), s);
    let s = choose!(BinaryA::Stop, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    close(s)
}

fn aux() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    let (thread, session) = fork_with_thread_id(black_box(binary_a));

    let session = send((), session);
    let session = send((), session);
    let session = send((), session);

    threads.push(thread);
    sessions.push(session);

    let main = spawn(move || {
        for _ in 0..LOOPS {
            let choice = thread_rng().gen_range(1..=2);

            if choice != 1 {
                sessions = sessions
                    .into_iter()
                    .map(|s| binary_up_b(s).unwrap())
                    .collect::<Vec<_>>();
            } else {
                sessions = sessions
                    .into_iter()
                    .map(|s| binary_failure_b(s).unwrap())
                    .collect::<Vec<_>>();
            }
        }

        sessions
            .into_iter()
            .for_each(|s| binary_close_b(s).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn circuit_breaker_binary(c: &mut Criterion) {
    c.bench_function("Circuit breaker binary", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = circuit_breaker_binary,
}

criterion_main! {
    bench
}
