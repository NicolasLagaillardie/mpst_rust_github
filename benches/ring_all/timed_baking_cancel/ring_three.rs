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
baker_timed!(MeshedChannelsThree, A, B, C);

// Types
// A
enum Branching0fromCtoA {
    Forward(
        MeshedChannelsThree<
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecursAtoC,
            RoleB<RoleC<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsThree<
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            RecursAtoC,
            RoleB<RoleC<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = RecvTimed<Branching0fromCtoA, End, 'a', 0, true, 1, true, false>;
// B
enum Branching0fromCtoB {
    Forward(
        MeshedChannelsThree<
            RecvTimed<(), End, 'a', 0, true, 1, true, false>,
            SendTimed<(), RecursBtoC, 'a', 0, true, 1, true, false>,
            RoleA<RoleC<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsThree<
            SendTimed<(), End, 'a', 0, true, 1, true, false>,
            RecvTimed<(), RecursBtoC, 'a', 0, true, 1, true, false>,
            RoleC<RoleA<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = RecvTimed<Branching0fromCtoB, End, 'a', 0, true, 1, true, false>;
// C
type Choose0fromCtoA = SendTimed<Branching0fromCtoA, End, 'a', 0, true, 1, true, false>;
type Choose0fromCtoB = SendTimed<Branching0fromCtoB, End, 'a', 0, true, 1, true, false>;
type EndpointForwardC = MeshedChannelsThree<
    Choose0fromCtoA,
    RecvTimed<(), Choose0fromCtoB, 'a', 0, true, 1, true, false>,
    RoleB<RoleBroadcast>,
    NameC,
>;
type EndpointBackwardC = MeshedChannelsThree<
    Choose0fromCtoA,
    SendTimed<(), Choose0fromCtoB, 'a', 0, true, 1, true, false>,
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
    c.bench_function(
        &format!("timed ring three baking protocol MPST {}", LOOPS),
        |b| b.iter(all_mpst),
    );
}
