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
generate_timed!(MeshedChannelsEight, A, B, C, D, E, F, G, H);

// Types
// SendTimed/RecvTimed
type RS =
    RecvTimed<(), 'a', 0, true, 10, true, ' ', SendTimed<(), 'a', 0, true, 10, true, ' ', End>>;
type SR =
    SendTimed<(), 'a', 0, true, 10, true, ' ', RecvTimed<(), 'a', 0, true, 10, true, ' ', End>>;

// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type R2D<R> = RoleD<RoleD<R>>;
type R2E<R> = RoleE<RoleE<R>>;
type R2F<R> = RoleF<RoleF<R>>;
type R2G<R> = RoleG<RoleG<R>>;
type R2H<R> = RoleH<RoleH<R>>;

// A
enum Branching0fromHtoA {
    More(
        MeshedChannelsEight<
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
                10,
                true,
                ' ',
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursAtoH>,
            >,
            R2H<R2B<R2C<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoH = RecvTimed<Branching0fromHtoA, 'a', 0, true, 10, true, ' ', End>;

// B
enum Branching0fromHtoB {
    More(
        MeshedChannelsEight<
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
                10,
                true,
                ' ',
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursBtoH>,
            >,
            R2H<R2A<R2C<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoH = RecvTimed<Branching0fromHtoB, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromHtoC {
    More(
        MeshedChannelsEight<
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
                10,
                true,
                ' ',
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursCtoH>,
            >,
            R2H<R2A<R2B<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoH = RecvTimed<Branching0fromHtoC, 'a', 0, true, 10, true, ' ', End>;

// D
enum Branching0fromHtoD {
    More(
        MeshedChannelsEight<
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
                10,
                true,
                ' ',
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursDtoH>,
            >,
            R2H<R2A<R2B<R2C<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoH = RecvTimed<Branching0fromHtoD, 'a', 0, true, 10, true, ' ', End>;

// E
enum Branching0fromHtoE {
    More(
        MeshedChannelsEight<
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
                10,
                true,
                ' ',
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursEtoH>,
            >,
            R2H<R2A<R2B<R2C<R2D<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoH = RecvTimed<Branching0fromHtoE, 'a', 0, true, 10, true, ' ', End>;

// F
enum Branching0fromHtoF {
    More(
        MeshedChannelsEight<
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
                10,
                true,
                ' ',
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursFtoH>,
            >,
            R2H<R2A<R2B<R2C<R2D<R2E<R2G<RoleH<RoleEnd>>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoH = RecvTimed<Branching0fromHtoF, 'a', 0, true, 10, true, ' ', End>;

// G
enum Branching0fromHtoG {
    More(
        MeshedChannelsEight<
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
                10,
                true,
                ' ',
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursGtoH>,
            >,
            R2H<R2A<R2B<R2C<R2D<R2E<R2F<RoleH<RoleEnd>>>>>>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoH = RecvTimed<Branching0fromHtoG, 'a', 0, true, 10, true, ' ', End>;

// H
type Choose0fromHtoA = SendTimed<Branching0fromHtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoB = SendTimed<Branching0fromHtoB, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoC = SendTimed<Branching0fromHtoC, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoD = SendTimed<Branching0fromHtoD, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoE = SendTimed<Branching0fromHtoE, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoF = SendTimed<Branching0fromHtoF, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromHtoG = SendTimed<Branching0fromHtoG, 'a', 0, true, 10, true, ' ', End>;
type EndpointMoreH = MeshedChannelsEight<
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromHtoA>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromHtoB>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromHtoC>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromHtoD>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromHtoE>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromHtoF>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromHtoG>,
    >,
    R2A<R2B<R2C<R2D<R2E<R2F<R2G<RoleBroadcast>>>>>>>,
    NameH,
>;

// Creating the MP sessions
type EndpointA =
    MeshedChannelsEight<End, End, End, End, End, End, RecursAtoH, RoleH<RoleEnd>, NameA>;
type EndpointB =
    MeshedChannelsEight<End, End, End, End, End, End, RecursBtoH, RoleH<RoleEnd>, NameB>;
type EndpointC =
    MeshedChannelsEight<End, End, End, End, End, End, RecursCtoH, RoleH<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsEight<End, End, End, End, End, End, RecursDtoH, RoleH<RoleEnd>, NameD>;
type EndpointE =
    MeshedChannelsEight<End, End, End, End, End, End, RecursEtoH, RoleH<RoleEnd>, NameE>;
type EndpointF =
    MeshedChannelsEight<End, End, End, End, End, End, RecursFtoH, RoleH<RoleEnd>, NameF>;
type EndpointG =
    MeshedChannelsEight<End, End, End, End, End, End, RecursGtoH, RoleH<RoleEnd>, NameG>;
type EndpointH = MeshedChannelsEight<
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    Choose0fromHtoG,
    RoleBroadcast,
    NameH,
>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoA::Done(s) => {
            s.close()
        },
        Branching0fromHtoA::More(s) => {
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
        Branching0fromHtoB::Done(s) => {
            s.close()
        },
        Branching0fromHtoB::More(s) => {
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
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoC::Done(s) => {
            s.close()
        },
        Branching0fromHtoC::More(s) => {
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
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoD::Done(s) => {
            s.close()
        },
        Branching0fromHtoD::More(s) => {
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
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoE::Done(s) => {
            s.close()
        },
        Branching0fromHtoE::More(s) => {
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
            endpoint_e(s, all_clocks)
        },
    })
}

fn endpoint_f(s: EndpointF, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoF::Done(s) => {
            s.close()
        },
        Branching0fromHtoF::More(s) => {
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
            endpoint_f(s, all_clocks)
        },
    })
}

fn endpoint_g(s: EndpointG, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromHtoG::Done(s) => {
            s.close()
        },
        Branching0fromHtoG::More(s) => {
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
            endpoint_g(s, all_clocks)
        },
    })
}

fn endpoint_h(s: EndpointH, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_h(s, LOOPS, all_clocks)
}

fn recurs_h(
    s: EndpointH,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_h_to_all!(
                s,
                all_clocks,
                Branching0fromHtoA::Done,
                Branching0fromHtoB::Done,
                Branching0fromHtoC::Done,
                Branching0fromHtoD::Done,
                Branching0fromHtoE::Done,
                Branching0fromHtoF::Done,
                Branching0fromHtoG::Done
            );

            s.close()
        }
        i => {
            let s: EndpointMoreH = choose_mpst_h_to_all!(
                s,
                all_clocks,
                Branching0fromHtoA::More,
                Branching0fromHtoB::More,
                Branching0fromHtoC::More,
                Branching0fromHtoD::More,
                Branching0fromHtoE::More,
                Branching0fromHtoF::More,
                Branching0fromHtoG::More
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

            recurs_h(s, i - 1, all_clocks)
        }
    }
}

fn aux() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g, thread_h) =
        fork_mpst(
            black_box(endpoint_a),
            black_box(endpoint_b),
            black_box(endpoint_c),
            black_box(endpoint_d),
            black_box(endpoint_e),
            black_box(endpoint_f),
            black_box(endpoint_g),
            black_box(endpoint_h),
        );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
    thread_h.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("mesh eight baking protocol ATMP {LOOPS}"), |b| {
        b.iter(aux)
    });
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
