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
generate_atmp!(MeshedChannels, A, B, C, D, E, F, G);

// Types
// A
enum Branching0fromGtoA {
    Forward(
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            RecursAtoG,
            RoleB<RoleG<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoG = RecvTimed<Branching0fromGtoA, 'a', 0, true, 10, true, ' ', End>;

// B
enum Branching0fromGtoB {
    Forward(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
type RecursBtoG = RecvTimed<Branching0fromGtoB, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromGtoC {
    Forward(
        MeshedChannels<
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RecursCtoG,
            RoleD<RoleB<RoleG<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoG = RecvTimed<Branching0fromGtoC, 'a', 0, true, 10, true, ' ', End>;

// D
enum Branching0fromGtoD {
    Forward(
        MeshedChannels<
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursDtoG,
            RoleE<RoleC<RoleG<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoG = RecvTimed<Branching0fromGtoD, 'a', 0, true, 10, true, ' ', End>;

// E
enum Branching0fromGtoE {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursEtoG,
            RoleF<RoleD<RoleG<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoG = RecvTimed<Branching0fromGtoE, 'a', 0, true, 10, true, ' ', End>;

// F
enum Branching0fromGtoF {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', RecursFtoG>,
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
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', RecursFtoG>,
            RoleG<RoleE<RoleG<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoG = RecvTimed<Branching0fromGtoF, 'a', 0, true, 10, true, ' ', End>;

// F
type Choose0fromGtoA = SendTimed<Branching0fromGtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromGtoB = SendTimed<Branching0fromGtoB, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromGtoC = SendTimed<Branching0fromGtoC, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromGtoD = SendTimed<Branching0fromGtoD, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromGtoE = SendTimed<Branching0fromGtoE, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromGtoF = SendTimed<Branching0fromGtoF, 'a', 0, true, 10, true, ' ', End>;
type EndpointForwardG = MeshedChannels<
    Choose0fromGtoA,
    Choose0fromGtoB,
    Choose0fromGtoC,
    Choose0fromGtoD,
    Choose0fromGtoE,
    RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromGtoF>,
    RoleF<RoleBroadcast>,
    NameG,
>;
type EndpointBackwardG = MeshedChannels<
    Choose0fromGtoA,
    Choose0fromGtoB,
    Choose0fromGtoC,
    Choose0fromGtoD,
    Choose0fromGtoE,
    SendTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromGtoF>,
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

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoA::Done(s) => {
            s.close()
        },
        Branching0fromGtoA::Forward(s) => {
            let s = s.send((), all_clocks)?;
            endpoint_a(s, all_clocks)
        },
        Branching0fromGtoA::Backward(s) => {
            let (_, s) = s.recv(all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoB::Done(s) => {
            s.close()
        },
        Branching0fromGtoB::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
        Branching0fromGtoB::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoC::Done(s) => {
            s.close()
        },
        Branching0fromGtoC::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
        Branching0fromGtoC::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoD::Done(s) => {
            s.close()
        },
        Branching0fromGtoD::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
        Branching0fromGtoD::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoE::Done(s) => {
            s.close()
        },
        Branching0fromGtoE::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
        Branching0fromGtoE::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
    })
}

fn endpoint_f(s: EndpointF, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoF::Done(s) => {
            s.close()
        },
        Branching0fromGtoF::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_f(s, all_clocks)
        },
        Branching0fromGtoF::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_f(s, all_clocks)
        },
    })
}

fn endpoint_g(s: EndpointG, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_g(s, LOOPS, all_clocks)
}

fn recurs_g(
    s: EndpointG,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_g_to_all!(
                s,
                all_clocks,
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
                all_clocks,
                Branching0fromGtoA::Forward,
                Branching0fromGtoB::Forward,
                Branching0fromGtoC::Forward,
                Branching0fromGtoD::Forward,
                Branching0fromGtoE::Forward,
                Branching0fromGtoF::Forward
            );

            let (_, s) = s.recv(all_clocks)?;

            recurs_g(s, i - 1, all_clocks)
        }
        i => {
            let s: EndpointBackwardG = choose_mpst_g_to_all!(
                s,
                all_clocks,
                Branching0fromGtoA::Backward,
                Branching0fromGtoB::Backward,
                Branching0fromGtoC::Backward,
                Branching0fromGtoD::Backward,
                Branching0fromGtoE::Backward,
                Branching0fromGtoF::Backward
            );

            let s = s.send((), all_clocks)?;

            recurs_g(s, i - 1, all_clocks)
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
    c.bench_function(&format!("ring seven baking ATMP {LOOPS}"), |b| b.iter(aux));
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
