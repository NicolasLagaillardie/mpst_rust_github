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
    bundle_struct_fork_close_multi, choose, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_name_short, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
// use std::time::Duration;

// Create the new MeshedChannels for seven participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsSeven, 7);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E, F, G);

// Create new names
create_multiple_normal_name_short!(A, B, C, D, E, F, G);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    NameA, MeshedChannelsSeven, 7
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 | =>
    NameB, MeshedChannelsSeven, 7
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 | =>
    NameC, MeshedChannelsSeven, 7
);
// D
create_send_mpst_session_bundle!(
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 | =>
    NameD, MeshedChannelsSeven, 7
);
// E
create_send_mpst_session_bundle!(
    send_mpst_e_to_d, RoleD, 4 |
    send_mpst_e_to_f, RoleF, 5 | =>
    NameE, MeshedChannelsSeven, 7
);
// F
create_send_mpst_session_bundle!(
    send_mpst_f_to_e, RoleE, 5 |
    send_mpst_f_to_g, RoleG, 6 | =>
    NameF, MeshedChannelsSeven, 7
);
// G
create_send_mpst_session_bundle!(
    send_mpst_g_to_f, RoleF, 6 | =>
    NameG, MeshedChannelsSeven, 7
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_g, RoleG, 6 | =>
    NameA, MeshedChannelsSeven, 7
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_g, RoleG, 6 | =>
    NameB, MeshedChannelsSeven, 7
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_g, RoleG, 6 | =>
    NameC, MeshedChannelsSeven, 7
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 |
    recv_mpst_d_from_g, RoleG, 6 | =>
    NameD, MeshedChannelsSeven, 7
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 4 |
    recv_mpst_e_from_f, RoleF, 5 |
    recv_mpst_e_from_g, RoleG, 6 | =>
    NameE, MeshedChannelsSeven, 7
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_e, RoleE, 5 |
    recv_mpst_f_from_g, RoleG, 6 | =>
    NameF, MeshedChannelsSeven, 7
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_f, RoleF, 6 | =>
    NameG, MeshedChannelsSeven, 7
);

