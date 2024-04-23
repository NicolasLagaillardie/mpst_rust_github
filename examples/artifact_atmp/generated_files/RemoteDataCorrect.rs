#![allow(dead_code, non_camel_case_types, unused_variables)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_atmp!(MeshedChannels, Satellite, Sensor, Server);

// Types of the payloads
struct Int;

struct Stop;

struct GetData;
struct Data {
    payload: Int,
}

// Binary sessions in depth 0
// Binary sessions for Sensor
type Message_0_v_0_FromSensorToServer =
    RecvTimed<Choice_0_FromServerToSensor, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromSensorToSatellite = End;

// Binary sessions for Server
type Message_0_v_0_FromServerToSatellite =
    SendTimed<Choice_0_FromServerToSatellite, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromServerToSensor =
    SendTimed<Choice_0_FromServerToSensor, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for Satellite
type Message_0_v_0_FromSatelliteToServer =
    RecvTimed<Choice_0_FromServerToSatellite, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromSatelliteToSensor = End;

// Binary sessions in depth 0.1
// Binary sessions for Satellite
type Message_0_1_v_0_FromSatelliteToSensor =
    SendTimed<Stop, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromSatelliteToSensor>;
type Message_0_1_v_1_FromSatelliteToSensor = End;
type Message_0_1_v_0_FromSatelliteToServer =
    RecvTimed<Stop, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromSatelliteToServer>;
type Message_0_1_v_1_FromSatelliteToServer = End;

// Binary sessions for Server
type Message_0_1_v_0_FromServerToSatellite =
    SendTimed<Stop, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromServerToSatellite>;
type Message_0_1_v_1_FromServerToSatellite = End;
type Message_0_1_v_0_FromServerToSensor = End;

// Binary sessions for Sensor
type Message_0_1_v_0_FromSensorToServer = End;
type Message_0_1_v_0_FromSensorToSatellite =
    RecvTimed<Stop, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromSensorToSatellite>;
type Message_0_1_v_1_FromSensorToSatellite = End;

// Binary sessions in depth 0.0
// Binary sessions for Sensor
type Message_0_0_v_0_FromSensorToSatellite =
    RecvTimed<GetData, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromSensorToSatellite>;
type Message_0_0_v_1_FromSensorToSatellite =
    SendTimed<Data, 'a', 0, true, 1, true, ' ', Message_0_0_v_2_FromSensorToSatellite>;
type Message_0_0_v_2_FromSensorToSatellite = End;
type Message_0_0_v_0_FromSensorToServer =
    RecvTimed<Choice_0_FromServerToSensor, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for Server
type Message_0_0_v_0_FromServerToSensor =
    SendTimed<Choice_0_FromServerToSensor, ' ', -2, false, -1, false, ' ', End>;
type Message_0_0_v_0_FromServerToSatellite =
    SendTimed<GetData, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromServerToSatellite>;
type Message_0_0_v_1_FromServerToSatellite =
    RecvTimed<Data, 'a', 0, true, 1, true, ' ', Message_0_0_v_2_FromServerToSatellite>;
type Message_0_0_v_2_FromServerToSatellite =
    SendTimed<Choice_0_FromServerToSatellite, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for Satellite
type Message_0_0_v_0_FromSatelliteToSensor =
    SendTimed<GetData, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromSatelliteToSensor>;
type Message_0_0_v_1_FromSatelliteToSensor =
    RecvTimed<Data, 'a', 0, true, 1, true, ' ', Message_0_0_v_2_FromSatelliteToSensor>;
type Message_0_0_v_2_FromSatelliteToSensor = End;
type Message_0_0_v_0_FromSatelliteToServer =
    RecvTimed<GetData, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromSatelliteToServer>;
type Message_0_0_v_1_FromSatelliteToServer =
    SendTimed<Data, 'a', 0, true, 1, true, ' ', Message_0_0_v_2_FromSatelliteToServer>;
type Message_0_0_v_2_FromSatelliteToServer =
    RecvTimed<Choice_0_FromServerToSatellite, ' ', -2, false, -1, false, ' ', End>;

// Stacks in depth 0
// Stacks for Server
type Ordering_0_v_0_ForServer = RoleBroadcast;

// Stacks for Sensor
type Ordering_0_v_0_ForSensor = RoleServer<RoleEnd>;

// Stacks for Satellite
type Ordering_0_v_0_ForSatellite = RoleServer<RoleEnd>;

