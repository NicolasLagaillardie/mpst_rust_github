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
generate!("rec_and_cancel", MeshedChannels, A, B);

// Types
// A
enum Branching0fromBtoA {
    More(MeshedChannels<Recv<(), Send<(), RecursAtoB>>, RoleB<RoleB<RoleB<RoleEnd>>>, NameA>),
    Done(MeshedChannels<End, RoleEnd, NameA>),
}
type RecursAtoB = Recv<Branching0fromBtoA, End>;

// C
type Choose0fromBtoA = Send<Branching0fromBtoA, End>;
type EndpointMoreB =
    MeshedChannels<Send<(), Recv<(), Choose0fromBtoA>>, RoleA<RoleA<RoleBroadcast>>, NameB>;

// Creating the MP sessions
type EndpointA = MeshedChannels<RecursAtoB, RoleB<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<Choose0fromBtoA, RoleBroadcast, NameB>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromBtoA::Done(s) => {
            s.close()
        },
        Branching0fromBtoA::More(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            endpoint_a(s)
        },
    })
}

#[inline]
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for _ in 1..LOOPS {
        temp_s = recurs_b(temp_s)?;
    }

    let s = choose_mpst_b_to_all!(temp_s, Branching0fromBtoA::Done);

    s.close()
}

fn recurs_b(s: EndpointB) -> Result<EndpointB, Box<dyn Error>> {
    let s: EndpointMoreB = choose_mpst_b_to_all!(s, Branching0fromBtoA::More);

    let s = s.send(())?;
    let (_, s) = s.recv()?;
    Ok(s)
}

fn aux() {
    let (thread_a, thread_b) = fork_mpst(black_box(endpoint_a), black_box(endpoint_b));

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn mesh_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("mesh two baking inline {LOOPS}"), |b| b.iter(aux));
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
