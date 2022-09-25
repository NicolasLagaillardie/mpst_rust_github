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
// SendTimed/RecvTimed
type RS =
    RecvTimed<(), SendTimed<(), End, 'a', 0, true, 1, true, false>, 'a', 0, true, 1, true, false>;
type SR =
    SendTimed<(), RecvTimed<(), End, 'a', 0, true, 1, true, false>, 'a', 0, true, 1, true, false>;
// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
// A
enum Branching0fromCtoA {
    More(
        MeshedChannelsThree<
            RS,
            RecvTimed<
                (),
                SendTimed<(), RecursAtoC, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2C<R2B<RoleC<RoleEnd>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = RecvTimed<Branching0fromCtoA, End, 'a', 0, true, 1, true, false>;
// B
enum Branching0fromCtoB {
    More(
        MeshedChannelsThree<
            SR,
            RecvTimed<
                (),
                SendTimed<(), RecursBtoC, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2C<R2A<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = RecvTimed<Branching0fromCtoB, End, 'a', 0, true, 1, true, false>;
// C
type Choose0fromCtoA = SendTimed<Branching0fromCtoA, End, 'a', 0, true, 1, true, false>;
type Choose0fromCtoB = SendTimed<Branching0fromCtoB, End, 'a', 0, true, 1, true, false>;
type EndpointMoreC = MeshedChannelsThree<
    SendTimed<
        (),
        RecvTimed<(), Choose0fromCtoA, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    SendTimed<
        (),
        RecvTimed<(), Choose0fromCtoB, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    R2A<R2B<RoleBroadcast>>,
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
        Branching0fromCtoA::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
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
        Branching0fromCtoB::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
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
        i => {
            let s: EndpointMoreC = choose_mpst_c_to_all!(
                s,
                all_clocks,
                Branching0fromCtoA::More,
                Branching0fromCtoB::More
            );

            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

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

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("timed mesh three baking protocol MPST {}", LOOPS),
        |b| b.iter(all_mpst),
    );
}
