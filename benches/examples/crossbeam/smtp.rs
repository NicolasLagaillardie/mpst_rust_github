use criterion::{criterion_group, criterion_main, Criterion};

use std::thread::spawn;

use crossbeam_channel::{bounded, Receiver};

type S0 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<
                    Receiver<
                        Receiver<
                            Receiver<
                                Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;
type S1 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<
                    Receiver<
                        Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>,
                    >,
                >,
            >,
        >,
    >,
>;
type S2 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>,
            >,
        >,
    >,
>;
type S3 = Receiver<
    Receiver<
        Receiver<
            Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>,
        >,
    >,
>;
type S4 = Receiver<
    Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>>,
>;
type S5 =
    Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>>;
type S6 = Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>;
type S7 = Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>;
type S8 = Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>;
type S9 = Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>;
type S10 = Receiver<Receiver<Receiver<Receiver<()>>>>;
type S11 = Receiver<Receiver<Receiver<()>>>;
type S12 = Receiver<Receiver<()>>;
type S13 = Receiver<()>;
type S14 = ();

fn aux() {
    let main = spawn(move || {
        for _ in 0..LOOPS {
            let (sender_s_0, receiver_s_0) = bounded::<S0>(1);
            let (sender_s_1, receiver_s_1) = bounded::<S1>(1);
            let (sender_s_2, receiver_s_2) = bounded::<S2>(1);
            let (sender_s_3, receiver_s_3) = bounded::<S3>(1);
            let (sender_s_4, receiver_s_4) = bounded::<S4>(1);
            let (sender_s_5, receiver_s_5) = bounded::<S5>(1);
            let (sender_s_6, receiver_s_6) = bounded::<S6>(1);
            let (sender_s_7, receiver_s_7) = bounded::<S7>(1);
            let (sender_s_8, receiver_s_8) = bounded::<S8>(1);
            let (sender_s_9, receiver_s_9) = bounded::<S9>(1);
            let (sender_s_10, receiver_s_10) = bounded::<S10>(1);
            let (sender_s_11, receiver_s_11) = bounded::<S11>(1);
            let (sender_s_12, receiver_s_12) = bounded::<S12>(1);
            let (sender_s_13, receiver_s_13) = bounded::<S13>(1);
            let (sender_s_14, receiver_s_14) = bounded::<S14>(1);

            sender_s_0.send(receiver_s_1).unwrap();
            let receiver_s_1_bis = receiver_s_0.recv().unwrap();

            sender_s_1.send(receiver_s_2).unwrap();
            let receiver_s_2_bis = receiver_s_1_bis.recv().unwrap();

            sender_s_2.send(receiver_s_3).unwrap();
            let receiver_s_3_bis = receiver_s_2_bis.recv().unwrap();

            sender_s_3.send(receiver_s_4).unwrap();
            let receiver_s_4_bis = receiver_s_3_bis.recv().unwrap();

            sender_s_4.send(receiver_s_5).unwrap();
            let receiver_s_5_bis = receiver_s_4_bis.recv().unwrap();

            sender_s_5.send(receiver_s_6).unwrap();
            let receiver_s_6_bis = receiver_s_5_bis.recv().unwrap();

            sender_s_6.send(receiver_s_7).unwrap();
            let receiver_s_7_bis = receiver_s_6_bis.recv().unwrap();

            sender_s_7.send(receiver_s_8).unwrap();
            let receiver_s_8_bis = receiver_s_7_bis.recv().unwrap();

            sender_s_8.send(receiver_s_9).unwrap();
            let receiver_s_9_bis = receiver_s_8_bis.recv().unwrap();

            sender_s_9.send(receiver_s_10).unwrap();
            let receiver_s_10_bis = receiver_s_9_bis.recv().unwrap();

            sender_s_10.send(receiver_s_11).unwrap();
            let receiver_s_11_bis = receiver_s_10_bis.recv().unwrap();

            sender_s_11.send(receiver_s_12).unwrap();
            let receiver_s_12_bis = receiver_s_11_bis.recv().unwrap();

            sender_s_12.send(receiver_s_13).unwrap();
            let receiver_s_13_bis = receiver_s_12_bis.recv().unwrap();

            sender_s_13.send(receiver_s_14).unwrap();
            let receiver_s_14_bis = receiver_s_13_bis.recv().unwrap();

            sender_s_14.send(()).unwrap();
            receiver_s_14_bis.recv().unwrap();
        }
    });

    main.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn smtp_crossbeam(c: &mut Criterion) {
    c.bench_function("SMTP crossbeam", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = smtp_crossbeam,
}

criterion_main! {
    bench
}
