#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_timed;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

static LOOPS: i64 = 100;

// Create new roles
generate_timed!(MeshedChannels, A, B);

// Types
// A
enum Branching0fromBtoA {
    Forward(
        MeshedChannels<
            SendTimed<(), 'a', 0, true, 10, true, ' ', RecursAtoB>,
            RoleB<RoleB<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannels<
            RecvTimed<(), 'a', 0, true, 10, true, ' ', RecursAtoB>,
            RoleB<RoleB<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, RoleEnd, NameA>),
}
type RecursAtoB = RecvTimed<Branching0fromBtoA, 'a', 0, true, 10, true, ' ', End>;

// B
type Choose0fromBtoA = SendTimed<Branching0fromBtoA, 'a', 0, true, 10, true, ' ', End>;
type EndpointForwardB = MeshedChannels<
    RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromBtoA>,
    RoleA<RoleBroadcast>,
    NameB,
>;
type EndpointBackwardB = MeshedChannels<
    SendTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromBtoA>,
    RoleA<RoleBroadcast>,
    NameB,
>;

// Creating the MP sessions
type EndpointA = MeshedChannels<RecursAtoB, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<Choose0fromBtoA, RoleBroadcast, NameB>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromBtoA::Done(s) => {
            s.close()
        },
        Branching0fromBtoA::Forward(s) => {
            let s = s.send((), all_clocks)?;
            endpoint_a(s, all_clocks)
        },
        Branching0fromBtoA::Backward(s) => {
            let (_, s) = s.recv(all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_b(s, LOOPS, all_clocks)
}

fn recurs_b(
    s: EndpointB,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_b_to_all!(s, all_clocks, Branching0fromBtoA::Done);

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardB =
                choose_mpst_b_to_all!(s, all_clocks, Branching0fromBtoA::Forward);

            let (_, s) = s.recv(all_clocks)?;

            recurs_b(s, i - 1, all_clocks)
        }
        i => {
            let s: EndpointBackwardB =
                choose_mpst_b_to_all!(s, all_clocks, Branching0fromBtoA::Backward);

            let s = s.send((), all_clocks)?;

            recurs_b(s, i - 1, all_clocks)
        }
    }
}

fn main() {
    let (thread_a, thread_b) = fork_mpst(endpoint_a, endpoint_b);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}
