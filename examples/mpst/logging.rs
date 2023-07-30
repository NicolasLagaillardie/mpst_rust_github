#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::error::Error;
use std::marker;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for two participants
generate!("recursive", MeshedChannels, Controller, Logs);

// RoleController
enum Branching0fromLtoC<N: marker::Send> {
    Success(
        MeshedChannels<Recv<N, Recurs0fromCtoL<N>>, RoleLogs<RoleLogs<RoleEnd>>, NameController>,
    ),
    Failure(MeshedChannels<Recv<N, Choose1fromCtoL<N>>, RoleLogs<RoleBroadcast>, NameController>),
}

type Recurs0fromCtoL<N> = Recv<Branching0fromLtoC<N>, End>;

type Choose1fromCtoL<N> = Send<Branching1fromCtoL<N>, End>;

// RoleLogs
type Choose0fromLtoC<N> = Send<Branching0fromLtoC<N>, End>;

enum Branching1fromCtoL<N: marker::Send> {
    Restart(MeshedChannels<Recv<N, Choose0fromLtoC<N>>, RoleController<RoleBroadcast>, NameLogs>),
    Stop(MeshedChannels<Recv<N, End>, RoleController<RoleEnd>, NameLogs>),
}

type Recurs1fromLtoC<N> = Recv<Branching1fromCtoL<N>, End>;

// Creating the MP sessions
// RoleController
type EndpointController1Stop<N> = MeshedChannels<Send<N, End>, RoleLogs<RoleEnd>, NameController>;
type EndpointController1Restart<N> =
    MeshedChannels<Send<N, Recurs0fromCtoL<N>>, RoleLogs<RoleLogs<RoleEnd>>, NameController>;
type EndpointController0<N> = MeshedChannels<Recurs0fromCtoL<N>, RoleLogs<RoleEnd>, NameController>;
type EndpointController1<N> = MeshedChannels<Choose1fromCtoL<N>, RoleBroadcast, NameController>;
type EndpointControllerInit<N> =
    MeshedChannels<Send<N, Recurs0fromCtoL<N>>, RoleLogs<RoleLogs<RoleEnd>>, NameController>;

// RoleLogs
type EndpointLogs0Success<N> =
    MeshedChannels<Send<N, Choose0fromLtoC<N>>, RoleController<RoleBroadcast>, NameLogs>;
type EndpointLogs0Failure<N> =
    MeshedChannels<Send<N, Recurs1fromLtoC<N>>, RoleController<RoleController<RoleEnd>>, NameLogs>;
type EndpointLogs0<N> = MeshedChannels<Choose0fromLtoC<N>, RoleBroadcast, NameLogs>;
type EndpointLogs1<N> = MeshedChannels<Recurs1fromLtoC<N>, RoleController<RoleEnd>, NameLogs>;
type EndpointLogsInit<N> =
    MeshedChannels<Recv<N, Choose0fromLtoC<N>>, RoleController<RoleBroadcast>, NameLogs>;

fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
    let start: i32 = thread_rng().gen_range(5..100);

    let s = s.send(start);

    recurs_0_controller(s, start)
}

fn recurs_0_controller(s: EndpointController0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromLtoC::Success(s) => {

            let (_, s) = s.recv();

            recurs_0_controller(s, loops)
        },
        Branching0fromLtoC::Failure(s) => {

            let (_, s) = s.recv();

            recurs_1_controller(s, loops)
        },
    })
}

fn recurs_1_controller(s: EndpointController1<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i <= 0 => {
            // Stop
            let s: EndpointController1Stop<i32> =
                choose_mpst_controller_to_all!(s, Branching1fromCtoL::Stop);

            let s = s.send(loops - 1);

            s.close()
        }
        _ => {
            // Restart
            let s: EndpointController1Restart<i32> =
                choose_mpst_controller_to_all!(s, Branching1fromCtoL::Restart);

            let s = s.send(loops - 1);

            recurs_0_controller(s, loops - 1)
        }
    }
}

fn endpoint_logs(s: EndpointLogsInit<i32>) -> Result<(), Box<dyn Error>> {
    let (status, s) = s.recv();
    recurs_0_logs(s, status)
}

fn recurs_0_logs(s: EndpointLogs0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i % 2 == 0 && i > 0 => {
            // Success
            let s: EndpointLogs0Success<i32> =
                choose_mpst_logs_to_all!(s, Branching0fromLtoC::Success);

            let s = s.send(loops - 1);

            recurs_0_logs(s, loops - 1)
        }
        _ => {
            // Failure
            let s: EndpointLogs0Failure<i32> =
                choose_mpst_logs_to_all!(s, Branching0fromLtoC::Failure);

            let s = s.send(loops - 1);

            recurs_1_logs(s)
        }
    }
}

fn recurs_1_logs(s: EndpointLogs1<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromCtoL::Restart(s) => {

            let (loops, s) = s.recv();

            recurs_0_logs(s, loops - 1)
        },
        Branching1fromCtoL::Stop(s) => {

            let (_, s) = s.recv();

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
