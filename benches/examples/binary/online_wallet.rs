#![allow(clippy::type_complexity, dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
use mpstthree::{choose, offer};

use std::error::Error;
use std::thread::spawn;

// S
enum Binary0A {
    Success(Recv<(), Recv<(), Offer1A>>),
    // Not used
    Failure(Recv<(), Recv<(), End>>),
}
enum Binary1A {
    Continue(Recv<(), Offer1A>),
    Quit(Recv<(), End>),
}
type Rec0A = Recv<Binary0A, End>;
type Rec1A = Recv<Binary1A, End>;
type Offer1A = Recv<(), Rec1A>;
type FullA = Recv<(), Rec0A>;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    let (_hard_ping, s) = recv(s)?;

    recurs_0_a(s)
}

fn recurs_0_a(s: Rec0A) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Binary0A::Success(s) => {
            let (_login_ok_c, s) = recv(s)?;
            let (_login_ok_s, s) = recv(s)?;
            recurs_1_a(s)
        },
        Binary0A::Failure(s) => {
            let (_login_fail_c, s) = recv(s)?;
            let (_login_fail_s, s) = recv(s)?;
            close(s)
        },
    })
}

fn recurs_1_a(s: Offer1A) -> Result<(), Box<dyn Error>> {
    let (_account, s) = recv(s)?;

    offer!(s, {
        Binary1A::Continue(s) => {
            let (_pay, s) = recv(s)?;
            recurs_1_a(s)
        },
        Binary1A::Quit(s) => {
            let (_quit, s) = recv(s)?;
            close(s)
        },
    })
}

// C
type Choice1B = <Offer1A as Session>::Dual;
type FullB = <FullA as Session>::Dual;

fn binary_success_pay_b(s: Choice1B) -> Result<Choice1B, Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Binary1A::Continue, s);
    let s = send((), s);
    Ok(s)
}

fn binary_success_quit_b(s: Choice1B) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Binary1A::Quit, s);
    let s = send((), s);
    close(s)
}

// Not used
fn binary_quit_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Binary0A::Failure, s);
    let s = send((), s);
    let s = send((), s);
    close(s)
}

fn aux() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    let (thread, session) = fork_with_thread_id(black_box(binary_a));

    let session = send((), session);
    let session = choose!(Binary0A::Success, session);
    let session = send((), session);
    let session = send((), session);

    threads.push(thread);
    sessions.push(session);

    let main = spawn(move || {
        for _ in 0..LOOPS {
            sessions = sessions
                .into_iter()
                .map(|s| binary_success_pay_b(s).unwrap())
                .collect::<Vec<_>>();
        }

        sessions
            .into_iter()
            .for_each(|s| binary_success_quit_b(s).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn online_wallet_binary(c: &mut Criterion) {
    c.bench_function("Online wallet binary", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = online_wallet_binary,
}

criterion_main! {
    bench
}
