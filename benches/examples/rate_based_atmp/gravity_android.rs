#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_timed;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;

// Create the new MeshedChannels for three participants and the close and fork functions
generate_timed!(MeshedChannels, Sensor, SensorManager);

// Labels
// Ask for the coordinates
struct GetCoordinates {}
// The coordinates
struct Coordinates {}
// Stop loop
struct Stop {}

// Types
// Scl
type Choice0FromSensorToSensorManager =
    RecvTimed<Branching0FromSensorToSensorManager, 'a', 0, true, 1, true, ' ', End>;

type SensorManagerToSensorLoop = RecvTimed<
    GetCoordinates,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    SendTimed<Coordinates, 'a', 0, true, 1, true, 'a', Choice0FromSensorToSensorManager>,
>;

type SensorManagerToSensorStop = RecvTimed<Stop, 'a', 0, true, 1, true, 'a', End>;

enum Branching0FromSensorToSensorManager {
    Loop(
        MeshedChannels<
            SensorManagerToSensorLoop,
            RoleSensorManager<RoleSensorManager<RoleSensorManager<RoleEnd>>>,
            NameSensor,
        >,
    ),
    Stop(MeshedChannels<SensorManagerToSensorStop, RoleSensorManager<RoleEnd>, NameSensor>),
}

// Sda
type ChooseFromSensorToSensorManager = <Choice0FromSensorToSensorManager as Session>::Dual;

type SensorToSensorManagerLoop = <SensorManagerToSensorLoop as Session>::Dual;

type SensorToSensorManagerStop = <SensorManagerToSensorStop as Session>::Dual;

// Endpoints
// Scl
type Endpoint0Car =
    MeshedChannels<Choice0FromSensorToSensorManager, RoleSensorManager<RoleEnd>, NameSensor>;

// Sda
type Endpoint0SensorManager =
    MeshedChannels<ChooseFromSensorToSensorManager, RoleBroadcast, NameSensorManager>;

type Endpoint1SensorManagerLoop = MeshedChannels<
    SensorToSensorManagerLoop,
    RoleSensor<RoleSensor<RoleBroadcast>>,
    NameSensorManager,
>;

type Endpoint1SensorManagerStop =
    MeshedChannels<SensorToSensorManagerStop, RoleSensor<RoleEnd>, NameSensorManager>;

// Functions

/////////////////////////

// Car
fn endpoint_sensor(
    s: Endpoint0Car,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_sensor(s, all_clocks)
}

fn recurs_sensor(
    s: Endpoint0Car,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {

    offer_mpst!(s, all_clocks, {
        Branching0FromSensorToSensorManager::Stop(s) => {

            let (_stop, s) = s.recv(all_clocks)?;

            s.close()
        },
        Branching0FromSensorToSensorManager::Loop(s) => {
            let (_buf, s) = s.recv(all_clocks)?;

            let s = s.send(Coordinates {}, all_clocks)?;

            recurs_sensor(s, all_clocks)
        },
    })
}

/////////////////////////

// SensorManager
fn endpoint_sensormanager(
    s: Endpoint0SensorManager,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_sensormanager(s, all_clocks, LOOPS)
}

fn recurs_sensormanager(
    s: Endpoint0SensorManager,
    all_clocks: &mut HashMap<char, Instant>,
    loops: i64,
) -> Result<(), Box<dyn Error>> {

    match loops {
        0 => {
            let s: Endpoint1SensorManagerStop = choose_mpst_sensormanager_to_all!(
                s,
                all_clocks,
                Branching0FromSensorToSensorManager::Stop,
            );

            let s = s.send(Stop {}, all_clocks)?;

            s.close()
        }
        i => {
            let s: Endpoint1SensorManagerLoop = choose_mpst_sensormanager_to_all!(
                s,
                all_clocks,
                Branching0FromSensorToSensorManager::Loop
            );

            let s = s.send(GetCoordinates {}, all_clocks)?;

            let (_buf, s) = s.recv(all_clocks)?;

            recurs_sensormanager(s, all_clocks, i - 1)
        }
    }
}

/////////////////////////////////////////

fn aux() {
    let (thread_sensor, thread_sensormanager) = fork_mpst(
        black_box(endpoint_sensor),
        black_box(endpoint_sensormanager),
    );

    thread_sensor.join().unwrap();
    thread_sensormanager.join().unwrap();
}

static LOOPS: i64 = 100;

/////////////////////////

pub fn gravity_android(c: &mut Criterion) {
    c.bench_function("Timed Gravity Android", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = gravity_android,
}

criterion_main! {
    bench
}
