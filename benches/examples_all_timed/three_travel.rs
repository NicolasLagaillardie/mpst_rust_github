#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_timed;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
generate_timed!(MeshedChannels, A, C, S);

// Types
type RS<S> =
    RecvTimed<i32, 'a', 0, true, 10, true, ' ', SendTimed<i32, 'a', 0, true, 10, true, ' ', S>>;

// C0
type Choose0fromCtoA = SendTimed<Branching0fromCtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose0fromCtoS = SendTimed<Branching0fromCtoS, 'a', 0, true, 10, true, ' ', End>;

// C1
type Choose1fromCtoA = SendTimed<Branching1fromCtoA, 'a', 0, true, 10, true, ' ', End>;
type Choose1fromCtoS = SendTimed<Branching1fromCtoS, 'a', 0, true, 10, true, ' ', End>;

// A
enum Branching0fromCtoA {
    Select(MeshedChannels<Choice1fromCtoA, End, RoleC<RoleEnd>, NameA>),
    Loops(
        MeshedChannels<
            RS<Choice0fromCtoA>,
            SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            RolesCCSC,
            NameA,
        >,
    ),
}
type RolesCCSC = RoleC<RoleC<RoleS<RoleC<RoleEnd>>>>;
type Choice0fromCtoA = RecvTimed<Branching0fromCtoA, 'a', 0, true, 10, true, ' ', End>;

enum Branching1fromCtoA {
    Yes(
        MeshedChannels<
            RecvTimed<
                i32,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            >,
            SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            RoleC<RoleS<RoleC<RoleEnd>>>,
            NameA,
        >,
    ),
    No(
        MeshedChannels<
            RecvTimed<
                i32,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            >,
            SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            RoleC<RoleS<RoleC<RoleEnd>>>,
            NameA,
        >,
    ),
}
type Choice1fromCtoA = RecvTimed<Branching1fromCtoA, 'a', 0, true, 10, true, ' ', End>;

// S
enum Branching0fromCtoS {
    Select(MeshedChannels<End, Choice1fromCtoS, RoleC<RoleEnd>, NameS>),
    Loops(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            Choice0fromCtoS,
            RolesAC,
            NameS,
        >,
    ),
}
type RolesAC = RoleA<RoleC<RoleEnd>>;
type Choice0fromCtoS = RecvTimed<Branching0fromCtoS, 'a', 0, true, 10, true, ' ', End>;

enum Branching1fromCtoS {
    Yes(MeshedChannels<RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>, RS<End>, RolesACC, NameS>),
    No(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            End,
            RoleA<RoleEnd>,
            NameS,
        >,
    ),
}
type RolesACC = RoleA<RoleC<RoleC<RoleEnd>>>;
type Choice1fromCtoS = RecvTimed<Branching1fromCtoS, 'a', 0, true, 10, true, ' ', End>;

// Creating the MP sessions
// A
type ChoiceA = MeshedChannels<Choice1fromCtoA, End, RoleC<RoleEnd>, NameA>;
type EndpointA = MeshedChannels<Choice0fromCtoA, End, RoleC<RoleEnd>, NameA>;

// C
type ChoiceC = MeshedChannels<Choose1fromCtoA, Choose1fromCtoS, RoleBroadcast, NameC>;
type EndpointC = MeshedChannels<Choose0fromCtoA, Choose0fromCtoS, RoleBroadcast, NameC>;
type EndpointCSelect = MeshedChannels<
    SendTimed<Branching1fromCtoA, 'a', 0, true, 10, true, ' ', End>,
    SendTimed<Branching1fromCtoS, 'a', 0, true, 10, true, ' ', End>,
    RoleBroadcast,
    NameC,
>;
type EndpointCLoops = MeshedChannels<
    SendTimed<
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
            SendTimed<Branching0fromCtoA, 'a', 0, true, 10, true, ' ', End>,
        >,
    >,
    SendTimed<Branching0fromCtoS, 'a', 0, true, 10, true, ' ', End>,
    RoleA<RoleA<RoleBroadcast>>,
    NameC,
