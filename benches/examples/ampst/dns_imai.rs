#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::random;

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
generate!("rec_and_cancel", MeshedChannels, Client, Other, Server);

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
type EndpointServerQuery = MeshedChannels<
    Send<(i32, i32), End>,
    Send<(i32, i32), Recv<(i32, i32), End>>,
    RoleOther<RoleOther<RoleClient<RoleEnd>>>,
    NameServer,
>;
type EndpointServerDummy = MeshedChannels<
    Send<(i32, i32), End>,
    Send<(), End>,
    RoleOther<RoleClient<RoleEnd>>,
    NameServer,
>;

// Functions
fn endpoint_client(s: EndpointClient) -> Result<(), Box<dyn Error>> {
    let address = random::<i32>();

    let s = s.send(address)?;

    offer_mpst!(s, {
        Branching0fromServerToClient::Dummy(s) => {

            let (_, s) = s.recv()?;

            s.close()
        },
        Branching0fromServerToClient::Query(s) => {

            let ((_new_address, _new_packet), s) = s.recv()?;

            s.close()
        },
    })
}

fn endpoint_other(s: EndpointOther) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromServerToOther::Dummy(s) => {
            let (_, s) = s.recv()?;
            s.close()
        },
        Branching0fromServerToOther::Query(s) => {
            let ((address, packet), s) = s.recv()?;

            let s = s.send((-address, -packet))?;

            s.close()
        },
    })
}

fn endpoint_server(s: EndpointServer) -> Result<(), Box<dyn Error>> {
    endpoint_server_recurs(s, 100)
}

fn endpoint_server_recurs(s: EndpointServer, loops: i32) -> Result<(), Box<dyn Error>> {
    let (address, s) = s.recv()?;

    match loops {
        0 => {
            let s: EndpointServerQuery = choose_mpst_server_to_all!(
                s,
                Branching0fromServerToClient::Query,
                Branching0fromServerToOther::Query
            );

            let packet = random::<i32>();

            let s = s.send((address, packet))?;

            let ((new_address, new_packet), s) = s.recv()?;

            let s = s.send((new_address, new_packet))?;

            s.close()
        }
        _ => {
            let s: EndpointServerDummy = choose_mpst_server_to_all!(
                s,
                Branching0fromServerToClient::Dummy,
                Branching0fromServerToOther::Dummy
            );

            let s = s.send(())?;

            let packet = random::<i32>();

            let s = s.send((address, packet))?;

            s.close()
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
    c.bench_function("DNS Imai AMPST", |b| b.iter(aux));
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
