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
struct Data { payload: Int }

// Binary sessions in depth 0
// Binary sessions for Server
type Message_0_v_0_FromServerToSensor = SendTimed<Choice_0_FromServerToSensor, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromServerToSatellite = SendTimed<Choice_0_FromServerToSatellite, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for Sensor
type Message_0_v_0_FromSensorToServer = RecvTimed<Choice_0_FromServerToSensor, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromSensorToSatellite = End;

// Binary sessions for Satellite
type Message_0_v_0_FromSatelliteToServer = RecvTimed<Choice_0_FromServerToSatellite, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromSatelliteToSensor = End;

// Binary sessions in depth 0.1
// Binary sessions for Sensor
type Message_0_1_v_0_FromSensorToSatellite = RecvTimed<Stop, 'b', 1, true, 2, true, ' ', Message_0_1_v_1_FromSensorToSatellite>;
type Message_0_1_v_1_FromSensorToSatellite = End;
type Message_0_1_v_0_FromSensorToServer = End;

// Binary sessions for Satellite
type Message_0_1_v_0_FromSatelliteToSensor = SendTimed<Stop, 'b', 1, true, 2, true, ' ', Message_0_1_v_1_FromSatelliteToSensor>;
type Message_0_1_v_1_FromSatelliteToSensor = End;
type Message_0_1_v_0_FromSatelliteToServer = RecvTimed<Stop, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromSatelliteToServer>;
type Message_0_1_v_1_FromSatelliteToServer = End;

// Binary sessions for Server
type Message_0_1_v_0_FromServerToSensor = End;
type Message_0_1_v_0_FromServerToSatellite = SendTimed<Stop, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromServerToSatellite>;
type Message_0_1_v_1_FromServerToSatellite = End;

// Binary sessions in depth 0.0
// Binary sessions for Satellite
type Message_0_0_v_0_FromSatelliteToServer = RecvTimed<GetData, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromSatelliteToServer>;
type Message_0_0_v_1_FromSatelliteToServer = SendTimed<Data, 'a', 4, true, 6, true, 'a', Message_0_0_v_2_FromSatelliteToServer>;
type Message_0_0_v_2_FromSatelliteToServer = RecvTimed<Choice_0_FromServerToSatellite, ' ', -2, false, -1, false, ' ', End>;
type Message_0_0_v_0_FromSatelliteToSensor = SendTimed<GetData, 'b', 1, true, 2, true, ' ', Message_0_0_v_1_FromSatelliteToSensor>;
type Message_0_0_v_1_FromSatelliteToSensor = RecvTimed<Data, 'b', 2, true, 4, true, 'a', Message_0_0_v_2_FromSatelliteToSensor>;
type Message_0_0_v_2_FromSatelliteToSensor = End;

// Binary sessions for Sensor
type Message_0_0_v_0_FromSensorToSatellite = RecvTimed<GetData, 'b', 1, true, 2, true, ' ', Message_0_0_v_1_FromSensorToSatellite>;
type Message_0_0_v_1_FromSensorToSatellite = SendTimed<Data, 'b', 2, true, 4, true, 'a', Message_0_0_v_2_FromSensorToSatellite>;
type Message_0_0_v_2_FromSensorToSatellite = End;
type Message_0_0_v_0_FromSensorToServer = RecvTimed<Choice_0_FromServerToSensor, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for Server
type Message_0_0_v_0_FromServerToSatellite = SendTimed<GetData, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromServerToSatellite>;
type Message_0_0_v_1_FromServerToSatellite = RecvTimed<Data, 'a', 4, true, 6, true, 'a', Message_0_0_v_2_FromServerToSatellite>;
type Message_0_0_v_2_FromServerToSatellite = SendTimed<Choice_0_FromServerToSatellite, ' ', -2, false, -1, false, ' ', End>;
type Message_0_0_v_0_FromServerToSensor = SendTimed<Choice_0_FromServerToSensor, ' ', -2, false, -1, false, ' ', End>;

// Stacks in depth 0
// Stacks for Sensor
type Ordering_0_v_0_ForSensor = RoleServer<RoleEnd>;

// Stacks for Satellite
type Ordering_0_v_0_ForSatellite = RoleServer<RoleEnd>;

// Stacks for Server
type Ordering_0_v_0_ForServer = RoleBroadcast;

// Stacks in depth 0.1
// Stacks for Satellite
type Ordering_0_1_v_0_ForSatellite = RoleServer<Ordering_0_1_v_1_ForSatellite>;
type Ordering_0_1_v_1_ForSatellite = RoleSensor<Ordering_0_1_v_2_ForSatellite>;
type Ordering_0_1_v_2_ForSatellite = RoleEnd;

