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
// SendTimed/RecvTimed
type RS =
    RecvTimed<(), 'a', 0, true, 1, true, false, SendTimed<(), 'a', 0, true, 1, true, false, End>>;
type SR =
    SendTimed<(), 'a', 0, true, 1, true, false, RecvTimed<(), 'a', 0, true, 1, true, false, End>>;
// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type R2D<R> = RoleD<RoleD<R>>;
// A
enum Branching0fromDtoA {
    More(
        MeshedChannelsFour<
            RS,
            RS,
            RecvTimed<
                (),
                'a',
                0,
                true,
                1,
                true,
                false,
                SendTimed<(), 'a', 0, true, 1, true, false, RecursAtoD>,
            >,
            R2D<R2B<R2C<RoleD<RoleEnd>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameA>),
}
type RecursAtoD = RecvTimed<Branching0fromDtoA, 'a', 0, true, 1, true, false, End>;
// B
enum Branching0fromDtoB {
    More(
        MeshedChannelsFour<
            SR,
            RS,
            RecvTimed<
                (),
                'a',
                0,
                true,
                1,
                true,
                false,
                SendTimed<(), 'a', 0, true, 1, true, false, RecursBtoD>,
            >,
            R2D<R2A<R2C<RoleD<RoleEnd>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoD = RecvTimed<Branching0fromDtoB, 'a', 0, true, 1, true, false, End>;
// C
enum Branching0fromDtoC {
    More(
        MeshedChannelsFour<
            SR,
            SR,
            RecvTimed<
                (),
                'a',
                0,
                true,
                1,
                true,
                false,
                SendTimed<(), 'a', 0, true, 1, true, false, RecursCtoD>,
            >,
            R2D<R2A<R2B<RoleD<RoleEnd>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameC>),
}
type RecursCtoD = RecvTimed<Branching0fromDtoC, 'a', 0, true, 1, true, false, End>;
// D
type Choose0fromDtoA = SendTimed<Branching0fromDtoA, 'a', 0, true, 1, true, false, End>;
type Choose0fromDtoB = SendTimed<Branching0fromDtoB, 'a', 0, true, 1, true, false, End>;
type Choose0fromDtoC = SendTimed<Branching0fromDtoC, 'a', 0, true, 1, true, false, End>;
type EndpointMoreD = MeshedChannelsFour<
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromDtoA>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromDtoB>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromDtoC>,
    >,
    R2A<R2B<R2C<RoleBroadcast>>>,
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
        Branching0fromDtoA::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
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
        Branching0fromDtoB::Done(s) => {
            s.close()
        },
        Branching0fromDtoB::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
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
        Branching0fromDtoC::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
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
        i => {
            let s: EndpointMoreD = choose_mpst_d_to_all!(
                s,
                all_clocks,
                Branching0fromDtoA::More,
                Branching0fromDtoB::More,
                Branching0fromDtoC::More
            );

            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

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
