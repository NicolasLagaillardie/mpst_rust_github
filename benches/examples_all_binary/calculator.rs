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
enum BinaryS {
    Sum(Send<i32, End>),
    Diff(Send<i32, End>),
}
type OfferS = Recv<BinaryS, End>;
type FullS = Recv<i32, Recv<i32, OfferS>>;

fn binary_s(s: FullS) -> Result<(), Box<dyn Error>> {
    let (elt_1, s) = recv(s)?;
    let (elt_2, s) = recv(s)?;

    offer!(s, {
        BinaryS::Sum(s) => {
            let s = send(elt_1 + elt_2, s);
            close(s)
        },
        BinaryS::Diff(s) => {
            let s = send(elt_1 - elt_2, s);
            close(s)
        },
    })
}

// C
type ChoiceC = <OfferS as Session>::Dual;
type FullC = <FullS as Session>::Dual;

fn binary_c(s: FullC) -> Result<ChoiceC, Box<dyn Error>> {
    let s = send(thread_rng().gen_range(1..=100), s);
    let s = send(thread_rng().gen_range(1..=100), s);
    Ok(s)
}

fn all_binaries() {
    let (thread, session) = fork_with_thread_id(black_box(binary_s));

    let main = spawn(move || {
        let choice: i32 = thread_rng().gen_range(1..=2);

        let session = binary_c(session).unwrap();

        if choice != 1 {
            let session = choose!(BinaryS::Sum, session);
            let (_, session) = recv(session).unwrap();
            close(session).unwrap();
        } else {
            let session = choose!(BinaryS::Diff, session);
            let (_, session) = recv(session).unwrap();
            close(session).unwrap();
        }

        thread.join().unwrap()
    });

    main.join().unwrap();
}

/////////////////////////

pub fn calculator_binary(c: &mut Criterion) {
    c.bench_function("Calculator binary", |b| b.iter(all_binaries));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = calculator_binary,
}

criterion_main! {
    bench
}
