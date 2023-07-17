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
generate!(
    "rec_and_cancel",
    MeshedChannelsNine,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I
);

// Types
// A
enum Branching0fromItoA {
    Forward(
        MeshedChannelsNine<
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoI,
            RoleB<RoleI<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsNine<
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoI,
            RoleB<RoleI<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoI = <Choose0fromItoA as Session>::Dual;

// B
enum Branching0fromItoB {
    Forward(
        MeshedChannelsNine<
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursBtoI,
            RoleA<RoleC<RoleI<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsNine<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursBtoI,
            RoleC<RoleA<RoleI<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoI = <Choose0fromItoB as Session>::Dual;

// C
enum Branching0fromItoC {
    Forward(
        MeshedChannelsNine<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            RecursCtoI,
            RoleB<RoleD<RoleI<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsNine<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            RecursCtoI,
            RoleD<RoleB<RoleI<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoI = <Choose0fromItoC as Session>::Dual;

// D
enum Branching0fromItoD {
    Forward(
        MeshedChannelsNine<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            RecursDtoI,
            RoleC<RoleE<RoleI<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsNine<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            RecursDtoI,
            RoleE<RoleC<RoleI<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoI = <Choose0fromItoD as Session>::Dual;

// E
enum Branching0fromItoE {
    Forward(
        MeshedChannelsNine<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursEtoI,
            RoleD<RoleF<RoleI<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannelsNine<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursEtoI,
            RoleF<RoleD<RoleI<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoI = <Choose0fromItoE as Session>::Dual;

// F
enum Branching0fromItoF {
    Forward(
        MeshedChannelsNine<
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursFtoI,
            RoleE<RoleG<RoleI<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannelsNine<
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursFtoI,
            RoleG<RoleE<RoleI<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoI = <Choose0fromItoF as Session>::Dual;

// G
enum Branching0fromItoG {
    Forward(
        MeshedChannelsNine<
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursGtoI,
            RoleF<RoleH<RoleI<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannelsNine<
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursGtoI,
            RoleH<RoleF<RoleI<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoI = <Choose0fromItoG as Session>::Dual;

// H
enum Branching0fromItoH {
    Forward(
        MeshedChannelsNine<
            End,
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursHtoI>,
            RoleG<RoleI<RoleI<RoleEnd>>>,
            NameH,
        >,
    ),
    Backward(
        MeshedChannelsNine<
            End,
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursHtoI>,
            RoleI<RoleG<RoleI<RoleEnd>>>,
            NameH,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoI = <Choose0fromItoH as Session>::Dual;

// I
type Choose0fromItoA = Send<Branching0fromItoA, End>;
type Choose0fromItoB = Send<Branching0fromItoB, End>;
type Choose0fromItoC = Send<Branching0fromItoC, End>;
type Choose0fromItoD = Send<Branching0fromItoD, End>;
type Choose0fromItoE = Send<Branching0fromItoE, End>;
type Choose0fromItoF = Send<Branching0fromItoF, End>;
type Choose0fromItoG = Send<Branching0fromItoG, End>;
type Choose0fromItoH = Send<Branching0fromItoH, End>;
type EndpointForwardI = MeshedChannelsNine<
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    Recv<(), Choose0fromItoH>,
    RoleH<RoleBroadcast>,
    NameI,
>;
type EndpointBackwardI = MeshedChannelsNine<
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    Send<(), Choose0fromItoH>,
    RoleH<RoleBroadcast>,
    NameI,
>;

// Creating the MP sessions
type EndpointA =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursAtoI, RoleI<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursBtoI, RoleI<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursCtoI, RoleI<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursDtoI, RoleI<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursEtoI, RoleI<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursFtoI, RoleI<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursGtoI, RoleI<RoleEnd>, NameG>;
type EndpointH =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursHtoI, RoleI<RoleEnd>, NameH>;
type EndpointI = MeshedChannelsNine<
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    Choose0fromItoH,
    RoleBroadcast,
    NameI,
>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromItoA::Done(s) => {
            s.close()
        },
        Branching0fromItoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromItoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

#[inline]
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromItoB::Done(s) => {
            s.close()
        },
        Branching0fromItoB::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
        Branching0fromItoB::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

#[inline]
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromItoC::Done(s) => {
            s.close()
        },
        Branching0fromItoC::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
        Branching0fromItoC::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
    })
}

#[inline]
fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromItoD::Done(s) => {
            s.close()
        },
        Branching0fromItoD::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
        Branching0fromItoD::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
    })
}

#[inline]
fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromItoE::Done(s) => {
            s.close()
        },
        Branching0fromItoE::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
        Branching0fromItoE::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
    })
}

#[inline]
fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromItoF::Done(s) => {
            s.close()
        },
        Branching0fromItoF::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
        Branching0fromItoF::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
    })
}

#[inline]
fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromItoG::Done(s) => {
            s.close()
        },
        Branching0fromItoG::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_g(s)
        },
        Branching0fromItoG::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_g(s)
        },
    })
}

#[inline]
fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromItoH::Done(s) => {
            s.close()
        },
        Branching0fromItoH::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_h(s)
        },
        Branching0fromItoH::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_h(s)
        },
    })
}

#[inline]
fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for i in 1..LOOPS {
        temp_s = recurs_i(temp_s, i)?;
    }

    let s = choose_mpst_i_to_all!(
        temp_s,
        Branching0fromItoA::Done,
        Branching0fromItoB::Done,
        Branching0fromItoC::Done,
        Branching0fromItoD::Done,
        Branching0fromItoE::Done,
        Branching0fromItoF::Done,
        Branching0fromItoG::Done,
        Branching0fromItoH::Done
    );

    s.close()
}

fn recurs_i(s: EndpointI, index: i64) -> Result<EndpointI, Box<dyn Error>> {
    match index {
        i if i % 2 == 0 => {
            let s: EndpointForwardI = choose_mpst_i_to_all!(
                s,
                Branching0fromItoA::Forward,
                Branching0fromItoB::Forward,
                Branching0fromItoC::Forward,
                Branching0fromItoD::Forward,
                Branching0fromItoE::Forward,
                Branching0fromItoF::Forward,
                Branching0fromItoG::Forward,
                Branching0fromItoH::Forward
            );

            let (_, s) = s.recv()?;

            Ok(s)
        }
        _ => {
            let s: EndpointBackwardI = choose_mpst_i_to_all!(
                s,
                Branching0fromItoA::Backward,
                Branching0fromItoB::Backward,
                Branching0fromItoC::Backward,
                Branching0fromItoD::Backward,
                Branching0fromItoE::Backward,
                Branching0fromItoF::Backward,
                Branching0fromItoG::Backward,
                Branching0fromItoH::Backward
            );

            let s = s.send(())?;

            Ok(s)
        }
    }
}

fn aux() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g, thread_h, thread_i) =
        fork_mpst(
            black_box(endpoint_a),
            black_box(endpoint_b),
            black_box(endpoint_c),
            black_box(endpoint_d),
            black_box(endpoint_e),
            black_box(endpoint_f),
            black_box(endpoint_g),
            black_box(endpoint_h),
            black_box(endpoint_i),
        );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
    thread_h.join().unwrap();
    thread_i.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("ring nine baking inline protocol {LOOPS}"),
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
