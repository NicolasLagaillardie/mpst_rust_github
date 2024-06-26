use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for two participants
generate_atmp!(MeshedChannels, Controller, Logs);

// RoleController
enum Branching0fromLtoC {
    Success(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoL>,
            RoleLogs<RoleLogs<RoleEnd>>,
            NameController,
        >,
    ),
    Failure(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose1fromCtoL>,
            RoleLogs<RoleBroadcast>,
            NameController,
        >,
    ),
}

type Recurs0fromCtoL = RecvTimed<Branching0fromLtoC, 'a', 0, true, 10, true, ' ', End>;

type Choose1fromCtoL = <Recurs1fromLtoC as Session>::Dual;

// RoleLogs
type Choose0fromLtoC = <Recurs0fromCtoL as Session>::Dual;

enum Branching1fromCtoL {
    Restart(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromLtoC>,
            RoleController<RoleBroadcast>,
            NameLogs,
        >,
    ),
    Stop(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            RoleController<RoleEnd>,
            NameLogs,
        >,
    ),
}

type Recurs1fromLtoC = RecvTimed<Branching1fromCtoL, 'a', 0, true, 10, true, ' ', End>;

// Creating the MP sessions
// RoleController
type EndpointController1Stop = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    RoleLogs<RoleEnd>,
    NameController,
>;
type EndpointController1Restart = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoL>,
    RoleLogs<RoleLogs<RoleEnd>>,
    NameController,
>;
type EndpointController0 = MeshedChannels<Recurs0fromCtoL, RoleLogs<RoleEnd>, NameController>;
type EndpointController1 = MeshedChannels<Choose1fromCtoL, RoleBroadcast, NameController>;
type EndpointControllerInit = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoL>,
    RoleLogs<RoleLogs<RoleEnd>>,
    NameController,
>;

// RoleLogs
type EndpointLogs0Success = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromLtoC>,
    RoleController<RoleBroadcast>,
    NameLogs,
>;
type EndpointLogs0Failure = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs1fromLtoC>,
    RoleController<RoleController<RoleEnd>>,
    NameLogs,
>;
type EndpointLogs0 = MeshedChannels<Choose0fromLtoC, RoleBroadcast, NameLogs>;
type EndpointLogs1 = MeshedChannels<Recurs1fromLtoC, RoleController<RoleEnd>, NameLogs>;
type EndpointLogsInit = MeshedChannels<
    RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromLtoC>,
    RoleController<RoleBroadcast>,
    NameLogs,
>;

fn endpoint_controller(
    s: EndpointControllerInit,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let start: i32 = thread_rng().gen_range(5..100);

    let s = s.send(start, all_clocks)?;

    recurs_0_controller(s, start, all_clocks)
}

fn recurs_0_controller(
    s: EndpointController0,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching0fromLtoC::Success(s) => {

            let (_, s) = s.recv(all_clocks)?;

            recurs_0_controller(s, loops, all_clocks)
        },
        Branching0fromLtoC::Failure(s) => {

            let (_, s) = s.recv(all_clocks)?;

            recurs_1_controller(s, loops, all_clocks)
        },
    })
}

fn recurs_1_controller(
    s: EndpointController1,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i <= 0 => {
            // Stop
            let s: EndpointController1Stop =
                choose_mpst_controller_to_all!(s, all_clocks, Branching1fromCtoL::Stop);

            let s = s.send(loops - 1, all_clocks)?;

            s.close()
        }
        _ => {
            // Restart
            let s: EndpointController1Restart =
                choose_mpst_controller_to_all!(s, all_clocks, Branching1fromCtoL::Restart);

            let s = s.send(loops - 1, all_clocks)?;

            recurs_0_controller(s, loops - 1, all_clocks)
        }
    }
}

fn endpoint_logs(
    s: EndpointLogsInit,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (status, s) = s.recv(all_clocks)?;
    recurs_0_logs(s, status, all_clocks)
}

fn recurs_0_logs(
    s: EndpointLogs0,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i % 2 == 0 && i > 0 => {
            // Success
            let s: EndpointLogs0Success =
                choose_mpst_logs_to_all!(s, all_clocks, Branching0fromLtoC::Success);

            let s = s.send(loops - 1, all_clocks)?;

            recurs_0_logs(s, loops - 1, all_clocks)
        }
        _ => {
            // Failure
            let s: EndpointLogs0Failure =
                choose_mpst_logs_to_all!(s, all_clocks, Branching0fromLtoC::Failure);

            let s = s.send(loops - 1, all_clocks)?;

            recurs_1_logs(s, all_clocks)
        }
    }
}

fn recurs_1_logs(
    s: EndpointLogs1,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching1fromCtoL::Restart(s) => {

            let (loops, s) = s.recv(all_clocks)?;

            recurs_0_logs(s, loops - 1, all_clocks)
        },
        Branching1fromCtoL::Stop(s) => {

            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
    })
}

/////////////////////////

fn main() {
    let (thread_controller, thread_logs) = fork_mpst(endpoint_controller, endpoint_logs);

    thread_controller.join().unwrap();
    thread_logs.join().unwrap();
}
