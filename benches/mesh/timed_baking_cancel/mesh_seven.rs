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
generate_timed!(MeshedChannels, A, B, C, D, E, F, G);

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

// A
enum Branching0fromGtoA {
    More(
        MeshedChannels<
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
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursAtoG>,
            >,
            R2G<R2B<R2C<R2D<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoG = RecvTimed<Branching0fromGtoA, 'a', 0, true, 10, true, ' ', End>;

// B
enum Branching0fromGtoB {
    More(
        MeshedChannels<
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
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursBtoG>,
            >,
            R2G<R2A<R2C<R2D<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoG = RecvTimed<Branching0fromGtoB, 'a', 0, true, 10, true, ' ', End>;

// C
enum Branching0fromGtoC {
    More(
        MeshedChannels<
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
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursCtoG>,
            >,
            R2G<R2A<R2B<R2D<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoG = RecvTimed<Branching0fromGtoC, 'a', 0, true, 10, true, ' ', End>;

// D
enum Branching0fromGtoD {
    More(
        MeshedChannels<
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
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursDtoG>,
            >,
            R2G<R2A<R2B<R2C<R2E<R2F<RoleG<RoleEnd>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoG = RecvTimed<Branching0fromGtoD, 'a', 0, true, 10, true, ' ', End>;

// E
enum Branching0fromGtoE {
    More(
        MeshedChannels<
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
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursEtoG>,
            >,
            R2G<R2A<R2B<R2C<R2D<R2F<RoleG<RoleEnd>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoG = RecvTimed<Branching0fromGtoE, 'a', 0, true, 10, true, ' ', End>;

// F
enum Branching0fromGtoF {
    More(
        MeshedChannels<
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
                SendTimed<(), 'a', 0, true, 10, true, ' ', RecursFtoG>,
            >,
            R2G<R2A<R2B<R2C<R2D<R2E<RoleG<RoleEnd>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoG = RecvTimed<Branching0fromGtoF, 'a', 0, true, 10, true, ' ', End>;

// F
type Choose0fromGtoA = SendTimed<Branching0fromGtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromGtoB = SendTimed<Branching0fromGtoB, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromGtoC = SendTimed<Branching0fromGtoC, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromGtoD = SendTimed<Branching0fromGtoD, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromGtoE = SendTimed<Branching0fromGtoE, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromGtoF = SendTimed<Branching0fromGtoF, 'a', 0, true, 10, true, ' ', End>;
type EndpointMoreG = MeshedChannels<
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromGtoA>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromGtoB>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromGtoC>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromGtoD>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromGtoE>,
    >,
    SendTimed<
        (),
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<(), 'a', 0, true, 10, true, ' ', Choose0fromGtoF>,
    >,
    R2A<R2B<R2C<R2D<R2E<R2F<RoleBroadcast>>>>>>,
    NameG,
>;

// Creating the MP sessions
type EndpointA = MeshedChannels<End, End, End, End, End, RecursAtoG, RoleG<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, End, End, End, End, RecursBtoG, RoleG<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<End, End, End, End, End, RecursCtoG, RoleG<RoleEnd>, NameC>;
type EndpointD = MeshedChannels<End, End, End, End, End, RecursDtoG, RoleG<RoleEnd>, NameD>;
type EndpointE = MeshedChannels<End, End, End, End, End, RecursEtoG, RoleG<RoleEnd>, NameE>;
type EndpointF = MeshedChannels<End, End, End, End, End, RecursFtoG, RoleG<RoleEnd>, NameF>;
type EndpointG = MeshedChannels<
    Choose0fromGtoA,
    Choose0fromGtoB,
    Choose0fromGtoC,
    Choose0fromGtoD,
    Choose0fromGtoE,
    Choose0fromGtoF,
    RoleBroadcast,
    NameG,
>;

fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoA::Done(s) => {
            s.close()
        },
        Branching0fromGtoA::More(s) => {
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
        Branching0fromGtoB::Done(s) => {
            s.close()
        },
        Branching0fromGtoB::More(s) => {
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
            endpoint_b(s, all_clocks)
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoC::Done(s) => {
            s.close()
        },
        Branching0fromGtoC::More(s) => {
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
            endpoint_c(s, all_clocks)
        },
    })
}

fn endpoint_d(s: EndpointD, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoD::Done(s) => {
            s.close()
        },
        Branching0fromGtoD::More(s) => {
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
            endpoint_d(s, all_clocks)
        },
    })
}

fn endpoint_e(s: EndpointE, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoE::Done(s) => {
            s.close()
        },
        Branching0fromGtoE::More(s) => {
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
            endpoint_e(s, all_clocks)
        },
    })
}

fn endpoint_f(s: EndpointF, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Branching0fromGtoF::Done(s) => {
            s.close()
        },
        Branching0fromGtoF::More(s) => {
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
            endpoint_f(s, all_clocks)
        },
    })
}

fn endpoint_g(s: EndpointG, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    recurs_g(s, LOOPS, all_clocks)
}

fn recurs_g(
    s: EndpointG,
    index: i64,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_g_to_all!(
                s,
                all_clocks,
                Branching0fromGtoA::Done,
                Branching0fromGtoB::Done,
                Branching0fromGtoC::Done,
                Branching0fromGtoD::Done,
                Branching0fromGtoE::Done,
                Branching0fromGtoF::Done
            );

            s.close()
        }
        i => {
            let s: EndpointMoreG = choose_mpst_g_to_all!(
                s,
                all_clocks,
                Branching0fromGtoA::More,
                Branching0fromGtoB::More,
                Branching0fromGtoC::More,
                Branching0fromGtoD::More,
                Branching0fromGtoE::More,
                Branching0fromGtoF::More
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

            recurs_g(s, i - 1, all_clocks)
        }
    }
}

fn aux() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
        black_box(endpoint_g),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
    thread_g.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh(c: &mut Criterion) {
    c.bench_function(&format!("mesh seven baking ATMP {LOOPS}"), |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = mesh,
}

criterion_main! {
    bench
}
