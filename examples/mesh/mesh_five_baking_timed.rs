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
baker_timed!(MeshedChannelsFive, A, B, C, D, E);

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
type R2D<R> = RoleD<RoleD<R>>;
type R2E<R> = RoleE<RoleE<R>>;
// A
enum Branching0fromEtoA {
    More(
        MeshedChannelsFive<
            RS,
            RS,
            RS,
            RecvTimed<
                (),
                SendTimed<(), RecursAtoE, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2E<R2B<R2C<R2D<RoleE<RoleEnd>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoE = RecvTimed<Branching0fromEtoA, End, 'a', 0, true, 1, true, false>;
// B
enum Branching0fromEtoB {
    More(
        MeshedChannelsFive<
            SR,
            RS,
            RS,
            RecvTimed<
                (),
                SendTimed<(), RecursBtoE, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2E<R2A<R2C<R2D<RoleE<RoleEnd>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoE = RecvTimed<Branching0fromEtoB, End, 'a', 0, true, 1, true, false>;
// C
enum Branching0fromEtoC {
    More(
        MeshedChannelsFive<
            SR,
            SR,
            RS,
            RecvTimed<
                (),
                SendTimed<(), RecursCtoE, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2E<R2A<R2B<R2D<RoleE<RoleEnd>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoE = RecvTimed<Branching0fromEtoC, End, 'a', 0, true, 1, true, false>;
// D
enum Branching0fromEtoD {
    More(
        MeshedChannelsFive<
            SR,
            SR,
            SR,
            RecvTimed<
                (),
                SendTimed<(), RecursDtoE, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2E<R2A<R2B<R2C<RoleE<RoleEnd>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsFive<End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoE = RecvTimed<Branching0fromEtoD, End, 'a', 0, true, 1, true, false>;
// E
type Choose0fromEtoA = SendTimed<Branching0fromEtoA, End, 'a', 0, true, 1, true, false>;
type Choose0fromEtoB = SendTimed<Branching0fromEtoB, End, 'a', 0, true, 1, true, false>;
type Choose0fromEtoC = SendTimed<Branching0fromEtoC, End, 'a', 0, true, 1, true, false>;
type Choose0fromEtoD = SendTimed<Branching0fromEtoD, End, 'a', 0, true, 1, true, false>;
type EndpointMoreE = MeshedChannelsFive<
    SendTimed<
        (),
        RecvTimed<(), Choose0fromEtoA, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    SendTimed<
        (),
        RecvTimed<(), Choose0fromEtoB, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    SendTimed<
        (),
        RecvTimed<(), Choose0fromEtoC, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    SendTimed<
        (),
        RecvTimed<(), Choose0fromEtoD, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    R2A<R2B<R2C<R2D<RoleBroadcast>>>>,
    NameE,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsFive<End, End, End, RecursAtoE, RoleE<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFive<End, End, End, RecursBtoE, RoleE<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFive<End, End, End, RecursCtoE, RoleE<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsFive<End, End, End, RecursDtoE, RoleE<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsFive<
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Choose0fromEtoD,
    RoleBroadcast,
    NameE,
>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromEtoA::Done(s) => {
            s.close()
        },
        Branching0fromEtoA::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
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
        Branching0fromEtoB::Done(s) => {
            s.close()
        },
        Branching0fromEtoB::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromEtoC::Done(s) => {
            s.close()
        },
        Branching0fromEtoC::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromEtoD::Done(s) => {
            s.close()
        },
        Branching0fromEtoD::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_e(s, LOOPS, all_clocks)
}

fn recurs_e(
    s: EndpointE,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_e_to_all!(
                s,
                all_clocks,
                Branching0fromEtoA::Done,
                Branching0fromEtoB::Done,
                Branching0fromEtoC::Done,
                Branching0fromEtoD::Done,
            );

            s.close()
        }
        i => {
            let s: EndpointMoreE = choose_mpst_e_to_all!(
                s,
                all_clocks,
                Branching0fromEtoA::More,
                Branching0fromEtoB::More,
                Branching0fromEtoC::More,
                Branching0fromEtoD::More,
            );

            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            recurs_e(s, i - 1, all_clocks)
        }
    }
}

fn main() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e) =
        fork_mpst(endpoint_a, endpoint_b, endpoint_c, endpoint_d, endpoint_e);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
}
