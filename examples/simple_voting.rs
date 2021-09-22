use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::marker;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsTwo, 2);

// Create new Roles
// normal
create_multiple_normal_role_short!(Voter, Server);

// Create new send functions
// SERVER
create_send_mpst_session_bundle!(
    send_mpst_server_to_voter, RoleVoter, 1 | =>
    RoleServer, MeshedChannelsTwo, 2
);
// VOTER
create_send_mpst_session_bundle!(
    send_mpst_voter_to_server, RoleServer, 1 | =>
    RoleVoter, MeshedChannelsTwo, 2
);

// Create new recv functions and related types
// SERVER
create_recv_mpst_session_bundle!(
    recv_mpst_server_to_voter, RoleVoter, 1 | =>
    RoleServer, MeshedChannelsTwo, 2
);
// VOTER
create_recv_mpst_session_bundle!(
    recv_mpst_voter_to_server, RoleServer, 1 | =>
    RoleVoter, MeshedChannelsTwo, 2
);

// Names
type NameServer = RoleServer<RoleEnd>;
type NameVoter = RoleVoter<RoleEnd>;

// Types
// SERVER
type Choose0fromStoV<N> = Send<Branching0fromStoV<N>, End>;
// VOTER
type Choose1fromVtoS<N> = <Choice1fromStoV<N> as Session>::Dual;

// VOTER
enum Branching0fromStoV<N: marker::Send> {
    Auth(MeshedChannelsTwo<Recv<N, Choose1fromVtoS<N>>, RoleServer<RoleBroadcast>, NameVoter>),
    Reject(MeshedChannelsTwo<Recv<N, End>, RoleServer<RoleEnd>, NameVoter>),
}
// SERVER
enum Branching1fromVtoS<N: marker::Send> {
    Yes(MeshedChannelsTwo<Recv<N, End>, RoleVoter<RoleEnd>, NameServer>),
    No(MeshedChannelsTwo<Recv<N, End>, RoleVoter<RoleEnd>, NameServer>),
}
type Choice1fromStoV<N> = Recv<Branching1fromVtoS<N>, End>;

// Creating the MP sessions
// VOTER
type ChoiceVoter<N> =
    MeshedChannelsTwo<Recv<N, Choose1fromVtoS<N>>, RoleServer<RoleBroadcast>, NameVoter>;
type EndpointVoter<N> = MeshedChannelsTwo<
    Send<N, Recv<Branching0fromStoV<N>, End>>,
    RoleServer<RoleServer<RoleEnd>>,
    NameVoter,
>;
// SERVER
type ChoiceServer<N> = MeshedChannelsTwo<Choice1fromStoV<N>, RoleVoter<RoleEnd>, NameServer>;
type EndpointServer<N> =
    MeshedChannelsTwo<Recv<N, Choose0fromStoV<N>>, RoleVoter<RoleBroadcast>, NameServer>;

// Functions
fn endpoint_voter(s: EndpointVoter<i32>) -> Result<(), Box<dyn Error>> {
    let auth = thread_rng().gen_range(1..=2);

    let s = send_mpst_voter_to_server(auth, s);

    offer_mpst!(s, recv_mpst_voter_to_server, {
        Branching0fromStoV::Reject(s) => {

            let (_, s) = recv_mpst_voter_to_server(s)?;

            close_mpst_multi(s)
        },
        Branching0fromStoV::Auth(s) => {
            choice_voter(s)
        },
    })
}

fn choice_voter(s: ChoiceVoter<i32>) -> Result<(), Box<dyn Error>> {
    let (ok, s) = recv_mpst_voter_to_server(s)?;

    let expected = thread_rng().gen_range(1..=2);

    if ok == expected {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching1fromVtoS::<i32>::Yes, =>
            RoleServer, =>
            RoleVoter,
            MeshedChannelsTwo,
            2
        );

        let s = send_mpst_voter_to_server(1, s);

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching1fromVtoS::<i32>::No, =>
            RoleServer, =>
            RoleVoter,
            MeshedChannelsTwo,
            2
        );

        let s = send_mpst_voter_to_server(0, s);

        close_mpst_multi(s)
    }
}

fn endpoint_server(s: EndpointServer<i32>) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..=2);

    let (auth, s) = recv_mpst_server_to_voter(s)?;

    if choice == auth {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromStoV::<i32>::Reject, =>
            RoleVoter, =>
            RoleServer,
            MeshedChannelsTwo,
            2
        );

        let s = send_mpst_server_to_voter(0, s);

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromStoV::<i32>::Auth, =>
            RoleVoter, =>
            RoleServer,
            MeshedChannelsTwo,
            2
        );

        let s = send_mpst_server_to_voter(1, s);

        choice_server(s)
    }
}

fn choice_server(s: ChoiceServer<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_server_to_voter, {
        Branching1fromVtoS::<i32>::Yes(s) => {

            let (answer, s) = recv_mpst_server_to_voter(s)?;

            assert_eq!(answer, 1);

            close_mpst_multi(s)
        },
        Branching1fromVtoS::<i32>::No(s) => {

            let (answer, s) = recv_mpst_server_to_voter(s)?;

            assert_eq!(answer, 0);

            close_mpst_multi(s)
        },
    })
}

fn main() {
    let (thread_server, thread_voter) = fork_mpst(endpoint_server, endpoint_voter);

    assert!(thread_voter.join().is_ok());
    assert!(thread_server.join().is_ok());
}
