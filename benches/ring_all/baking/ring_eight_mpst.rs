#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// use std::time::Duration;

generate!("recursive", MeshedChannelsEight, A, B, C, D, E, F, G, H);

// Types
// A
enum Branching0fromHtoA {
    Forward(
        MeshedChannelsEight<
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursAtoH,
            RoleB<RoleH<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursAtoH,
            RoleB<RoleH<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoH = Recv<Branching0fromHtoA, End>;

// B
enum Branching0fromHtoB {
    Forward(
        MeshedChannelsEight<
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            RecursBtoH,
            RoleA<RoleC<RoleH<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            RecursBtoH,
            RoleC<RoleA<RoleH<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoH = Recv<Branching0fromHtoB, End>;

// C
enum Branching0fromHtoC {
    Forward(
        MeshedChannelsEight<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            RecursCtoH,
            RoleB<RoleD<RoleH<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            RecursCtoH,
            RoleD<RoleB<RoleH<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoH = Recv<Branching0fromHtoC, End>;

// D
enum Branching0fromHtoD {
    Forward(
        MeshedChannelsEight<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursDtoH,
            RoleC<RoleE<RoleH<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursDtoH,
            RoleE<RoleC<RoleH<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoH = Recv<Branching0fromHtoD, End>;

// E
enum Branching0fromHtoE {
    Forward(
        MeshedChannelsEight<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursEtoH,
            RoleD<RoleF<RoleH<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursEtoH,
            RoleF<RoleD<RoleH<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoH = Recv<Branching0fromHtoE, End>;

// F
enum Branching0fromHtoF {
    Forward(
        MeshedChannelsEight<
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursFtoH,
            RoleE<RoleG<RoleH<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursFtoH,
            RoleG<RoleE<RoleH<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoH = Recv<Branching0fromHtoF, End>;

// G
enum Branching0fromHtoG {
    Forward(
        MeshedChannelsEight<
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursGtoH>,
            RoleF<RoleH<RoleH<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursGtoH>,
            RoleH<RoleF<RoleH<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoH = Recv<Branching0fromHtoG, End>;

// H
type Choose0fromHtoA = Send<Branching0fromHtoA, End>;
type Choose0fromHtoB = Send<Branching0fromHtoB, End>;
type Choose0fromHtoC = Send<Branching0fromHtoC, End>;
type Choose0fromHtoD = Send<Branching0fromHtoD, End>;
type Choose0fromHtoE = Send<Branching0fromHtoE, End>;
type Choose0fromHtoF = Send<Branching0fromHtoF, End>;
type Choose0fromHtoG = Send<Branching0fromHtoG, End>;
type EndpointForwardH = MeshedChannelsEight<
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    Recv<(), Choose0fromHtoG>,
    RoleG<RoleBroadcast>,
    NameH,
>;
type EndpointBackwardH = MeshedChannelsEight<
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    Send<(), Choose0fromHtoG>,
    RoleG<RoleBroadcast>,
    NameH,
>;

// Creating the MP sessions
type EndpointA =
    MeshedChannelsEight<End, End, End, End, End, End, RecursAtoH, RoleH<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannelsEight<End, End, End, End, End, End, RecursBtoH, RoleH<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannelsEight<End, End, End, End, End, End, RecursCtoH, RoleH<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsEight<End, End, End, End, End, End, RecursDtoH, RoleH<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannelsEight<End, End, End, End, End, End, RecursEtoH, RoleH<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannelsEight<End, End, End, End, End, End, RecursFtoH, RoleH<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannelsEight<End, End, End, End, End, End, RecursGtoH, RoleH<RoleEnd>, NameG>;
type EndpointH = MeshedChannelsEight<
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    Choose0fromHtoG,
    RoleBroadcast,
    NameH,
>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoA::Done(s) => {
            s.close()
        },
        Branching0fromHtoA::Forward(s) => {
            let s = s.send(());
            endpoint_a(s)
        },
        Branching0fromHtoA::Backward(s) => {
            let (_, s) = s.recv();
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoB::Done(s) => {
            s.close()
        },
        Branching0fromHtoB::Forward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_b(s)
        },
        Branching0fromHtoB::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoC::Done(s) => {
            s.close()
        },
        Branching0fromHtoC::Forward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_c(s)
        },
        Branching0fromHtoC::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoD::Done(s) => {
            s.close()
        },
        Branching0fromHtoD::Forward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_d(s)
        },
        Branching0fromHtoD::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoE::Done(s) => {
            s.close()
        },
        Branching0fromHtoE::Forward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_e(s)
        },
        Branching0fromHtoE::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoF::Done(s) => {
            s.close()
        },
        Branching0fromHtoF::Forward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_f(s)
        },
        Branching0fromHtoF::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoG::Done(s) => {
            s.close()
        },
        Branching0fromHtoG::Forward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_g(s)
        },
        Branching0fromHtoG::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    recurs_h(s, LOOPS)
}

fn recurs_h(s: EndpointH, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_h_to_all!(
                s,
                Branching0fromHtoA::Done,
                Branching0fromHtoB::Done,
                Branching0fromHtoC::Done,
                Branching0fromHtoD::Done,
                Branching0fromHtoE::Done,
                Branching0fromHtoF::Done,
                Branching0fromHtoG::Done
            );

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardH = choose_mpst_h_to_all!(
                s,
                Branching0fromHtoA::Forward,
                Branching0fromHtoB::Forward,
                Branching0fromHtoC::Forward,
                Branching0fromHtoD::Forward,
                Branching0fromHtoE::Forward,
                Branching0fromHtoF::Forward,
                Branching0fromHtoG::Forward
            );

            let (_, s) = s.recv();

            recurs_h(s, i - 1)
        }
        i => {
            let s: EndpointBackwardH = choose_mpst_h_to_all!(
                s,
                Branching0fromHtoA::Backward,
                Branching0fromHtoB::Backward,
                Branching0fromHtoC::Backward,
                Branching0fromHtoD::Backward,
                Branching0fromHtoE::Backward,
                Branching0fromHtoF::Backward,
                Branching0fromHtoG::Backward
            );

            let s = s.send(());

            recurs_h(s, i - 1)
        }
    }
}

fn aux() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g, thread_h) =
        fork_mpst(
            black_box(endpoint_a),
            black_box(endpoint_b),
            black_box(endpoint_c),
            black_box(endpoint_d),
            black_box(endpoint_e),
            black_box(endpoint_f),
            black_box(endpoint_g),
            black_box(endpoint_h),
        );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
    thread_h.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring eight baking protocol MPST {LOOPS}"), |b| {
        b.iter(aux)
    });
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = ring_protocol_mpst,
}

criterion_main! {
    bench
}
