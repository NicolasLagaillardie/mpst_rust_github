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
generate_timed!(MeshedChannels, A, B, C, D, E, F, G, H, I);

// Types
// A
enum Branching0fromItoA {
    Forward(
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoI = RecvTimed<Branching0fromItoA, 'a', 0, true, 10, true, ' ', End>;

// B
enum Branching0fromItoB {
    Forward(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoI = RecvTimed<Branching0fromItoB, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromItoC {
    Forward(
        MeshedChannels<
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
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
        MeshedChannels<
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            RecursCtoI,
            RoleD<RoleB<RoleI<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoI = RecvTimed<Branching0fromItoC, 'a', 0, true, 10, true, ' ', End>;

// D
enum Branching0fromItoD {
    Forward(
        MeshedChannels<
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            RecursDtoI,
            RoleC<RoleE<RoleI<RoleEnd>>>,
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
            End,
            RecursDtoI,
            RoleE<RoleC<RoleI<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoI = RecvTimed<Branching0fromItoD, 'a', 0, true, 10, true, ' ', End>;

// E
enum Branching0fromItoE {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RecursEtoI,
            RoleD<RoleF<RoleI<RoleEnd>>>,
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
            End,
            RecursEtoI,
            RoleF<RoleD<RoleI<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoI = RecvTimed<Branching0fromItoE, 'a', 0, true, 10, true, ' ', End>;

// F
enum Branching0fromItoF {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursFtoI,
            RoleE<RoleG<RoleI<RoleEnd>>>,
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
            End,
            RecursFtoI,
            RoleG<RoleE<RoleI<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoI = RecvTimed<Branching0fromItoF, 'a', 0, true, 10, true, ' ', End>;

// G
enum Branching0fromItoG {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursGtoI,
            RoleF<RoleH<RoleI<RoleEnd>>>,
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
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursGtoI,
            RoleH<RoleF<RoleI<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoI = RecvTimed<Branching0fromItoG, 'a', 0, true, 10, true, ' ', End>;

// H
enum Branching0fromItoH {
    Forward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', RecursHtoI>,
            RoleG<RoleI<RoleI<RoleEnd>>>,
            NameH,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            End,
            End,
            End,
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', RecursHtoI>,
            RoleI<RoleG<RoleI<RoleEnd>>>,
            NameH,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoI = RecvTimed<Branching0fromItoH, 'a', 0, true, 10, true, ' ', End>;

// I
type Choose0fromItoA = SendTimed<Branching0fromItoA, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromItoB = SendTimed<Branching0fromItoB, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromItoC = SendTimed<Branching0fromItoC, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromItoD = SendTimed<Branching0fromItoD, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromItoE = SendTimed<Branching0fromItoE, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromItoF = SendTimed<Branching0fromItoF, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromItoG = SendTimed<Branching0fromItoG, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromItoH = SendTimed<Branching0fromItoH, 'a', 0, true, 10, true, ' ', End>;
type EndpointForwardI = MeshedChannels<
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromItoH>,
    RoleH<RoleBroadcast>,
    NameI,
>;
type EndpointBackwardI = MeshedChannels<
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    SendTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromItoH>,
    RoleH<RoleBroadcast>,
    NameI,
>;

// Creating the MP sessions
type EndpointA =
    MeshedChannels<End, End, End, End, End, End, End, RecursAtoI, RoleI<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannels<End, End, End, End, End, End, End, RecursBtoI, RoleI<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannels<End, End, End, End, End, End, End, RecursCtoI, RoleI<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannels<End, End, End, End, End, End, End, RecursDtoI, RoleI<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannels<End, End, End, End, End, End, End, RecursEtoI, RoleI<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannels<End, End, End, End, End, End, End, RecursFtoI, RoleI<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannels<End, End, End, End, End, End, End, RecursGtoI, RoleI<RoleEnd>, NameG>;
type EndpointH =
    MeshedChannels<End, End, End, End, End, End, End, RecursHtoI, RoleI<RoleEnd>, NameH>;
type EndpointI = MeshedChannels<
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

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoA::Done(s) => {
            s.close()
        },
        Branching0fromItoA::Forward(s) => {
            let s = s.send((), all_clocks)?;
            endpoint_a(s, all_clocks)
        },
        Branching0fromItoA::Backward(s) => {
            let (_, s) = s.recv(all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoB::Done(s) => {
            s.close()
        },
        Branching0fromItoB::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
        Branching0fromItoB::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoC::Done(s) => {
            s.close()
        },
        Branching0fromItoC::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
        Branching0fromItoC::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoD::Done(s) => {
            s.close()
        },
        Branching0fromItoD::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
        Branching0fromItoD::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoE::Done(s) => {
            s.close()
        },
        Branching0fromItoE::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
        Branching0fromItoE::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
    })
}

fn endpoint_f(s: EndpointF, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoF::Done(s) => {
            s.close()
        },
        Branching0fromItoF::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_f(s, all_clocks)
        },
        Branching0fromItoF::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_f(s, all_clocks)
        },
    })
}

fn endpoint_g(s: EndpointG, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoG::Done(s) => {
            s.close()
        },
        Branching0fromItoG::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_g(s, all_clocks)
        },
        Branching0fromItoG::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_g(s, all_clocks)
        },
    })
}

fn endpoint_h(s: EndpointH, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoH::Done(s) => {
            s.close()
        },
        Branching0fromItoH::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_h(s, all_clocks)
        },
        Branching0fromItoH::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_h(s, all_clocks)
        },
    })
}

fn endpoint_i(s: EndpointI, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_i(s, LOOPS, all_clocks)
}

fn recurs_i(
    s: EndpointI,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_i_to_all!(
                s,
                all_clocks,
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
        i if i % 2 == 0 => {
            let s: EndpointForwardI = choose_mpst_i_to_all!(
                s,
                all_clocks,
                Branching0fromItoA::Forward,
                Branching0fromItoB::Forward,
                Branching0fromItoC::Forward,
                Branching0fromItoD::Forward,
                Branching0fromItoE::Forward,
                Branching0fromItoF::Forward,
                Branching0fromItoG::Forward,
                Branching0fromItoH::Forward
            );

            let (_, s) = s.recv(all_clocks)?;

            recurs_i(s, i - 1, all_clocks)
        }
        i => {
            let s: EndpointBackwardI = choose_mpst_i_to_all!(
                s,
                all_clocks,
                Branching0fromItoA::Backward,
                Branching0fromItoB::Backward,
                Branching0fromItoC::Backward,
                Branching0fromItoD::Backward,
                Branching0fromItoE::Backward,
                Branching0fromItoF::Backward,
                Branching0fromItoG::Backward,
                Branching0fromItoH::Backward
            );

            let s = s.send((), all_clocks)?;

            recurs_i(s, i - 1, all_clocks)
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
    c.bench_function(&format!("ring nine baking ATMP {LOOPS}"), |b| b.iter(aux));
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
