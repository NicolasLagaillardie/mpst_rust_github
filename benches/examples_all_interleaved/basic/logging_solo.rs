use criterion::{black_box, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{baker, close_mpst_interleaved, fork_mpst_multi_solo, offer_mpst_interleaved};

use rand::random;

use std::error::Error;
use std::marker;
// use std::time::Duration;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for two participants
baker!("rec_and_cancel", MeshedChannelsTwo, Controller, Logs);

close_mpst_interleaved!(close_mpst, MeshedChannelsTwo, 2);

fork_mpst_multi_solo!(fork_mpst_solo, MeshedChannelsTwo, 2);

// RoleController
enum Branching0fromLtoC<N: marker::Send> {
    Success(
        MeshedChannelsTwo<Recv<N, Recurs0fromCtoL<N>>, RoleLogs<RoleLogs<RoleEnd>>, NameController>,
    ),
    Failure(
        MeshedChannelsTwo<Recv<N, Choose1fromCtoL<N>>, RoleLogs<RoleBroadcast>, NameController>,
    ),
}

type Recurs0fromCtoL<N> = Recv<Branching0fromLtoC<N>, End>;

type Choose1fromCtoL<N> = Send<Branching1fromCtoL<N>, End>;

// RoleLogs
type Choose0fromLtoC<N> = Send<Branching0fromLtoC<N>, End>;

enum Branching1fromCtoL<N: marker::Send> {
    Restart(
        MeshedChannelsTwo<Recv<N, Choose0fromLtoC<N>>, RoleController<RoleBroadcast>, NameLogs>,
    ),
    Stop(MeshedChannelsTwo<Recv<N, End>, RoleController<RoleEnd>, NameLogs>),
}

type Recurs1fromLtoC<N> = Recv<Branching1fromCtoL<N>, End>;

// Creating the MP sessions
// RoleController
type EndpointController1Stop<N> =
    MeshedChannelsTwo<Send<N, End>, RoleLogs<RoleEnd>, NameController>;
type EndpointController1Restart<N> =
    MeshedChannelsTwo<Send<N, Recurs0fromCtoL<N>>, RoleLogs<RoleLogs<RoleEnd>>, NameController>;
type EndpointController0<N> =
    MeshedChannelsTwo<Recurs0fromCtoL<N>, RoleLogs<RoleEnd>, NameController>;
type EndpointController1<N> = MeshedChannelsTwo<Choose1fromCtoL<N>, RoleBroadcast, NameController>;
type EndpointControllerInit<N> =
    MeshedChannelsTwo<Send<N, Recurs0fromCtoL<N>>, RoleLogs<RoleLogs<RoleEnd>>, NameController>;
// RoleLogs
type EndpointLogs0Success<N> =
    MeshedChannelsTwo<Send<N, Choose0fromLtoC<N>>, RoleController<RoleBroadcast>, NameLogs>;
type EndpointLogs0Failure<N> = MeshedChannelsTwo<
    Send<N, Recurs1fromLtoC<N>>,
    RoleController<RoleController<RoleEnd>>,
    NameLogs,
>;
type EndpointLogs0<N> = MeshedChannelsTwo<Choose0fromLtoC<N>, RoleBroadcast, NameLogs>;
type EndpointLogs1<N> = MeshedChannelsTwo<Recurs1fromLtoC<N>, RoleController<RoleEnd>, NameLogs>;
type EndpointLogsInit<N> =
    MeshedChannelsTwo<Recv<N, Choose0fromLtoC<N>>, RoleController<RoleBroadcast>, NameLogs>;

/////////////////////////

fn start(
    s_controller: EndpointControllerInit<i32>,
    s_logs: EndpointLogsInit<i32>,
) -> Result<(), Box<dyn Error>> {
    let s_controller = s_controller.send(random::<i32>())?;
    let (_, s_logs) = s_logs.recv()?;

    rec_loop_0(s_controller, s_logs, 100)
}

fn rec_loop_0(
    s_controller: EndpointController0<i32>,
    s_logs: EndpointLogs0<i32>,
    loops: i32,
) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i % 2 == 0 && i > 0 => {
            // Success
            let s_logs: EndpointLogs0Success<i32> =
                choose_mpst_logs_to_all!(s_logs, Branching0fromLtoC::Success);

            let (s_controller,) =
                offer_mpst_interleaved!(s_controller, Branching0fromLtoC::Success);

            let s_logs = s_logs.send(loops - 1)?;
            let (_, s_controller) = s_controller.recv()?;

            rec_loop_0(s_controller, s_logs, loops - 1)
        }
        _ => {
            // Failure
            let s_logs: EndpointLogs0Failure<i32> =
                choose_mpst_logs_to_all!(s_logs, Branching0fromLtoC::Failure);

            let (s_controller,) =
                offer_mpst_interleaved!(s_controller, Branching0fromLtoC::Failure);

            let s_logs = s_logs.send(loops - 1)?;
            let (_, s_controller) = s_controller.recv()?;

            rec_loop_1(s_controller, s_logs, loops - 1)
        }
    }
}

fn rec_loop_1(
    s_controller: EndpointController1<i32>,
    s_logs: EndpointLogs1<i32>,
    loops: i32,
) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i <= 0 => {
            // Stop
            let s_controller: EndpointController1Stop<i32> =
                choose_mpst_controller_to_all!(s_controller, Branching1fromCtoL::Stop);

            let (s_logs,) = offer_mpst_interleaved!(s_logs, Branching1fromCtoL::Stop);

            let s_controller = s_controller.send(loops - 1)?;
            let (_, s_logs) = s_logs.recv()?;

            close_mpst(s_controller, s_logs)
        }
        _ => {
            // Restart
            let s_controller: EndpointController1Restart<i32> =
                choose_mpst_controller_to_all!(s_controller, Branching1fromCtoL::Restart);

            let (s_logs,) = offer_mpst_interleaved!(s_logs, Branching1fromCtoL::Restart);

            let s_controller = s_controller.send(loops - 1)?;
            let (_, s_logs) = s_logs.recv()?;

            rec_loop_0(s_controller, s_logs, loops - 1)
        }
    }
}

/////////////////////////

fn all_mpst() {
    fork_mpst_solo(black_box(start)).unwrap();
}

/////////////////////////

pub fn logging_solo_main(c: &mut Criterion) {
    c.bench_function("Logging solo", |b| b.iter(all_mpst));
}
