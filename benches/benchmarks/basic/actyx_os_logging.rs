#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    choose_mpst_multi_to_all, close_mpst, create_meshedchannels, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_cancel_bundle, fork_mpst_multi, offer_mpst,
};

use std::error::Error;
use std::marker;
use std::time::Duration;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for seven participants
create_meshedchannels!(MeshedChannelsTwo, 2);

// Create Roles
create_multiple_normal_role!(
    Controller, DualController |
    Logs, DualLogs |
);

// Create send
create_send_mpst_cancel_bundle!(
    send_controller_to_logs,
    Logs,
    1 | =>
    Controller,
    MeshedChannelsTwo,
    2
);
create_send_mpst_cancel_bundle!(
    send_logs_to_controller,
    Controller,
    1 | =>
    Logs,
    MeshedChannelsTwo,
    2
);

// Create recv
create_recv_mpst_session_bundle!(
    recv_controller_from_logs,
    Logs,
    1 | =>
    Controller,
    MeshedChannelsTwo,
    2
);
create_recv_mpst_session_bundle!(
    recv_logs_from_controller,
    Controller,
    1 | =>
    Logs,
    MeshedChannelsTwo,
    2
);

// Create close function
close_mpst!(close_mpst_multi, MeshedChannelsTwo, 2);

// Create fork function
fork_mpst_multi!(fork_mpst, MeshedChannelsTwo, 2);

// Names
type NameController = Controller<RoleEnd>;
type NameLogs = Logs<RoleEnd>;

// Controller
enum Branching0fromLtoC<N: marker::Send> {
    Success(MeshedChannelsTwo<Recv<N, Recurs0fromCtoL<N>>, Logs<Logs<RoleEnd>>, NameController>),
    Failure(MeshedChannelsTwo<Recv<N, Choose1fromCtoL<N>>, Logs<RoleBroadcast>, NameController>),
}

type Recurs0fromCtoL<N> = Recv<Branching0fromLtoC<N>, End>;

type Choose1fromCtoL<N> = Send<Branching1fromCtoL<N>, End>;

// Logs
type Choose0fromLtoC<N> = Send<Branching0fromLtoC<N>, End>;

enum Branching1fromCtoL<N: marker::Send> {
    Restart(MeshedChannelsTwo<Recv<N, Choose0fromLtoC<N>>, Controller<RoleBroadcast>, NameLogs>),
    Stop(MeshedChannelsTwo<Recv<N, End>, Controller<RoleEnd>, NameLogs>),
}

type Recurs1fromLtoC<N> = Recv<Branching1fromCtoL<N>, End>;

// Creating the MP sessions
// Controller
type EndpointController0<N> = MeshedChannelsTwo<Recurs0fromCtoL<N>, Logs<RoleEnd>, NameController>;
type EndpointController1<N> = MeshedChannelsTwo<Choose1fromCtoL<N>, RoleBroadcast, NameController>;
type EndpointControllerInit<N> =
    MeshedChannelsTwo<Send<N, Recurs0fromCtoL<N>>, Logs<Logs<RoleEnd>>, NameController>;
// Logs
type EndpointLogs0<N> = MeshedChannelsTwo<Choose0fromLtoC<N>, RoleBroadcast, NameLogs>;
type EndpointLogs1<N> = MeshedChannelsTwo<Recurs1fromLtoC<N>, Controller<RoleEnd>, NameLogs>;
type EndpointLogsInit<N> =
    MeshedChannelsTwo<Recv<N, Choose0fromLtoC<N>>, Controller<RoleBroadcast>, NameLogs>;

fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_controller_to_logs(0, s)?;

    recurs_0_controller(s, 200)
}

fn recurs_0_controller(s: EndpointController0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_controller_from_logs, {
        Branching0fromLtoC::Success(s) => {

            let (_, s) = recv_controller_from_logs(s)?;

            recurs_0_controller(s, loops - 1)
        },
        Branching0fromLtoC::Failure(s) => {

            let (_, s) = recv_controller_from_logs(s)?;

            recurs_1_controller(s, loops - 1)
        },
    })
}

fn recurs_1_controller(s: EndpointController1<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i <= 0 => {
            // Stop
            let s = choose_mpst_multi_to_all!(
                s,
                Branching1fromCtoL::Stop, =>
                Logs, =>
                Controller,
                MeshedChannelsTwo,
                2
            );

            let s = send_controller_to_logs(loops - 1, s)?;

            close_mpst_multi(s)
        }
        _ => {
            // Restart
            let s = choose_mpst_multi_to_all!(
                s,
                Branching1fromCtoL::Restart, =>
                Logs, =>
                Controller,
                MeshedChannelsTwo,
                2
            );

            let s = send_controller_to_logs(loops - 1, s)?;

            recurs_0_controller(s, loops - 1)
        }
    }
}

fn endpoint_logs(s: EndpointLogsInit<i32>) -> Result<(), Box<dyn Error>> {
    let (status, s) = recv_logs_from_controller(s)?;
    recurs_0_logs(s, status)
}

fn recurs_0_logs(s: EndpointLogs0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i <= 0 || i % 2 == 0 => {
            // Failure
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromLtoC::Failure, =>
                Controller, =>
                Logs,
                MeshedChannelsTwo,
                2
            );

            let s = send_logs_to_controller(loops - 1, s)?;

            recurs_1_logs(s)
        }
        _ => {
            // Success
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromLtoC::Success, =>
                Controller, =>
                Logs,
                MeshedChannelsTwo,
                2
            );

            let s = send_logs_to_controller(loops - 1, s)?;

            recurs_0_logs(s, loops - 1)
        }
    }
}

fn recurs_1_logs(s: EndpointLogs1<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_logs_from_controller, {
        Branching1fromCtoL::Restart(s) => {

            let (loops, s) = recv_logs_from_controller(s)?;

            recurs_0_logs(s, loops - 1)
        },
        Branching1fromCtoL::Stop(s) => {

            let (_, s) = recv_logs_from_controller(s)?;

            close_mpst_multi(s)
        },
    })
}

/////////////////////////

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_controller, thread_logs) =
        fork_mpst(black_box(endpoint_controller), black_box(endpoint_logs));

    thread_controller.join()?;
    thread_logs.join()?;

    Ok(())
}

/////////////////////////

fn actyx_os_logging_main(c: &mut Criterion) {
    c.bench_function(&format!("Actyx OS Logging"), |b| b.iter(|| all_mpst()));
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(30, 0))
}

criterion_group! {
    name = actyx_os_logging;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = actyx_os_logging_main
}

criterion_main!(actyx_os_logging);
