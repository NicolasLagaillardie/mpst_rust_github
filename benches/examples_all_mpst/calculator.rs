use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for three participants
generate!("recursive", MeshedChannels, C, S);

// Types
// C
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;

// S
enum Branching0fromCtoS {
    Sum(MeshedChannels<Send<i32, End>, RoleC<RoleEnd>, NameS>),
    Diff(MeshedChannels<Send<i32, End>, RoleC<RoleEnd>, NameS>),
}

// Creating the MP sessions
// C
type EndpointC =
    MeshedChannels<Send<i32, Send<i32, Choose0fromCtoS>>, RoleS<RoleS<RoleBroadcast>>, NameC>;
type EndpointCSum = MeshedChannels<Recv<i32, End>, RoleS<RoleEnd>, NameC>;
type EndpointCDiff = MeshedChannels<Recv<i32, End>, RoleS<RoleEnd>, NameC>;

// S
type EndpointS = MeshedChannels<
    Recv<i32, Recv<i32, Recv<Branching0fromCtoS, End>>>,
    RoleC<RoleC<RoleC<RoleEnd>>>,
    NameS,
>;

// Functions
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = s.send(thread_rng().gen_range(1..=100));
    let s = s.send(thread_rng().gen_range(1..=100));

    let choice: i32 = thread_rng().gen_range(1..=2);

    if choice != 1 {
        let s: EndpointCSum = choose_mpst_c_to_all!(s, Branching0fromCtoS::Sum);

        let (_sum, s) = s.recv();
        s.close()
    } else {
        let s: EndpointCDiff = choose_mpst_c_to_all!(s, Branching0fromCtoS::Diff);

        let (_diff, s) = s.recv();
        s.close()
    }
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    let (elt_1, s) = s.recv();
    let (elt_2, s) = s.recv();

    offer_mpst!(s, {
        Branching0fromCtoS::Sum(s) => {
            let s = s.send(elt_1 + elt_2);
            s.close()
        },
        Branching0fromCtoS::Diff(s) => {
            let s = s.send(elt_1 - elt_2);
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
    c.bench_function("Calculator MPST", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = calculator,
}

criterion_main! {
    bench
}