// Stacks for Server
type Ordering_0_1_v_0_ForServer = RoleSatellite<Ordering_0_1_v_1_ForServer>;
type Ordering_0_1_v_1_ForServer = RoleEnd;

// Stacks for Sensor
type Ordering_0_1_v_0_ForSensor = RoleSatellite<Ordering_0_1_v_1_ForSensor>;
type Ordering_0_1_v_1_ForSensor = RoleEnd;

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
type Endpoint_0_ForSatellite = MeshedChannels<Message_0_v_0_FromSatelliteToSensor, Message_0_v_0_FromSatelliteToServer, Ordering_0_v_0_ForSatellite, NameSatellite>;

// Endpoint for role Sensor
type Endpoint_0_ForSensor = MeshedChannels<Message_0_v_0_FromSensorToSatellite, Message_0_v_0_FromSensorToServer, Ordering_0_v_0_ForSensor, NameSensor>;

// Endpoint for role Server
type Endpoint_0_ForServer = MeshedChannels<Message_0_v_0_FromServerToSatellite, Message_0_v_0_FromServerToSensor, Ordering_0_v_0_ForServer, NameServer>;

// Endpoints in depth 0.1
// Endpoint for role Satellite
type Endpoint_0_1_ForSatellite = MeshedChannels<Message_0_1_v_0_FromSatelliteToSensor, Message_0_1_v_0_FromSatelliteToServer, Ordering_0_1_v_0_ForSatellite, NameSatellite>;

// Endpoint for role Sensor
type Endpoint_0_1_ForSensor = MeshedChannels<Message_0_1_v_0_FromSensorToSatellite, Message_0_1_v_0_FromSensorToServer, Ordering_0_1_v_0_ForSensor, NameSensor>;

// Endpoint for role Server
type Endpoint_0_1_ForServer = MeshedChannels<Message_0_1_v_0_FromServerToSatellite, Message_0_1_v_0_FromServerToSensor, Ordering_0_1_v_0_ForServer, NameServer>;

// Endpoints in depth 0.0
// Endpoint for role Satellite
type Endpoint_0_0_ForSatellite = MeshedChannels<Message_0_0_v_0_FromSatelliteToSensor, Message_0_0_v_0_FromSatelliteToServer, Ordering_0_0_v_0_ForSatellite, NameSatellite>;

// Endpoint for role Sensor
type Endpoint_0_0_ForSensor = MeshedChannels<Message_0_0_v_0_FromSensorToSatellite, Message_0_0_v_0_FromSensorToServer, Ordering_0_0_v_0_ForSensor, NameSensor>;

// Endpoint for role Server
type Endpoint_0_0_ForServer = MeshedChannels<Message_0_0_v_0_FromServerToSatellite, Message_0_0_v_0_FromServerToSensor, Ordering_0_0_v_0_ForServer, NameServer>;

// Fill in the functions here.
fn endpoint_satellite_0_v_0(s: Endpoint_0_ForSatellite, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('b', Instant::now());
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn endpoint_sensor_0_v_0(s: Endpoint_0_ForSensor, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('b', Instant::now());

	Ok(())
}

fn endpoint_server_0_v_0(s: Endpoint_0_ForServer, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn endpoint_satellite_0_1_v_0(s: Endpoint_0_1_ForSatellite, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('b', Instant::now());
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn endpoint_satellite_0_1_v_1(s: Endpoint_0_1_ForSatellite, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('b', Instant::now());
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn endpoint_sensor_0_1_v_0(s: Endpoint_0_1_ForSensor, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('b', Instant::now());

	Ok(())
}

fn endpoint_sensor_0_1_v_1(s: Endpoint_0_1_ForSensor, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('b', Instant::now());

	Ok(())
}

fn endpoint_server_0_1_v_0(s: Endpoint_0_1_ForServer, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn endpoint_server_0_1_v_1(s: Endpoint_0_1_ForServer, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn endpoint_satellite_0_0_v_0(s: Endpoint_0_0_ForSatellite, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('b', Instant::now());
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn endpoint_sensor_0_0_v_0(s: Endpoint_0_0_ForSensor, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('b', Instant::now());

	Ok(())
}

fn endpoint_server_0_0_v_0(s: Endpoint_0_0_ForServer, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn main() {
	let (thread_satellite, thread_sensor, thread_server, ) = fork_mpst(endpoint_satellite_0_v_0, endpoint_sensor_0_v_0, endpoint_server_0_v_0, );

	println!("Thread Satellite: {:?}", thread_satellite.join());
	println!("Thread Sensor: {:?}", thread_sensor.join());
	println!("Thread Server: {:?}", thread_server.join());
}