// Stacks in depth 0.1
// Stacks for Satellite
type Ordering_0_1_v_0_ForSatellite = RoleServer<Ordering_0_1_v_1_ForSatellite>;
type Ordering_0_1_v_1_ForSatellite = RoleSensor<Ordering_0_1_v_2_ForSatellite>;
type Ordering_0_1_v_2_ForSatellite = RoleEnd;

// Stacks for Sensor
type Ordering_0_1_v_0_ForSensor = RoleSatellite<Ordering_0_1_v_1_ForSensor>;
type Ordering_0_1_v_1_ForSensor = RoleEnd;

// Stacks for Server
type Ordering_0_1_v_0_ForServer = RoleSatellite<Ordering_0_1_v_1_ForServer>;
type Ordering_0_1_v_1_ForServer = RoleEnd;

// Stacks in depth 0.0
// Stacks for Server
type Ordering_0_0_v_0_ForServer = RoleSatellite<Ordering_0_0_v_1_ForServer>;
type Ordering_0_0_v_1_ForServer = RoleSatellite<Ordering_0_0_v_2_ForServer>;
type Ordering_0_0_v_2_ForServer = RoleBroadcast;

// Stacks for Satellite
type Ordering_0_0_v_0_ForSatellite = RoleServer<Ordering_0_0_v_1_ForSatellite>;
type Ordering_0_0_v_1_ForSatellite = RoleSensor<Ordering_0_0_v_2_ForSatellite>;
type Ordering_0_0_v_2_ForSatellite = RoleSensor<Ordering_0_0_v_3_ForSatellite>;
type Ordering_0_0_v_3_ForSatellite = RoleServer<Ordering_0_0_v_4_ForSatellite>;
type Ordering_0_0_v_4_ForSatellite = RoleServer<RoleEnd>;

// Stacks for Sensor
type Ordering_0_0_v_0_ForSensor = RoleSatellite<Ordering_0_0_v_1_ForSensor>;
type Ordering_0_0_v_1_ForSensor = RoleSatellite<Ordering_0_0_v_2_ForSensor>;
type Ordering_0_0_v_2_ForSensor = RoleServer<RoleEnd>;

// Enums (Branchings) in depth 0
// Enums (Branchings) for Satellite
enum Choice_0_FromServerToSatellite {
    Branching0(Endpoint_0_0_ForSatellite),
    Branching1(Endpoint_0_1_ForSatellite),
}

// Enums (Branchings) for Sensor
enum Choice_0_FromServerToSensor {
    Branching0(Endpoint_0_0_ForSensor),
    Branching1(Endpoint_0_1_ForSensor),
}

// Endpoints in depth 0
// Endpoint for role Satellite
type Endpoint_0_ForSatellite = MeshedChannels<
    Message_0_v_0_FromSatelliteToSensor,
    Message_0_v_0_FromSatelliteToServer,
    Ordering_0_v_0_ForSatellite,
    NameSatellite,
>;

// Endpoint for role Sensor
type Endpoint_0_ForSensor = MeshedChannels<
    Message_0_v_0_FromSensorToSatellite,
    Message_0_v_0_FromSensorToServer,
    Ordering_0_v_0_ForSensor,
    NameSensor,
>;

// Endpoint for role Server
type Endpoint_0_ForServer = MeshedChannels<
    Message_0_v_0_FromServerToSatellite,
    Message_0_v_0_FromServerToSensor,
    Ordering_0_v_0_ForServer,
    NameServer,
>;

// Endpoints in depth 0.1
// Endpoint for role Satellite
type Endpoint_0_1_ForSatellite = MeshedChannels<
    Message_0_1_v_0_FromSatelliteToSensor,
    Message_0_1_v_0_FromSatelliteToServer,
    Ordering_0_1_v_0_ForSatellite,
    NameSatellite,
>;

// Endpoint for role Sensor
type Endpoint_0_1_ForSensor = MeshedChannels<
    Message_0_1_v_0_FromSensorToSatellite,
    Message_0_1_v_0_FromSensorToServer,
    Ordering_0_1_v_0_ForSensor,
    NameSensor,
>;

// Endpoint for role Server
type Endpoint_0_1_ForServer = MeshedChannels<
    Message_0_1_v_0_FromServerToSatellite,
    Message_0_1_v_0_FromServerToSensor,
    Ordering_0_1_v_0_ForServer,
    NameServer,
>;

