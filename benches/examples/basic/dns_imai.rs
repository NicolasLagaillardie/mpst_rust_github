#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_create_multi_to_all,
    create_multiple_normal_name_short, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use rand::random;

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannels, 3);

// Create new Roles
// normal
create_multiple_normal_role_short!(Client, Other, Server);

// Create new Names
create_multiple_normal_name_short!(Client, Other, Server);

// Create new send functions
// CLIENT
create_send_mpst_session_bundle!(
    send_mpst_client_to_server, RoleServer, 2 | =>
    NameClient, MeshedChannels, 3
);

// OTHER
create_send_mpst_session_bundle!(
    send_mpst_other_to_server, RoleServer, 2 | =>
    NameOther, MeshedChannels, 3
);

// SERVER
create_send_mpst_session_bundle!(
    send_mpst_server_to_client, RoleClient, 1 |
    send_mpst_server_to_other, RoleOther, 2 | =>
    NameServer, MeshedChannels, 3
);

// Create new recv functions and related types
// CLIENT
create_recv_mpst_session_bundle!(
    recv_mpst_client_from_server, RoleServer, 2 | =>
    NameClient, MeshedChannels, 3
);

// OTHER
create_recv_mpst_session_bundle!(
    recv_mpst_other_from_server, RoleServer, 2 | =>
    NameOther, MeshedChannels, 3
);

// SERVER
create_recv_mpst_session_bundle!(
    recv_mpst_server_from_client, RoleClient, 1 |
    recv_mpst_server_from_other, RoleOther, 2 | =>
    NameServer, MeshedChannels, 3
);

// Types
// SERVER
type Choose0fromServerToClient = Send<Branching0fromServerToClient, End>;
type Choose0fromServerToOther = Send<Branching0fromServerToOther, End>;

// CLIENT
enum Branching0fromServerToClient {
    Dummy(MeshedChannels<End, Recv<(i32, i32), End>, RoleServer<RoleEnd>, NameClient>),
    Query(MeshedChannels<End, Recv<(i32, i32), End>, RoleServer<RoleEnd>, NameClient>),
}

// OTHER
enum Branching0fromServerToOther {
    Dummy(MeshedChannels<End, Recv<(), End>, RoleServer<RoleEnd>, NameOther>),
    Query(
        MeshedChannels<
            End,
            Recv<(i32, i32), Send<(i32, i32), End>>,
            RoleServer<RoleServer<RoleEnd>>,
            NameOther,
        >,
    ),
}

// Creating the MP sessions
// CLIENT
type EndpointClient = MeshedChannels<
    End,
    Send<i32, Recv<Branching0fromServerToClient, End>>,
    RoleServer<RoleServer<RoleEnd>>,
    NameClient,
>;

// OTHER
type EndpointOther =
    MeshedChannels<End, Recv<Branching0fromServerToOther, End>, RoleServer<RoleEnd>, NameOther>;

// SERVER
type EndpointServer = MeshedChannels<
    Recv<i32, Choose0fromServerToClient>,
    Choose0fromServerToOther,
    RoleClient<RoleBroadcast>,
    NameServer,
>;

choose_mpst_create_multi_to_all!(choose_mpst_server_to_all, NameServer, MeshedChannels, 3);

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
    endpoint_server_recurs(s, 100)
}

fn endpoint_server_recurs(s: EndpointServer, loops: i32) -> Result<(), Box<dyn Error>> {
    let (address, s) = recv_mpst_server_from_client(s)?;

    match loops {
        0 => {
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
        _ => {
            let s = choose_mpst_server_to_all!(
                s,
                Branching0fromServerToClient::Dummy,
                Branching0fromServerToOther::Dummy
            );

            let s = send_mpst_server_to_other((), s);

            let packet = random::<i32>();

            let s = send_mpst_server_to_client((address, packet), s);

            close_mpst_multi(s)
        }
    }
}

fn aux() {
    let (thread_other, thread_server, thread_client) = fork_mpst(
        black_box(endpoint_client),
        black_box(endpoint_other),
        black_box(endpoint_server),
    );

    thread_client.join().unwrap();
    thread_server.join().unwrap();
    thread_other.join().unwrap();
}

/////////////////////////

pub fn dns_imai(c: &mut Criterion) {
    c.bench_function("DNS Imai basic", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = dns_imai,
}

criterion_main! {
    bench
}
