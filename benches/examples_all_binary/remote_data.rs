#![allow(clippy::type_complexity)]

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
enum BinaryA {
    Data(Recv<(), Recv<(), Recv<(), Recv<(), RecA>>>>),
    Stop(Recv<(), Recv<(), End>>),
}
type RecA = Recv<BinaryA, End>;

fn binary_a(s: RecA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Data(s) => {
            let (_get_data_satellite, s) = recv(s)?;
            let (_get_data_sensor, s) = recv(s)?;
            let (_data_satellite, s) = recv(s)?;
            let (_data_sensor, s) = recv(s)?;
            binary_a(s)
        },
        BinaryA::Stop(s) => {
            let (_close_satellite, s) = recv(s)?;
            let (_close_sensor, s) = recv(s)?;
            close(s)
        },
    })
}

// C
type RecB = <RecA as Session>::Dual;

fn binary_data_b(s: RecB) -> Result<RecB, Box<dyn Error>> {
    let s = choose!(BinaryA::Data, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn binary_close_b(s: RecB) -> Result<(), Box<dyn Error>> {
    let s = choose!(BinaryA::Stop, s);
    let s = send((), s);
    let s = send((), s);
    close(s)
}

fn aux() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    let (thread, session) = fork_with_thread_id(black_box(binary_a));

    threads.push(thread);
    sessions.push(session);

    let main = spawn(move || {
        for _ in 0..LOOPS {
            sessions = sessions
                .into_iter()
                .map(|s| binary_data_b(s).unwrap())
                .collect::<Vec<_>>();
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

pub fn remote_data_binary(c: &mut Criterion) {
    c.bench_function(&format!("Remote data binary {LOOPS}"), |b| {
        b.iter(aux)
    });
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = remote_data_binary,
}

criterion_main! {
    bench
}
