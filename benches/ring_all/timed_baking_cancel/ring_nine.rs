use criterion::{black_box, Criterion};

use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// Create new roles
baker_timed!(MeshedChannelsNine, A, B, C, D, E, F, G, H, I);

// Types
// A
enum Branching0fromItoA {
    Forward(
        MeshedChannelsNine<
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
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
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
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
type RecursAtoI = RecvTimed<Branching0fromItoA, End, 'a', 0, true, 1, true, false>;
// B
enum Branching0fromItoB {
    Forward(
        MeshedChannelsNine<
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
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
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
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
type RecursBtoI = RecvTimed<Branching0fromItoB, End, 'a', 0, true, 1, true, false>;
// C
enum Branching0fromItoC {
    Forward(
        MeshedChannelsNine<
            End,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
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
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
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
type RecursCtoI = RecvTimed<Branching0fromItoC, End, 'a', 0, true, 1, true, false>;
// D
enum Branching0fromItoD {
    Forward(
        MeshedChannelsNine<
            End,
            End,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
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
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
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
type RecursDtoI = RecvTimed<Branching0fromItoD, End, 'a', 0, true, 1, true, false>;
// E
enum Branching0fromItoE {
    Forward(
        MeshedChannelsNine<
            End,
            End,
            End,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
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
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            End,
            End,
            RecursEtoI,
            RoleF<RoleD<RoleI<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoI = RecvTimed<Branching0fromItoE, End, 'a', 0, true, 1, true, false>;
// F
enum Branching0fromItoF {
    Forward(
        MeshedChannelsNine<
            End,
            End,
            End,
            End,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
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
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            End,
            RecursFtoI,
            RoleG<RoleE<RoleI<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoI = RecvTimed<Branching0fromItoF, End, 'a', 0, true, 1, true, false>;
// G
enum Branching0fromItoG {
    Forward(
        MeshedChannelsNine<
            End,
            End,
            End,
            End,
            End,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
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
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            RecursGtoI,
            RoleH<RoleF<RoleI<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoI = RecvTimed<Branching0fromItoG, End, 'a', 0, true, 1, true, false>;
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
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), RecursHtoI, 'a', 0, true, 1, true, false>,
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
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), RecursHtoI, 'a', 0, true, 1, true, false>,
            RoleI<RoleG<RoleI<RoleEnd>>>,
            NameH,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoI = RecvTimed<Branching0fromItoH, End, 'a', 0, true, 1, true, false>;
// I
type Choose0fromItoA = SendTimed<Branching0fromItoA, End, 'a', 0, true, 1, true, false>;
type Choose0fromItoB = SendTimed<Branching0fromItoB, End, 'a', 0, true, 1, true, false>;
type Choose0fromItoC = SendTimed<Branching0fromItoC, End, 'a', 0, true, 1, true, false>;
type Choose0fromItoD = SendTimed<Branching0fromItoD, End, 'a', 0, true, 1, true, false>;
type Choose0fromItoE = SendTimed<Branching0fromItoE, End, 'a', 0, true, 1, true, false>;
type Choose0fromItoF = SendTimed<Branching0fromItoF, End, 'a', 0, true, 1, true, false>;
type Choose0fromItoG = SendTimed<Branching0fromItoG, End, 'a', 0, true, 1, true, false>;
type Choose0fromItoH = SendTimed<Branching0fromItoH, End, 'a', 0, true, 1, true, false>;
type EndpointForwardI = MeshedChannelsNine<
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    RecvTimed<(), Choose0fromItoH, 'a', 0, true, 1, true, false>,
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
    SendTimed<(), Choose0fromItoH, 'a', 0, true, 1, true, false>,
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

fn all_mpst() {
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
        &format!("timed ring nine baking protocol MPST {LOOPS}"),
        |b| b.iter(all_mpst),
    );
}