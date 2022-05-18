use crossbeam_channel::bounded;

use criterion::{black_box, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum_and_cancel, choose, offer};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
// use std::time::Duration;

// Create new roles
bundle_impl_with_enum_and_cancel!(MeshedChannelsSeven, A, B, C, D, E, F, G);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;
type NameF = RoleF<RoleEnd>;
type NameG = RoleG<RoleEnd>;

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

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromGtoA::Done(s) => {
            s.close()
        },
        Branching0fromGtoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromGtoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromGtoB::Done(s) => {
            s.close()
        },
        Branching0fromGtoB::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
        Branching0fromGtoB::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromGtoC::Done(s) => {
            s.close()
        },
        Branching0fromGtoC::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
        Branching0fromGtoC::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromGtoD::Done(s) => {
            s.close()
        },
        Branching0fromGtoD::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
        Branching0fromGtoD::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromGtoE::Done(s) => {
            s.close()
        },
        Branching0fromGtoE::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
        Branching0fromGtoE::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromGtoF::Done(s) => {
            s.close()
        },
        Branching0fromGtoF::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
        Branching0fromGtoF::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
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
            let s = choose_mpst_g_to_all!(
                s,
                Branching0fromGtoA::Done,
                Branching0fromGtoB::Done,
                Branching0fromGtoC::Done,
                Branching0fromGtoD::Done,
                Branching0fromGtoE::Done,
                Branching0fromGtoF::Done
            );

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardG = choose_mpst_g_to_all!(
                s,
                Branching0fromGtoA::Forward,
                Branching0fromGtoB::Forward,
                Branching0fromGtoC::Forward,
                Branching0fromGtoD::Forward,
                Branching0fromGtoE::Forward,
                Branching0fromGtoF::Forward
            );

            let (_, s) = s.recv()?;

            recurs_g(s, i - 1)
        }
        i => {
            let s: EndpointBackwardG = choose_mpst_g_to_all!(
                s,
                Branching0fromGtoA::Backward,
                Branching0fromGtoB::Backward,
                Branching0fromGtoC::Backward,
                Branching0fromGtoD::Backward,
                Branching0fromGtoE::Backward,
                Branching0fromGtoF::Backward
            );

            let s = s.send(())?;

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
    c.bench_function(&format!("ring seven baking protocol MPST {}", LOOPS), |b| {
        b.iter(all_mpst)
    });
}

pub fn ring_protocol_binary(c: &mut Criterion) {
    c.bench_function(
        &format!("ring seven baking protocol binary {}", LOOPS),
        |b| b.iter(all_binaries),
    );
}

pub fn ring_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(
        &format!("ring seven baking protocol crossbeam {}", LOOPS),
        |b| b.iter(all_crossbeam),
    );
}
