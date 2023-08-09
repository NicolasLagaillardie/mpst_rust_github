#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// Create new roles
generate!("rec_and_cancel", MeshedChannels, A, B, C);

// Types
// A
enum Branching0fromCtoA {
    Forward(MeshedChannels<Send<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Backward(MeshedChannels<Recv<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Done(MeshedChannels<End, End, RoleEnd, NameA>),
}
type RecursAtoC = <Choose0fromCtoA as Session>::Dual;

// B
enum Branching0fromCtoB {
    Forward(
        MeshedChannels<Recv<(), End>, Send<(), RecursBtoC>, RoleA<RoleC<RoleC<RoleEnd>>>, NameB>,
    ),
    Backward(
        MeshedChannels<Send<(), End>, Recv<(), RecursBtoC>, RoleC<RoleA<RoleC<RoleEnd>>>, NameB>,
    ),
    Done(MeshedChannels<End, End, RoleEnd, NameB>),
}
type RecursBtoC = <Choose0fromCtoB as Session>::Dual;

// C
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoB = Send<Branching0fromCtoB, End>;
type EndpointForwardC =
    MeshedChannels<Choose0fromCtoA, Recv<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;
type EndpointBackwardC =
    MeshedChannels<Choose0fromCtoA, Send<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;

// Creating the MP sessions
type EndpointA = MeshedChannels<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannels<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannels<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoA::Done(s) => {
            s.close()
        },
        Branching0fromCtoA::Forward(s) => {
            let s = s.send(())?;
            endpoint_a(s)
        },
        Branching0fromCtoA::Backward(s) => {
            let (_, s) = s.recv()?;
            endpoint_a(s)
        },
    })
}

#[inline]
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoB::Done(s) => {
            s.close()
        },
        Branching0fromCtoB::Forward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
        Branching0fromCtoB::Backward(s) => {
            let ((), s) = s.recv()?;
            let s = s.send(())?;
            endpoint_b(s)
        },
    })
}

#[inline]
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let mut temp_s = s;

    for i in 1..LOOPS {
        temp_s = recurs_c(temp_s, i)?;
    }

    let s = choose_mpst_c_to_all!(temp_s, Branching0fromCtoA::Done, Branching0fromCtoB::Done);

    s.close()
}

fn recurs_c(s: EndpointC, index: i64) -> Result<EndpointC, Box<dyn Error>> {
    match index {
        i if i % 2 == 0 => {
            let s: EndpointForwardC =
                choose_mpst_c_to_all!(s, Branching0fromCtoA::Forward, Branching0fromCtoB::Forward);

            let (_, s) = s.recv()?;

            Ok(s)
        }
        _ => {
            let s: EndpointBackwardC = choose_mpst_c_to_all!(
                s,
                Branching0fromCtoA::Backward,
                Branching0fromCtoB::Backward
            );

            let s = s.send(())?;

            Ok(s)
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

pub fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring three baking inline {LOOPS}"), |b| {
        b.iter(aux)
    });
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(10000);
    targets = ring_protocol_mpst,
}

criterion_main! {
    bench
}
