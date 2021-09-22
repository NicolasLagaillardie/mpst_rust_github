use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_create_multi_to_all,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer_mpst,
};

use rand::{random, thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new Roles
// normal
create_multiple_normal_role_short!(Client, Other, Server);

// Create new send functions
// CLIENT
create_send_mpst_session_bundle!(
    send_mpst_client_to_server, RoleServer, 2 | =>
    RoleClient, MeshedChannelsThree, 3
);
// OTHER
create_send_mpst_session_bundle!(
    send_mpst_other_to_server, RoleServer, 2 | =>
    RoleOther, MeshedChannelsThree, 3
);
// SERVER
create_send_mpst_session_bundle!(
    send_mpst_server_to_client, RoleClient, 1 |
    send_mpst_server_to_other, RoleOther, 2 | =>
    RoleServer, MeshedChannelsThree, 3
);

// Create new recv functions and related types
// CLIENT
create_recv_mpst_session_bundle!(
    recv_mpst_client_from_server, RoleServer, 2 | =>
    RoleClient, MeshedChannelsThree, 3
);
// OTHER
create_recv_mpst_session_bundle!(
    recv_mpst_other_from_server, RoleServer, 2 | =>
    RoleOther, MeshedChannelsThree, 3
);
// SERVER
create_recv_mpst_session_bundle!(
    recv_mpst_server_from_client, RoleClient, 1 |
    recv_mpst_server_from_other, RoleOther, 2 | =>
    RoleServer, MeshedChannelsThree, 3
);

// Names
type NameClient = RoleClient<RoleEnd>;
type NameOther = RoleOther<RoleEnd>;
type NameServer = RoleServer<RoleEnd>;

// Types
// SERVER
type Choose0fromServerToClient = Send<Branching0fromServerToClient, End>;
type Choose0fromServerToOther = Send<Branching0fromServerToOther, End>;
// CLIENT
enum Branching0fromServerToClient {
    Dummy(MeshedChannelsThree<End, Recv<(i32, i32), End>, RoleServer<RoleEnd>, NameClient>),
    Query(MeshedChannelsThree<End, Recv<(i32, i32), End>, RoleServer<RoleEnd>, NameClient>),
}
// OTHER
enum Branching0fromServerToOther {
    Dummy(MeshedChannelsThree<End, Recv<(), End>, RoleServer<RoleEnd>, NameOther>),
    Query(
        MeshedChannelsThree<
            End,
            Recv<(i32, i32), Send<(i32, i32), End>>,
            RoleServer<RoleServer<RoleEnd>>,
            NameOther,
        >,
    ),
}

// Creating the MP sessions
// CLIENT
type EndpointClient = MeshedChannelsThree<
    End,
    Send<i32, Recv<Branching0fromServerToClient, End>>,
    RoleServer<RoleServer<RoleEnd>>,
    NameClient,
>;
// OTHER
type EndpointOther = MeshedChannelsThree<
    End,
    Recv<Branching0fromServerToOther, End>,
    RoleServer<RoleEnd>,
    NameOther,
>;
// SERVER
type EndpointServer = MeshedChannelsThree<
    Recv<i32, Choose0fromServerToClient>,
    Choose0fromServerToOther,
    RoleClient<RoleBroadcast>,
    NameServer,
>;

choose_mpst_create_multi_to_all!(
    choose_mpst_server_to_all,
    RoleClient,
    RoleOther, =>
    RoleServer,
    MeshedChannelsThree,
    3
);

// Functions
fn endpoint_client(s: EndpointClient) -> Result<(), Box<dyn Error>> {
    let address = random::<i32>();

    let s = send_mpst_client_to_server(address, s);

    offer_mpst!(s, recv_mpst_client_from_server, {
        Branching0fromServerToClient::Dummy(s) => {

            let (_, s) = recv_mpst_client_from_server(s)?;

            close_mpst_multi(s)
        },
        Branching0fromServerToClient::Query(s) => {

            let ((new_address, _new_packet), s) = recv_mpst_client_from_server(s)?;

            assert_eq!(new_address, -address);

            close_mpst_multi(s)
        },
    })
}

fn endpoint_other(s: EndpointOther) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_other_from_server, {
        Branching0fromServerToOther::Dummy(s) => {
            let (_, s) = recv_mpst_other_from_server(s)?;
            close_mpst_multi(s)
        },
        Branching0fromServerToOther::Query(s) => {
            let ((address, packet), s) = recv_mpst_other_from_server(s)?;

            let s = send_mpst_other_to_server((-address, -packet), s);

            close_mpst_multi(s)
        },
    })
}

fn endpoint_server(s: EndpointServer) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..3);

    let (address, s) = recv_mpst_server_from_client(s)?;

    if choice == 1 {
        let s = choose_mpst_server_to_all!(
            s,
            Branching0fromServerToClient::Dummy,
            Branching0fromServerToOther::Dummy
        );

        let s = send_mpst_server_to_other((), s);

        let packet = random::<i32>();

        let s = send_mpst_server_to_client((address, packet), s);

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_server_to_all!(
            s,
            Branching0fromServerToClient::Query,
            Branching0fromServerToOther::Query
        );

        let packet = random::<i32>();

        let s = send_mpst_server_to_other((address, packet), s);

        let ((new_address, new_packet), s) = recv_mpst_server_from_other(s)?;

        let s = send_mpst_server_to_client((new_address, new_packet), s);

        close_mpst_multi(s)
    }
}

fn main() {
    let (thread_other, thread_server, thread_client) =
        fork_mpst(endpoint_client, endpoint_other, endpoint_server);

    assert!(thread_client.join().is_ok());
    assert!(thread_server.join().is_ok());
    assert!(thread_other.join().is_ok());
}
