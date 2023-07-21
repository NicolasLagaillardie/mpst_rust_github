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

// Create new roles
generate_timed!(MeshedChannels, A, B, C, D, E);

// Types
// A
enum Branching0fromEtoA {
    Forward(
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RecursAtoE,
            RoleB<RoleE<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RecursAtoE,
            RoleB<RoleE<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoE = RecvTimed<Branching0fromEtoA, 'a', 0, true, 10, true, ' ', End>;

// B
enum Branching0fromEtoB {
    Forward(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursBtoE,
            RoleA<RoleC<RoleE<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursBtoE,
            RoleC<RoleA<RoleE<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoE = RecvTimed<Branching0fromEtoB, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromEtoC {
    Forward(
        MeshedChannels<
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursCtoE,
            RoleB<RoleD<RoleE<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursCtoE,
            RoleD<RoleB<RoleE<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoE = RecvTimed<Branching0fromEtoC, 'a', 0, true, 10, true, ' ', End>;

// D
enum Branching0fromEtoD {
    Forward(
        MeshedChannels<
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', RecursDtoE>,
            RoleC<RoleE<RoleE<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', RecursDtoE>,
            RoleE<RoleC<RoleE<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoE = RecvTimed<Branching0fromEtoD, 'a', 0, true, 10, true, ' ', End>;

// E
type Choose0fromEtoA = SendTimed<Branching0fromEtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromEtoB = SendTimed<Branching0fromEtoB, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromEtoC = SendTimed<Branching0fromEtoC, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromEtoD = SendTimed<Branching0fromEtoD, 'a', 0, true, 10, true, ' ', End>;
type EndpointForwardE = MeshedChannels<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromEtoD>,
    RoleD<RoleBroadcast>,
    NameE,
>;
type EndpointBackwardE = MeshedChannels<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    SendTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromEtoD>,
    RoleD<RoleBroadcast>,
    NameE,
>;

// Creating the MP sessions
type EndpointA = MeshedChannels<End, End, End, RecursAtoE, RoleE<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, End, End, RecursBtoE, RoleE<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<End, End, End, RecursCtoE, RoleE<RoleEnd>, NameC>;
type EndpointD = MeshedChannels<End, End, End, RecursDtoE, RoleE<RoleEnd>, NameD>;
type EndpointE = MeshedChannels<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Choose0fromEtoD,
    RoleBroadcast,
    NameE,
>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromEtoA::Done(s) => {
            s.close()
        },
        Branching0fromEtoA::Forward(s) => {
            let s = s.send((), all_clocks)?;
            endpoint_a(s, all_clocks)
        },
        Branching0fromEtoA::Backward(s) => {
            let (_, s) = s.recv(all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromEtoB::Done(s) => {
            s.close()
        },
        Branching0fromEtoB::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
        Branching0fromEtoB::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromEtoC::Done(s) => {
            s.close()
        },
        Branching0fromEtoC::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
        Branching0fromEtoC::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromEtoD::Done(s) => {
            s.close()
        },
        Branching0fromEtoD::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
        Branching0fromEtoD::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_e(s, LOOPS, all_clocks)
}

fn recurs_e(
    s: EndpointE,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_e_to_all!(
                s,
                all_clocks,
                Branching0fromEtoA::Done,
                Branching0fromEtoB::Done,
                Branching0fromEtoC::Done,
                Branching0fromEtoD::Done
            );

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardE = choose_mpst_e_to_all!(
                s,
                all_clocks,
                Branching0fromEtoA::Forward,
                Branching0fromEtoB::Forward,
                Branching0fromEtoC::Forward,
                Branching0fromEtoD::Forward
            );

            let (_, s) = s.recv(all_clocks)?;

            recurs_e(s, i - 1, all_clocks)
        }
        i => {
            let s: EndpointBackwardE = choose_mpst_e_to_all!(
                s,
                all_clocks,
                Branching0fromEtoA::Backward,
                Branching0fromEtoB::Backward,
                Branching0fromEtoC::Backward,
                Branching0fromEtoD::Backward
            );

            let s = s.send((), all_clocks)?;

            recurs_e(s, i - 1, all_clocks)
        }
    }
}

fn aux() {
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

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring five baking protocol ATMP {LOOPS}"), |b| {
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
