#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// Create new roles
generate_atmp!(MeshedChannels, A, B, C, D, E, F);

// Types
// A
enum Branching0fromFtoA {
    Forward(
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            RecursAtoF,
            RoleB<RoleF<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            RecursAtoF,
            RoleB<RoleF<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoF = RecvTimed<Branching0fromFtoA, 'a', 0, true, 10, true, ' ', End>;

// B
enum Branching0fromFtoB {
    Forward(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RecursBtoF,
            RoleA<RoleC<RoleF<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RecursBtoF,
            RoleC<RoleA<RoleF<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoF = RecvTimed<Branching0fromFtoB, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromFtoC {
    Forward(
        MeshedChannels<
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursCtoF,
            RoleB<RoleD<RoleF<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursCtoF,
            RoleD<RoleB<RoleF<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoF = RecvTimed<Branching0fromFtoC, 'a', 0, true, 10, true, ' ', End>;

// D
enum Branching0fromFtoD {
    Forward(
        MeshedChannels<
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursDtoF,
            RoleC<RoleE<RoleF<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursDtoF,
            RoleE<RoleC<RoleF<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoF = RecvTimed<Branching0fromFtoD, 'a', 0, true, 10, true, ' ', End>;

// E
enum Branching0fromFtoE {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', RecursEtoF>,
            RoleD<RoleF<RoleF<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', RecursEtoF>,
            RoleF<RoleD<RoleF<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoF = RecvTimed<Branching0fromFtoE, 'a', 0, true, 10, true, ' ', End>;

// F
type Choose0fromFtoA = SendTimed<Branching0fromFtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromFtoB = SendTimed<Branching0fromFtoB, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromFtoC = SendTimed<Branching0fromFtoC, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromFtoD = SendTimed<Branching0fromFtoD, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromFtoE = SendTimed<Branching0fromFtoE, 'a', 0, true, 10, true, ' ', End>;
type EndpointForwardF = MeshedChannels<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromFtoE>,
    RoleE<RoleBroadcast>,
    NameF,
>;
type EndpointBackwardF = MeshedChannels<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    SendTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromFtoE>,
    RoleE<RoleBroadcast>,
    NameF,
>;

// Creating the MP sessions
type EndpointA = MeshedChannels<End, End, End, End, RecursAtoF, RoleF<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, End, End, End, RecursBtoF, RoleF<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<End, End, End, End, RecursCtoF, RoleF<RoleEnd>, NameC>;
type EndpointD = MeshedChannels<End, End, End, End, RecursDtoF, RoleF<RoleEnd>, NameD>;
type EndpointE = MeshedChannels<End, End, End, End, RecursEtoF, RoleF<RoleEnd>, NameE>;
type EndpointF = MeshedChannels<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Choose0fromFtoE,
    RoleBroadcast,
    NameF,
>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromFtoA::Done(s) => {
            s.close()
        },
        Branching0fromFtoA::Forward(s) => {
            let s = s.send((), all_clocks)?;
            endpoint_a(s, all_clocks)
        },
        Branching0fromFtoA::Backward(s) => {
            let (_, s) = s.recv(all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromFtoB::Done(s) => {
            s.close()
        },
        Branching0fromFtoB::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
        Branching0fromFtoB::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromFtoC::Done(s) => {
            s.close()
        },
        Branching0fromFtoC::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
        Branching0fromFtoC::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromFtoD::Done(s) => {
            s.close()
        },
        Branching0fromFtoD::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
        Branching0fromFtoD::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromFtoE::Done(s) => {
            s.close()
        },
        Branching0fromFtoE::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
        Branching0fromFtoE::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
    })
}

fn endpoint_f(s: EndpointF, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_f(s, LOOPS, all_clocks)
}

fn recurs_f(
    s: EndpointF,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_f_to_all!(
                s,
                all_clocks,
                Branching0fromFtoA::Done,
                Branching0fromFtoB::Done,
                Branching0fromFtoC::Done,
                Branching0fromFtoD::Done,
                Branching0fromFtoE::Done
            );

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardF = choose_mpst_f_to_all!(
                s,
                all_clocks,
                Branching0fromFtoA::Forward,
                Branching0fromFtoB::Forward,
                Branching0fromFtoC::Forward,
                Branching0fromFtoD::Forward,
                Branching0fromFtoE::Forward
            );

            let (_, s) = s.recv(all_clocks)?;

            recurs_f(s, i - 1, all_clocks)
        }
        i => {
            let s: EndpointBackwardF = choose_mpst_f_to_all!(
                s,
                all_clocks,
                Branching0fromFtoA::Backward,
                Branching0fromFtoB::Backward,
                Branching0fromFtoC::Backward,
                Branching0fromFtoD::Backward,
                Branching0fromFtoE::Backward
            );

            let s = s.send((), all_clocks)?;

            recurs_f(s, i - 1, all_clocks)
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

pub fn ring(c: &mut Criterion) {
    c.bench_function(&format!("ring six baking ATMP {LOOPS}"), |b| b.iter(aux));
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
