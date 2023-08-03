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

// Create new roles
generate!("recursive", MeshedChannels, A, B, C, D, E, F);

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

// A
enum Branching0fromFtoA {
    More(
        MeshedChannels<
            RS,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursAtoF>>,
            R2F<R2B<R2C<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameA,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoF = Recv<Branching0fromFtoA, End>;

// B
enum Branching0fromFtoB {
    More(
        MeshedChannels<
            SR,
            RS,
            RS,
            RS,
            Recv<(), Send<(), RecursBtoF>>,
            R2F<R2A<R2C<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameB,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoF = Recv<Branching0fromFtoB, End>;

// C
enum Branching0fromFtoC {
    More(
        MeshedChannels<
            SR,
            SR,
            RS,
            RS,
            Recv<(), Send<(), RecursCtoF>>,
            R2F<R2A<R2B<R2D<R2E<RoleF<RoleEnd>>>>>>,
            NameC,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoF = Recv<Branching0fromFtoC, End>;

// D
enum Branching0fromFtoD {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            RS,
            Recv<(), Send<(), RecursDtoF>>,
            R2F<R2A<R2B<R2C<R2E<RoleF<RoleEnd>>>>>>,
            NameD,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoF = Recv<Branching0fromFtoD, End>;

// E
enum Branching0fromFtoE {
    More(
        MeshedChannels<
            SR,
            SR,
            SR,
            SR,
            Recv<(), Send<(), RecursEtoF>>,
            R2F<R2A<R2B<R2C<R2D<RoleF<RoleEnd>>>>>>,
            NameE,
        >,
    ),
    Done(MeshedChannels<End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoF = Recv<Branching0fromFtoE, End>;

// F
type Choose0fromFtoA = Send<Branching0fromFtoA, End>;
type Choose0fromFtoB = Send<Branching0fromFtoB, End>;
type Choose0fromFtoC = Send<Branching0fromFtoC, End>;
type Choose0fromFtoD = Send<Branching0fromFtoD, End>;
type Choose0fromFtoE = Send<Branching0fromFtoE, End>;
type EndpointMoreF = MeshedChannels<
    Send<(), Recv<(), Choose0fromFtoA>>,
    Send<(), Recv<(), Choose0fromFtoB>>,
    Send<(), Recv<(), Choose0fromFtoC>>,
    Send<(), Recv<(), Choose0fromFtoD>>,
    Send<(), Recv<(), Choose0fromFtoE>>,
    R2A<R2B<R2C<R2D<R2E<RoleBroadcast>>>>>,
    NameF,
>;

// Creating the MP sessions
type EndpointA = MeshedChannels<End, End, End, End, RecursAtoF, RoleF<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, End, End, End, RecursBtoF, RoleF<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<End, End, End, End, RecursCtoF, RoleF<RoleEnd>, NameC>;
type EndpointD = MeshedChannels<End, End, End, End, RecursDtoF, RoleF<RoleEnd>, NameD>;
type EndpointE = MeshedChannels<End, End, End, End, RecursEtoF, RoleF<RoleEnd>, NameE>;
type EndpointF = MeshedChannels<
    Choose0fromFtoA,
    Choose0fromFtoB,
    Choose0fromFtoC,
    Choose0fromFtoD,
    Choose0fromFtoE,
    RoleBroadcast,
    NameF,
>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromFtoA::Done(s) => {
            s.close()
        },
        Branching0fromFtoA::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromFtoB::Done(s) => {
            s.close()
        },
        Branching0fromFtoB::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromFtoC::Done(s) => {
            s.close()
        },
        Branching0fromFtoC::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromFtoD::Done(s) => {
            s.close()
        },
        Branching0fromFtoD::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let (_, s) = s.recv();
            let s = s.send(());
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromFtoE::Done(s) => {
            s.close()
        },
        Branching0fromFtoE::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    recurs_f(s, LOOPS)
}

fn recurs_f(s: EndpointF, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_f_to_all!(
                s,
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
                Branching0fromFtoA::More,
                Branching0fromFtoB::More,
                Branching0fromFtoC::More,
                Branching0fromFtoD::More,
                Branching0fromFtoE::More
            );

            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();

            recurs_f(s, i - 1)
        }
    }
}

fn aux() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
        black_box(endpoint_f),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    thread_d.join().unwrap();
    thread_e.join().unwrap();
    thread_f.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("mesh six baking MPST {LOOPS}"), |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(10000);
    targets = mesh_protocol_mpst,
}

criterion_main! {
    bench
}
