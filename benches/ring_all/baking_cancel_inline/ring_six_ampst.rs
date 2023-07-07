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
generate!("rec_and_cancel", MeshedChannelsSix, A, B, C, D, E, F);

// Types
// A
enum Branching0fromFtoA {
    Forward(
        MeshedChannelsSix<Send<(), End>, End, End, End, RecursAtoF, RoleB<RoleF<RoleEnd>>, NameA>,
    ),
    Backward(
        MeshedChannelsSix<Recv<(), End>, End, End, End, RecursAtoF, RoleB<RoleF<RoleEnd>>, NameA>,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoF = <Choose0fromFtoA as Session>::Dual;

// B
enum Branching0fromFtoB {
    Forward(
        MeshedChannelsSix<
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursBtoF,
            RoleA<RoleC<RoleF<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsSix<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursBtoF,
            RoleC<RoleA<RoleF<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoF = <Choose0fromFtoB as Session>::Dual;

// C
enum Branching0fromFtoC {
    Forward(
        MeshedChannelsSix<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursCtoF,
            RoleB<RoleD<RoleF<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsSix<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursCtoF,
            RoleD<RoleB<RoleF<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoF = <Choose0fromFtoC as Session>::Dual;

// D
enum Branching0fromFtoD {
    Forward(
        MeshedChannelsSix<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursDtoF,
            RoleC<RoleE<RoleF<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsSix<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursDtoF,
            RoleE<RoleC<RoleF<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoF = <Choose0fromFtoD as Session>::Dual;

// E
enum Branching0fromFtoE {
    Forward(
        MeshedChannelsSix<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursEtoF>,
            RoleD<RoleF<RoleF<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannelsSix<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursEtoF>,
            RoleF<RoleD<RoleF<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoF = <Choose0fromFtoE as Session>::Dual;

// F
type Choose0fromFtoA = Send<Branching0fromFtoA, End>;
type Choose0fromFtoB = Send<Branching0fromFtoB, End>;
type Choose0fromFtoC = Send<Branching0fromFtoC, End>;
type Choose0fromFtoD = Send<Branching0fromFtoD, End>;
type Choose0fromFtoE = Send<Branching0fromFtoE, End>;
type EndpointForwardF = MeshedChannelsSix<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Recv<(), Choose0fromFtoE>,
    RoleE<RoleBroadcast>,
    NameF,
>;
type EndpointBackwardF = MeshedChannelsSix<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Send<(), Choose0fromFtoE>,
    RoleE<RoleBroadcast>,
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

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromFtoA::Done(s) => {
            s.close()
        },
        Branching0fromFtoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromFtoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

#[inline]
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromFtoB::Done(s) => {
            s.close()
        },
        Branching0fromFtoB::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
        Branching0fromFtoB::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

#[inline]
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromFtoC::Done(s) => {
            s.close()
        },
        Branching0fromFtoC::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
        Branching0fromFtoC::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
    })
}

#[inline]
fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromFtoD::Done(s) => {
            s.close()
        },
        Branching0fromFtoD::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
        Branching0fromFtoD::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
    })
}

#[inline]
fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromFtoE::Done(s) => {
            s.close()
        },
        Branching0fromFtoE::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
        Branching0fromFtoE::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
    })
}

#[inline]
fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for i in 1..LOOPS {
        temp_s = recurs_f(temp_s, i)?;
    }

    let s = choose_mpst_f_to_all!(
        temp_s,
        Branching0fromFtoA::Done,
        Branching0fromFtoB::Done,
        Branching0fromFtoC::Done,
        Branching0fromFtoD::Done,
        Branching0fromFtoE::Done
    );

    s.close()
}

fn recurs_f(s: EndpointF, index: i64) -> Result<EndpointF, Box<dyn Error>> {
    match index {
        i if i % 2 == 0 => {
            let s: EndpointForwardF = choose_mpst_f_to_all!(
                s,
                Branching0fromFtoA::Forward,
                Branching0fromFtoB::Forward,
                Branching0fromFtoC::Forward,
                Branching0fromFtoD::Forward,
                Branching0fromFtoE::Forward
            );

            let (_, s) = s.recv()?;

            Ok(s)
        }
        _ => {
            let s: EndpointBackwardF = choose_mpst_f_to_all!(
                s,
                Branching0fromFtoA::Backward,
                Branching0fromFtoB::Backward,
                Branching0fromFtoC::Backward,
                Branching0fromFtoD::Backward,
                Branching0fromFtoE::Backward
            );

            let s = s.send(())?;

            Ok(s)
        }
    }
}

fn aux() {
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

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("ring six baking inline protocol MPST {LOOPS}"),
        |b| b.iter(aux),
    );
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
