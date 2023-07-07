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
generate_timed!(MeshedChannelsTen, A, B, C, D, E, F, G, H, I, J);

// Types
// A
enum Branching0fromJtoA {
    Forward(
        MeshedChannelsTen<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoJ,
            RoleB<RoleJ<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursAtoJ,
            RoleB<RoleJ<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoJ = RecvTimed<Branching0fromJtoA, 'a', 0, true, 10, true, ' ', End>;

// B
enum Branching0fromJtoB {
    Forward(
        MeshedChannelsTen<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursBtoJ,
            RoleA<RoleC<RoleJ<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            End,
            End,
            RecursBtoJ,
            RoleC<RoleA<RoleJ<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoJ = RecvTimed<Branching0fromJtoB, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromJtoC {
    Forward(
        MeshedChannelsTen<
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            End,
            RecursCtoJ,
            RoleB<RoleD<RoleJ<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            End,
            RecursCtoJ,
            RoleD<RoleB<RoleJ<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoJ = RecvTimed<Branching0fromJtoC, 'a', 0, true, 10, true, ' ', End>;

// D
enum Branching0fromJtoD {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            RecursDtoJ,
            RoleC<RoleE<RoleJ<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            End,
            RecursDtoJ,
            RoleE<RoleC<RoleJ<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoJ = RecvTimed<Branching0fromJtoD, 'a', 0, true, 10, true, ' ', End>;

// E
enum Branching0fromJtoE {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            RecursEtoJ,
            RoleD<RoleF<RoleJ<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            End,
            RecursEtoJ,
            RoleF<RoleD<RoleJ<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoJ = RecvTimed<Branching0fromJtoE, 'a', 0, true, 10, true, ' ', End>;

// F
enum Branching0fromJtoF {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RecursFtoJ,
            RoleE<RoleG<RoleJ<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RecursFtoJ,
            RoleG<RoleE<RoleJ<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoJ = RecvTimed<Branching0fromJtoF, 'a', 0, true, 10, true, ' ', End>;

// G
enum Branching0fromJtoG {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursGtoJ,
            RoleF<RoleH<RoleJ<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursGtoJ,
            RoleH<RoleF<RoleJ<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoJ = RecvTimed<Branching0fromJtoG, 'a', 0, true, 10, true, ' ', End>;

// H
enum Branching0fromJtoH {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursHtoJ,
            RoleG<RoleI<RoleJ<RoleEnd>>>,
            NameH,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursHtoJ,
            RoleI<RoleG<RoleJ<RoleEnd>>>,
            NameH,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoJ = RecvTimed<Branching0fromJtoH, 'a', 0, true, 10, true, ' ', End>;

// I
enum Branching0fromJtoI {
    Forward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', RecursItoJ>,
            RoleH<RoleJ<RoleJ<RoleEnd>>>,
            NameI,
        >,
    ),
    Backward(
        MeshedChannelsTen<
            End,
            End,
            End,
            End,
            End,
            End,
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', RecursItoJ>,
            RoleJ<RoleH<RoleJ<RoleEnd>>>,
            NameI,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoJ = RecvTimed<Branching0fromJtoI, 'a', 0, true, 10, true, ' ', End>;

// J
type Choose0fromJtoA = SendTimed<Branching0fromJtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromJtoB = SendTimed<Branching0fromJtoB, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromJtoC = SendTimed<Branching0fromJtoC, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromJtoD = SendTimed<Branching0fromJtoD, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromJtoE = SendTimed<Branching0fromJtoE, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromJtoF = SendTimed<Branching0fromJtoF, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromJtoG = SendTimed<Branching0fromJtoG, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromJtoH = SendTimed<Branching0fromJtoH, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromJtoI = SendTimed<Branching0fromJtoI, 'a', 0, true, 10, true, ' ', End>;
type EndpointForwardJ = MeshedChannelsTen<
    Choose0fromJtoA,
    Choose0fromJtoB,
    Choose0fromJtoC,
    Choose0fromJtoD,
    Choose0fromJtoE,
    Choose0fromJtoF,
    Choose0fromJtoG,
    Choose0fromJtoH,
    RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromJtoI>,
    RoleI<RoleBroadcast>,
    NameJ,
>;
type EndpointBackwardJ = MeshedChannelsTen<
    Choose0fromJtoA,
    Choose0fromJtoB,
    Choose0fromJtoC,
    Choose0fromJtoD,
    Choose0fromJtoE,
    Choose0fromJtoF,
    Choose0fromJtoG,
    Choose0fromJtoH,
    SendTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromJtoI>,
    RoleI<RoleBroadcast>,
    NameJ,
>;

// Creating the MP sessions
type EndpointA =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursAtoJ, RoleJ<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursBtoJ, RoleJ<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursCtoJ, RoleJ<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursDtoJ, RoleJ<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursEtoJ, RoleJ<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursFtoJ, RoleJ<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursGtoJ, RoleJ<RoleEnd>, NameG>;
type EndpointH =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursHtoJ, RoleJ<RoleEnd>, NameH>;
type EndpointI =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursItoJ, RoleJ<RoleEnd>, NameI>;
type EndpointJ = MeshedChannelsTen<
    Choose0fromJtoA,
    Choose0fromJtoB,
    Choose0fromJtoC,
    Choose0fromJtoD,
    Choose0fromJtoE,
    Choose0fromJtoF,
    Choose0fromJtoG,
    Choose0fromJtoH,
    Choose0fromJtoI,
    RoleBroadcast,
    NameJ,
>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoA::Done(s) => {
            s.close()
        },
        Branching0fromJtoA::Forward(s) => {
            let s = s.send((), all_clocks)?;
            endpoint_a(s, all_clocks)
        },
        Branching0fromJtoA::Backward(s) => {
            let (_, s) = s.recv(all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoB::Done(s) => {
            s.close()
        },
        Branching0fromJtoB::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
        Branching0fromJtoB::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoC::Done(s) => {
            s.close()
        },
        Branching0fromJtoC::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
        Branching0fromJtoC::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoD::Done(s) => {
            s.close()
        },
        Branching0fromJtoD::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
        Branching0fromJtoD::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoE::Done(s) => {
            s.close()
        },
        Branching0fromJtoE::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
        Branching0fromJtoE::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
    })
}

fn endpoint_f(s: EndpointF, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoF::Done(s) => {
            s.close()
        },
        Branching0fromJtoF::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_f(s, all_clocks)
        },
        Branching0fromJtoF::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_f(s, all_clocks)
        },
    })
}

fn endpoint_g(s: EndpointG, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoG::Done(s) => {
            s.close()
        },
        Branching0fromJtoG::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_g(s, all_clocks)
        },
        Branching0fromJtoG::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_g(s, all_clocks)
        },
    })
}

fn endpoint_h(s: EndpointH, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoH::Done(s) => {
            s.close()
        },
        Branching0fromJtoH::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_h(s, all_clocks)
        },
        Branching0fromJtoH::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_h(s, all_clocks)
        },
    })
}

