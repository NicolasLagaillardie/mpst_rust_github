#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
baker_timed!(MeshedChannels, A, C, S);

// Types
// A
type Choose0fromCtoA = SendTimed<Branching0fromCtoA, End, 'a', 0, true, 1, true, false>;
type Choose0fromCtoS = SendTimed<Branching0fromCtoS, End, 'a', 0, true, 1, true, false>;

// A
enum Branching0fromCtoA {
    Sum(MeshedChannels<End, End, RoleEnd, NameA>),
    Diff(MeshedChannels<End, End, RoleEnd, NameA>),
}
// S
enum Branching0fromCtoS {
    Sum(
        MeshedChannels<
            End,
            SendTimed<i32, End, 'a', 0, true, 1, true, false>,
            RoleC<RoleEnd>,
            NameS,
        >,
    ),
    Diff(
        MeshedChannels<
            End,
            SendTimed<i32, End, 'a', 0, true, 1, true, false>,
            RoleC<RoleEnd>,
            NameS,
        >,
    ),
}

// Creating the MP sessions
// A
type EndpointA = MeshedChannels<
    RecvTimed<Branching0fromCtoA, End, 'a', 0, true, 1, true, false>,
    End,
    RoleC<RoleEnd>,
    NameA,
>;
// C
type EndpointC = MeshedChannels<
    Choose0fromCtoA,
    SendTimed<
        i32,
        SendTimed<i32, Choose0fromCtoS, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    RoleS<RoleS<RoleBroadcast>>,
    NameC,
>;
// S
type EndpointS = MeshedChannels<
    End,
    RecvTimed<
        i32,
        RecvTimed<
            i32,
            RecvTimed<Branching0fromCtoS, End, 'a', 0, true, 1, true, false>,
            'a',
            0,
            true,
            1,
            true,
            false,
        >,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    RoleC<RoleC<RoleC<RoleEnd>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    offer_mpst!(s, all_clocks, {
        Branching0fromCtoA::Sum(s) => {
            s.close()
        },
        Branching0fromCtoA::Diff(s) => {
            s.close()
        },
    })
}

fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let elt_1: i32 = thread_rng().gen_range(1..=100);
    let elt_2: i32 = thread_rng().gen_range(1..=100);
    let s = s.send(elt_1, all_clocks)?;
    let s = s.send(elt_2, all_clocks)?;

    let choice: i32 = thread_rng().gen_range(1..=2);

    if choice != 1 {
        let s = choose_mpst_c_to_all!(
            s,
            all_clocks,
            Branching0fromCtoA::Sum,
            Branching0fromCtoS::Sum,
        );

        let (_sum, s) = s.recv(all_clocks)?;

        s.close()
    } else {
        let s = choose_mpst_c_to_all!(
            s,
            all_clocks,
            Branching0fromCtoA::Diff,
            Branching0fromCtoS::Diff,
        );

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
            let s = s.send(elt_1 + elt_2,all_clocks)?;
            s.close()
        },
        Branching0fromCtoS::Diff(s) => {
            let s = s.send(elt_1 - elt_2, all_clocks)?;
            s.close()
        },
    })
}

fn main() {
    let (thread_a, thread_c, thread_s) = fork_mpst(endpoint_a, endpoint_c, endpoint_s);

    assert!(thread_a.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_s.join().is_ok());
}