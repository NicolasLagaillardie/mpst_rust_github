#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// use std::time::Duration;

// Create new roles
generate!("rec_and_cancel", MeshedChannelsSeven, A, B, C, D, E, F, G);

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

#[inline]
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

#[inline]
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

#[inline]
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

#[inline]
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

#[inline]
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

#[inline]
fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for i in 1..LOOPS {
        temp_s = recurs_g(temp_s, i)?;
    }

    let s = choose_mpst_g_to_all!(
        temp_s,
        Branching0fromGtoA::Done,
        Branching0fromGtoB::Done,
        Branching0fromGtoC::Done,
        Branching0fromGtoD::Done,
        Branching0fromGtoE::Done,
        Branching0fromGtoF::Done
    );

    s.close()
}

fn recurs_g(s: EndpointG, index: i64) -> Result<EndpointG, Box<dyn Error>> {
    match index {
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

            Ok(s)
        }
        _ => {
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

            Ok(s)
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

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring seven baking inline protocol {LOOPS}"), |b| {
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
