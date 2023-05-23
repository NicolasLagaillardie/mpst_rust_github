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
baker_timed!(MeshedChannelsNine, A, B, C, D, E, F, G, H, I);

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
type R2E<R> = RoleE<RoleE<R>>;
type R2F<R> = RoleF<RoleF<R>>;
type R2G<R> = RoleG<RoleG<R>>;
type R2H<R> = RoleH<RoleH<R>>;
type R2I<R> = RoleI<RoleI<R>>;

// A
enum Branching0fromItoA {
    More(
        MeshedChannelsNine<
            RS,
            RS,
            RS,
            RS,
            RS,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursAtoI>,
            >,
            R2I<R2B<R2C<R2D<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoI = RecvTimed<Branching0fromItoA, 'a', 0, true, 1, true, false, End>;

// B
enum Branching0fromItoB {
    More(
        MeshedChannelsNine<
            SR,
            RS,
            RS,
            RS,
            RS,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursBtoI>,
            >,
            R2I<R2A<R2C<R2D<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoI = RecvTimed<Branching0fromItoB, 'a', 0, true, 1, true, false, End>;

// C
enum Branching0fromItoC {
    More(
        MeshedChannelsNine<
            SR,
            SR,
            RS,
            RS,
            RS,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursCtoI>,
            >,
            R2I<R2A<R2B<R2D<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoI = RecvTimed<Branching0fromItoC, 'a', 0, true, 1, true, false, End>;

// D
enum Branching0fromItoD {
    More(
        MeshedChannelsNine<
            SR,
            SR,
            SR,
            RS,
            RS,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursDtoI>,
            >,
            R2I<R2A<R2B<R2C<R2E<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoI = RecvTimed<Branching0fromItoD, 'a', 0, true, 1, true, false, End>;

// E
enum Branching0fromItoE {
    More(
        MeshedChannelsNine<
            SR,
            SR,
            SR,
            SR,
            RS,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursEtoI>,
            >,
            R2I<R2A<R2B<R2C<R2D<R2F<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoI = RecvTimed<Branching0fromItoE, 'a', 0, true, 1, true, false, End>;

// F
enum Branching0fromItoF {
    More(
        MeshedChannelsNine<
            SR,
            SR,
            SR,
            SR,
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursFtoI>,
            >,
            R2I<R2A<R2B<R2C<R2D<R2E<R2G<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoI = RecvTimed<Branching0fromItoF, 'a', 0, true, 1, true, false, End>;

// G
enum Branching0fromItoG {
    More(
        MeshedChannelsNine<
            SR,
            SR,
            SR,
            SR,
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursGtoI>,
            >,
            R2I<R2A<R2B<R2C<R2D<R2E<R2F<R2H<RoleI<RoleEnd>>>>>>>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoI = RecvTimed<Branching0fromItoG, 'a', 0, true, 1, true, false, End>;

// H
enum Branching0fromItoH {
    More(
        MeshedChannelsNine<
            SR,
            SR,
            SR,
            SR,
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursHtoI>,
            >,
            R2I<R2A<R2B<R2C<R2D<R2E<R2F<R2G<RoleI<RoleEnd>>>>>>>>>,
            NameH,
        >,
    ),
    Done(MeshedChannelsNine<End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoI = RecvTimed<Branching0fromItoH, 'a', 0, true, 1, true, false, End>;

// I
type Choose0fromItoA = SendTimed<Branching0fromItoA, 'a', 0, true, 1, true, false, End>;
type Choose0fromItoB = SendTimed<Branching0fromItoB, 'a', 0, true, 1, true, false, End>;
type Choose0fromItoC = SendTimed<Branching0fromItoC, 'a', 0, true, 1, true, false, End>;
type Choose0fromItoD = SendTimed<Branching0fromItoD, 'a', 0, true, 1, true, false, End>;
type Choose0fromItoE = SendTimed<Branching0fromItoE, 'a', 0, true, 1, true, false, End>;
type Choose0fromItoF = SendTimed<Branching0fromItoF, 'a', 0, true, 1, true, false, End>;
type Choose0fromItoG = SendTimed<Branching0fromItoG, 'a', 0, true, 1, true, false, End>;
type Choose0fromItoH = SendTimed<Branching0fromItoH, 'a', 0, true, 1, true, false, End>;
type EndpointMoreI = MeshedChannelsNine<
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromItoA>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromItoB>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromItoC>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromItoD>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromItoE>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromItoF>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromItoG>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromItoH>,
    >,
    R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<RoleBroadcast>>>>>>>>,
    NameI,
>;

// Creating the MP sessions
type EndpointA =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursAtoI, RoleI<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursBtoI, RoleI<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursCtoI, RoleI<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursDtoI, RoleI<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursEtoI, RoleI<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursFtoI, RoleI<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursGtoI, RoleI<RoleEnd>, NameG>;
type EndpointH =
    MeshedChannelsNine<End, End, End, End, End, End, End, RecursHtoI, RoleI<RoleEnd>, NameH>;
type EndpointI = MeshedChannelsNine<
    Choose0fromItoA,
    Choose0fromItoB,
    Choose0fromItoC,
    Choose0fromItoD,
    Choose0fromItoE,
    Choose0fromItoF,
    Choose0fromItoG,
    Choose0fromItoH,
    RoleBroadcast,
    NameI,
>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoA::Done(s) => {
            s.close()
        },
        Branching0fromItoA::More(s) => {
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
        Branching0fromItoB::Done(s) => {
            s.close()
        },
        Branching0fromItoB::More(s) => {
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
        Branching0fromItoC::Done(s) => {
            s.close()
        },
        Branching0fromItoC::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
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
        Branching0fromItoD::Done(s) => {
            s.close()
        },
        Branching0fromItoD::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoE::Done(s) => {
            s.close()
        },
        Branching0fromItoE::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
    })
}

fn endpoint_f(s: EndpointF, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoF::Done(s) => {
            s.close()
        },
        Branching0fromItoF::More(s) => {
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
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_f(s, all_clocks)
        },
    })
}

fn endpoint_g(s: EndpointG, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoG::Done(s) => {
            s.close()
        },
        Branching0fromItoG::More(s) => {
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
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_g(s, all_clocks)
        },
    })
}

fn endpoint_h(s: EndpointH, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromItoH::Done(s) => {
            s.close()
        },
        Branching0fromItoH::More(s) => {
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
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            endpoint_h(s, all_clocks)
        },
    })
}

fn endpoint_i(s: EndpointI, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_i(s, LOOPS, all_clocks)
}

fn recurs_i(
    s: EndpointI,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_i_to_all!(
                s,
                all_clocks,
                Branching0fromItoA::Done,
                Branching0fromItoB::Done,
                Branching0fromItoC::Done,
                Branching0fromItoD::Done,
                Branching0fromItoE::Done,
                Branching0fromItoF::Done,
                Branching0fromItoG::Done,
                Branching0fromItoH::Done
            );

            s.close()
        }
        i => {
            let s: EndpointMoreI = choose_mpst_i_to_all!(
                s,
                all_clocks,
                Branching0fromItoA::More,
                Branching0fromItoB::More,
                Branching0fromItoC::More,
                Branching0fromItoD::More,
                Branching0fromItoE::More,
                Branching0fromItoF::More,
                Branching0fromItoG::More,
                Branching0fromItoH::More
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
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            recurs_i(s, i - 1, all_clocks)
        }
    }
}

fn main() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g, thread_h, thread_i) =
        fork_mpst(
            endpoint_a, endpoint_b, endpoint_c, endpoint_d, endpoint_e, endpoint_f, endpoint_g,
            endpoint_h, endpoint_i,
        );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
    thread_h.join().unwrap();
    thread_i.join().unwrap();
}
