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
baker_timed!(MeshedChannelsTen, A, B, C, D, E, F, G, H, I, J);

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
type R2J<R> = RoleJ<RoleJ<R>>;
// A
enum Branching0fromJtoA {
    More(
        MeshedChannelsTen<
            RS,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursAtoJ>,
            >,
            R2J<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoJ = RecvTimed<Branching0fromJtoA, 'a', 0, true, 1, true, false, End>;
// B
enum Branching0fromJtoB {
    More(
        MeshedChannelsTen<
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursBtoJ>,
            >,
            R2J<R2A<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoJ = RecvTimed<Branching0fromJtoB, 'a', 0, true, 1, true, false, End>;
// C
enum Branching0fromJtoC {
    More(
        MeshedChannelsTen<
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursCtoJ>,
            >,
            R2J<R2A<R2B<R2D<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoJ = RecvTimed<Branching0fromJtoC, 'a', 0, true, 1, true, false, End>;
// D
enum Branching0fromJtoD {
    More(
        MeshedChannelsTen<
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursDtoJ>,
            >,
            R2J<R2A<R2B<R2C<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoJ = RecvTimed<Branching0fromJtoD, 'a', 0, true, 1, true, false, End>;
// E
enum Branching0fromJtoE {
    More(
        MeshedChannelsTen<
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursEtoJ>,
            >,
            R2J<R2A<R2B<R2C<R2D<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoJ = RecvTimed<Branching0fromJtoE, 'a', 0, true, 1, true, false, End>;
// F
enum Branching0fromJtoF {
    More(
        MeshedChannelsTen<
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursFtoJ>,
            >,
            R2J<R2A<R2B<R2C<R2D<R2E<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoJ = RecvTimed<Branching0fromJtoF, 'a', 0, true, 1, true, false, End>;
// G
enum Branching0fromJtoG {
    More(
        MeshedChannelsTen<
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursGtoJ>,
            >,
            R2J<R2A<R2B<R2C<R2D<R2E<R2F<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoJ = RecvTimed<Branching0fromJtoG, 'a', 0, true, 1, true, false, End>;
// H
enum Branching0fromJtoH {
    More(
        MeshedChannelsTen<
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursHtoJ>,
            >,
            R2J<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameH,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoJ = RecvTimed<Branching0fromJtoH, 'a', 0, true, 1, true, false, End>;
// I
enum Branching0fromJtoI {
    More(
        MeshedChannelsTen<
            SR,
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
                SendTimed<(), 'a', 0, true, 1, true, false, RecursItoJ>,
            >,
            R2J<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<RoleJ<RoleEnd>>>>>>>>>>,
            NameI,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoJ = RecvTimed<Branching0fromJtoI, 'a', 0, true, 1, true, false, End>;
// J
type Choose0fromJtoA = SendTimed<Branching0fromJtoA, 'a', 0, true, 1, true, false, End>;
type Choose0fromJtoB = SendTimed<Branching0fromJtoB, 'a', 0, true, 1, true, false, End>;
type Choose0fromJtoC = SendTimed<Branching0fromJtoC, 'a', 0, true, 1, true, false, End>;
type Choose0fromJtoD = SendTimed<Branching0fromJtoD, 'a', 0, true, 1, true, false, End>;
type Choose0fromJtoE = SendTimed<Branching0fromJtoE, 'a', 0, true, 1, true, false, End>;
type Choose0fromJtoF = SendTimed<Branching0fromJtoF, 'a', 0, true, 1, true, false, End>;
type Choose0fromJtoG = SendTimed<Branching0fromJtoG, 'a', 0, true, 1, true, false, End>;
type Choose0fromJtoH = SendTimed<Branching0fromJtoH, 'a', 0, true, 1, true, false, End>;
type Choose0fromJtoI = SendTimed<Branching0fromJtoI, 'a', 0, true, 1, true, false, End>;
type EndpointMoreJ = MeshedChannelsTen<
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromJtoA>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromJtoB>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromJtoC>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromJtoD>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromJtoE>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromJtoF>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromJtoG>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromJtoH>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<(), 'a', 0, true, 1, true, false, Choose0fromJtoI>,
    >,
    R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleBroadcast>>>>>>>>>,
    NameJ,
>;

// Creating the MP sessions
type EndpointA =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursAtoJ, RoleJ<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursBtoJ, RoleJ<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursCtoJ, RoleJ<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursDtoJ, RoleJ<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursEtoJ, RoleJ<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursFtoJ, RoleJ<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursGtoJ, RoleJ<RoleEnd>, NameG>;
type EndpointH =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursHtoJ, RoleJ<RoleEnd>, NameH>;
type EndpointI =
    MeshedChannelsTen<End, End, End, End, End, End, End, End, RecursItoJ, RoleJ<RoleEnd>, NameI>;
type EndpointJ = MeshedChannelsTen<
    Choose0fromJtoA,
    Choose0fromJtoB,
    Choose0fromJtoC,
    Choose0fromJtoD,
    Choose0fromJtoE,
    Choose0fromJtoF,
    Choose0fromJtoG,
    Choose0fromJtoH,
    Choose0fromJtoI,
    RoleBroadcast,
    NameJ,
>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoA::Done(s) => {
            s.close()
        },
        Branching0fromJtoA::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn endpoint_b(s: EndpointB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoB::Done(s) => {
            s.close()
        },
        Branching0fromJtoB::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoC::Done(s) => {
            s.close()
        },
        Branching0fromJtoC::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoD::Done(s) => {
            s.close()
        },
        Branching0fromJtoD::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoE::Done(s) => {
            s.close()
        },
        Branching0fromJtoE::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_e(s, all_clocks)
        },
    })
}

fn endpoint_f(s: EndpointF, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoF::Done(s) => {
            s.close()
        },
        Branching0fromJtoF::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_f(s, all_clocks)
        },
    })
}

fn endpoint_g(s: EndpointG, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoG::Done(s) => {
            s.close()
        },
        Branching0fromJtoG::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_g(s, all_clocks)
        },
    })
}

fn endpoint_h(s: EndpointH, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoH::Done(s) => {
            s.close()
        },
        Branching0fromJtoH::More(s) => {
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
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            endpoint_h(s, all_clocks)
        },
    })
}

fn endpoint_i(s: EndpointI, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromJtoI::Done(s) => {
            s.close()
        },
        Branching0fromJtoI::More(s) => {
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
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            endpoint_i(s, all_clocks)
        },
    })
}

fn endpoint_j(s: EndpointJ, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_j(s, LOOPS, all_clocks)
}

fn recurs_j(
    s: EndpointJ,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_j_to_all!(
                s,
                all_clocks,
                Branching0fromJtoA::Done,
                Branching0fromJtoB::Done,
                Branching0fromJtoC::Done,
                Branching0fromJtoD::Done,
                Branching0fromJtoE::Done,
                Branching0fromJtoF::Done,
                Branching0fromJtoG::Done,
                Branching0fromJtoH::Done,
                Branching0fromJtoI::Done
            );

            s.close()
        }
        i => {
            let s: EndpointMoreJ = choose_mpst_j_to_all!(
                s,
                all_clocks,
                Branching0fromJtoA::More,
                Branching0fromJtoB::More,
                Branching0fromJtoC::More,
                Branching0fromJtoD::More,
                Branching0fromJtoE::More,
                Branching0fromJtoF::More,
                Branching0fromJtoG::More,
                Branching0fromJtoH::More,
                Branching0fromJtoI::More
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
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            recurs_j(s, i - 1, all_clocks)
        }
    }
}

fn main() {
    let (
        thread_a,
        thread_b,
        thread_c,
        thread_d,
        thread_e,
        thread_f,
        thread_g,
        thread_h,
        thread_i,
        thread_j,
    ) = fork_mpst(
        endpoint_a, endpoint_b, endpoint_c, endpoint_d, endpoint_e, endpoint_f, endpoint_g,
        endpoint_h, endpoint_i, endpoint_j,
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
    thread_j.join().unwrap();
}
