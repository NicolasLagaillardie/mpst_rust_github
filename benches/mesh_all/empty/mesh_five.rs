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

// Create the new MeshedChannels for five participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsFive, 5);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 |
    send_mpst_a_to_c, RoleC, 2 |
    send_mpst_a_to_d, RoleD, 3 |
    send_mpst_a_to_e, RoleE, 4 | =>
    RoleA, MeshedChannelsFive, 5
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 |
    send_mpst_b_to_d, RoleD, 3 |
    send_mpst_b_to_e, RoleE, 4 | =>
    RoleB, MeshedChannelsFive, 5
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, 1 |
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 |
    send_mpst_c_to_e, RoleE, 4 | =>
    RoleC, MeshedChannelsFive, 5
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_a, RoleA, 1 |
    send_mpst_d_to_b, RoleB, 2 |
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 | =>
    RoleD, MeshedChannelsFive, 5
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_a, RoleA, 1 |
    send_mpst_e_to_b, RoleB, 2 |
    send_mpst_e_to_c, RoleC, 3 |
    send_mpst_e_to_d, RoleD, 4 | =>
    RoleE, MeshedChannelsFive, 5
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_c, RoleC, 2 |
    recv_mpst_a_from_d, RoleD, 3 |
    recv_mpst_a_from_e, RoleE, 4 | =>
    RoleA, MeshedChannelsFive, 5
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_d, RoleD, 3 |
    recv_mpst_b_from_e, RoleE, 4 | =>
    RoleB, MeshedChannelsFive, 5
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 |
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_e, RoleE, 4 | =>
    RoleC, MeshedChannelsFive, 5
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_a, RoleA, 1 |
    recv_mpst_d_from_b, RoleB, 2 |
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 | =>
    RoleD, MeshedChannelsFive, 5
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_a, RoleA, 1 |
    recv_mpst_e_from_b, RoleB, 2 |
    recv_mpst_e_from_c, RoleC, 3 |
    recv_mpst_e_from_d, RoleD, 4 | =>
    RoleE, MeshedChannelsFive, 5
);

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
type EndpointDoneE = MeshedChannelsFive<End, End, End, End, RoleEnd, NameE>;
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

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_e_to_all, more_from_e_to_all, =>
    Done, More, =>
    EndpointDoneE, EndpointMoreE, =>
    Branching0fromEtoA,
    Branching0fromEtoB,
    Branching0fromEtoC,
    Branching0fromEtoD, =>
    RoleA,
    RoleB,
    RoleC,
    RoleD, =>
    RoleE, MeshedChannelsFive, 5
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_e, {
        Branching0fromEtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoA::More(s) => {
            let (_, s) = recv_mpst_a_from_e(s)?;
            let s = send_mpst_a_to_e((), s);
            let (_, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_b((), s);
            let (_, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_c((), s);
            let (_, s) = recv_mpst_a_from_d(s)?;
            let s = send_mpst_a_to_d((), s);
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_e, {
        Branching0fromEtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoB::More(s) => {
            let (_, s) = recv_mpst_b_from_e(s)?;
            let s = send_mpst_b_to_e((), s);
            let s = send_mpst_b_to_a((), s);
            let (_, s) = recv_mpst_b_from_a(s)?;
            let (_, s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_c((), s);
            let (_, s) = recv_mpst_b_from_d(s)?;
            let s = send_mpst_b_to_d((), s);
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_e, {
        Branching0fromEtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoC::More(s) => {
            let (_, s) = recv_mpst_c_from_e(s)?;
            let s = send_mpst_c_to_e((), s);
            let s = send_mpst_c_to_a((), s);
            let (_, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b((), s);
            let (_, s) = recv_mpst_c_from_b(s)?;
            let (_, s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_d((), s);
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_e, {
        Branching0fromEtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoD::More(s) => {
            let (_, s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_e((), s);
            let s = send_mpst_d_to_a((), s);
            let (_, s) = recv_mpst_d_from_a(s)?;
            let s = send_mpst_d_to_b((), s);
            let (_, s) = recv_mpst_d_from_b(s)?;
            let s = send_mpst_d_to_c((), s);
            let (_, s) = recv_mpst_d_from_c(s)?;
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    recurs_e(s, LOOPS)
}

fn recurs_e(s: EndpointE, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_e_to_all(s);

            close_mpst_multi(s)
        }
        i => {
            let s = more_from_e_to_all(s);

            let s = send_mpst_e_to_a((), s);
            let (_, s) = recv_mpst_e_from_a(s)?;
            let s = send_mpst_e_to_b((), s);
            let (_, s) = recv_mpst_e_from_b(s)?;
            let s = send_mpst_e_to_c((), s);
            let (_, s) = recv_mpst_e_from_c(s)?;
            let s = send_mpst_e_to_d((), s);
            let (_, s) = recv_mpst_e_from_d(s)?;

            recurs_e(s, i - 1)
        }
    }
}

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

static LOOPS: i64 = 0;

fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("mesh five empty protocol MPST {}", LOOPS), |b| {
        b.iter(all_mpst)
    });
}

fn mesh_protocol_binary(c: &mut Criterion) {
    c.bench_function(&format!("mesh five empty protocol binary {}", LOOPS), |b| {
        b.iter(all_binaries)
    });
}

fn mesh_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(
        &format!("mesh five empty protocol crossbeam {}", LOOPS),
        |b| b.iter(all_crossbeam),
    );
}

criterion_group! {
    name = mesh_five;
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = mesh_protocol_mpst, mesh_protocol_binary, mesh_protocol_crossbeam
}

criterion_main!(mesh_five);
