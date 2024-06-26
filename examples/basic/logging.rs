use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    choose_mpst_multi_to_all, close_mpst, create_meshedchannels, create_multiple_normal_name,
    create_multiple_normal_role, create_recv_mpst_session_bundle, create_send_mpst_cancel_bundle,
    fork_mpst_multi, offer_mpst,
};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::marker;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for two participants
create_meshedchannels!(MeshedChannels, 2);

// Create Roles
create_multiple_normal_role!(
    Controller, DualController |
    Logs, DualLogs |
);

// Create Names
create_multiple_normal_name!(NameController, NameLogs);

// Create send
create_send_mpst_cancel_bundle!(
    send_controller_to_logs,
    Logs,
    1 | =>
    NameController,
    MeshedChannels,
    2
);
create_send_mpst_cancel_bundle!(
    send_logs_to_controller,
    Controller,
    1 | =>
    NameLogs,
    MeshedChannels,
    2
);

// Create recv
create_recv_mpst_session_bundle!(
    recv_controller_from_logs,
    Logs,
    1 | =>
    NameController,
    MeshedChannels,
    2
);
create_recv_mpst_session_bundle!(
    recv_logs_from_controller,
    Controller,
    1 | =>
    NameLogs,
    MeshedChannels,
    2
);

// Create close function
close_mpst!(close_mpst_multi, MeshedChannels, 2);

// Create fork function
fork_mpst_multi!(fork_mpst, MeshedChannels, 2);

// Controller
enum Branching0fromLtoC<N: marker::Send> {
    Success(MeshedChannels<Recv<N, Recurs0fromCtoL<N>>, Logs<Logs<RoleEnd>>, NameController>),
    Failure(MeshedChannels<Recv<N, Choose1fromCtoL<N>>, Logs<RoleBroadcast>, NameController>),
}

type Recurs0fromCtoL<N> = Recv<Branching0fromLtoC<N>, End>;

type Choose1fromCtoL<N> = Send<Branching1fromCtoL<N>, End>;

// Logs
type Choose0fromLtoC<N> = Send<Branching0fromLtoC<N>, End>;

enum Branching1fromCtoL<N: marker::Send> {
    Restart(MeshedChannels<Recv<N, Choose0fromLtoC<N>>, Controller<RoleBroadcast>, NameLogs>),
    Stop(MeshedChannels<Recv<N, End>, Controller<RoleEnd>, NameLogs>),
}

type Recurs1fromLtoC<N> = Recv<Branching1fromCtoL<N>, End>;

// Creating the MP sessions
// Controller
type EndpointController0<N> = MeshedChannels<Recurs0fromCtoL<N>, Logs<RoleEnd>, NameController>;
type EndpointController1<N> = MeshedChannels<Choose1fromCtoL<N>, RoleBroadcast, NameController>;
type EndpointControllerInit<N> =
    MeshedChannels<Send<N, Recurs0fromCtoL<N>>, Logs<Logs<RoleEnd>>, NameController>;

// Logs
type EndpointLogs0<N> = MeshedChannels<Choose0fromLtoC<N>, RoleBroadcast, NameLogs>;
type EndpointLogs1<N> = MeshedChannels<Recurs1fromLtoC<N>, Controller<RoleEnd>, NameLogs>;
type EndpointLogsInit<N> =
    MeshedChannels<Recv<N, Choose0fromLtoC<N>>, Controller<RoleBroadcast>, NameLogs>;

fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
    let start: i32 = thread_rng().gen_range(5..100);

    let s = send_controller_to_logs(start, s)?;

    recurs_0_controller(s, start)
}

fn recurs_0_controller(s: EndpointController0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_controller_from_logs, {
        Branching0fromLtoC::Success(s) => {

            let (_, s) = recv_controller_from_logs(s)?;

            recurs_0_controller(s, loops)
        },
        Branching0fromLtoC::Failure(s) => {

            let (_, s) = recv_controller_from_logs(s)?;

            recurs_1_controller(s, loops)
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
                NameController,
                MeshedChannels,
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
                NameController,
                MeshedChannels,
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
        i if i % 2 == 0 && i > 0 => {
            // Success
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromLtoC::Success, =>
                NameLogs,
                MeshedChannels,
                2
            );

            let s = send_logs_to_controller(loops - 1, s)?;

            recurs_0_logs(s, loops - 1)
        }
        _ => {
            // Failure
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromLtoC::Failure, =>
                NameLogs,
                MeshedChannels,
                2
            );

            let s = send_logs_to_controller(loops - 1, s)?;

            recurs_1_logs(s)
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

fn main() {
    let (thread_controller, thread_logs) = fork_mpst(endpoint_controller, endpoint_logs);

    thread_controller.join().unwrap();
    thread_logs.join().unwrap();
}
