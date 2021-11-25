use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
// use std::time::Duration;

// Create the new MeshedChannels for six participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsSix, 6);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E, F);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 |
    send_mpst_a_to_c, RoleC, 2 |
    send_mpst_a_to_d, RoleD, 3 |
    send_mpst_a_to_e, RoleE, 4 |
    send_mpst_a_to_f, RoleF, 5 | =>
    RoleA, MeshedChannelsSix, 6
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 |
    send_mpst_b_to_d, RoleD, 3 |
    send_mpst_b_to_e, RoleE, 4 |
    send_mpst_b_to_f, RoleF, 5 | =>
    RoleB, MeshedChannelsSix, 6
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, 1 |
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 |
    send_mpst_c_to_e, RoleE, 4 |
    send_mpst_c_to_f, RoleF, 5 | =>
    RoleC, MeshedChannelsSix, 6
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a, RoleA, 1 |
    send_mpst_d_to_b, RoleB, 2 |
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 |
    send_mpst_d_to_f, RoleF, 5 | =>
    RoleD, MeshedChannelsSix, 6
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a, RoleA, 1 |
    send_mpst_e_to_b, RoleB, 2 |
    send_mpst_e_to_c, RoleC, 3 |
    send_mpst_e_to_d, RoleD, 4 |
    send_mpst_e_to_f, RoleF, 5 | =>
    RoleE, MeshedChannelsSix, 6
);
// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_a, RoleA, 1 |
    send_mpst_f_to_b, RoleB, 2 |
    send_mpst_f_to_c, RoleC, 3 |
    send_mpst_f_to_d, RoleD, 4 |
    send_mpst_f_to_e, RoleE, 5 | =>
    RoleF, MeshedChannelsSix, 6
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_c, RoleC, 2 |
    recv_mpst_a_from_d, RoleD, 3 |
    recv_mpst_a_from_e, RoleE, 4 |
    recv_mpst_a_from_f, RoleF, 5 | =>
    RoleA, MeshedChannelsSix, 6
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_d, RoleD, 3 |
    recv_mpst_b_from_e, RoleE, 4 |
    recv_mpst_b_from_f, RoleF, 5 | =>
    RoleB, MeshedChannelsSix, 6
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 |
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_e, RoleE, 4 |
    recv_mpst_c_from_f, RoleF, 5 | =>
    RoleC, MeshedChannelsSix, 6
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, 1 |
    recv_mpst_d_from_b, RoleB, 2 |
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 |
    recv_mpst_d_from_f, RoleF, 5 | =>
    RoleD, MeshedChannelsSix, 6
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, 1 |
    recv_mpst_e_from_b, RoleB, 2 |
    recv_mpst_e_from_c, RoleC, 3 |
    recv_mpst_e_from_d, RoleD, 4 |
    recv_mpst_e_from_f, RoleF, 5 | =>
    RoleE, MeshedChannelsSix, 6
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_a, RoleA, 1 |
    recv_mpst_f_from_b, RoleB, 2 |
    recv_mpst_f_from_c, RoleC, 3 |
    recv_mpst_f_from_d, RoleD, 4 |
    recv_mpst_f_from_e, RoleE, 5 | =>
    RoleF, MeshedChannelsSix, 6
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;
type NameF = RoleF<RoleEnd>;

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
type R2F<R> = RoleF<RoleF<R>>;
// A
enum Branching0fromFtoA {
    More(
        MeshedChannelsSix<
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoF>>,
            R2F<R2B<R2C<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoF = Recv<Branching0fromFtoA, End>;
// B
enum Branching0fromFtoB {
    More(
        MeshedChannelsSix<
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoF>>,
            R2F<R2A<R2C<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoF = Recv<Branching0fromFtoB, End>;
// C
enum Branching0fromFtoC {
    More(
        MeshedChannelsSix<
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoF>>,
            R2F<R2A<R2B<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoF = Recv<Branching0fromFtoC, End>;
// D
enum Branching0fromFtoD {
    More(
        MeshedChannelsSix<
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursDtoF>>,
            R2F<R2A<R2B<R2C<R2E<RoleF<RoleEnd>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoF = Recv<Branching0fromFtoD, End>;
// E
enum Branching0fromFtoE {
    More(
        MeshedChannelsSix<
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursEtoF>>,
            R2F<R2A<R2B<R2C<R2D<RoleF<RoleEnd>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoF = Recv<Branching0fromFtoE, End>;
// F
type Choose0fromFtoA = Send<Branching0fromFtoA, End>;
type Choose0fromFtoB = Send<Branching0fromFtoB, End>;
type Choose0fromFtoC = Send<Branching0fromFtoC, End>;
type Choose0fromFtoD = Send<Branching0fromFtoD, End>;
type Choose0fromFtoE = Send<Branching0fromFtoE, End>;
type EndpointDoneF = MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameF>;
type EndpointMoreF = MeshedChannelsSix<
    Send<(), Recv<(), Choose0fromFtoA>>,
    Send<(), Recv<(), Choose0fromFtoB>>,
    Send<(), Recv<(), Choose0fromFtoC>>,
    Send<(), Recv<(), Choose0fromFtoD>>,
    Send<(), Recv<(), Choose0fromFtoE>>,
    R2A<R2B<R2C<R2D<R2E<RoleBroadcast>>>>>,
    NameF,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsSix<End, End, End, End, RecursAtoF, RoleF<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsSix<End, End, End, End, RecursBtoF, RoleF<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsSix<End, End, End, End, RecursCtoF, RoleF<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsSix<End, End, End, End, RecursDtoF, RoleF<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsSix<End, End, End, End, RecursEtoF, RoleF<RoleEnd>, NameE>;
type EndpointF = MeshedChannelsSix<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Choose0fromFtoE,
    RoleBroadcast,
    NameF,
>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_f_to_all, more_from_f_to_all, =>
    Done, More, =>
    EndpointDoneF, EndpointMoreF, =>
    Branching0fromFtoA,
    Branching0fromFtoB,
    Branching0fromFtoC,
    Branching0fromFtoD,
    Branching0fromFtoE, =>
    RoleA,
    RoleB,
    RoleC,
    RoleD,
    RoleE, =>
    RoleF, MeshedChannelsSix, 6
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_f, {
        Branching0fromFtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_f(s)?;
            let s = send_mpst_a_to_f((), s);
            let (_, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c((), s);
            let (_, s) = recv_mpst_a_from_d(s)?;
            let s = send_mpst_a_to_d((), s);
            let (_, s) = recv_mpst_a_from_e(s)?;
            let s = send_mpst_a_to_e((), s);
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_f, {
        Branching0fromFtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_f(s)?;
            let s = send_mpst_b_to_f((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_from_a(s)?;
            let (_, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_c((), s);
            let (_, s) = recv_mpst_b_from_d(s)?;
            let s = send_mpst_b_to_d((), s);
            let (_, s) = recv_mpst_b_from_e(s)?;
            let s = send_mpst_b_to_e((), s);
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_f, {
        Branching0fromFtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_f(s)?;
            let s = send_mpst_c_to_f((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_from_b(s)?;
            let (_, s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_d((), s);
            let (_, s) = recv_mpst_c_from_e(s)?;
            let s = send_mpst_c_to_e((), s);
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_f, {
        Branching0fromFtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoD::More(s) => {
            let (_, s) = recv_mpst_d_from_f(s)?;
            let s = send_mpst_d_to_f((), s);
            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_from_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_from_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_from_c(s)?;
            let (_, s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_e((), s);
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_from_f, {
        Branching0fromFtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromFtoE::More(s) => {
            let (_, s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_f((), s);
            let s = send_mpst_e_to_a((), s);
            let (_, s) = recv_mpst_e_from_a(s)?;
            let s = send_mpst_e_to_b((), s);
            let (_, s) = recv_mpst_e_from_b(s)?;
            let s = send_mpst_e_to_c((), s);
            let (_, s) = recv_mpst_e_from_c(s)?;
            let s = send_mpst_e_to_d((), s);
            let (_, s) = recv_mpst_e_from_d(s)?;
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    recurs_f(s, LOOPS)
}

fn recurs_f(s: EndpointF, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_f_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_f_to_all(s);

            let s = send_mpst_f_to_a((), s);
            let (_, s) = recv_mpst_f_from_a(s)?;
            let s = send_mpst_f_to_b((), s);
            let (_, s) = recv_mpst_f_from_b(s)?;
            let s = send_mpst_f_to_c((), s);
            let (_, s) = recv_mpst_f_from_c(s)?;
            let s = send_mpst_f_to_d((), s);
            let (_, s) = recv_mpst_f_from_d(s)?;
            let s = send_mpst_f_to_e((), s);
            let (_, s) = recv_mpst_f_from_e(s)?;

            recurs_f(s, i - 1)
        }
    }
}

fn all_mpst() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
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

    for _ in 0..15 {
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

    for _ in 0..15 {
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
    c.bench_function(&format!("mesh six protocol MPST {}", LOOPS), |b| {
        b.iter(all_mpst)
    });
}

fn mesh_protocol_binary(c: &mut Criterion) {
    c.bench_function(&format!("mesh six protocol binary {}", LOOPS), |b| {
        b.iter(all_binaries)
    });
}

fn mesh_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(&format!("mesh six protocol crossbeam {}", LOOPS), |b| {
        b.iter(all_crossbeam)
    });
}

// fn long_warmup() -> Criterion {
//     Criterion::default().measurement_time(Duration::new(1800, 0))
// }

criterion_group! {
    name = mesh_six;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = mesh_protocol_mpst, mesh_protocol_binary, mesh_protocol_crossbeam
}

criterion_main!(mesh_six);
