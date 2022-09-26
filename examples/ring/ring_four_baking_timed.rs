#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

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
baker_timed!(MeshedChannelsFour, A, B, C, D);

// Types
// A
enum Branching0fromDtoA {
    Forward(
        MeshedChannelsFour<
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            End,
            RecursAtoD,
            RoleB<RoleD<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsFour<
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            End,
            RecursAtoD,
            RoleB<RoleD<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameA>),
}
type RecursAtoD = RecvTimed<Branching0fromDtoA, End, 'a', 0, true, 1, true, false>;
// B
enum Branching0fromDtoB {
    Forward(
        MeshedChannelsFour<
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecursBtoD,
            RoleA<RoleC<RoleD<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsFour<
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            RecursBtoD,
            RoleC<RoleA<RoleD<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoD = RecvTimed<Branching0fromDtoB, End, 'a', 0, true, 1, true, false>;
// C
enum Branching0fromDtoC {
    Forward(
        MeshedChannelsFour<
            End,
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), RecursCtoD, 'a', 0, true, 1, true, false>,
            RoleB<RoleD<RoleD<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsFour<
            End,
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), RecursCtoD, 'a', 0, true, 1, true, false>,
            RoleD<RoleB<RoleD<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameC>),
}
type RecursCtoD = RecvTimed<Branching0fromDtoC, End, 'a', 0, true, 1, true, false>;
// D
type Choose0fromDtoA = SendTimed<Branching0fromDtoA, End, 'a', 0, true, 1, true, false>;
type Choose0fromDtoB = SendTimed<Branching0fromDtoB, End, 'a', 0, true, 1, true, false>;
type Choose0fromDtoC = SendTimed<Branching0fromDtoC, End, 'a', 0, true, 1, true, false>;
type EndpointForwardD = MeshedChannelsFour<
    Choose0fromDtoA,
    Choose0fromDtoB,
    RecvTimed<(), Choose0fromDtoC, 'a', 0, true, 1, true, false>,
    RoleC<RoleBroadcast>,
    NameD,
>;
type EndpointBackwardD = MeshedChannelsFour<
    Choose0fromDtoA,
    Choose0fromDtoB,
    SendTimed<(), Choose0fromDtoC, 'a', 0, true, 1, true, false>,
    RoleC<RoleBroadcast>,
    NameD,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsFour<End, End, RecursAtoD, RoleD<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFour<End, End, RecursBtoD, RoleD<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFour<End, End, RecursCtoD, RoleD<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsFour<Choose0fromDtoA, Choose0fromDtoB, Choose0fromDtoC, RoleBroadcast, NameD>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromDtoA::Done(s) => {
            s.close()
        },
        Branching0fromDtoA::Forward(s) => {
            let s = s.send((), all_clocks)?;
            endpoint_a(s, all_clocks)
        },
        Branching0fromDtoA::Backward(s) => {
            let (_, s) = s.recv(all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromDtoB::Done(s) => {
            s.close()
        },
        Branching0fromDtoB::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
        Branching0fromDtoB::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromDtoC::Done(s) => {
            s.close()
        },
        Branching0fromDtoC::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
        Branching0fromDtoC::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_d(s, LOOPS, all_clocks)
}

fn recurs_d(
    s: EndpointD,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_d_to_all!(
                s,
                all_clocks,
                Branching0fromDtoA::Done,
                Branching0fromDtoB::Done,
                Branching0fromDtoC::Done
            );

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardD = choose_mpst_d_to_all!(
                s,
                all_clocks,
                Branching0fromDtoA::Forward,
                Branching0fromDtoB::Forward,
                Branching0fromDtoC::Forward
            );

            let (_, s) = s.recv(all_clocks)?;

            recurs_d(s, i - 1, all_clocks)
        }
        i => {
            let s: EndpointBackwardD = choose_mpst_d_to_all!(
                s,
                all_clocks,
                Branching0fromDtoA::Backward,
                Branching0fromDtoB::Backward,
                Branching0fromDtoC::Backward
            );

            let s = s.send((), all_clocks)?;

            recurs_d(s, i - 1, all_clocks)
        }
    }
}

fn main() {
    let (thread_a, thread_b, thread_c, thread_d) =
        fork_mpst(endpoint_a, endpoint_b, endpoint_c, endpoint_d);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
}
