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
    Video(Recv<(), Recv<(), Recv<(), Recv<(), RecA>>>>),
    Stop(Recv<(), Recv<(), End>>),
}
type RecA = Recv<BinaryA, End>;
type FullA = Recv<(), Recv<(), RecA>>;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    let (_declare, s) = recv(s)?;
    let (_accept, s) = recv(s)?;

    recurs_a(s)
}

fn recurs_a(s: RecA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Video(s) => {
            let (_request_video_auth, s) = recv(s)?;
            let (_request_server, s) = recv(s)?;
            let (_send_video_auth, s) = recv(s)?;
            let (_send_video_server, s) = recv(s)?;
            recurs_a(s)
        },
        BinaryA::Stop(s) => {
            let (_close_auth, s) = recv(s)?;
            let (_close_server, s) = recv(s)?;
            close(s)
        },
    })
}

// C
type RecB = <RecA as Session>::Dual;

fn binary_video_b(s: RecB) -> Result<RecB, Box<dyn Error>> {
    let s = choose!(BinaryA::Video, s);
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

    let session = send((), session);
    let session = send((), session);

    threads.push(thread);
    sessions.push(session);

    let main = spawn(move || {
        for _ in 0..LOOPS {
            sessions = sessions
                .into_iter()
                .map(|s| binary_video_b(s).unwrap())
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

pub fn video_stream_binary(c: &mut Criterion) {
    c.bench_function("Video stream binary", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = video_stream_binary,
}

criterion_main! {
    bench
}
