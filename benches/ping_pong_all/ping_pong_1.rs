use crossbeam_channel::bounded;

use criterion::{black_box, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, choose_mpst_multi_to_all, create_multiple_normal_name,
    create_multiple_normal_role, create_recv_mpst_session_bundle, create_send_mpst_session_bundle,
    offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};

// use std::time::Duration;

// global protocol ping_pong(role A, role B)
// {
//     rec PP
//     {
//         choice at A
//         {
//             ping(()) from A to B;

//             pong(()) from B to A;

//             continue PP;

//         }

//         or
//         {
//             stop() from A to B;

//         }

//     }

// }

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsTwo, 2);

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
);

// Create new names
create_multiple_normal_name!(NameA, NameB);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    NameA, MeshedChannelsTwo, 2
);

// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 | =>
    NameB, MeshedChannelsTwo, 2
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 | =>
    NameA, MeshedChannelsTwo, 2
);

// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 | =>
    NameB, MeshedChannelsTwo, 2
);

// Types
// A
type Choose0fromAtoB = <RecursBtoA as Session>::Dual;

// B
enum Branching0fromAtoB {
    More(MeshedChannelsTwo<Recv<(), Send<(), RecursBtoA>>, RoleA<RoleA<RoleA<RoleEnd>>>, NameB>),
    Done(MeshedChannelsTwo<End, RoleEnd, NameB>),
}
type RecursBtoA = Recv<Branching0fromAtoB, End>;

// Creating the MP sessions
type EndpointA = MeshedChannelsTwo<Choose0fromAtoB, RoleBroadcast, NameA>;
type EndpointB = MeshedChannelsTwo<RecursBtoA, RoleA<RoleEnd>, NameB>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    recurs_a(s, LOOPS)
}

fn recurs_a(s: EndpointA, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromAtoB::Done, =>
                NameA,
                MeshedChannelsTwo,
                1
            );

            close_mpst_multi(s)
        }
        i => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromAtoB::More, =>
                NameA,
                MeshedChannelsTwo,
                1
            );

            let s = send_mpst_a_to_b((), s);
            let ((), s) = recv_mpst_a_from_b(s)?;

            recurs_a(s, i - 1)
        }
    }
}

fn recurs_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_a, {
        Branching0fromAtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromAtoB::More(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a((), s);
            recurs_b(s)
        },
    })
}

fn all_mpst() {
    let (thread_a, thread_b) = fork_mpst(black_box(endpoint_a), black_box(recurs_b));

    thread_a.join().unwrap();
    thread_b.join().unwrap();
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

    let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(black_box(binary_a_to_b));

    threads.push(thread);
    sessions.push(s);

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

    threads.into_iter().for_each(|elt| elt.join().unwrap());
}

/////////////////////////

static LOOPS: i64 = 1;

pub fn ping_pong_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ping pong protocol MPST {LOOPS}"), |b| {
        b.iter(all_mpst)
    });
}

pub fn ping_pong_protocol_binary(c: &mut Criterion) {
    c.bench_function(&format!("ping pong protocol binary {LOOPS}"), |b| {
        b.iter(all_binaries)
    });
}

pub fn ping_pong_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(&format!("ping pong protocol crossbeam {LOOPS}"), |b| {
        b.iter(all_crossbeam)
    });
}
