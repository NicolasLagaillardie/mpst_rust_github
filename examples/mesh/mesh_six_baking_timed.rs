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
baker_timed!(MeshedChannelsSix, A, B, C, D, E, F);

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
type R2F<R> = RoleF<RoleF<R>>;
// A
enum Branching0fromFtoA {
    More(
        MeshedChannelsSix<
            RS,
            RS,
            RS,
            RS,
            RecvTimed<
                (),
                SendTimed<(), RecursAtoF, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2F<R2B<R2C<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoF = RecvTimed<Branching0fromFtoA, End, 'a', 0, true, 1, true, false>;
// B
enum Branching0fromFtoB {
    More(
        MeshedChannelsSix<
            SR,
            RS,
            RS,
            RS,
            RecvTimed<
                (),
                SendTimed<(), RecursBtoF, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2F<R2A<R2C<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoF = RecvTimed<Branching0fromFtoB, End, 'a', 0, true, 1, true, false>;
// C
enum Branching0fromFtoC {
    More(
        MeshedChannelsSix<
            SR,
            SR,
            RS,
            RS,
            RecvTimed<
                (),
                SendTimed<(), RecursCtoF, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2F<R2A<R2B<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoF = RecvTimed<Branching0fromFtoC, End, 'a', 0, true, 1, true, false>;
// D
enum Branching0fromFtoD {
    More(
        MeshedChannelsSix<
            SR,
            SR,
            SR,
            RS,
            RecvTimed<
                (),
                SendTimed<(), RecursDtoF, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2F<R2A<R2B<R2C<R2E<RoleF<RoleEnd>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoF = RecvTimed<Branching0fromFtoD, End, 'a', 0, true, 1, true, false>;
// E
enum Branching0fromFtoE {
    More(
        MeshedChannelsSix<
            SR,
            SR,
            SR,
            SR,
            RecvTimed<
                (),
                SendTimed<(), RecursEtoF, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            R2F<R2A<R2B<R2C<R2D<RoleF<RoleEnd>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsSix<End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoF = RecvTimed<Branching0fromFtoE, End, 'a', 0, true, 1, true, false>;
// F
type Choose0fromFtoA = SendTimed<Branching0fromFtoA, End, 'a', 0, true, 1, true, false>;
type Choose0fromFtoB = SendTimed<Branching0fromFtoB, End, 'a', 0, true, 1, true, false>;
type Choose0fromFtoC = SendTimed<Branching0fromFtoC, End, 'a', 0, true, 1, true, false>;
type Choose0fromFtoD = SendTimed<Branching0fromFtoD, End, 'a', 0, true, 1, true, false>;
type Choose0fromFtoE = SendTimed<Branching0fromFtoE, End, 'a', 0, true, 1, true, false>;
type EndpointMoreF = MeshedChannelsSix<
    SendTimed<
        (),
        RecvTimed<(), Choose0fromFtoA, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    SendTimed<
        (),
        RecvTimed<(), Choose0fromFtoB, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    SendTimed<
        (),
        RecvTimed<(), Choose0fromFtoC, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    SendTimed<
        (),
        RecvTimed<(), Choose0fromFtoD, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    SendTimed<
        (),
        RecvTimed<(), Choose0fromFtoE, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    R2A<R2B<R2C<R2D<R2E<RoleBroadcast>>>>>,
    NameF,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsSix<End, End, End, End, RecursAtoF, RoleF<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsSix<End, End, End, End, RecursBtoF, RoleF<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsSix<End, End, End, End, RecursCtoF, RoleF<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsSix<End, End, End, End, RecursDtoF, RoleF<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsSix<End, End, End, End, RecursEtoF, RoleF<RoleEnd>, NameE>;
type EndpointF = MeshedChannelsSix<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Choose0fromFtoE,
    RoleBroadcast,
    NameF,
>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromFtoA::Done(s) => {
            s.close()
        },
        Branching0fromFtoA::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
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
        Branching0fromFtoB::Done(s) => {
            s.close()
        },
        Branching0fromFtoB::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
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
        Branching0fromFtoC::Done(s) => {
            s.close()
        },
        Branching0fromFtoC::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromFtoD::Done(s) => {
            s.close()
        },
        Branching0fromFtoD::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromFtoE::Done(s) => {
            s.close()
        },
        Branching0fromFtoE::More(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            endpoint_e(s, all_clocks)
        },
    })
}

fn endpoint_f(s: EndpointF, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_f(s, LOOPS, all_clocks)
}

fn recurs_f(
    s: EndpointF,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_f_to_all!(
                s,
                all_clocks,
                Branching0fromFtoA::Done,
                Branching0fromFtoB::Done,
                Branching0fromFtoC::Done,
                Branching0fromFtoD::Done,
                Branching0fromFtoE::Done
            );

            s.close()
        }
        i => {
            let s: EndpointMoreF = choose_mpst_f_to_all!(
                s,
                all_clocks,
                Branching0fromFtoA::More,
                Branching0fromFtoB::More,
                Branching0fromFtoC::More,
                Branching0fromFtoD::More,
                Branching0fromFtoE::More
            );

            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            recurs_f(s, i - 1, all_clocks)
        }
    }
}

fn main() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f) = fork_mpst(
        endpoint_a, endpoint_b, endpoint_c, endpoint_d, endpoint_e, endpoint_f,
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
}
