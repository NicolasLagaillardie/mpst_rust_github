#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_impl_with_enum_and_cancel, close_mpst_interleaved, fork_mpst_multi_solo,
    offer_mpst_interleaved,
};

use rand::random;

use std::error::Error;
use std::marker;
// use std::time::Duration;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for two participants
bundle_impl_with_enum_and_cancel!(
    MeshedChannelsTwo =>
    Controller,
    Logs
);

close_mpst_interleaved!(close_mpst, MeshedChannelsTwo, 2);

fork_mpst_multi_solo!(fork_mpst, MeshedChannelsTwo, 2);

// Names
type NameRoleController = RoleController<RoleEnd>;
type NameRoleLogs = RoleLogs<RoleEnd>;

// RoleController
enum Branching0fromLtoC<N: marker::Send> {
    Success(
        MeshedChannelsTwo<
            Recv<N, Recurs0fromCtoL<N>>,
            RoleLogs<RoleLogs<RoleEnd>>,
            NameRoleController,
        >,
    ),
    Failure(
        MeshedChannelsTwo<Recv<N, Choose1fromCtoL<N>>, RoleLogs<RoleBroadcast>, NameRoleController>,
    ),
}

type Recurs0fromCtoL<N> = Recv<Branching0fromLtoC<N>, End>;

type Choose1fromCtoL<N> = Send<Branching1fromCtoL<N>, End>;

// RoleLogs
type Choose0fromLtoC<N> = Send<Branching0fromLtoC<N>, End>;

enum Branching1fromCtoL<N: marker::Send> {
    Restart(
        MeshedChannelsTwo<Recv<N, Choose0fromLtoC<N>>, RoleController<RoleBroadcast>, NameRoleLogs>,
    ),
    Stop(MeshedChannelsTwo<Recv<N, End>, RoleController<RoleEnd>, NameRoleLogs>),
}

type Recurs1fromLtoC<N> = Recv<Branching1fromCtoL<N>, End>;

// Creating the MP sessions
// RoleController
type EndpointController1Stop<N> =
    MeshedChannelsTwo<Send<N, End>, RoleLogs<RoleEnd>, NameRoleController>;
type EndpointController1Restart<N> =
    MeshedChannelsTwo<Send<N, Recurs0fromCtoL<N>>, RoleLogs<RoleLogs<RoleEnd>>, NameRoleController>;
type EndpointController0<N> =
    MeshedChannelsTwo<Recurs0fromCtoL<N>, RoleLogs<RoleEnd>, NameRoleController>;
type EndpointController1<N> =
    MeshedChannelsTwo<Choose1fromCtoL<N>, RoleBroadcast, NameRoleController>;
type EndpointControllerInit<N> =
    MeshedChannelsTwo<Send<N, Recurs0fromCtoL<N>>, RoleLogs<RoleLogs<RoleEnd>>, NameRoleController>;
// RoleLogs
type EndpointLogs0Success<N> =
    MeshedChannelsTwo<Send<N, Choose0fromLtoC<N>>, RoleController<RoleBroadcast>, NameRoleLogs>;
type EndpointLogs0Failure<N> = MeshedChannelsTwo<
    Send<N, Recurs1fromLtoC<N>>,
    RoleController<RoleController<RoleEnd>>,
    NameRoleLogs,
>;
type EndpointLogs0<N> = MeshedChannelsTwo<Choose0fromLtoC<N>, RoleBroadcast, NameRoleLogs>;
type EndpointLogs1<N> =
    MeshedChannelsTwo<Recurs1fromLtoC<N>, RoleController<RoleEnd>, NameRoleLogs>;
type EndpointLogsInit<N> =
    MeshedChannelsTwo<Recv<N, Choose0fromLtoC<N>>, RoleController<RoleBroadcast>, NameRoleLogs>;

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

fn all_mpst() -> Result<(), Box<dyn std::error::Error>> {
    fork_mpst(black_box(start))
}

/////////////////////////

fn logging_interleaved_main(c: &mut Criterion) {
    c.bench_function(&format!("Logging interleaved"), |b| b.iter(|| all_mpst()));
}

// fn long_warmup() -> Criterion {
//     Criterion::default().measurement_time(Duration::new(30, 0))
// }

criterion_group! {
    name = logging_interleaved;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = logging_interleaved_main
}

criterion_main!(logging_interleaved);
