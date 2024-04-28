#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

static LOOPS: i64 = 100;

// Create new roles
generate_atmp!(MeshedChannels, A, B, C, D);

// Types
// A
enum Branching0fromDtoA {
    Forward(
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursAtoD,
            RoleB<RoleD<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            End,
            RecursAtoD,
            RoleB<RoleD<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, End, End, RoleEnd, NameA>),
}
type RecursAtoD = RecvTimed<Branching0fromDtoA, 'a', 0, true, 10, true, ' ', End>;

// B
enum Branching0fromDtoB {
    Forward(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursBtoD,
            RoleA<RoleC<RoleD<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecursBtoD,
            RoleC<RoleA<RoleD<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, RoleEnd, NameB>),
}
type RecursBtoD = RecvTimed<Branching0fromDtoB, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromDtoC {
    Forward(
        MeshedChannels<
            End,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 10, true, ' ', RecursCtoD>,
            RoleB<RoleD<RoleD<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannels<
            End,
            SendTimed<(), 'a', 0, true, 10, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 10, true, ' ', RecursCtoD>,
            RoleD<RoleB<RoleD<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, RoleEnd, NameC>),
}
type RecursCtoD = RecvTimed<Branching0fromDtoC, 'a', 0, true, 10, true, ' ', End>;

// D
type Choose0fromDtoA = SendTimed<Branching0fromDtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromDtoB = SendTimed<Branching0fromDtoB, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromDtoC = SendTimed<Branching0fromDtoC, 'a', 0, true, 10, true, ' ', End>;
type EndpointForwardD = MeshedChannels<
    Choose0fromDtoA,
    Choose0fromDtoB,
    RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromDtoC>,
    RoleC<RoleBroadcast>,
    NameD,
>;
type EndpointBackwardD = MeshedChannels<
    Choose0fromDtoA,
    Choose0fromDtoB,
    SendTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromDtoC>,
    RoleC<RoleBroadcast>,
    NameD,
>;

// Creating the MP sessions
type EndpointA = MeshedChannels<End, End, RecursAtoD, RoleD<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, End, RecursBtoD, RoleD<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<End, End, RecursCtoD, RoleD<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannels<Choose0fromDtoA, Choose0fromDtoB, Choose0fromDtoC, RoleBroadcast, NameD>;

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
