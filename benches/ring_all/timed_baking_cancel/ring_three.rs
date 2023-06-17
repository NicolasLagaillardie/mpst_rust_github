#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// Create new roles
baker_timed!(MeshedChannelsThree, A, B, C);

// Types
// A
enum Branching0fromCtoA {
    Forward(
        MeshedChannelsThree<
            SendTimed<(), 'a', 0, true, 1, true, ' ', End>,
            RecursAtoC,
            RoleB<RoleC<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsThree<
            RecvTimed<(), 'a', 0, true, 1, true, ' ', End>,
            RecursAtoC,
            RoleB<RoleC<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = RecvTimed<Branching0fromCtoA, 'a', 0, true, 1, true, ' ', End>;

// B
enum Branching0fromCtoB {
    Forward(
        MeshedChannelsThree<
            RecvTimed<(), 'a', 0, true, 1, true, ' ', End>,
            SendTimed<(), 'a', 0, true, 1, true, ' ', RecursBtoC>,
            RoleA<RoleC<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsThree<
            SendTimed<(), 'a', 0, true, 1, true, ' ', End>,
            RecvTimed<(), 'a', 0, true, 1, true, ' ', RecursBtoC>,
            RoleC<RoleA<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = RecvTimed<Branching0fromCtoB, 'a', 0, true, 1, true, ' ', End>;

// C
type Choose0fromCtoA = SendTimed<Branching0fromCtoA, 'a', 0, true, 1, true, ' ', End>;
type Choose0fromCtoB = SendTimed<Branching0fromCtoB, 'a', 0, true, 1, true, ' ', End>;
type EndpointForwardC = MeshedChannelsThree<
    Choose0fromCtoA,
    RecvTimed<(), 'a', 0, true, 1, true, ' ', Choose0fromCtoB>,
    RoleB<RoleBroadcast>,
    NameC,
>;
type EndpointBackwardC = MeshedChannelsThree<
    Choose0fromCtoA,
    SendTimed<(), 'a', 0, true, 1, true, ' ', Choose0fromCtoB>,
    RoleB<RoleBroadcast>,
    NameC,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsThree<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsThree<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromCtoA::Done(s) => {
            s.close()
        },
        Branching0fromCtoA::Forward(s) => {
            let s = s.send((), all_clocks)?;
            endpoint_a(s, all_clocks)
        },
        Branching0fromCtoA::Backward(s) => {
            let (_, s) = s.recv(all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromCtoB::Done(s) => {
            s.close()
        },
        Branching0fromCtoB::Forward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
        Branching0fromCtoB::Backward(s) => {
            let ((), s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_c(s, LOOPS, all_clocks)
}

fn recurs_c(
    s: EndpointC,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_c_to_all!(
                s,
                all_clocks,
                Branching0fromCtoA::Done,
                Branching0fromCtoB::Done
            );

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointForwardC = choose_mpst_c_to_all!(
                s,
                all_clocks,
                Branching0fromCtoA::Forward,
                Branching0fromCtoB::Forward
            );

            let (_, s) = s.recv(all_clocks)?;

            recurs_c(s, i - 1, all_clocks)
        }
        i => {
            let s: EndpointBackwardC = choose_mpst_c_to_all!(
                s,
                all_clocks,
                Branching0fromCtoA::Backward,
                Branching0fromCtoB::Backward
            );

            let s = s.send((), all_clocks)?;

            recurs_c(s, i - 1, all_clocks)
        }
    }
}

fn all_mpst() {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring three baking protocol ATMP {LOOPS}"), |b| {
        b.iter(all_mpst)
    });
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets = ring_protocol_mpst,
}

criterion_main! {
    bench
}
