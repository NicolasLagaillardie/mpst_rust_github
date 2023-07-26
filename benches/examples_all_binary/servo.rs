#![allow(clippy::type_complexity)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};

use std::error::Error;
use std::thread::spawn;

// S
type FullA = Recv<(), Recv<(), Recv<(), Recv<(), Recv<(), End>>>>>;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    let (_get_web_page_load_state, s) = recv(s)?;
    let (_outstanding_web_fonts, s) = recv(s)?;
    let (_get_current_state, s) = recv(s)?;
    let (_document_loading, s) = recv(s)?;
    let (_web_font_loaded, _s) = recv(s)?;

    Ok(())
}

// C
type FullB = <FullA as Session>::Dual;

fn binary_b(s: FullB) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
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
        sessions.into_iter().for_each(|s| binary_b(s).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

/////////////////////////

pub fn servo_binary(c: &mut Criterion) {
    c.bench_function("Servo binary", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = servo_binary,
}

criterion_main! {
    bench
}
