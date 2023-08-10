#![allow(clippy::type_complexity)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::{choose, offer};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::thread::spawn;

// S
enum Binary0A {
    Success(Recv<(), Rec0A>),
    Failure(Recv<(), Rec1A>),
}
enum Binary1A {
    Restart(Recv<(), Rec0A>),
    Stop(Recv<(), End>),
}
type Rec0A = Recv<Binary0A, End>;
type Rec1A = Recv<Binary1A, End>;
type FullA = Recv<(), Rec0A>;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    let (_start_logs, s) = recv(s)?;

    recurs_0_a(s)
}

fn recurs_0_a(s: Rec0A) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Binary0A::Success(s) => {
            let (_success, s) = recv(s)?;
            recurs_0_a(s)
        },
        Binary0A::Failure(s) => {
            let (_failure_controller, s) = recv(s)?;
            recurs_1_a(s)
        },
    })
}

fn recurs_1_a(s: Rec1A) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Binary1A::Restart(s) => {
            let (_restart_logs, s) = recv(s)?;
            recurs_0_a(s)
        },
        Binary1A::Stop(s) => {
            let (_stop_logs, s) = recv(s)?;
            close(s)
        },
    })
}

// C
type Choice0B = <Rec0A as Session>::Dual;

fn binary_success_b(s: Choice0B) -> Result<Choice0B, Box<dyn Error>> {
    let s = choose!(Binary0A::Success, s);
    let s = send((), s);
    Ok(s)
}

fn binary_failure_restart_b(s: Choice0B) -> Result<Choice0B, Box<dyn Error>> {
    let s = choose!(Binary0A::Failure, s);
    let s: Send<Binary1A, End> = send((), s);
    let s = choose!(Binary1A::Restart, s);
    let s = send((), s);
    Ok(s)
}

fn binary_failure_close_b(s: Choice0B) -> Result<(), Box<dyn Error>> {
    let s = choose!(Binary0A::Failure, s);
    let s: Send<Binary1A, End> = send((), s);
    let s = choose!(Binary1A::Stop, s);
    let s = send((), s);
    close(s)
}

fn aux() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    let (thread, session) = fork_with_thread_id(black_box(binary_a));

    let session = send((), session);

    threads.push(thread);
    sessions.push(session);

    let main = spawn(move || {
        for _ in 0..LOOPS {
            let choice = thread_rng().gen_range(1..=2);

            if choice != 1 {
                sessions = sessions
                    .into_iter()
                    .map(|s| binary_success_b(s).unwrap())
                    .collect::<Vec<_>>();
            } else {
                sessions = sessions
                    .into_iter()
                    .map(|s| binary_failure_restart_b(s).unwrap())
                    .collect::<Vec<_>>();
            }
        }

        sessions
            .into_iter()
            .for_each(|s| binary_failure_close_b(s).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn logging_binary(c: &mut Criterion) {
    c.bench_function("Logging binary", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(10000);
    targets = logging_binary,
}

criterion_main! {
    bench
}
