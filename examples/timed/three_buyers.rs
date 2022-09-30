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
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannels, 3);
baker_timed!(MeshedChannels, A, C, S);

// Types
// A
type Choose0fromCtoA = SendTimed<Branching0fromCtoA, End, 'a', 0, true, 1, true, false>;
type Choose0fromCtoS = SendTimed<Branching0fromCtoS, End, 'a', 0, true, 1, true, false>;

// A
enum Branching0fromCtoA {
    Accept(
        MeshedChannels<
            RecvTimed<i32, End, 'a', 0, true, 1, true, false>,
            End,
            RoleC<RoleEnd>,
            NameA,
        >,
    ),
    Quit(
        MeshedChannels<
            RecvTimed<i32, End, 'a', 0, true, 1, true, false>,
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
            RecvTimed<i32, SendTimed<i32, End, 'a', 0, true, 1, true, false>>,
            TwoRoleC,
            NameS,
        >,
    ),
    Quit(
        MeshedChannels<
            End,
            RecvTimed<i32, End, 'a', 0, true, 1, true, false>,
            RoleC<RoleEnd>,
            NameS,
        >,
    ),
}
type TwoRoleC = RoleC<RoleC<RoleEnd>>;

// Creating the MP sessions
// A
type EndpointA = MeshedChannels<
    SendTimed<i32, RecvTimed<Branching0fromCtoA, End, 'a', 0, true, 1, true, false>>,
    SendTimed<i32, RecvTimed<i32, End, 'a', 0, true, 1, true, false>>,
    RoleS<RoleS<TwoRoleC>>,
    NameA,
>;

// C
type EndpointC = MeshedChannels<
    RecvTimed<i32, Choose0fromCtoA, 'a', 0, true, 1, true, false>,
    RecvTimed<i32, Choose0fromCtoS, 'a', 0, true, 1, true, false>,
    RoleS<RoleA<RoleBroadcast>>,
    NameC,
>;

// S
type EndpointS = MeshedChannels<
    RecvTimed<i32, SendTimed<i32, End, 'a', 0, true, 1, true, false>, 'a', 0, true, 1, true, false>,
    SendTimed<
        i32,
        RecvTimed<Branching0fromCtoS, End, 'a', 0, true, 1, true, false>,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    RoleA<RoleA<RoleC<RoleC<RoleEnd>>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send((), all_clocks)?;
    let (_empty2, s) = s.recv(all_clocks)?;
    let s = s.send((), all_clocks)?;
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
        let s = choose_mpst_multi_to_all!(
            s,
            all_clocks,
            Branching0fromCtoA::Accept,
            Branching0fromCtoS::Accept,
        );

        let s = s.send((), all_clocks)?;
        let s = s.send((), all_clocks)?;
        let (_empty5, s) = s.recv(all_clocks)?;

        s.close()
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            all_clocks,
            Branching0fromCtoA::Quit,
            Branching0fromCtoS::Quit,
        );
        let s = s.send((), all_clocks)?;
        let s = s.send((), all_clocks)?;
        s.close()
    }
}

fn endpoint_s(s: EndpointS, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_empty1, s) = s.recv(all_clocks)?;
    let s = s.send((), all_clocks)?;
    let s = s.send((), all_clocks)?;
    offer_mpst!(s, all_clocks, {
        Branching0fromCtoS::Accept(s) => {
            let (_ok, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            s.close()
        },
        Branching0fromCtoS::Quit(s) => {
            let (_ok, s) = s.recv(all_clocks)?;
            s.close()
        },
    })
}

/////////////////////////////////////////

fn main() {
    let (thread_a, thread_c, thread_s) = fork_mpst(endpoint_a, endpoint_c, endpoint_s);

    assert!(thread_a.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_s.join().is_ok());
}
