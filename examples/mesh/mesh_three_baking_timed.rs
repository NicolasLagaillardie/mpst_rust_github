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
baker_timed!(MeshedChannelsThree, A, B, C);

// Types
// SendTimed/RecvTimed
type RS =
    RecvTimed<(), 'a', 0, true, 1, true, false, SendTimed<(), 'a', 0, true, 1, true, false, End>>;
type SR =
    SendTimed<(), 'a', 0, true, 1, true, false, RecvTimed<(), 'a', 0, true, 1, true, false, End>>;

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
                'a',
                0,
                true,
                1,
                true,
                false,
                SendTimed<(), 'a', 0, true, 1, true, false, RecursAtoC>,
            >,
            R2C<R2B<RoleC<RoleEnd>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = RecvTimed<Branching0fromCtoA, 'a', 0, true, 1, true, false, End>;

// B
enum Branching0fromCtoB {
    More(
        MeshedChannelsThree<
            SR,
            RecvTimed<
                (),
                'a',
                0,
                true,
                1,
                true,
                false,
                SendTimed<(), 'a', 0, true, 1, true, false, RecursBtoC>,
            >,
            R2C<R2A<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = RecvTimed<Branching0fromCtoB, 'a', 0, true, 1, true, false, End>;

// C
type Choose0fromCtoA = SendTimed<Branching0fromCtoA, 'a', 0, true, 1, true, false, End>;
type Choose0fromCtoB = SendTimed<Branching0fromCtoB, 'a', 0, true, 1, true, false, End>;
type EndpointMoreC = MeshedChannelsThree<
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromCtoA>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromCtoB>,
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

fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}
