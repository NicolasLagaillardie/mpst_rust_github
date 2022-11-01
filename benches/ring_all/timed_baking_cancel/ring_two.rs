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
baker_timed!(MeshedChannelsTwo, A, B);

// Types
// A
enum Branching0fromBtoA {
    Forward(
        MeshedChannelsTwo<
            SendTimed<(), 'a', 0, true, 1, true, false, RecursAtoB>,
            RoleB<RoleB<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsTwo<
            RecvTimed<(), 'a', 0, true, 1, true, false, RecursAtoB>,
            RoleB<RoleB<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsTwo<End, RoleEnd, NameA>),
}
type RecursAtoB = RecvTimed<Branching0fromBtoA, 'a', 0, true, 1, true, false, End>;
// B
type Choose0fromBtoA = SendTimed<Branching0fromBtoA, 'a', 0, true, 1, true, false, End>;
type EndpointForwardB = MeshedChannelsTwo<
    RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromBtoA>,
    RoleA<RoleBroadcast>,
    NameB,
>;
type EndpointBackwardB = MeshedChannelsTwo<
    SendTimed<(), 'a', 0, true, 1, true, false, Choose0fromBtoA>,
    RoleA<RoleBroadcast>,
    NameB,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsTwo<RecursAtoB, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsTwo<Choose0fromBtoA, RoleBroadcast, NameB>;

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

fn all_mpst() {
    let (thread_a, thread_b) = fork_mpst(black_box(endpoint_a), black_box(endpoint_b));

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("timed ring two baking protocol MPST {LOOPS}"),
        |b| b.iter(all_mpst),
    );
}
