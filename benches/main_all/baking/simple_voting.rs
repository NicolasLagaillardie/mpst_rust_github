use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::bundle_impl_with_enum_and_cancel;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
bundle_impl_with_enum_and_cancel!(MeshedChannelsTwo, Voter, Server);

// Names
type NameServer = RoleServer<RoleEnd>;
type NameVoter = RoleVoter<RoleEnd>;

// Types
// SERVER
type Choose0fromStoV = Send<Branching0fromStoV, End>;
// VOTER
type Choose1fromVtoS = <Choice1fromStoV as Session>::Dual;

// VOTER
enum Branching0fromStoV {
    Auth(MeshedChannelsTwo<Recv<i32, Choose1fromVtoS>, RoleServer<RoleBroadcast>, NameVoter>),
    Reject(MeshedChannelsTwo<Recv<i32, End>, RoleServer<RoleEnd>, NameVoter>),
}
// SERVER
enum Branching1fromVtoS {
    Yes(MeshedChannelsTwo<Recv<i32, End>, RoleVoter<RoleEnd>, NameServer>),
    No(MeshedChannelsTwo<Recv<i32, End>, RoleVoter<RoleEnd>, NameServer>),
}
type Choice1fromStoV = Recv<Branching1fromVtoS, End>;

// Creating the MP sessions
// VOTER
type ChoiceVoter =
    MeshedChannelsTwo<Recv<i32, Choose1fromVtoS>, RoleServer<RoleBroadcast>, NameVoter>;
type EndpointVoter = MeshedChannelsTwo<
    Send<i32, Recv<Branching0fromStoV, End>>,
    RoleServer<RoleServer<RoleEnd>>,
    NameVoter,
>;
// SERVER
type ChoiceServer = MeshedChannelsTwo<Choice1fromStoV, RoleVoter<RoleEnd>, NameServer>;
type EndpointServer =
    MeshedChannelsTwo<Recv<i32, Choose0fromStoV>, RoleVoter<RoleBroadcast>, NameServer>;

// Functions
fn endpoint_voter(s: EndpointVoter) -> Result<(), Box<dyn Error>> {
    let auth = thread_rng().gen_range(1..=2);

    let s = s.send(auth)?;

    offer_mpst!(s, {
        Branching0fromStoV::Reject(s) => {

            let (_, s) = s.recv()?;

            s.close()
        },
        Branching0fromStoV::Auth(s) => {
            choice_voter(s)
        },
    })
}

fn choice_voter(s: ChoiceVoter) -> Result<(), Box<dyn Error>> {
    let (ok, s) = s.recv()?;

    let expected = thread_rng().gen_range(1..=2);

    if ok == expected {
        let s = choose_mpst_voter_to_all!(s, Branching1fromVtoS::Yes);

        let s = s.send(1)?;

        s.close()
    } else {
        let s = choose_mpst_voter_to_all!(s, Branching1fromVtoS::No);

        let s = s.send(0)?;

        s.close()
    }
}

fn endpoint_server(s: EndpointServer) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..=2);

    let (auth, s) = s.recv()?;

    if choice == auth {
        let s = choose_mpst_server_to_all!(s, Branching0fromStoV::Reject);

        let s = s.send(0)?;

        s.close()
    } else {
        let s = choose_mpst_server_to_all!(s, Branching0fromStoV::Auth);

        let s = s.send(1)?;

        choice_server(s)
    }
}

fn choice_server(s: ChoiceServer) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromVtoS::Yes(s) => {

            let (_answer, s) = s.recv()?;

            s.close()
        },
        Branching1fromVtoS::No(s) => {

            let (_answer, s) = s.recv()?;

            s.close()
        },
    })
}

fn all_mpst() {
    let (thread_server, thread_voter) =
        fork_mpst(black_box(endpoint_server), black_box(endpoint_voter));

    thread_voter.join().unwrap();
    thread_server.join().unwrap();
}

/////////////////////////

fn simple_voting_mpst(c: &mut Criterion) {
    c.bench_function("Simple voting MPST baking", |b| b.iter(all_mpst));
}

criterion_group! {
    name = simple_voting;
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = simple_voting_mpst,
}

criterion_main!(simple_voting);
