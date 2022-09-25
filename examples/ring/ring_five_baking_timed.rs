use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

static LOOPS: i64 = 100;

// Create new roles
baker_timed!(MeshedChannelsFive, A, B, C, D, E);

// Types
// A
enum Branching0fromEtoA {
    Forward(
        MeshedChannelsFive<
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            End,
            End,
            RecursAtoE,
            RoleB<RoleE<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsFive<
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            End,
            End,
            RecursAtoE,
            RoleB<RoleE<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoE = RecvTimed<Branching0fromEtoA, End, 'a', 0, true, 1, true, false>;
// B
enum Branching0fromEtoB {
    Forward(
        MeshedChannelsFive<
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            End,
            RecursBtoE,
            RoleA<RoleC<RoleE<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsFive<
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            End,
            RecursBtoE,
            RoleC<RoleA<RoleE<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoE = RecvTimed<Branching0fromEtoB, End, 'a', 0, true, 1, true, false>;
// C
enum Branching0fromEtoC {
    Forward(
        MeshedChannelsFive<
            End,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecursCtoE,
            RoleB<RoleD<RoleE<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsFive<
            End,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            RecursCtoE,
            RoleD<RoleB<RoleE<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoE = RecvTimed<Branching0fromEtoC, End, 'a', 0, true, 1, true, false>;
// D
enum Branching0fromEtoD {
    Forward(
        MeshedChannelsFive<
            End,
            End,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), RecursDtoE, 'a', 0, true, 1, true, false>,
            RoleC<RoleE<RoleE<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsFive<
            End,
            End,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), RecursDtoE, 'a', 0, true, 1, true, false>,
            RoleE<RoleC<RoleE<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoE = RecvTimed<Branching0fromEtoD, End, 'a', 0, true, 1, true, false>;
// E
type Choose0fromEtoA = SendTimed<Branching0fromEtoA, End, 'a', 0, true, 1, true, false>;
type Choose0fromEtoB = SendTimed<Branching0fromEtoB, End, 'a', 0, true, 1, true, false>;
type Choose0fromEtoC = SendTimed<Branching0fromEtoC, End, 'a', 0, true, 1, true, false>;
type Choose0fromEtoD = SendTimed<Branching0fromEtoD, End, 'a', 0, true, 1, true, false>;
type EndpointForwardE = MeshedChannelsFive<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    RecvTimed<(), Choose0fromEtoD, 'a', 0, true, 1, true, false>,
    RoleD<RoleBroadcast>,
    NameE,
>;
type EndpointBackwardE = MeshedChannelsFive<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    SendTimed<(), Choose0fromEtoD, 'a', 0, true, 1, true, false>,
    RoleD<RoleBroadcast>,
    NameE,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsFive<End, End, End, RecursAtoE, RoleE<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFive<End, End, End, RecursBtoE, RoleE<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFive<End, End, End, RecursCtoE, RoleE<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsFive<End, End, End, RecursDtoE, RoleE<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsFive<
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

fn main() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e) =
        fork_mpst(endpoint_a, endpoint_b, endpoint_c, endpoint_d, endpoint_e);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
}
