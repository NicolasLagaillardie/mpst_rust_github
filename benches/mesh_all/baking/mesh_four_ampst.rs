#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::baker;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// use std::time::Duration;

// Create new roles
baker!("rec_and_cancel", MeshedChannelsFour, A, B, C, D);

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;

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
            Recv<(), Send<(), RecursAtoD>>,
            R2D<R2B<R2C<RoleD<RoleEnd>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameA>),
}
type RecursAtoD = Recv<Branching0fromDtoA, End>;

// B
enum Branching0fromDtoB {
    More(
        MeshedChannelsFour<
            SR,
            RS,
            Recv<(), Send<(), RecursBtoD>>,
            R2D<R2A<R2C<RoleD<RoleEnd>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameB>),
}
type RecursBtoD = Recv<Branching0fromDtoB, End>;

// C
enum Branching0fromDtoC {
    More(
        MeshedChannelsFour<
            SR,
            SR,
            Recv<(), Send<(), RecursCtoD>>,
            R2D<R2A<R2B<RoleD<RoleEnd>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsFour<End, End, End, RoleEnd, NameC>),
}
type RecursCtoD = Recv<Branching0fromDtoC, End>;

// D
type Choose0fromDtoA = Send<Branching0fromDtoA, End>;
type Choose0fromDtoB = Send<Branching0fromDtoB, End>;
type Choose0fromDtoC = Send<Branching0fromDtoC, End>;
type EndpointMoreD = MeshedChannelsFour<
    Send<(), Recv<(), Choose0fromDtoA>>,
    Send<(), Recv<(), Choose0fromDtoB>>,
    Send<(), Recv<(), Choose0fromDtoC>>,
    R2A<R2B<R2C<RoleBroadcast>>>,
    NameD,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsFour<End, End, RecursAtoD, RoleD<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsFour<End, End, RecursBtoD, RoleD<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsFour<End, End, RecursCtoD, RoleD<RoleEnd>, NameC>;
type EndpointD =
    MeshedChannelsFour<Choose0fromDtoA, Choose0fromDtoB, Choose0fromDtoC, RoleBroadcast, NameD>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromDtoA::Done(s) => {
            s.close()
        },
        Branching0fromDtoA::More(s) => {
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

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromDtoB::Done(s) => {
            s.close()
        },
        Branching0fromDtoB::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromDtoC::Done(s) => {
            s.close()
        },
        Branching0fromDtoC::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    recurs_d(s, LOOPS)
}

fn recurs_d(s: EndpointD, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_d_to_all!(
                s,
                Branching0fromDtoA::Done,
                Branching0fromDtoB::Done,
                Branching0fromDtoC::Done
            );

            s.close()
        }
        i => {
            let s: EndpointMoreD = choose_mpst_d_to_all!(
                s,
                Branching0fromDtoA::More,
                Branching0fromDtoB::More,
                Branching0fromDtoC::More
            );

            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;

            recurs_d(s, i - 1)
        }
    }
}

fn aux() {
    let (thread_a, thread_b, thread_c, thread_d) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_ampst(c: &mut Criterion) {
    c.bench_function(&format!("mesh four baking protocol AMPST {LOOPS}"), |b| {
        b.iter(aux)
    });
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = mesh_protocol_ampst,
}

criterion_main! {
    bench
}
