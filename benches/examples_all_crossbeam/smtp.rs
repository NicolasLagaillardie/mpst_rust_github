#![allow(clippy::type_complexity)]

use criterion::{criterion_group, criterion_main, Criterion};

use std::thread::spawn;

use crossbeam_channel::{bounded, Receiver};

type S0 = Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>>>>>>>>>>>>>>>>>>>>;
type S1 = Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>>>>>>>>>>>>>>>>>>>;
type S2 = Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>>>>>>>>>>>>>>>>>>;
type S3 = Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>>>>>>>>>>>>>>>>>;
type S4 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<
                    Receiver<
                        Receiver<
                            Receiver<
                                Receiver<
                                    Receiver<
                                        Receiver<
                                            Receiver<
                                                Receiver<
                                                    Receiver<
                                                        Receiver<
                                                            Receiver<
                                                                Receiver<
                                                                    Receiver<
                                                                        Receiver<
                                                                            Receiver<
                                                                                Receiver<
                                                                                    Receiver<
                                                                                        Receiver<()>,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;
type S5 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<
                    Receiver<
                        Receiver<
                            Receiver<
                                Receiver<
                                    Receiver<
                                        Receiver<
                                            Receiver<
                                                Receiver<
                                                    Receiver<
                                                        Receiver<
                                                            Receiver<
                                                                Receiver<
                                                                    Receiver<
                                                                        Receiver<
                                                                            Receiver<
                                                                                Receiver<
                                                                                    Receiver<()>,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;
type S6 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<
                    Receiver<
                        Receiver<
                            Receiver<
                                Receiver<
                                    Receiver<
                                        Receiver<
                                            Receiver<
                                                Receiver<
                                                    Receiver<
                                                        Receiver<
                                                            Receiver<
                                                                Receiver<
                                                                    Receiver<
                                                                        Receiver<
                                                                            Receiver<Receiver<()>>,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;
type S7 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<
                    Receiver<
                        Receiver<
                            Receiver<
                                Receiver<
                                    Receiver<
                                        Receiver<
                                            Receiver<
                                                Receiver<
                                                    Receiver<
                                                        Receiver<
                                                            Receiver<
                                                                Receiver<
                                                                    Receiver<
                                                                        Receiver<Receiver<()>>,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;
type S8 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<
                    Receiver<
                        Receiver<
                            Receiver<
                                Receiver<
                                    Receiver<
                                        Receiver<
                                            Receiver<
                                                Receiver<
                                                    Receiver<
                                                        Receiver<
                                                            Receiver<
                                                                Receiver<Receiver<Receiver<()>>>,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;
type S9 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<
                    Receiver<
                        Receiver<
                            Receiver<
                                Receiver<
                                    Receiver<
                                        Receiver<
                                            Receiver<
                                                Receiver<
                                                    Receiver<
                                                        Receiver<Receiver<Receiver<Receiver<()>>>>,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;
type S10 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<
                    Receiver<
                        Receiver<
                            Receiver<
                                Receiver<
                                    Receiver<
                                        Receiver<
                                            Receiver<
                                                Receiver<
                                                    Receiver<Receiver<Receiver<Receiver<()>>>>,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;
type S11 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<
                    Receiver<
                        Receiver<
                            Receiver<
                                Receiver<
                                    Receiver<
                                        Receiver<
                                            Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;
type S12 = Receiver<
    Receiver<
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
    >,
>;
type S13 = Receiver<
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
type S14 = Receiver<
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
type S15 = Receiver<
    Receiver<
        Receiver<
            Receiver<
                Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>,
            >,
        >,
    >,
>;
type S16 = Receiver<
    Receiver<
        Receiver<
            Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>,
        >,
    >,
>;
type S17 = Receiver<
    Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>>,
>;
type S18 =
    Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>>;
type S19 = Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>>;
type S20 = Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>>;
type S21 = Receiver<Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>>;
type S22 = Receiver<Receiver<Receiver<Receiver<Receiver<()>>>>>;
type S23 = Receiver<Receiver<Receiver<Receiver<()>>>>;
type S24 = Receiver<Receiver<Receiver<()>>>;
type S25 = Receiver<Receiver<()>>;
type S26 = Receiver<()>;
type S27 = ();

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
            let (sender_s_15, receiver_s_15) = bounded::<S15>(1);
            let (sender_s_16, receiver_s_16) = bounded::<S16>(1);
            let (sender_s_17, receiver_s_17) = bounded::<S17>(1);
            let (sender_s_18, receiver_s_18) = bounded::<S18>(1);
            let (sender_s_19, receiver_s_19) = bounded::<S19>(1);
            let (sender_s_20, receiver_s_20) = bounded::<S20>(1);
            let (sender_s_21, receiver_s_21) = bounded::<S21>(1);
            let (sender_s_22, receiver_s_22) = bounded::<S22>(1);
            let (sender_s_23, receiver_s_23) = bounded::<S23>(1);
            let (sender_s_24, receiver_s_24) = bounded::<S24>(1);
            let (sender_s_25, receiver_s_25) = bounded::<S25>(1);
            let (sender_s_26, receiver_s_26) = bounded::<S26>(1);
            let (sender_s_27, receiver_s_27) = bounded::<S27>(1);

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

            sender_s_14.send(receiver_s_15).unwrap();
            let receiver_s_15_bis = receiver_s_14_bis.recv().unwrap();

            sender_s_15.send(receiver_s_16).unwrap();
            let receiver_s_16_bis = receiver_s_15_bis.recv().unwrap();

            sender_s_16.send(receiver_s_17).unwrap();
            let receiver_s_17_bis = receiver_s_16_bis.recv().unwrap();

            sender_s_17.send(receiver_s_18).unwrap();
            let receiver_s_18_bis = receiver_s_17_bis.recv().unwrap();

            sender_s_18.send(receiver_s_19).unwrap();
            let receiver_s_19_bis = receiver_s_18_bis.recv().unwrap();

            sender_s_19.send(receiver_s_20).unwrap();
            let receiver_s_20_bis = receiver_s_19_bis.recv().unwrap();

            sender_s_20.send(receiver_s_21).unwrap();
            let receiver_s_21_bis = receiver_s_20_bis.recv().unwrap();

            sender_s_21.send(receiver_s_22).unwrap();
            let receiver_s_22_bis = receiver_s_21_bis.recv().unwrap();

            sender_s_22.send(receiver_s_23).unwrap();
            let receiver_s_23_bis = receiver_s_22_bis.recv().unwrap();

            sender_s_23.send(receiver_s_24).unwrap();
            let receiver_s_24_bis = receiver_s_23_bis.recv().unwrap();

            sender_s_24.send(receiver_s_25).unwrap();
            let receiver_s_25_bis = receiver_s_24_bis.recv().unwrap();

            sender_s_25.send(receiver_s_26).unwrap();
            let receiver_s_26_bis = receiver_s_25_bis.recv().unwrap();

            sender_s_26.send(receiver_s_27).unwrap();
            let receiver_s_27_bis = receiver_s_26_bis.recv().unwrap();

            sender_s_27.send(()).unwrap();
            receiver_s_27_bis.recv().unwrap();
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
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = smtp_crossbeam,
}

criterion_main! {
    bench
}
