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
    More(
        MeshedChannelsTwo<
            RecvTimed<
                (),
                SendTimed<(), RecursAtoB, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            RoleB<RoleB<RoleB<RoleEnd>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsTwo<End, RoleEnd, NameA>),
}
type RecursAtoB = RecvTimed<Branching0fromBtoA, End, 'a', 0, true, 1, true, false>;
// C
type Choose0fromBtoA = SendTimed<Branching0fromBtoA, End, 'a', 0, true, 1, true, false>;
type EndpointMoreB = MeshedChannelsTwo<
    SendTimed<
        (),
        RecvTimed<(), Choose0fromBtoA, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    RoleA<RoleA<RoleBroadcast>>,
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
        Branching0fromBtoA::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
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
        i => {
            let s: EndpointMoreB = choose_mpst_b_to_all!(s, all_clocks, Branching0fromBtoA::More);

            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

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

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("timed mesh two baking protocol MPST {}", LOOPS),
        |b| b.iter(all_mpst),
    );
}
