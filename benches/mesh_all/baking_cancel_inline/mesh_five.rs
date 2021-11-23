#![allow(dead_code)]

use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum_and_cancel, choose, offer, offer_mpst};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
// use std::time::Duration;

// Create new roles
bundle_impl_with_enum_and_cancel!(MeshedChannelsFive, A, B, C, D, E);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;
// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type R2D<R> = RoleD<RoleD<R>>;
type R2E<R> = RoleE<RoleE<R>>;
// A
enum Branching0fromEtoA {
    More(
        MeshedChannelsFive<
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoE>>,
            R2E<R2B<R2C<R2D<RoleE<RoleEnd>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoE = Recv<Branching0fromEtoA, End>;
// B
enum Branching0fromEtoB {
    More(
        MeshedChannelsFive<
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoE>>,
            R2E<R2A<R2C<R2D<RoleE<RoleEnd>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoE = Recv<Branching0fromEtoB, End>;
// C
enum Branching0fromEtoC {
    More(
        MeshedChannelsFive<
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursCtoE>>,
            R2E<R2A<R2B<R2D<RoleE<RoleEnd>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoE = Recv<Branching0fromEtoC, End>;
// D
enum Branching0fromEtoD {
    More(
        MeshedChannelsFive<
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursDtoE>>,
            R2E<R2A<R2B<R2C<RoleE<RoleEnd>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoE = Recv<Branching0fromEtoD, End>;
// E
type Choose0fromEtoA = Send<Branching0fromEtoA, End>;
type Choose0fromEtoB = Send<Branching0fromEtoB, End>;
type Choose0fromEtoC = Send<Branching0fromEtoC, End>;
type Choose0fromEtoD = Send<Branching0fromEtoD, End>;
type EndpointMoreE = MeshedChannelsFive<
    Send<(), Recv<(), Choose0fromEtoA>>,
    Send<(), Recv<(), Choose0fromEtoB>>,
    Send<(), Recv<(), Choose0fromEtoC>>,
    Send<(), Recv<(), Choose0fromEtoD>>,
    R2A<R2B<R2C<R2D<RoleBroadcast>>>>,
    NameE,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsFive<End, End, End, RecursAtoE, RoleE<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFive<End, End, End, RecursBtoE, RoleE<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFive<End, End, End, RecursCtoE, RoleE<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsFive<End, End, End, RecursDtoE, RoleE<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsFive<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Choose0fromEtoD,
    RoleBroadcast,
    NameE,
>;

#[inline]
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromEtoA::Done(s) => {
            s.close()
        },
        Branching0fromEtoA::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_a(s)
        },
    })
}

#[inline]
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromEtoB::Done(s) => {
            s.close()
        },
        Branching0fromEtoB::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

#[inline]
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromEtoC::Done(s) => {
            s.close()
        },
        Branching0fromEtoC::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
    })
}

#[inline]
fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromEtoD::Done(s) => {
            s.close()
        },
        Branching0fromEtoD::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            endpoint_d(s)
        },
    })
}

#[inline]
fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for _ in 1..LOOPS {
        temp_s = recurs_e(temp_s)?;
    }

    let s = choose_mpst_e_to_all!(
        temp_s,
        Branching0fromEtoA::Done,
        Branching0fromEtoB::Done,
        Branching0fromEtoC::Done,
        Branching0fromEtoD::Done,
    );

    s.close()
}

#[inline]
fn recurs_e(s: EndpointE) -> Result<EndpointE, Box<dyn Error>> {
    let s: EndpointMoreE = choose_mpst_e_to_all!(
        s,
        Branching0fromEtoA::More,
        Branching0fromEtoB::More,
        Branching0fromEtoC::More,
        Branching0fromEtoD::More,
    );

    let s = s.send(())?;
    let (_, s) = s.recv()?;
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    Ok(s)
}

#[inline]
fn all_mpst() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
}

/////////////////////////
// A
enum BinaryA {
    More(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_a_to_b(s: RecursA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::More(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            binary_a_to_b(s)
        },
    })
}

