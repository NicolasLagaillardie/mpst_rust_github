#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// use std::time::Duration;

// Create new roles
generate!(
    "rec_and_cancel",
    MeshedChannelsEight,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
);

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;

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
            Recv<(), Send<(), RecursAtoH>>,
            R2H<R2B<R2C<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoH = Recv<Branching0fromHtoA, End>;

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
            Recv<(), Send<(), RecursBtoH>>,
            R2H<R2A<R2C<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoH = Recv<Branching0fromHtoB, End>;

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
            Recv<(), Send<(), RecursCtoH>>,
            R2H<R2A<R2B<R2D<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoH = Recv<Branching0fromHtoC, End>;

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
            Recv<(), Send<(), RecursDtoH>>,
            R2H<R2A<R2B<R2C<R2E<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoH = Recv<Branching0fromHtoD, End>;

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
            Recv<(), Send<(), RecursEtoH>>,
            R2H<R2A<R2B<R2C<R2D<R2F<R2G<RoleH<RoleEnd>>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoH = Recv<Branching0fromHtoE, End>;

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
            Recv<(), Send<(), RecursFtoH>>,
            R2H<R2A<R2B<R2C<R2D<R2E<R2G<RoleH<RoleEnd>>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoH = Recv<Branching0fromHtoF, End>;

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
            Recv<(), Send<(), RecursGtoH>>,
            R2H<R2A<R2B<R2C<R2D<R2E<R2F<RoleH<RoleEnd>>>>>>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoH = Recv<Branching0fromHtoG, End>;

// H
type Choose0fromHtoA = Send<Branching0fromHtoA, End>;
type Choose0fromHtoB = Send<Branching0fromHtoB, End>;
type Choose0fromHtoC = Send<Branching0fromHtoC, End>;
type Choose0fromHtoD = Send<Branching0fromHtoD, End>;
type Choose0fromHtoE = Send<Branching0fromHtoE, End>;
type Choose0fromHtoF = Send<Branching0fromHtoF, End>;
type Choose0fromHtoG = Send<Branching0fromHtoG, End>;
type EndpointMoreH = MeshedChannelsEight<
    Send<(), Recv<(), Choose0fromHtoA>>,
    Send<(), Recv<(), Choose0fromHtoB>>,
    Send<(), Recv<(), Choose0fromHtoC>>,
    Send<(), Recv<(), Choose0fromHtoD>>,
    Send<(), Recv<(), Choose0fromHtoE>>,
    Send<(), Recv<(), Choose0fromHtoF>>,
    Send<(), Recv<(), Choose0fromHtoG>>,
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

#[inline]
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoA::Done(s) => {
            s.close()
        },
        Branching0fromHtoA::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_a(s)
        },
    })
}

#[inline]
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoB::Done(s) => {
            s.close()
        },
        Branching0fromHtoB::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

#[inline]
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoC::Done(s) => {
            s.close()
        },
        Branching0fromHtoC::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_c(s)
        },
    })
}

#[inline]
fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoD::Done(s) => {
            s.close()
        },
        Branching0fromHtoD::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_d(s)
        },
    })
}

#[inline]
fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoE::Done(s) => {
            s.close()
        },
        Branching0fromHtoE::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_e(s)
        },
    })
}

#[inline]
fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoF::Done(s) => {
            s.close()
        },
        Branching0fromHtoF::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
    })
}

#[inline]
fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromHtoG::Done(s) => {
            s.close()
        },
        Branching0fromHtoG::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            endpoint_g(s)
        },
    })
}

#[inline]
fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for _ in 1..LOOPS {
        temp_s = recurs_h(temp_s)?;
    }

    let s = choose_mpst_h_to_all!(
        temp_s,
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

fn recurs_h(s: EndpointH) -> Result<EndpointH, Box<dyn Error>> {
    let s: EndpointMoreH = choose_mpst_h_to_all!(
        s,
        Branching0fromHtoA::More,
        Branching0fromHtoB::More,
        Branching0fromHtoC::More,
        Branching0fromHtoD::More,
        Branching0fromHtoE::More,
        Branching0fromHtoF::More,
        Branching0fromHtoG::More
    );

    let s = s.send(())?;
    let (_, s) = s.recv()?;
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    Ok(s)
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
    c.bench_function(&format!("mesh eight baking inline protocol {LOOPS}"), |b| {
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
