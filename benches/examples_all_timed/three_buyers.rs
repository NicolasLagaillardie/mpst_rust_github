#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use criterion::{black_box, Criterion};

use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{random, thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
baker_timed!(MeshedChannels, A, C, S);

// Types
// A
type Choose0fromCtoA = SendTimed<Branching0fromCtoA, 'a', 0, true, 1, true, false, End>;
type Choose0fromCtoS = SendTimed<Branching0fromCtoS, 'a', 0, true, 1, true, false, End>;

// A
enum Branching0fromCtoA {
    Accept(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 1, true, false, End>,
            End,
            RoleC<RoleEnd>,
            NameA,
        >,
    ),
    Quit(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 1, true, false, End>,
            End,
            RoleC<RoleEnd>,
            NameA,
        >,
    ),
}
// S
enum Branching0fromCtoS {
    Accept(
        MeshedChannels<
            End,
            RecvTimed<
                i32,
                'a',
                0,
                true,
                1,
                true,
                false,
                SendTimed<i32, 'a', 0, true, 1, true, false, End>,
            >,
            TwoRoleC,
            NameS,
        >,
    ),
    Quit(
        MeshedChannels<
            End,
            RecvTimed<i32, 'a', 0, true, 1, true, false, End>,
            RoleC<RoleEnd>,
            NameS,
        >,
    ),
}
type TwoRoleC = RoleC<RoleC<RoleEnd>>;

// Creating the MP sessions
// A
type EndpointA = MeshedChannels<
    SendTimed<
        i32,
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<Branching0fromCtoA, 'a', 0, true, 1, true, false, End>,
    >,
    SendTimed<i32, 'a', 0, true, 1, true, false, RecvTimed<i32, 'a', 0, true, 1, true, false, End>>,
    RoleS<RoleS<TwoRoleC>>,
    NameA,
>;

// C
type EndpointC = MeshedChannels<
    RecvTimed<i32, 'a', 0, true, 1, true, false, Choose0fromCtoA>,
    RecvTimed<i32, 'a', 0, true, 1, true, false, Choose0fromCtoS>,
    RoleS<RoleA<RoleBroadcast>>,
    NameC,
>;
type EndpointCAccept = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 1, true, false, End>,
    SendTimed<i32, 'a', 0, true, 1, true, false, RecvTimed<i32, 'a', 0, true, 1, true, false, End>>,
    RoleA<RoleS<RoleS<RoleEnd>>>,
    NameC,
>;
type EndpointCQuit = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 1, true, false, End>,
    SendTimed<i32, 'a', 0, true, 1, true, false, End>,
    RoleS<RoleA<RoleEnd>>,
    NameC,
>;

// S
type EndpointS = MeshedChannels<
    RecvTimed<i32, 'a', 0, true, 1, true, false, SendTimed<i32, 'a', 0, true, 1, true, false, End>>,
    SendTimed<
        i32,
        'a',
        0,
        true,
        1,
        true,
        false,
        RecvTimed<Branching0fromCtoS, 'a', 0, true, 1, true, false, End>,
    >,
    RoleA<RoleA<RoleC<RoleC<RoleEnd>>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(random(), all_clocks)?;
    let (_empty2, s) = s.recv(all_clocks)?;
    let s = s.send(random(), all_clocks)?;
    offer_mpst!(s, all_clocks, {
        Branching0fromCtoA::Accept(s) => {
            let (_ok, s) = s.recv(all_clocks)?;
            s.close()
        },
        Branching0fromCtoA::Quit(s) => {
            let (_ok, s) = s.recv(all_clocks)?;
            s.close()
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_empty3, s) = s.recv(all_clocks)?;
    let (_empty4, s) = s.recv(all_clocks)?;

    let choice: i32 = thread_rng().gen_range(1..=3);

    if choice != 1 {
        let s: EndpointCAccept = choose_mpst_c_to_all!(
            s,
            all_clocks,
            Branching0fromCtoA::Accept,
            Branching0fromCtoS::Accept,
        );

        let s = s.send(random(), all_clocks)?;
        let s = s.send(random(), all_clocks)?;
        let (_empty5, s) = s.recv(all_clocks)?;

        s.close()
    } else {
        let s: EndpointCQuit = choose_mpst_c_to_all!(
            s,
            all_clocks,
            Branching0fromCtoA::Quit,
            Branching0fromCtoS::Quit,
        );
        let s = s.send(random(), all_clocks)?;
        let s = s.send(random(), all_clocks)?;
        s.close()
    }
}

fn endpoint_s(s: EndpointS, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_empty1, s) = s.recv(all_clocks)?;
    let s = s.send(random(), all_clocks)?;
    let s = s.send(random(), all_clocks)?;
    offer_mpst!(s, all_clocks, {
        Branching0fromCtoS::Accept(s) => {
            let (_ok, s) = s.recv(all_clocks)?;
            let s = s.send(random(), all_clocks)?;
            s.close()
        },
        Branching0fromCtoS::Quit(s) => {
            let (_ok, s) = s.recv(all_clocks)?;
            s.close()
        },
    })
}

/////////////////////////////////////////

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

pub fn three_buyers_main(c: &mut Criterion) {
    c.bench_function("Timed Three buyers MPST baking", |b| b.iter(all_mpst));
}
