#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// Create the new MeshedChannels for three participants and the close and fork functions
generate!("rec_and_cancel", MeshedChannels, Sensor, SensorManager);

// Labels
// Ask for the coordinates
struct GetCoordinates {}
// The coordinates
struct Coordinates {}
// Stop loop
struct Stop {}

// Types
// Scl
type Choice0FromSensorToSensorManager = Recv<Branching0FromSensorToSensorManager, End>;

type SensorManagerToSensorLoop =
    Recv<GetCoordinates, Send<Coordinates, Choice0FromSensorToSensorManager>>;

type SensorManagerToSensorStop = Recv<Stop, End>;

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
fn endpoint_sensor(s: Endpoint0Car) -> Result<(), Box<dyn Error>> {
    recurs_sensor(s)
}

fn recurs_sensor(s: Endpoint0Car) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0FromSensorToSensorManager::Stop(s) => {

            let (_stop, s) = s.recv()?;

            s.close()
        },
        Branching0FromSensorToSensorManager::Loop(s) => {
            let (_buf, s) = s.recv()?;

            let s = s.send(Coordinates {})?;

            recurs_sensor(s)
        },
    })
}

/////////////////////////

// SensorManager
fn endpoint_sensormanager(s: Endpoint0SensorManager) -> Result<(), Box<dyn Error>> {
    recurs_sensormanager(s, LOOPS)
}

fn recurs_sensormanager(s: Endpoint0SensorManager, loops: i64) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s: Endpoint1SensorManagerStop =
                choose_mpst_sensormanager_to_all!(s, Branching0FromSensorToSensorManager::Stop);

            let s = s.send(Stop {})?;

            s.close()
        }
        i => {
            let s: Endpoint1SensorManagerLoop =
                choose_mpst_sensormanager_to_all!(s, Branching0FromSensorToSensorManager::Loop);

            let s = s.send(GetCoordinates {})?;

            let (_buf, s) = s.recv()?;

            recurs_sensormanager(s, i - 1)
        }
    }
}

/////////////////////////

fn main() {
    let (thread_sensor, thread_sensormanager) = fork_mpst(endpoint_sensor, endpoint_sensormanager);

    thread_sensor.join().unwrap();
    thread_sensormanager.join().unwrap();

    println!("Done");
}

static LOOPS: i64 = 100;
