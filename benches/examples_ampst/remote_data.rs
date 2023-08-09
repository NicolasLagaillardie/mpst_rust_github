#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    dead_code
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::random;

use std::error::Error;

// Create the new MeshedChannels for three participants and the close and fork functions
generate!("rec_and_cancel", MeshedChannels, Satellite, Sensor, Server);

// Labels
struct Stop {}
struct GetData {}
struct Data {
    payload: i32,
}

// Types
// Satellite
type OfferFromServertoSatellite = Recv<BranchingFromServertoSatellite, End>;

type BinaryDataFromSensortoSatellite = Send<GetData, Recv<Data, End>>;
type BinaryDataFromServertoSatellite = Recv<GetData, Send<Data, OfferFromServertoSatellite>>;

type BinaryStopFromSensortoSatellite = Send<Stop, End>;
type BinaryStopFromServertoSatellite = Recv<Stop, End>;

enum BranchingFromServertoSatellite {
    Data(
        MeshedChannels<
            BinaryDataFromSensortoSatellite,
            BinaryDataFromServertoSatellite,
            RoleServer<RoleSensor<RoleSensor<RoleServer<RoleServer<RoleEnd>>>>>,
            NameSatellite,
        >,
    ),
    Stop(
        MeshedChannels<
            BinaryStopFromSensortoSatellite,
            BinaryStopFromServertoSatellite,
            RoleServer<RoleSensor<RoleEnd>>,
            NameSatellite,
        >,
    ),
}

// Sensor
type OfferFromServertoSensor = Recv<BranchingFromServertoSensor, End>;

type BinaryDataFromSatellitetoSensor = <BinaryDataFromSensortoSatellite as Session>::Dual;

type BinaryStopFromSatellitetoSensor = <BinaryStopFromSensortoSatellite as Session>::Dual;

enum BranchingFromServertoSensor {
    Data(
        MeshedChannels<
            BinaryDataFromSatellitetoSensor,
            OfferFromServertoSensor,
            RoleSatellite<RoleSatellite<RoleServer<RoleEnd>>>,
            NameSensor,
        >,
    ),
    Stop(MeshedChannels<BinaryStopFromSatellitetoSensor, End, RoleSatellite<RoleEnd>, NameSensor>),
}

// Server
type ChooseFromServertoSatellite = <OfferFromServertoSatellite as Session>::Dual;
type ChooseFromServertoSensor = <OfferFromServertoSensor as Session>::Dual;

type BinaryDataFromSatellitetoServer = <BinaryDataFromServertoSatellite as Session>::Dual;

type BinaryStopFromSatellitetoServer = <BinaryStopFromServertoSatellite as Session>::Dual;

// Endpoints
// Satellite
type EndpointSatellite0 =
    MeshedChannels<End, OfferFromServertoSatellite, RoleServer<RoleEnd>, NameSatellite>;

// Sensor
type EndpointSensor0 =
    MeshedChannels<End, OfferFromServertoSensor, RoleServer<RoleEnd>, NameSensor>;

// Server
type EndpointServer0 = MeshedChannels<
    ChooseFromServertoSatellite,
    ChooseFromServertoSensor,
    RoleBroadcast,
    NameServer,
>;
type EndpointServer1Data = MeshedChannels<
    BinaryDataFromSatellitetoServer,
    ChooseFromServertoSensor,
    RoleSatellite<RoleSatellite<RoleBroadcast>>,
    NameServer,
>;
type EndpointServer1Stop =
    MeshedChannels<BinaryStopFromSatellitetoServer, End, RoleSatellite<RoleEnd>, NameServer>;

// Functions

/////////////////////////

// Satellite
fn endpoint_satellite(s: EndpointSatellite0) -> Result<(), Box<dyn Error>> {
    recurs_satellite(s)
}

fn recurs_satellite(s: EndpointSatellite0) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        BranchingFromServertoSatellite::Stop(s) => {
            let (stop, s) = s.recv()?;
            let s = s.send(stop)?;

            s.close()
        },
        BranchingFromServertoSatellite::Data(s) => {
            // Forward get_data signal
            let (get_data, s) = s.recv()?;
            let s = s.send(get_data)?;

            // Forward the data
            let (data, s) = s.recv()?;
            let s = s.send(data)?;

            recurs_satellite(s)
        },
    })
}

/////////////////////////

// Sensor
fn endpoint_sensor(s: EndpointSensor0) -> Result<(), Box<dyn Error>> {
    recurs_sensor(s)
}

fn recurs_sensor(s: EndpointSensor0) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        BranchingFromServertoSensor::Stop(s) => {
            let (_stop, s) = s.recv()?;

            s.close()
        },
        BranchingFromServertoSensor::Data(s) => {
            // Get get_data signal
            let (_get_data, s) = s.recv()?;

            // Send the data
            let s = s.send(Data { payload : random::<i32>() })?;

            recurs_sensor(s)
        },
    })
}

/////////////////////////

// Server
fn endpoint_server(s: EndpointServer0) -> Result<(), Box<dyn Error>> {
    recurs_server(s, LOOPS)
}

fn recurs_server(s: EndpointServer0, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s: EndpointServer1Stop = choose_mpst_server_to_all!(
                s,
                BranchingFromServertoSatellite::Stop,
                BranchingFromServertoSensor::Stop,
            );

            let s = s.send(Stop {})?;

            s.close()
        }
        i => {
            let s: EndpointServer1Data = choose_mpst_server_to_all!(
                s,
                BranchingFromServertoSatellite::Data,
                BranchingFromServertoSensor::Data,
            );

            let s = s.send(GetData {})?;

            let (_data, s) = s.recv()?;

            recurs_server(s, i - 1)
        }
    }
}

/////////////////////////

fn aux() {
    let (thread_satellite, thread_sensor, thread_server) = fork_mpst(
        black_box(endpoint_satellite),
        black_box(endpoint_sensor),
        black_box(endpoint_server),
    );

    thread_satellite.join().unwrap();
    thread_sensor.join().unwrap();
    thread_server.join().unwrap();
}

/////////////////////////

static LOOPS: i32 = 100;

pub fn remote_data(c: &mut Criterion) {
    c.bench_function("Remote data AMPST", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(10000);
    targets = remote_data,
}

criterion_main! {
    bench
}