>;
type EndpointCYes = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', SendTimed<i32, 'a', 0, true, 10, true, ' ', End>>,
    SendTimed<i32, 'a', 0, true, 10, true, ' ', RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>>,
    RoleA<RoleS<RoleS<RoleA<RoleEnd>>>>,
    NameC,
>;
type EndpointCNo = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', SendTimed<i32, 'a', 0, true, 10, true, ' ', End>>,
    End,
    RoleA<RoleA<RoleEnd>>,
    NameC,
>;

// S
type ChoiceS = MeshedChannels<End, Choice1fromCtoS, RoleC<RoleEnd>, NameS>;
type EndpointS = MeshedChannels<End, Choice0fromCtoS, RoleC<RoleEnd>, NameS>;

// Functions
// A
fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    offer_mpst!(s, all_clocks, {
        Branching0fromCtoA::Select(s) => {
            choice_a(s, all_clocks)
        },
        Branching0fromCtoA::Loops(s) => {
            let (query, s) = s.recv(all_clocks)?;
            let s = s.send(query, all_clocks)?;
            let s = s.send(1, all_clocks)?;
            endpoint_a(s, all_clocks)
        },
    })
}

fn choice_a(s: ChoiceA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching1fromCtoA::Yes(s) => {
            let (yes, s) = s.recv(all_clocks)?;
            let s = s.send(yes, all_clocks)?;
            let (_yes, s) = s.recv(all_clocks)?;
            s.close()
        },
        Branching1fromCtoA::No(s) => {
            let (no, s) = s.recv(all_clocks)?;
            let s = s.send(no, all_clocks)?;
            let (_yes, s) = s.recv(all_clocks)?;
            s.close()
        },
    })
}

fn endpoint_c_init(
    s: EndpointC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    endpoint_c(s, 100, all_clocks)
}

fn endpoint_c(
    s: EndpointC,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s: EndpointCSelect = choose_mpst_c_to_all!(
                s,
                all_clocks,
                Branching0fromCtoA::Select,
                Branching0fromCtoS::Select,
            );
            choice_c(s, all_clocks)
        }
        _ => {
            let s: EndpointCLoops = choose_mpst_c_to_all!(
                s,
                all_clocks,
                Branching0fromCtoA::Loops,
                Branching0fromCtoS::Loops,
            );

            let s = s.send(1, all_clocks)?;
            let (_quote, s) = s.recv(all_clocks)?;
            endpoint_c(s, loops - 1, all_clocks)
        }
    }
}

fn choice_c(s: ChoiceC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    let choice: i32 = thread_rng().gen_range(1..=3);

    if choice != 1 {
        let s: EndpointCYes = choose_mpst_c_to_all!(
            s,
            all_clocks,
            Branching1fromCtoA::Yes,
            Branching1fromCtoS::Yes,
        );

        let s = s.send(1, all_clocks)?;
        let s = s.send(1, all_clocks)?;
        let (_ack, s) = s.recv(all_clocks)?;
        let s = s.send(1, all_clocks)?;
        s.close()
    } else {
        let s: EndpointCNo = choose_mpst_c_to_all!(
            s,
            all_clocks,
            Branching1fromCtoA::No,
            Branching1fromCtoS::No,
        );

        let s = s.send(0, all_clocks)?;
        let s = s.send(1, all_clocks)?;
        s.close()
    }
}

fn endpoint_s(s: EndpointS, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    offer_mpst!(s, all_clocks, {
        Branching0fromCtoS::Select(s) => {
            choice_s(s, all_clocks)
        },
        Branching0fromCtoS::Loops(s) => {
            let (_dummy, s) = s.recv(all_clocks)?;
            endpoint_s(s, all_clocks)
        },
    })
}

fn choice_s(s: ChoiceS, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching1fromCtoS::Yes(s) => {
            let (_yes, s) = s.recv(all_clocks)?;
            let (payment, s) = s.recv(all_clocks)?;
            let s = s.send(payment, all_clocks)?;
            s.close()
        },
        Branching1fromCtoS::No(s) => {
            let (_no, s) = s.recv(all_clocks)?;
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

pub fn travel(c: &mut Criterion) {
    c.bench_function("Timed Travel MPST baking", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = travel,
}

criterion_main! {
    bench
}
