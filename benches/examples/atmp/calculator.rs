#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants
// and the close and fork functions
generate_atmp!(MeshedChannels, C, S);

// Types
// C
type Choose0fromCtoS = SendTimed<Branching0fromCtoS, 'a', 0, true, 10, true, ' ', End>;

// S
enum Branching0fromCtoS {
    Sum(MeshedChannels<SendTimed<i32, 'a', 0, true, 10, true, ' ', End>, RoleC<RoleEnd>, NameS>),
    Diff(MeshedChannels<SendTimed<i32, 'a', 0, true, 10, true, ' ', End>, RoleC<RoleEnd>, NameS>),
}

// Creating the MP sessions
// C
type EndpointC = MeshedChannels<
    SendTimed<
        i32,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        SendTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromCtoS>,
    >,
    RoleS<RoleS<RoleBroadcast>>,
    NameC,
>;

// S
type EndpointS = MeshedChannels<
    RecvTimed<
        i32,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<
            i32,
            'a',
            0,
            true,
            10,
            true,
            ' ',
            RecvTimed<Branching0fromCtoS, 'a', 0, true, 10, true, ' ', End>,
        >,
    >,
    RoleC<RoleC<RoleC<RoleEnd>>>,
    NameS,
>;

// Functions
fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let elt_1: i32 = thread_rng().gen_range(1..=100);
    let elt_2: i32 = thread_rng().gen_range(1..=100);

    let s = s.send(elt_1, all_clocks)?;
    let s = s.send(elt_2, all_clocks)?;

    let choice: i32 = thread_rng().gen_range(1..=2);

    if choice != 1 {
        let s = choose_mpst_c_to_all!(s, all_clocks, Branching0fromCtoS::Sum);

        let (_sum, s) = s.recv(all_clocks)?;

        s.close()
    } else {
        let s = choose_mpst_c_to_all!(s, all_clocks, Branching0fromCtoS::Diff);

        let (_diff, s) = s.recv(all_clocks)?;

        s.close()
    }
}

fn endpoint_s(s: EndpointS, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (elt_1, s) = s.recv(all_clocks)?;
    let (elt_2, s) = s.recv(all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Branching0fromCtoS::Sum(s) => {
            let s = s.send(elt_1 + elt_2, all_clocks)?;
            s.close()
        },
        Branching0fromCtoS::Diff(s) => {
            let s = s.send(elt_1 - elt_2, all_clocks)?;
            s.close()
        },
    })
}

fn aux() {
    let (thread_c, thread_s) = fork_mpst(black_box(endpoint_c), black_box(endpoint_s));

    thread_c.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

pub fn calculator(c: &mut Criterion) {
    c.bench_function("Timed Calculator", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = calculator,
}

criterion_main! {
    bench
}
