#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{baker, choose, offer};

use std::error::Error;
use std::thread::{spawn, JoinHandle};

// use std::time::Duration;

// Create new roles
baker!(
    "rec_and_cancel",
    MeshedChannelsTen,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J
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
            Recv<(), Send<(), RecursAtoJ>>,
            R2J<R2B<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoJ = Recv<Branching0fromJtoA, End>;

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
            Recv<(), Send<(), RecursBtoJ>>,
            R2J<R2A<R2C<R2D<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoJ = Recv<Branching0fromJtoB, End>;

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
            Recv<(), Send<(), RecursCtoJ>>,
            R2J<R2A<R2B<R2D<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoJ = Recv<Branching0fromJtoC, End>;

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
            Recv<(), Send<(), RecursDtoJ>>,
            R2J<R2A<R2B<R2C<R2E<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoJ = Recv<Branching0fromJtoD, End>;

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
            Recv<(), Send<(), RecursEtoJ>>,
            R2J<R2A<R2B<R2C<R2D<R2F<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoJ = Recv<Branching0fromJtoE, End>;

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
            Recv<(), Send<(), RecursFtoJ>>,
            R2J<R2A<R2B<R2C<R2D<R2E<R2G<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoJ = Recv<Branching0fromJtoF, End>;

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
            Recv<(), Send<(), RecursGtoJ>>,
            R2J<R2A<R2B<R2C<R2D<R2E<R2F<R2H<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoJ = Recv<Branching0fromJtoG, End>;

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
            Recv<(), Send<(), RecursHtoJ>>,
            R2J<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2I<RoleJ<RoleEnd>>>>>>>>>>,
            NameH,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameH>),
}
type RecursHtoJ = Recv<Branching0fromJtoH, End>;

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
            Recv<(), Send<(), RecursItoJ>>,
            R2J<R2A<R2B<R2C<R2D<R2E<R2F<R2G<R2H<RoleJ<RoleEnd>>>>>>>>>>,
            NameI,
        >,
    ),
    Done(MeshedChannelsTen<End, End, End, End, End, End, End, End, End, RoleEnd, NameI>),
}
type RecursItoJ = Recv<Branching0fromJtoI, End>;

// J
type Choose0fromJtoA = Send<Branching0fromJtoA, End>;
type Choose0fromJtoB = Send<Branching0fromJtoB, End>;
type Choose0fromJtoC = Send<Branching0fromJtoC, End>;
type Choose0fromJtoD = Send<Branching0fromJtoD, End>;
type Choose0fromJtoE = Send<Branching0fromJtoE, End>;
type Choose0fromJtoF = Send<Branching0fromJtoF, End>;
type Choose0fromJtoG = Send<Branching0fromJtoG, End>;
type Choose0fromJtoH = Send<Branching0fromJtoH, End>;
type Choose0fromJtoI = Send<Branching0fromJtoI, End>;
type EndpointMoreJ = MeshedChannelsTen<
    Send<(), Recv<(), Choose0fromJtoA>>,
    Send<(), Recv<(), Choose0fromJtoB>>,
    Send<(), Recv<(), Choose0fromJtoC>>,
    Send<(), Recv<(), Choose0fromJtoD>>,
    Send<(), Recv<(), Choose0fromJtoE>>,
    Send<(), Recv<(), Choose0fromJtoF>>,
    Send<(), Recv<(), Choose0fromJtoG>>,
    Send<(), Recv<(), Choose0fromJtoH>>,
    Send<(), Recv<(), Choose0fromJtoI>>,
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

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoA::Done(s) => {
            s.close()
        },
        Branching0fromJtoA::More(s) => {
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
        Branching0fromJtoB::Done(s) => {
            s.close()
        },
        Branching0fromJtoB::More(s) => {
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
        Branching0fromJtoC::Done(s) => {
            s.close()
        },
        Branching0fromJtoC::More(s) => {
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
        Branching0fromJtoD::Done(s) => {
            s.close()
        },
        Branching0fromJtoD::More(s) => {
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
        Branching0fromJtoE::Done(s) => {
            s.close()
        },
        Branching0fromJtoE::More(s) => {
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
        Branching0fromJtoF::Done(s) => {
            s.close()
        },
        Branching0fromJtoF::More(s) => {
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
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_f(s)
        },
    })
}

#[inline]
fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoG::Done(s) => {
            s.close()
        },
        Branching0fromJtoG::More(s) => {
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
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_g(s)
        },
    })
}

#[inline]
fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoH::Done(s) => {
            s.close()
        },
        Branching0fromJtoH::More(s) => {
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
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_h(s)
        },
    })
}

#[inline]
fn endpoint_i(s: EndpointI) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromJtoI::Done(s) => {
            s.close()
        },
        Branching0fromJtoI::More(s) => {
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
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            endpoint_i(s)
        },
    })
}

#[inline]
fn endpoint_j(s: EndpointJ) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for _ in 1..LOOPS {
        temp_s = recurs_j(temp_s)?;
    }

    let s = choose_mpst_j_to_all!(
        temp_s,
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

fn recurs_j(s: EndpointJ) -> Result<EndpointJ, Box<dyn Error>> {
    let s: EndpointMoreJ = choose_mpst_j_to_all!(
        s,
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
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    let s = s.send(())?;
    let (_, s) = s.recv()?;
    Ok(s)
}

fn all_mpst() {
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
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
        black_box(endpoint_g),
        black_box(endpoint_h),
        black_box(endpoint_i),
        black_box(endpoint_j),
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

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(
        &format!("mesh ten baking inline protocol MPST {LOOPS}"),
        |b| b.iter(all_mpst),
    );
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets = mesh_protocol_mpst,
}

criterion_main! {
    bench
}
