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
generate!("recursive", MeshedChannels, A, B, C);

// Types
// Send/Recv
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;

// Roles
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;

// A
enum Branching0fromCtoA {
    More(MeshedChannels<RS, Recv<(), Send<(), RecursAtoC>>, R2C<R2B<RoleC<RoleEnd>>>, NameA>),
    Done(MeshedChannels<End, End, RoleEnd, NameA>),
}
type RecursAtoC = Recv<Branching0fromCtoA, End>;

// B
enum Branching0fromCtoB {
    More(MeshedChannels<SR, Recv<(), Send<(), RecursBtoC>>, R2C<R2A<RoleC<RoleEnd>>>, NameB>),
    Done(MeshedChannels<End, End, RoleEnd, NameB>),
}
type RecursBtoC = Recv<Branching0fromCtoB, End>;

// C
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoB = Send<Branching0fromCtoB, End>;
type EndpointMoreC = MeshedChannels<
    Send<(), Recv<(), Choose0fromCtoA>>,
    Send<(), Recv<(), Choose0fromCtoB>>,
    R2A<R2B<RoleBroadcast>>,
    NameC,
>;

// Creating the MP sessions
type EndpointA = MeshedChannels<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoA::Done(s) => {
            s.close()
        },
        Branching0fromCtoA::More(s) => {
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
        Branching0fromCtoB::Done(s) => {
            s.close()
        },
        Branching0fromCtoB::More(s) => {
            let (_, s) = s.recv();
            let s = s.send(());
            let s = s.send(());
            let (_, s) = s.recv();
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, LOOPS)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branching0fromCtoA::Done, Branching0fromCtoB::Done);

            s.close()
        }
        i => {
            let s: EndpointMoreC =
                choose_mpst_c_to_all!(s, Branching0fromCtoA::More, Branching0fromCtoB::More);

            let s = s.send(());
            let (_, s) = s.recv();
            let s = s.send(());
            let (_, s) = s.recv();

            recurs_c(s, i - 1)
        }
    }
}

fn aux() {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh(c: &mut Criterion) {
    c.bench_function(&format!("mesh three baking MPST {LOOPS}"), |b| b.iter(aux));
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
