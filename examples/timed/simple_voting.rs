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

baker_timed!(MeshedChannels, Voter, Server);

// Types
// SERVER
type Choose0fromStoV = SendTimed<Branching0fromStoV, 'a', 0, true, 1, true, ' ', End>;

// VOTER
type Choose1fromVtoS = SendTimed<Branching1fromVtoS, 'a', 0, true, 1, true, ' ', End>;

// VOTER
enum Branching0fromStoV {
    Auth(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 1, true, ' ', Choose1fromVtoS>,
            RoleServer<RoleBroadcast>,
            NameVoter,
        >,
    ),
    Reject(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 1, true, ' ', End>,
            RoleServer<RoleEnd>,
            NameVoter,
        >,
    ),
}

// SERVER
enum Branching1fromVtoS {
    Yes(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 1, true, ' ', End>,
            RoleVoter<RoleEnd>,
            NameServer,
        >,
    ),
    No(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 1, true, ' ', End>,
            RoleVoter<RoleEnd>,
            NameServer,
        >,
    ),
}
type Choice1fromStoV = RecvTimed<Branching1fromVtoS, 'a', 0, true, 1, true, ' ', End>;

// Creating the MP sessions
// VOTER
type ChoiceVoter = MeshedChannels<
    RecvTimed<i32, 'a', 0, true, 1, true, ' ', Choose1fromVtoS>,
    RoleServer<RoleBroadcast>,
    NameVoter,
>;
type EndpointVoter = MeshedChannels<
    SendTimed<
        i32,
        'a',
        0,
        true,
        1,
        true,
        ' ',
        RecvTimed<Branching0fromStoV, 'a', 0, true, 1, true, ' ', End>,
    >,
    RoleServer<RoleServer<RoleEnd>>,
    NameVoter,
>;

// SERVER
type ChoiceServer = MeshedChannels<Choice1fromStoV, RoleVoter<RoleEnd>, NameServer>;
type EndpointServer = MeshedChannels<
    RecvTimed<i32, 'a', 0, true, 1, true, ' ', Choose0fromStoV>,
    RoleVoter<RoleBroadcast>,
    NameServer,
>;

// Functions
fn endpoint_voter(
    s: EndpointVoter,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    let auth: i32 = thread_rng().gen_range(1..=2);
    all_clocks.insert('a', Instant::now());

    let s = s.send(auth, all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Branching0fromStoV::Reject(s) => {

            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
        Branching0fromStoV::Auth(s) => {
            choice_voter(s, all_clocks)
        },
    })
}

fn choice_voter(
    s: ChoiceVoter,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    let (ok, s) = s.recv(all_clocks)?;

    let expected: i32 = thread_rng().gen_range(1..=2);

    if ok == expected {
        let s = choose_mpst_voter_to_all!(s, all_clocks, Branching1fromVtoS::Yes);

        let s = s.send(1, all_clocks)?;

        s.close()
    } else {
        let s = choose_mpst_voter_to_all!(s, all_clocks, Branching1fromVtoS::No);

        let s = s.send(0, all_clocks)?;

        s.close()
    }
}

fn endpoint_server(
    s: EndpointServer,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    let choice: i32 = thread_rng().gen_range(1..=2);
    all_clocks.insert('a', Instant::now());

    let (auth, s) = s.recv(all_clocks)?;

    if choice == auth {
        let s = choose_mpst_server_to_all!(s, all_clocks, Branching0fromStoV::Reject);

        let s = s.send(0, all_clocks)?;

        s.close()
    } else {
        let s = choose_mpst_server_to_all!(s, all_clocks, Branching0fromStoV::Auth);

        let s = s.send(1, all_clocks)?;

        choice_server(s, all_clocks)
    }
}

fn choice_server(
    s: ChoiceServer,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching1fromVtoS::Yes(s) => {

            let (_answer, s) = s.recv(all_clocks)?;

            s.close()
        },
        Branching1fromVtoS::No(s) => {

            let (_answer, s) = s.recv(all_clocks)?;

            s.close()
        },
    })
}

fn main() {
    let (thread_server, thread_voter) = fork_mpst(endpoint_server, endpoint_voter);

    assert!(thread_voter.join().is_ok());
    assert!(thread_server.join().is_ok());
}
