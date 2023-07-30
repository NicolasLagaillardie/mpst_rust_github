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
generate_timed!(MeshedChannels, A, B);

// Types
// A
enum Branching0fromBtoA {
    More(
        MeshedChannels<
            RecvTimed<
                (),
                'a',
                0,
                true,
                10,
                true,
                ' ',
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursAtoB>,
            >,
            RoleB<RoleB<RoleB<RoleEnd>>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, RoleEnd, NameA>),
}
type RecursAtoB = RecvTimed<Branching0fromBtoA, 'a', 0, true, 10, true, ' ', End>;

// C
type Choose0fromBtoA = SendTimed<Branching0fromBtoA, 'a', 0, true, 10, true, ' ', End>;
type EndpointMoreB = MeshedChannels<
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromBtoA>,
    >,
    RoleA<RoleA<RoleBroadcast>>,
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

fn aux() {
    let (thread_a, thread_b) = fork_mpst(black_box(endpoint_a), black_box(endpoint_b));

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("mesh two baking ATMP {LOOPS}"), |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = mesh_protocol_mpst,
}

criterion_main! {
    bench
}
