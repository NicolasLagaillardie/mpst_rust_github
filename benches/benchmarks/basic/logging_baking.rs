#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum_and_cancel, offer_mpst};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::marker;
// use std::time::Duration;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for two participants
bundle_impl_with_enum_and_cancel!(MeshedChannelsTwo, Controller, Logs);

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

fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
    let start = thread_rng().gen_range(5..100);

    let s = s.send(start)?;

    recurs_0_controller(s, start)
}

fn recurs_0_controller(s: EndpointController0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromLtoC::Success(s) => {

            let (_, s) = s.recv()?;

            recurs_0_controller(s, loops)
        },
        Branching0fromLtoC::Failure(s) => {

            let (_, s) = s.recv()?;

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

            let s = s.send(loops - 1)?;

            s.close()
        }
        _ => {
            // Restart
            let s: EndpointController1Restart<i32> =
                choose_mpst_controller_to_all!(s, Branching1fromCtoL::Restart);

            let s = s.send(loops - 1)?;

            recurs_0_controller(s, loops - 1)
        }
    }
}

fn endpoint_logs(s: EndpointLogsInit<i32>) -> Result<(), Box<dyn Error>> {
    let (status, s) = s.recv()?;
    recurs_0_logs(s, status)
}

fn recurs_0_logs(s: EndpointLogs0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i % 2 == 0 && i > 0 => {
            // Success
            let s: EndpointLogs0Success<i32> =
                choose_mpst_logs_to_all!(s, Branching0fromLtoC::Success);

            let s = s.send(loops - 1)?;

            recurs_0_logs(s, loops - 1)
        }
        _ => {
            // Failure
            let s: EndpointLogs0Failure<i32> =
                choose_mpst_logs_to_all!(s, Branching0fromLtoC::Failure);

            let s = s.send(loops - 1)?;

            recurs_1_logs(s)
        }
    }
}

fn recurs_1_logs(s: EndpointLogs1<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromCtoL::Restart(s) => {

            let (loops, s) = s.recv()?;

            recurs_0_logs(s, loops - 1)
        },
        Branching1fromCtoL::Stop(s) => {

            let (_, s) = s.recv()?;

            s.close()
        },
    })
}

/////////////////////////

fn all_mpst() {
    let (thread_controller, thread_logs) =
        fork_mpst(black_box(endpoint_controller), black_box(endpoint_logs));

    thread_controller.join().unwrap();
    thread_logs.join().unwrap();
}

/////////////////////////

fn logging_baking_main(c: &mut Criterion) {
    c.bench_function(&format!("Logging baking"), |b| b.iter(|| all_mpst()));
}

// fn long_warmup() -> Criterion {
//     Criterion::default().measurement_time(Duration::new(30, 0))
// }

criterion_group! {
    name = logging_baking;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = logging_baking_main
}

criterion_main!(logging_baking);
