#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
generate!("recursive", MeshedChannels, Voter, Server);

// Types
// SERVER
type Choose0fromStoV = Send<Branching0fromStoV, End>;

// VOTER
type Choose1fromVtoS = <Choice1fromStoV as Session>::Dual;

// VOTER
enum Branching0fromStoV {
    Auth(MeshedChannels<Recv<i32, Choose1fromVtoS>, RoleServer<RoleBroadcast>, NameVoter>),
    Reject(MeshedChannels<Recv<i32, End>, RoleServer<RoleEnd>, NameVoter>),
}

// SERVER
enum Branching1fromVtoS {
    Yes(MeshedChannels<Recv<i32, End>, RoleVoter<RoleEnd>, NameServer>),
    No(MeshedChannels<Recv<i32, End>, RoleVoter<RoleEnd>, NameServer>),
}
type Choice1fromStoV = Recv<Branching1fromVtoS, End>;

// Creating the MP sessions
// VOTER
type ChoiceVoter = MeshedChannels<Recv<i32, Choose1fromVtoS>, RoleServer<RoleBroadcast>, NameVoter>;
type EndpointVoter = MeshedChannels<
    Send<i32, Recv<Branching0fromStoV, End>>,
    RoleServer<RoleServer<RoleEnd>>,
    NameVoter,
>;

// SERVER
type ChoiceServer = MeshedChannels<Choice1fromStoV, RoleVoter<RoleEnd>, NameServer>;
type EndpointServer =
    MeshedChannels<Recv<i32, Choose0fromStoV>, RoleVoter<RoleBroadcast>, NameServer>;

// Functions
fn endpoint_voter(s: EndpointVoter) -> Result<(), Box<dyn Error>> {
    let auth: i32 = thread_rng().gen_range(1..=2);

    let s = s.send(auth);

    offer_mpst!(s, {
        Branching0fromStoV::Reject(s) => {

            let (_, s) = s.recv();

            s.close()
        },
        Branching0fromStoV::Auth(s) => {
            choice_voter(s)
        },
    })
}

fn choice_voter(s: ChoiceVoter) -> Result<(), Box<dyn Error>> {
    let (ok, s) = s.recv();

    let expected: i32 = thread_rng().gen_range(1..=2);

    if ok == expected {
        let s = choose_mpst_voter_to_all!(s, Branching1fromVtoS::Yes);

        let s = s.send(1);

        s.close()
    } else {
        let s = choose_mpst_voter_to_all!(s, Branching1fromVtoS::No);

        let s = s.send(0);

        s.close()
    }
}

fn endpoint_server(s: EndpointServer) -> Result<(), Box<dyn Error>> {
    let choice: i32 = thread_rng().gen_range(1..=2);

    let (auth, s) = s.recv();

    if choice == auth {
        let s = choose_mpst_server_to_all!(s, Branching0fromStoV::Reject);

        let s = s.send(0);

        s.close()
    } else {
        let s = choose_mpst_server_to_all!(s, Branching0fromStoV::Auth);

        let s = s.send(1);

        choice_server(s)
    }
}

fn choice_server(s: ChoiceServer) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromVtoS::Yes(s) => {

            let (_answer, s) = s.recv();

            s.close()
        },
        Branching1fromVtoS::No(s) => {

            let (_answer, s) = s.recv();

            s.close()
        },
    })
}

fn main() {
    let (thread_server, thread_voter) = fork_mpst(endpoint_server, endpoint_voter);

    thread_voter.join().unwrap();
    thread_server.join().unwrap();
}