// Endpoints in depth 0.0
// Endpoint for role Satellite
type Endpoint_0_0_ForSatellite = MeshedChannels<
    Message_0_0_v_0_FromSatelliteToSensor,
    Message_0_0_v_0_FromSatelliteToServer,
    Ordering_0_0_v_0_ForSatellite,
    NameSatellite,
>;

// Endpoint for role Sensor
type Endpoint_0_0_ForSensor = MeshedChannels<
    Message_0_0_v_0_FromSensorToSatellite,
    Message_0_0_v_0_FromSensorToServer,
    Ordering_0_0_v_0_ForSensor,
    NameSensor,
>;

// Endpoint for role Server
type Endpoint_0_0_ForServer = MeshedChannels<
    Message_0_0_v_0_FromServerToSatellite,
    Message_0_0_v_0_FromServerToSensor,
    Ordering_0_0_v_0_ForServer,
    NameServer,
>;

// Fill in the functions here.
fn endpoint_satellite_0_v_0(
    s: Endpoint_0_ForSatellite,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_satellite(s, all_clocks)
}

fn recurs_satellite(
    s: Endpoint_0_ForSatellite,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Choice_0_FromServerToSatellite::Branching0(s) => {
            // Forward get_data signal
            let (get_data, s) = s.recv(all_clocks)?;
            let s = s.send(get_data, all_clocks)?;

            // Forward the data
            let (data, s) = s.recv(all_clocks)?;
            let s = s.send(data, all_clocks)?;

            recurs_satellite(s, all_clocks)
        },
        Choice_0_FromServerToSatellite::Branching1(s) => {
            let (stop, s) = s.recv(all_clocks)?;
            let s = s.send(stop, all_clocks)?;

            s.close()
        },
    })
}

fn endpoint_sensor_0_v_0(
    s: Endpoint_0_ForSensor,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_sensor(s, all_clocks)
}

fn recurs_sensor(
    s: Endpoint_0_ForSensor,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Choice_0_FromServerToSensor::Branching0(s) => {
            // Get get_data signal
            let (_get_data, s) = s.recv(all_clocks)?;

            // Send the data
            let s = s.send(Data {payload: Int}, all_clocks)?;

            recurs_sensor(s, all_clocks)

        },
        Choice_0_FromServerToSensor::Branching1(s) => {

            let (_stop, s) = s.recv(all_clocks)?;

            s.close()
        },
    })
}

fn endpoint_server_0_v_0(
    s: Endpoint_0_ForServer,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_server(s, 100, all_clocks)
}

fn recurs_server(
    s: Endpoint_0_ForServer,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s: Endpoint_0_1_ForServer = choose_mpst_server_to_all!(
                s,
                all_clocks,
                Choice_0_FromServerToSatellite::Branching1,
                Choice_0_FromServerToSensor::Branching1,
            );

            let s = s.send(Stop {}, all_clocks)?;

            s.close()
        }
        i => {
            let s: Endpoint_0_0_ForServer = choose_mpst_server_to_all!(
                s,
                all_clocks,
                Choice_0_FromServerToSatellite::Branching0,
                Choice_0_FromServerToSensor::Branching0,
            );

            let s = s.send(GetData {}, all_clocks)?;

            let (_data, s) = s.recv(all_clocks)?;

            recurs_server(s, i - 1, all_clocks)
        }
    }
}

fn endpoint_satellite_0_1_v_0(
    s: Endpoint_0_1_ForSatellite,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_satellite_0_1_v_1(
    s: Endpoint_0_1_ForSatellite,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_sensor_0_1_v_0(
    s: Endpoint_0_1_ForSensor,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_sensor_0_1_v_1(
    s: Endpoint_0_1_ForSensor,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_server_0_1_v_0(
    s: Endpoint_0_1_ForServer,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_server_0_1_v_1(
    s: Endpoint_0_1_ForServer,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_satellite_0_0_v_0(
    s: Endpoint_0_0_ForSatellite,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_sensor_0_0_v_0(
    s: Endpoint_0_0_ForSensor,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_server_0_0_v_0(
    s: Endpoint_0_0_ForServer,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn main() {
    let (thread_satellite, thread_sensor, thread_server) = fork_mpst(
        endpoint_satellite_0_v_0,
        endpoint_sensor_0_v_0,
        endpoint_server_0_v_0,
    );

    println!("Thread Satellite: {:?}", thread_satellite.join());
    println!("Thread Sensor: {:?}", thread_sensor.join());
    println!("Thread Server: {:?}", thread_server.join());
}
