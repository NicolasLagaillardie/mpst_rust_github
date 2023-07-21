#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_timed;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_timed!(MeshedChannels, A, B, C, D, E, F, G, H);

// Types
// A
enum Branching0fromHtoA {
    Forward(
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
    Done(MeshedChannels<End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoH = RecvTimed<Branching0fromHtoA, 'a', 0, true, 10, true, ' ', End>;

// B
enum Branching0fromHtoB {
    Forward(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            RecursBtoH,
            RoleC<RoleA<RoleH<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoH = RecvTimed<Branching0fromHtoB, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromHtoC {
    Forward(
        MeshedChannels<
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            RecursCtoH,
            RoleB<RoleD<RoleH<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            RecursCtoH,
            RoleD<RoleB<RoleH<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoH = RecvTimed<Branching0fromHtoC, 'a', 0, true, 10, true, ' ', End>;

// D
enum Branching0fromHtoD {
    Forward(
        MeshedChannels<
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RecursDtoH,
            RoleC<RoleE<RoleH<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RecursDtoH,
            RoleE<RoleC<RoleH<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoH = RecvTimed<Branching0fromHtoD, 'a', 0, true, 10, true, ' ', End>;

// E
enum Branching0fromHtoE {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursEtoH,
            RoleD<RoleF<RoleH<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursEtoH,
            RoleF<RoleD<RoleH<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoH = RecvTimed<Branching0fromHtoE, 'a', 0, true, 10, true, ' ', End>;

// F
enum Branching0fromHtoF {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursFtoH,
            RoleE<RoleG<RoleH<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursFtoH,
            RoleG<RoleE<RoleH<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoH = RecvTimed<Branching0fromHtoF, 'a', 0, true, 10, true, ' ', End>;

// G
enum Branching0fromHtoG {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', RecursGtoH>,
            RoleF<RoleH<RoleH<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', RecursGtoH>,
            RoleH<RoleF<RoleH<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoH = RecvTimed<Branching0fromHtoG, 'a', 0, true, 10, true, ' ', End>;

// H
type Choose0fromHtoA = SendTimed<Branching0fromHtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoB = SendTimed<Branching0fromHtoB, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoC = SendTimed<Branching0fromHtoC, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoD = SendTimed<Branching0fromHtoD, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoE = SendTimed<Branching0fromHtoE, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoF = SendTimed<Branching0fromHtoF, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoG = SendTimed<Branching0fromHtoG, 'a', 0, true, 10, true, ' ', End>;
type EndpointForwardH = MeshedChannels<
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromHtoG>,
    RoleG<RoleBroadcast>,
    NameH,
>;
type EndpointBackwardH = MeshedChannels<
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    SendTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromHtoG>,
    RoleG<RoleBroadcast>,
    NameH,
>;

// Creating the MP sessions
type EndpointA = MeshedChannels<End, End, End, End, End, End, RecursAtoH, RoleH<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, End, End, End, End, End, RecursBtoH, RoleH<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<End, End, End, End, End, End, RecursCtoH, RoleH<RoleEnd>, NameC>;
type EndpointD = MeshedChannels<End, End, End, End, End, End, RecursDtoH, RoleH<RoleEnd>, NameD>;
type EndpointE = MeshedChannels<End, End, End, End, End, End, RecursEtoH, RoleH<RoleEnd>, NameE>;
type EndpointF = MeshedChannels<End, End, End, End, End, End, RecursFtoH, RoleH<RoleEnd>, NameF>;
type EndpointG = MeshedChannels<End, End, End, End, End, End, RecursGtoH, RoleH<RoleEnd>, NameG>;
type EndpointH = MeshedChannels<
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

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoA::Done(s) => {
            s.close()
        },
        Branching0fromHtoA::Forward(s) => {
            let s = s.send((), all_clocks)?;
            endpoint_a(s, all_clocks)
        },
        Branching0fromHtoA::Backward(s) => {
            let (_, s) = s.recv(all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoB::Done(s) => {
            s.close()
        },
        Branching0fromHtoB::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
        Branching0fromHtoB::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoC::Done(s) => {
            s.close()
        },
        Branching0fromHtoC::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
        Branching0fromHtoC::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoD::Done(s) => {
            s.close()
        },
        Branching0fromHtoD::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
        Branching0fromHtoD::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoE::Done(s) => {
            s.close()
        },
        Branching0fromHtoE::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
        Branching0fromHtoE::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
    })
}

fn endpoint_f(s: EndpointF, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoF::Done(s) => {
            s.close()
        },
        Branching0fromHtoF::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_f(s, all_clocks)
        },
        Branching0fromHtoF::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_f(s, all_clocks)
        },
    })
}

fn endpoint_g(s: EndpointG, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoG::Done(s) => {
            s.close()
        },
        Branching0fromHtoG::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_g(s, all_clocks)
        },
        Branching0fromHtoG::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_g(s, all_clocks)
        },
    })
}

fn endpoint_h(s: EndpointH, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_h(s, LOOPS, all_clocks)
}

fn recurs_h(
    s: EndpointH,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_h_to_all!(
                s,
                all_clocks,
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
                all_clocks,
                Branching0fromHtoA::Forward,
                Branching0fromHtoB::Forward,
                Branching0fromHtoC::Forward,
                Branching0fromHtoD::Forward,
                Branching0fromHtoE::Forward,
                Branching0fromHtoF::Forward,
                Branching0fromHtoG::Forward
            );

            let (_, s) = s.recv(all_clocks)?;

            recurs_h(s, i - 1, all_clocks)
        }
        i => {
            let s: EndpointBackwardH = choose_mpst_h_to_all!(
                s,
                all_clocks,
                Branching0fromHtoA::Backward,
                Branching0fromHtoB::Backward,
                Branching0fromHtoC::Backward,
                Branching0fromHtoD::Backward,
                Branching0fromHtoE::Backward,
                Branching0fromHtoF::Backward,
                Branching0fromHtoG::Backward
            );

            let s = s.send((), all_clocks)?;

            recurs_h(s, i - 1, all_clocks)
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
    c.bench_function(&format!("ring eight baking protocol ATMP {LOOPS}"), |b| {
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