fn endpoint_i(s: EndpointI, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoI::Done(s) => {
            s.close()
        },
        Branching0fromJtoI::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_i(s, all_clocks)
        },
        Branching0fromJtoI::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_i(s, all_clocks)
        },
    })
}

fn endpoint_j(s: EndpointJ, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_j(s, LOOPS, all_clocks)
}

fn recurs_j(
    s: EndpointJ,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_j_to_all!(
                s,
                all_clocks,
                Branching0fromJtoA::Done,
                Branching0fromJtoB::Done,
                Branching0fromJtoC::Done,
                Branching0fromJtoD::Done,
                Branching0fromJtoE::Done,
                Branching0fromJtoF::Done,
                Branching0fromJtoG::Done,
                Branching0fromJtoH::Done,
                Branching0fromJtoI::Done
            );

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardJ = choose_mpst_j_to_all!(
                s,
                all_clocks,
                Branching0fromJtoA::Forward,
                Branching0fromJtoB::Forward,
                Branching0fromJtoC::Forward,
                Branching0fromJtoD::Forward,
                Branching0fromJtoE::Forward,
                Branching0fromJtoF::Forward,
                Branching0fromJtoG::Forward,
                Branching0fromJtoH::Forward,
                Branching0fromJtoI::Forward
            );

            let (_, s) = s.recv(all_clocks)?;

            recurs_j(s, i - 1, all_clocks)
        }
        i => {
            let s: EndpointBackwardJ = choose_mpst_j_to_all!(
                s,
                all_clocks,
                Branching0fromJtoA::Backward,
                Branching0fromJtoB::Backward,
                Branching0fromJtoC::Backward,
                Branching0fromJtoD::Backward,
                Branching0fromJtoE::Backward,
                Branching0fromJtoF::Backward,
                Branching0fromJtoG::Backward,
                Branching0fromJtoH::Backward,
                Branching0fromJtoI::Backward
            );

            let s = s.send((), all_clocks)?;

            recurs_j(s, i - 1, all_clocks)
        }
    }
}

fn aux() {
    let (
        thread_a,
        thread_b,
        thread_c,
        thread_d,
        thread_e,
        thread_f,
        thread_g,
        thread_h,
        thread_i,
        thread_j,
    ) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
        black_box(endpoint_g),
        black_box(endpoint_h),
        black_box(endpoint_i),
        black_box(endpoint_j),
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
    thread_j.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring ten baking protocol ATMP {LOOPS}"), |b| {
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
