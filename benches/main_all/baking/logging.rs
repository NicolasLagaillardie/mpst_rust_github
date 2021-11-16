// #![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum_and_cancel, offer_mpst};

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for two participants
bundle_impl_with_enum_and_cancel!(MeshedChannelsTwo, Controller, Logs);

// Names
type NameRoleController = RoleController<RoleEnd>;
type NameRoleLogs = RoleLogs<RoleEnd>;

// RoleController
enum Branching0fromLtoC {
    Success(
        MeshedChannelsTwo<
            Recv<i32, Recurs0fromCtoL>,
            RoleLogs<RoleLogs<RoleEnd>>,
            NameRoleController,
        >,
    ),
    Failure(
        MeshedChannelsTwo<Recv<i32, Choose1fromCtoL>, RoleLogs<RoleBroadcast>, NameRoleController>,
    ),
}

type Recurs0fromCtoL = Recv<Branching0fromLtoC, End>;

type Choose1fromCtoL = Send<Branching1fromCtoL, End>;

// RoleLogs
type Choose0fromLtoC = Send<Branching0fromLtoC, End>;

enum Branching1fromCtoL {
    Restart(
        MeshedChannelsTwo<Recv<i32, Choose0fromLtoC>, RoleController<RoleBroadcast>, NameRoleLogs>,
    ),
    Stop(MeshedChannelsTwo<Recv<i32, End>, RoleController<RoleEnd>, NameRoleLogs>),
}

type Recurs1fromLtoC = Recv<Branching1fromCtoL, End>;

// Creating the MP sessions
// RoleController
type EndpointController1Stop =
    MeshedChannelsTwo<Send<i32, End>, RoleLogs<RoleEnd>, NameRoleController>;
type EndpointController1Restart =
    MeshedChannelsTwo<Send<i32, Recurs0fromCtoL>, RoleLogs<RoleLogs<RoleEnd>>, NameRoleController>;
type EndpointController0 =
    MeshedChannelsTwo<Recurs0fromCtoL, RoleLogs<RoleEnd>, NameRoleController>;
type EndpointController1 = MeshedChannelsTwo<Choose1fromCtoL, RoleBroadcast, NameRoleController>;
type EndpointControllerInit =
    MeshedChannelsTwo<Send<i32, Recurs0fromCtoL>, RoleLogs<RoleLogs<RoleEnd>>, NameRoleController>;
// RoleLogs
type EndpointLogs0Success =
    MeshedChannelsTwo<Send<i32, Choose0fromLtoC>, RoleController<RoleBroadcast>, NameRoleLogs>;
type EndpointLogs0Failure = MeshedChannelsTwo<
    Send<i32, Recurs1fromLtoC>,
    RoleController<RoleController<RoleEnd>>,
    NameRoleLogs,
>;
type EndpointLogs0 = MeshedChannelsTwo<Choose0fromLtoC, RoleBroadcast, NameRoleLogs>;
type EndpointLogs1 = MeshedChannelsTwo<Recurs1fromLtoC, RoleController<RoleEnd>, NameRoleLogs>;
type EndpointLogsInit =
    MeshedChannelsTwo<Recv<i32, Choose0fromLtoC>, RoleController<RoleBroadcast>, NameRoleLogs>;

fn endpoint_controller(s: EndpointControllerInit) -> Result<(), Box<dyn Error>> {
    let start = thread_rng().gen_range(5..100);

    let s = s.send(start)?;

    recurs_0_controller(s, start)
}

fn recurs_0_controller(s: EndpointController0, loops: i32) -> Result<(), Box<dyn Error>> {
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

fn recurs_1_controller(s: EndpointController1, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i <= 0 => {
            // Stop
            let s: EndpointController1Stop =
                choose_mpst_controller_to_all!(s, Branching1fromCtoL::Stop);

            let s = s.send(loops - 1)?;

            s.close()
        }
        _ => {
            // Restart
            let s: EndpointController1Restart =
                choose_mpst_controller_to_all!(s, Branching1fromCtoL::Restart);

            let s = s.send(loops - 1)?;

            recurs_0_controller(s, loops - 1)
        }
    }
}

fn endpoint_logs(s: EndpointLogsInit) -> Result<(), Box<dyn Error>> {
    let (status, s) = s.recv()?;
    recurs_0_logs(s, status)
}

fn recurs_0_logs(s: EndpointLogs0, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i % 2 == 0 && i > 0 => {
            // Success
            let s: EndpointLogs0Success = choose_mpst_logs_to_all!(s, Branching0fromLtoC::Success);

            let s = s.send(loops - 1)?;

            recurs_0_logs(s, loops - 1)
        }
        _ => {
            // Failure
            let s: EndpointLogs0Failure = choose_mpst_logs_to_all!(s, Branching0fromLtoC::Failure);

            let s = s.send(loops - 1)?;

            recurs_1_logs(s)
        }
    }
}

fn recurs_1_logs(s: EndpointLogs1) -> Result<(), Box<dyn Error>> {
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

fn logging_main(c: &mut Criterion) {
    c.bench_function(&format!("Logging baking"), |b| b.iter(|| all_mpst()));
}

criterion_group! {
    name = logging;
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = logging_main
}

criterion_main!(logging);
