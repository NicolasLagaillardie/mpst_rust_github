use criterion::{criterion_group, criterion_main, Criterion};

use std::thread::spawn;

use crossbeam_channel::{bounded, Receiver};

type S0 = Receiver<Receiver<Receiver<()>>>;
type S1 = Receiver<Receiver<()>>;
type S2 = Receiver<()>;
type S3 = ();

fn aux() {
    let main = spawn(move || {
        for _ in 0..LOOPS {
            let (sender_s_0, receiver_s_0) = bounded::<S0>(1);
            let (sender_s_1, receiver_s_1) = bounded::<S1>(1);
            let (sender_s_2, receiver_s_2) = bounded::<S2>(1);
            let (sender_s_3, receiver_s_3) = bounded::<S3>(1);

            sender_s_0.send(receiver_s_1).unwrap();
            let receiver_s_1_bis = receiver_s_0.recv().unwrap();

            sender_s_1.send(receiver_s_2).unwrap();
            let receiver_s_2_bis = receiver_s_1_bis.recv().unwrap();

            sender_s_2.send(receiver_s_3).unwrap();
            let receiver_s_3_bis = receiver_s_2_bis.recv().unwrap();

            sender_s_3.send(()).unwrap();
            receiver_s_3_bis.recv().unwrap();
        }
    });

    main.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 20;

pub fn fib_crossbeam(c: &mut Criterion) {
    c.bench_function(&format!("Fibo {LOOPS} crossbeam"), |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(10000);
    targets = fib_crossbeam,
}

criterion_main! {
    bench
}