// B
type RecursB = <RecursA as Session>::Dual;
fn binary_b_to_a(s: Send<(), Recv<(), RecursB>>) -> Result<RecursB, Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..10 {
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..LOOPS {
            sessions = sessions
                .into_iter()
                .map(|s| binary_b_to_a(choose!(BinaryA::More, s)).unwrap())
                .collect::<Vec<_>>();
        }

        sessions
            .into_iter()
            .for_each(|s| close(choose!(BinaryA::Done, s)).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

/////////////////////////

type ReceivingSendingReceiving = crossbeam_channel::Receiver<SendingReceiving>;
type SendingReceivingSending = crossbeam_channel::Sender<ReceivingSending>;

type SendingReceiving = crossbeam_channel::Sender<Receiving>;
type ReceivingSending = crossbeam_channel::Receiver<Sending>;

type Receiving = crossbeam_channel::Receiver<()>;
type Sending = crossbeam_channel::Sender<()>;

fn all_crossbeam() {
    let mut threads = Vec::new();

    for _ in 0..10 {
        let main = spawn(move || {
            for _ in 0..LOOPS {
                let (sender_0, receiver_0) = bounded::<ReceivingSendingReceiving>(1);
                let (sender_4, receiver_4) = bounded::<SendingReceivingSending>(1);

                let (sender_1, receiver_1) = bounded::<SendingReceiving>(1);
                let (sender_5, receiver_5) = bounded::<ReceivingSending>(1);

                let (sender_2, receiver_2) = bounded::<Receiving>(1);
                let (sender_6, receiver_6) = bounded::<Sending>(1);

                let (sender_3, receiver_3) = bounded::<()>(1);
                let (sender_7, receiver_7) = bounded::<()>(1);

                sender_0.send(receiver_1).unwrap();
                sender_4.send(sender_5).unwrap();

                let receiver_1_bis = receiver_0.recv().unwrap();
                let sender_5_bis = receiver_4.recv().unwrap();

                sender_1.send(sender_2).unwrap();
                sender_5_bis.send(receiver_6).unwrap();

                let sender_2_bis = receiver_1_bis.recv().unwrap();
                let receiver_6_bis = receiver_5.recv().unwrap();

                sender_2_bis.send(receiver_3).unwrap();
                sender_6.send(sender_7).unwrap();

                let receiver_2_bis = receiver_2.recv().unwrap();
                let sender_7_bis = receiver_6_bis.recv().unwrap();

                sender_3.send(()).unwrap();
                sender_7_bis.send(()).unwrap();

                receiver_2_bis.recv().unwrap();
                receiver_7.recv().unwrap();
            }

            // "Close" connection
            let (sender_close_1, receiver_close_1) = bounded::<()>(1);
            let (sender_close_2, receiver_close_2) = bounded::<()>(1);

            sender_close_1.send(()).unwrap_or(());
            sender_close_2.send(()).unwrap_or(());

            receiver_close_1.recv().unwrap_or(());
            receiver_close_2.recv().unwrap_or(());
        });

        threads.push(main);
    }

    threads.into_iter().for_each(|elt| elt.join().unwrap());
}

/////////////////////////

static LOOPS: i64 = 100;

fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("mesh five baking inline protocol MPST {}", LOOPS), |b| {
        b.iter(|| all_mpst())
    });
}

fn mesh_protocol_binary(c: &mut Criterion) {
    c.bench_function(
        &format!("mesh five baking inline protocol binary {}", LOOPS),
        |b| b.iter(|| all_binaries()),
    );
}

fn mesh_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(
        &format!("mesh five baking inline protocol crossbeam {}", LOOPS),
        |b| b.iter(|| all_crossbeam()),
    );
}

// fn long_warmup() -> Criterion {
//     Criterion::default().measurement_time(Duration::new(1800, 0))
// }

criterion_group! {
    name = mesh_five;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = mesh_protocol_mpst, mesh_protocol_binary, mesh_protocol_crossbeam
}

criterion_main!(mesh_five);