// Types
// A
enum Branching0fromGtoA {
    Forward(
        MeshedChannelsSeven<
            Send<(), End>,
            End,
            End,
            End,
            End,
            RecursAtoG,
            RoleB<RoleG<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsSeven<
            Recv<(), End>,
            End,
            End,
            End,
            End,
            RecursAtoG,
            RoleB<RoleG<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoG = <Choose0fromGtoA as Session>::Dual;
// B
enum Branching0fromGtoB {
    Forward(
        MeshedChannelsSeven<
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            RecursBtoG,
            RoleA<RoleC<RoleG<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsSeven<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            RecursBtoG,
            RoleC<RoleA<RoleG<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoG = <Choose0fromGtoB as Session>::Dual;
// C
enum Branching0fromGtoC {
    Forward(
        MeshedChannelsSeven<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursCtoG,
            RoleB<RoleD<RoleG<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsSeven<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursCtoG,
            RoleD<RoleB<RoleG<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoG = <Choose0fromGtoC as Session>::Dual;
// D
enum Branching0fromGtoD {
    Forward(
        MeshedChannelsSeven<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursDtoG,
            RoleC<RoleE<RoleG<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsSeven<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursDtoG,
            RoleE<RoleC<RoleG<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoG = <Choose0fromGtoD as Session>::Dual;
// E
enum Branching0fromGtoE {
    Forward(
        MeshedChannelsSeven<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursEtoG,
            RoleD<RoleF<RoleG<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannelsSeven<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursEtoG,
            RoleF<RoleD<RoleG<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoG = <Choose0fromGtoE as Session>::Dual;
// F
enum Branching0fromGtoF {
    Forward(
        MeshedChannelsSeven<
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursFtoG>,
            RoleE<RoleG<RoleG<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannelsSeven<
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursFtoG>,
            RoleG<RoleE<RoleG<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoG = <Choose0fromGtoF as Session>::Dual;
// F
type Choose0fromGtoA = Send<Branching0fromGtoA, End>;
type Choose0fromGtoB = Send<Branching0fromGtoB, End>;
type Choose0fromGtoC = Send<Branching0fromGtoC, End>;
type Choose0fromGtoD = Send<Branching0fromGtoD, End>;
type Choose0fromGtoE = Send<Branching0fromGtoE, End>;
type Choose0fromGtoF = Send<Branching0fromGtoF, End>;
type EndpointDoneG = MeshedChannelsSeven<End, End, End, End, End, End, RoleEnd, NameG>;
type EndpointForwardG = MeshedChannelsSeven<
    Choose0fromGtoA,
    Choose0fromGtoB,
    Choose0fromGtoC,
    Choose0fromGtoD,
    Choose0fromGtoE,
    Recv<(), Choose0fromGtoF>,
    RoleF<RoleBroadcast>,
    NameG,
>;
type EndpointBackwardG = MeshedChannelsSeven<
    Choose0fromGtoA,
    Choose0fromGtoB,
    Choose0fromGtoC,
    Choose0fromGtoD,
    Choose0fromGtoE,
    Send<(), Choose0fromGtoF>,
    RoleF<RoleBroadcast>,
    NameG,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsSeven<End, End, End, End, End, RecursAtoG, RoleG<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsSeven<End, End, End, End, End, RecursBtoG, RoleG<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsSeven<End, End, End, End, End, RecursCtoG, RoleG<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsSeven<End, End, End, End, End, RecursDtoG, RoleG<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsSeven<End, End, End, End, End, RecursEtoG, RoleG<RoleEnd>, NameE>;
type EndpointF = MeshedChannelsSeven<End, End, End, End, End, RecursFtoG, RoleG<RoleEnd>, NameF>;
type EndpointG = MeshedChannelsSeven<
    Choose0fromGtoA,
    Choose0fromGtoB,
    Choose0fromGtoC,
    Choose0fromGtoD,
    Choose0fromGtoE,
    Choose0fromGtoF,
    RoleBroadcast,
    NameG,
>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_g_to_all, forward_from_g_to_all, backward_from_g_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneG, EndpointForwardG, EndpointBackwardG, =>
    Branching0fromGtoA,
    Branching0fromGtoB,
    Branching0fromGtoC,
    Branching0fromGtoD,
    Branching0fromGtoE,
    Branching0fromGtoF, =>
    NameA, NameB, NameC, NameD, NameE, NameF, =>
    NameG, MeshedChannelsSeven, 7
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_g, {
        Branching0fromGtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s);
            endpoint_a(s)
        },
        Branching0fromGtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_g, {
        Branching0fromGtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s);
            endpoint_b(s)
        },
        Branching0fromGtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s);
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_g, {
        Branching0fromGtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s);
            endpoint_c(s)
        },
        Branching0fromGtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s);
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_g, {
        Branching0fromGtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoD::Forward(s) => {
            let ((), s) = recv_mpst_d_from_c(s)?;
            let s = send_mpst_d_to_e((), s);
            endpoint_d(s)
        },
        Branching0fromGtoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s);
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_from_g, {
        Branching0fromGtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoE::Forward(s) => {
            let ((), s) = recv_mpst_e_from_d(s)?;
            let s = send_mpst_e_to_f((), s);
            endpoint_e(s)
        },
        Branching0fromGtoE::Backward(s) => {
            let ((), s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_d((), s);
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_from_g, {
        Branching0fromGtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromGtoF::Forward(s) => {
            let ((), s) = recv_mpst_f_from_e(s)?;
            let s = send_mpst_f_to_g((), s);
            endpoint_f(s)
        },
        Branching0fromGtoF::Backward(s) => {
            let ((), s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_e((), s);
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    recurs_g(s, LOOPS)
}

fn recurs_g(s: EndpointG, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_g_to_all(s);

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_g_to_all(s);

            let (_, s) = recv_mpst_g_from_f(s)?;

            recurs_g(s, i - 1)
        }
        i => {
            let s = backward_from_g_to_all(s);

            let s = send_mpst_g_to_f((), s);

            recurs_g(s, i - 1)
        }
    }
}

fn all_mpst() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
        black_box(endpoint_g),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
}

/////////////////////////
// A
enum BinaryA {
    Forward(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_a_to_b(s: RecursA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::Forward(s) => {
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

    for _ in 0..6 {
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..LOOPS {
            sessions = sessions
                .into_iter()
                .map(|s| binary_b_to_a(choose!(BinaryA::Forward, s)).unwrap())
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

    for _ in 0..6 {
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

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring seven protocol MPST {}", LOOPS), |b| {
        b.iter(all_mpst)
    });
}

pub fn ring_protocol_binary(c: &mut Criterion) {
    c.bench_function(&format!("ring seven protocol binary {}", LOOPS), |b| {
        b.iter(all_binaries)
    });
}

pub fn ring_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(&format!("ring seven protocol crossbeam {}", LOOPS), |b| {
        b.iter(all_crossbeam)
    });
}
