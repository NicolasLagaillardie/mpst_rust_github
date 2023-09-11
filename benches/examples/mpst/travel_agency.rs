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

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
generate!("recursive", MeshedChannels, A, C, S);

// Types
// C0
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;

// C1
type Choose1fromCtoA = <Choice1fromCtoA as Session>::Dual;
type Choose1fromCtoS = <Choice1fromCtoS as Session>::Dual;

// A
enum Branching0fromCtoA {
    Select(MeshedChannels<Choice1fromCtoA, End, RoleC<RoleEnd>, NameA>),
    Loop(
        MeshedChannels<
            Recv<i32, Send<i32, Choice0fromCtoA>>,
            Send<i32, End>,
            RoleC<RoleC<RoleS<RoleC<RoleEnd>>>>,
            NameA,
        >,
    ),
}
type Choice0fromCtoA = Recv<Branching0fromCtoA, End>;
enum Branching1fromCtoA {
    Yes(MeshedChannels<Recv<i32, End>, Send<i32, End>, RoleC<RoleS<RoleEnd>>, NameA>),
    No(MeshedChannels<Recv<i32, End>, Send<i32, End>, RoleC<RoleS<RoleEnd>>, NameA>),
}
type Choice1fromCtoA = Recv<Branching1fromCtoA, End>;

// S
enum Branching0fromCtoS {
    Select(MeshedChannels<End, Choice1fromCtoS, RoleC<RoleEnd>, NameS>),
    Loop(MeshedChannels<Recv<i32, End>, Choice0fromCtoS, RoleA<RoleC<RoleEnd>>, NameS>),
}
type Choice0fromCtoS = Recv<Branching0fromCtoS, End>;
enum Branching1fromCtoS {
    Yes(
        MeshedChannels<
            Recv<i32, End>,
            Recv<i32, Send<i32, End>>,
            RoleA<RoleC<RoleC<RoleEnd>>>,
            NameS,
        >,
    ),
    No(MeshedChannels<Recv<i32, End>, End, RoleA<RoleEnd>, NameS>),
}
type Choice1fromCtoS = Recv<Branching1fromCtoS, End>;

// Creating the MP sessions
// A
type ChoiceA = MeshedChannels<Choice1fromCtoA, End, RoleC<RoleEnd>, NameA>;
type EndpointA = MeshedChannels<Choice0fromCtoA, End, RoleC<RoleEnd>, NameA>;

// C
type ChoiceC = MeshedChannels<Choose1fromCtoA, Choose1fromCtoS, RoleBroadcast, NameC>;
type ChoiceCYes =
    MeshedChannels<Send<i32, End>, Send<i32, Recv<i32, End>>, RoleA<RoleS<RoleS<RoleEnd>>>, NameC>;
type ChoiceCNo = MeshedChannels<Send<i32, End>, End, RoleA<RoleEnd>, NameC>;
type EndpointC = MeshedChannels<Choose0fromCtoA, Choose0fromCtoS, RoleBroadcast, NameC>;
type EndpointCLoop = MeshedChannels<
    Send<i32, Recv<i32, Choose0fromCtoA>>,
    Choose0fromCtoS,
    RoleA<RoleA<RoleBroadcast>>,
    NameC,
>;

// S
type ChoiceS = MeshedChannels<End, Choice1fromCtoS, RoleC<RoleEnd>, NameS>;
type EndpointS = MeshedChannels<End, Choice0fromCtoS, RoleC<RoleEnd>, NameS>;

// Functions
// A
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoA::Select(s) => {
            choice_a(s)
        },
        Branching0fromCtoA::Loop(s) => {
            let (query, s) = s.recv();
            let s = s.send(query);
            let s = s.send(1);
            endpoint_a(s)
        },
    })
}

fn choice_a(s: ChoiceA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromCtoA::Yes(s) => {
            let (yes, s) = s.recv();
            let s = s.send(yes);
            s.close()
        },
        Branching1fromCtoA::No(s) => {
            let (no, s) = s.recv();
            let s = s.send(no);
            s.close()
        },
    })
}

fn endpoint_c_init(s: EndpointC) -> Result<(), Box<dyn Error>> {
    endpoint_c(s, LOOPS)
}

fn endpoint_c(s: EndpointC, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s =
                choose_mpst_c_to_all!(s, Branching0fromCtoA::Select, Branching0fromCtoS::Select);
            choice_c(s)
        }
        _ => {
            let s: EndpointCLoop =
                choose_mpst_c_to_all!(s, Branching0fromCtoA::Loop, Branching0fromCtoS::Loop);

            let s = s.send(1);
            let (_quote, s) = s.recv();
            endpoint_c(s, loops - 1)
        }
    }
}

fn choice_c(s: ChoiceC) -> Result<(), Box<dyn Error>> {
    let choice: i32 = thread_rng().gen_range(1..3);

    if choice != 1 {
        let s: ChoiceCYes =
            choose_mpst_c_to_all!(s, Branching1fromCtoA::Yes, Branching1fromCtoS::Yes);

        let s = s.send(1);
        let s = s.send(1);
        let (_ack, s) = s.recv();
        s.close()
    } else {
        let s: ChoiceCNo = choose_mpst_c_to_all!(s, Branching1fromCtoA::No, Branching1fromCtoS::No);

        let s = s.send(0);
        s.close()
    }
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoS::Select(s) => {
            choice_s(s)
        },
        Branching0fromCtoS::Loop(s) => {
            let (_dummy, s) = s.recv();
            endpoint_s(s)
        },
    })
}

fn choice_s(s: ChoiceS) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromCtoS::Yes(s) => {
            let (_yes, s) = s.recv();
            let (payment, s) = s.recv();
            let s = s.send(payment);
            s.close()
        },
        Branching1fromCtoS::No(s) => {
            let (_no, s) = s.recv();
            s.close()
        },
    })
}

fn aux() {
    let (thread_a, thread_c, thread_s) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_c_init),
        black_box(endpoint_s),
    );

    thread_a.join().unwrap();
    thread_c.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

static LOOPS: i32 = 100;

pub fn travel(c: &mut Criterion) {
    c.bench_function("Travel agency MPST", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = travel,
}

criterion_main! {
    bench
}
