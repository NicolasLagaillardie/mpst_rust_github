use criterion::{black_box, Criterion};

use mpstthree::baker;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
baker!("rec_and_cancel", MeshedChannelsThree, A, C, S);

// Types
// A
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;

// A
enum Branching0fromCtoA {
    Sum(MeshedChannelsThree<End, End, RoleEnd, NameA>),
    Diff(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}
// S
enum Branching0fromCtoS {
    Sum(MeshedChannelsThree<End, Send<i32, End>, RoleC<RoleEnd>, NameS>),
    Diff(MeshedChannelsThree<End, Send<i32, End>, RoleC<RoleEnd>, NameS>),
}

// Creating the MP sessions
// A
type EndpointA = MeshedChannelsThree<Recv<Branching0fromCtoA, End>, End, RoleC<RoleEnd>, NameA>;
// C
type EndpointC = MeshedChannelsThree<
    Choose0fromCtoA,
    Send<i32, Send<i32, Choose0fromCtoS>>,
    RoleS<RoleS<RoleBroadcast>>,
    NameC,
>;
type EndpointCSum = MeshedChannelsThree<End, Recv<i32, End>, RoleS<RoleEnd>, NameC>;
type EndpointCDiff = MeshedChannelsThree<End, Recv<i32, End>, RoleS<RoleEnd>, NameC>;
// S
type EndpointS = MeshedChannelsThree<
    End,
    Recv<i32, Recv<i32, Recv<Branching0fromCtoS, End>>>,
    RoleC<RoleC<RoleC<RoleEnd>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoA::Sum(s) => {
            s.close()
        },
        Branching0fromCtoA::Diff(s) => {
            s.close()
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = s.send(thread_rng().gen_range(1..=100) as i32)?;
    let s = s.send(thread_rng().gen_range(1..=100) as i32)?;

    let choice: i32 = thread_rng().gen_range(1..=2);

    if choice != 1 {
        let s: EndpointCSum =
            choose_mpst_c_to_all!(s, Branching0fromCtoA::Sum, Branching0fromCtoS::Sum);

        let (_sum, s) = s.recv()?;

        s.close()
    } else {
        let s: EndpointCDiff =
            choose_mpst_c_to_all!(s, Branching0fromCtoA::Diff, Branching0fromCtoS::Diff);

        let (_diff, s) = s.recv()?;

        s.close()
    }
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    let (elt_1, s) = s.recv()?;
    let (elt_2, s) = s.recv()?;

    offer_mpst!(s, {
        Branching0fromCtoS::Sum(s) => {
            let s = s.send(elt_1 + elt_2)?;
            s.close()
        },
        Branching0fromCtoS::Diff(s) => {
            let s = s.send(elt_1 - elt_2)?;
            s.close()
        },
    })
}

fn all_mpst() {
    let (thread_a, thread_c, thread_s) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_c),
        black_box(endpoint_s),
    );

    thread_a.join().unwrap();
    thread_c.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

pub fn distributed_calc_main(c: &mut Criterion) {
    c.bench_function("Distributed calculator baking", |b| b.iter(all_mpst));
}
