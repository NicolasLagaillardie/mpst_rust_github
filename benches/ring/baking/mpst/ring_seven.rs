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

// Create new roles
generate!("recursive", MeshedChannels, A, B, C, D, E, F, G);

// Types
// A
enum Branching0fromGtoA {
    Forward(
        MeshedChannels<Send<(), End>, End, End, End, End, RecursAtoG, RoleB<RoleG<RoleEnd>>, NameA>,
    ),
    Backward(
        MeshedChannels<Recv<(), End>, End, End, End, End, RecursAtoG, RoleB<RoleG<RoleEnd>>, NameA>,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoG = Recv<Branching0fromGtoA, End>;

// B
enum Branching0fromGtoB {
    Forward(
        MeshedChannels<
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
        MeshedChannels<
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
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoG = Recv<Branching0fromGtoB, End>;

// C
enum Branching0fromGtoC {
    Forward(
        MeshedChannels<
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
        MeshedChannels<
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
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoG = Recv<Branching0fromGtoC, End>;

// D
enum Branching0fromGtoD {
    Forward(
        MeshedChannels<
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
        MeshedChannels<
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
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoG = Recv<Branching0fromGtoD, End>;

// E
enum Branching0fromGtoE {
    Forward(
        MeshedChannels<
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
        MeshedChannels<
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
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoG = Recv<Branching0fromGtoE, End>;

// F
enum Branching0fromGtoF {
    Forward(
        MeshedChannels<
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
        MeshedChannels<
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
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoG = Recv<Branching0fromGtoF, End>;

// F
type Choose0fromGtoA = Send<Branching0fromGtoA, End>;
type Choose0fromGtoB = Send<Branching0fromGtoB, End>;
type Choose0fromGtoC = Send<Branching0fromGtoC, End>;
type Choose0fromGtoD = Send<Branching0fromGtoD, End>;
type Choose0fromGtoE = Send<Branching0fromGtoE, End>;
type Choose0fromGtoF = Send<Branching0fromGtoF, End>;
type EndpointForwardG = MeshedChannels<
    Choose0fromGtoA,
    Choose0fromGtoB,
    Choose0fromGtoC,
    Choose0fromGtoD,
    Choose0fromGtoE,
    Recv<(), Choose0fromGtoF>,
    RoleF<RoleBroadcast>,
    NameG,
>;
type EndpointBackwardG = MeshedChannels<
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
type EndpointA = MeshedChannels<End, End, End, End, End, RecursAtoG, RoleG<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, End, End, End, End, RecursBtoG, RoleG<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<End, End, End, End, End, RecursCtoG, RoleG<RoleEnd>, NameC>;
type EndpointD = MeshedChannels<End, End, End, End, End, RecursDtoG, RoleG<RoleEnd>, NameD>;
type EndpointE = MeshedChannels<End, End, End, End, End, RecursEtoG, RoleG<RoleEnd>, NameE>;
type EndpointF = MeshedChannels<End, End, End, End, End, RecursFtoG, RoleG<RoleEnd>, NameF>;
type EndpointG = MeshedChannels<
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
            let s = s.send(());
            endpoint_a(s)
        },
        Branching0fromGtoA::Backward(s) => {
            let (_, s) = s.recv();
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
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_b(s)
        },
        Branching0fromGtoB::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
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
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_c(s)
        },
        Branching0fromGtoC::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
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
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_d(s)
        },
        Branching0fromGtoD::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
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
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_e(s)
        },
        Branching0fromGtoE::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
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
            let ((), s) = s.recv();
            let s = s.send(());
            endpoint_f(s)
        },
        Branching0fromGtoF::Backward(s) => {
            let ((), s) = s.recv();
            let s = s.send(());
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

            let (_, s) = s.recv();

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

            let s = s.send(());

            recurs_g(s, i - 1)
        }
    }
}

fn aux() {
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

static LOOPS: i64 = 100;

pub fn ring(c: &mut Criterion) {
    c.bench_function(&format!("ring seven baking MPST {LOOPS}"), |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = ring,
}

criterion_main! {
    bench
}
