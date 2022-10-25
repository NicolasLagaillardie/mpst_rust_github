#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    dead_code
)]

use criterion::{black_box, Criterion};

use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::random;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// Create the new MeshedChannels for three participants and the close and fork functions
baker_timed!(MeshedChannels, Satellite, Sensor, Server);

// Labels
struct Stop {}
struct GetData {}
struct Data {
    payload: i32,
}

// Types
// Satellite
type OfferFromServertoSatellite =
    RecvTimed<BranchingFromServertoSatellite, End, 'a', 0, true, 1, true, true>;

type BinaryDataFromSensortoSatellite = SendTimed<
    GetData,
    RecvTimed<Data, End, 'a', 0, true, 1, true, false>,
    'a',
    0,
    true,
    1,
    true,
    false,
>;
type BinaryDataFromServertoSatellite = RecvTimed<
    GetData,
    SendTimed<Data, OfferFromServertoSatellite, 'a', 0, true, 1, true, true>,
    'a',
    0,
    true,
    1,
    true,
    false,
>;

type BinaryStopFromSensortoSatellite = SendTimed<Stop, End, 'a', 0, true, 1, true, false>;
type BinaryStopFromServertoSatellite = RecvTimed<Stop, End, 'a', 0, true, 1, true, false>;

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
type OfferFromServertoSensor =
    RecvTimed<BranchingFromServertoSensor, End, 'a', 0, true, 1, true, true>;

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
// Satellite
fn endpoint_satellite(
    s: EndpointSatellite0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_satellite(s, all_clocks)
}

fn recurs_satellite(
    s: EndpointSatellite0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        BranchingFromServertoSatellite::Stop(s) => {
            let (stop, s) = s.recv(all_clocks)?;
            let s = s.send(stop, all_clocks)?;

            s.close()
        },
        BranchingFromServertoSatellite::Data(s) => {
            // Forward get_data signal
            let (get_data, s) = s.recv(all_clocks)?;
            let s = s.send(get_data, all_clocks)?;

            // Forward the data
            let (data, s) = s.recv(all_clocks)?;
            let s = s.send(data, all_clocks)?;

            recurs_satellite(s, all_clocks)
        },
    })
}

// Sensor
fn endpoint_sensor(
    s: EndpointSensor0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_sensor(s, all_clocks)
}

fn recurs_sensor(
    s: EndpointSensor0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        BranchingFromServertoSensor::Stop(s) => {
            let (_stop, s) = s.recv(all_clocks)?;

            s.close()
        },
        BranchingFromServertoSensor::Data(s) => {
            // Get get_data signal
            let (_get_data, s) = s.recv(all_clocks)?;

            // Send the data
            let s = s.send(Data { payload : random::<i32>() }, all_clocks)?;

            recurs_sensor(s, all_clocks)
        },
    })
}

// Server
fn endpoint_server(
    s: EndpointServer0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_server(s, 100, all_clocks)
}

fn recurs_server(
    s: EndpointServer0,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s: EndpointServer1Stop = choose_mpst_server_to_all!(
                s,
                all_clocks,
                BranchingFromServertoSatellite::Stop,
                BranchingFromServertoSensor::Stop,
            );

            let s = s.send(Stop {}, all_clocks)?;

            s.close()
        }
        i => {
            let s: EndpointServer1Data = choose_mpst_server_to_all!(
                s,
                all_clocks,
                BranchingFromServertoSatellite::Data,
                BranchingFromServertoSensor::Data,
            );

            let s = s.send(GetData {}, all_clocks)?;

            let (_data, s) = s.recv(all_clocks)?;

            recurs_server(s, i - 1, all_clocks)
        }
    }
}

fn all_mpst() {
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

pub fn remote_data_main(c: &mut Criterion) {
    c.bench_function("Timed Remote data", |b| b.iter(all_mpst));
}