use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    choose_mpst_multi_to_all, close_mpst, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, create_sessionmpst,
    fork_mpst_multi, offer_mpst,
};

use mpstthree::role::broadcast::RoleBroadcast;
use rand::{random, thread_rng, Rng};
use std::error::Error;
use std::marker;

// Create new SessionMpst for seven participants
create_sessionmpst!(SessionMpstFour, 4);

// global protocol ActyxOS1(role Controller, role Logs)
// {
//     Start(Int) from Controller to Logs;
//     rec Loop {
//         choice at Logs
//         {
//             Success(int) from Logs to Controller; // Logs is up
//         }
//         or
//         {
//             Failure(Int) from Logs to Controller;
//             Restart(Int) from Controller to Logs;
//         }
//     }
// }

// Create Roles
create_multiple_normal_role!(
    Api, DualAPI |
    Controller, DualController |
    Logs, DualLogs |
    Storage, DualStorage |
);

// Create send
create_send_mpst_session_bundle!(
    send_start_controller_to_logs,
    Logs,
    2 | =>
    Controller,
    SessionMpstFour,
    4
);
create_send_mpst_session_bundle!(
    send_logs_to_api,
    Api,
    1 |
    send_failure_logs_to_controller,
    Controller,
    2 |
    send_logs_to_storage,
    Storage,
    3 | =>
    Logs,
    SessionMpstFour,
    4
);

// Create recv
create_recv_mpst_session_bundle!(
    recv_api_from_logs,
    Logs,
    2 | =>
    Api,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
    recv_start_controller_from_logs,
    Logs,
    2 | =>
    Controller,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
    recv_start_logs_from_controller,
    Controller,
    2 | =>
    Logs,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
    recv_storage_from_logs,
    Logs,
    3 | =>
    Storage,
    SessionMpstFour,
    4
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstFour, 4);

// Create fork function
fork_mpst_multi!(fork_mpst, SessionMpstFour, 4);

// Names
type NameApi = Api<RoleEnd>;
type NameController = Controller<RoleEnd>;
type NameLogs = Logs<RoleEnd>;
type NameStorage = Storage<RoleEnd>;

// Receive choice from Storage
type RecvLogsChoice = Logs<RoleEnd>;

// Api
enum Branching0fromLtoA<N: marker::Send> {
    Up(SessionMpstFour<End, RecursAtoL<N>, End, RecvLogsChoice, NameApi>),
    Down(SessionMpstFour<End, RecursAtoL<N>, End, RecvLogsChoice, NameApi>),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameApi>),
}
type RecursAtoL<N> = Recv<Branching0fromLtoA<N>, End>;

// Controller
enum Branching0fromLtoC<N: marker::Send> {
    Up(SessionMpstFour<End, Recv<N, RecursCtoL<N>>, End, Logs<RecvLogsChoice>, NameController>),
    Down(SessionMpstFour<End, FullRecursCtoL<N>, End, Logs<Logs<RecvLogsChoice>>, NameController>),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameController>),
}
type FullRecursCtoL<N> = Recv<N, Send<N, RecursCtoL<N>>>;
type RecursCtoL<N> = Recv<Branching0fromLtoC<N>, End>;

// Storage
enum Branching0fromLtoS<N: marker::Send> {
    Up(SessionMpstFour<End, End, RecursStoL<N>, RecvLogsChoice, NameStorage>),
    Down(SessionMpstFour<End, End, RecursStoL<N>, RecvLogsChoice, NameStorage>),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameStorage>),
}
type RecursStoL<N> = Recv<Branching0fromLtoS<N>, End>;

// Storage
type Choose0fromLtoA<N> = Send<Branching0fromLtoA<N>, End>;
type Choose0fromLtoC<N> = Send<Branching0fromLtoC<N>, End>;
type Choose0fromLtoS<N> = Send<Branching0fromLtoS<N>, End>;

// Creating the MP sessions
// Api
type EndpointApi<N> = SessionMpstFour<End, RecursAtoL<N>, End, RecvLogsChoice, NameApi>;

// Controller
type EndpointController<N> =
    SessionMpstFour<End, RecursCtoL<N>, End, RecvLogsChoice, NameController>;
type EndpointControllerInit<N> =
    SessionMpstFour<End, Send<N, RecursCtoL<N>>, End, Logs<RecvLogsChoice>, NameController>;

// Storage
type EndpointStorage<N> = SessionMpstFour<End, End, RecursStoL<N>, RecvLogsChoice, NameStorage>;

// Storage
type EndpointLogs<N> = SessionMpstFour<
    Choose0fromLtoA<N>,
    Choose0fromLtoC<N>,
    Choose0fromLtoS<N>,
    RoleBroadcast,
    NameLogs,
>;
type EndpointLogsInit<N> = SessionMpstFour<
    Choose0fromLtoA<N>,
    Recv<N, Choose0fromLtoC<N>>,
    Choose0fromLtoS<N>,
    Controller<RoleBroadcast>,
    NameLogs,
>;

fn endpoint_api(s: EndpointApi<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_api_from_logs, {
        Branching0fromLtoA::Up(s) => {
            endpoint_api(s)
        },
        Branching0fromLtoA::Down(s) => {
            endpoint_api(s)
        },
        Branching0fromLtoA::Close(s) => {
            close_mpst_multi(s)
        },
    })
}

fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
    println!("Send start to Logs: {}", 0);

    let s = send_start_controller_to_logs(0, s);

    recurs_controller(s)
}

fn recurs_controller(s: EndpointController<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_start_controller_from_logs, {
        Branching0fromLtoC::Up(s) => {

            let (success, s) = recv_start_controller_from_logs(s)?;

            println!("Success from Logs: {}", success);

            recurs_controller(s)
        },
        Branching0fromLtoC::Down(s) => {

            let (failure, s) = recv_start_controller_from_logs(s)?;

            println!("Failure from Logs: {}", failure);

            println!("Send restart to Logs: {}", 0);

            let s = send_start_controller_to_logs(0, s);

            recurs_controller(s)
        },
        Branching0fromLtoC::Close(s) => {
            close_mpst_multi(s)
        },
    })
}

fn endpoint_storage(s: EndpointStorage<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_storage_from_logs, {
        Branching0fromLtoS::Up(s) => {
            endpoint_storage(s)
        },
        Branching0fromLtoS::Down(s) => {
            endpoint_storage(s)
        },
        Branching0fromLtoS::Close(s) => {
            close_mpst_multi(s)
        },
    })
}

fn endpoint_logs(s: EndpointLogsInit<i32>) -> Result<(), Box<dyn Error>> {
    let (status, s) = recv_start_logs_from_controller(s)?;
    recurs_logs(s, status, 20)
}

fn recurs_logs(s: EndpointLogs<i32>, status: i32, loops: i32) -> Result<(), Box<dyn Error>> {
    match status {
        0 => {
            // Up
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromLtoA::Up,
                Branching0fromLtoC::Up,
                Branching0fromLtoS::Up, =>
                Api,
                Controller,
                Storage, =>
                Logs,
                SessionMpstFour,
                4,
                3
            );

            let success = random::<i32>();

            println!("Logs is up: {}", success);

            let s = send_failure_logs_to_controller(success, s);

            if loops < 0 {
                recurs_logs(s, 2, loops - 1)
            } else {
                let mut rng = thread_rng();
                let failure: i32 = rng.gen_range(1..=6);

                if failure == 1 {
                    recurs_logs(s, 1, loops - 1)
                } else {
                    recurs_logs(s, 0, loops - 1)
                }
            }
        }
        1 => {
            // Down
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromLtoA::Down,
                Branching0fromLtoC::Down,
                Branching0fromLtoS::Down, =>
                Api,
                Controller,
                Storage, =>
                Logs,
                SessionMpstFour,
                4,
                3
            );

            let failure = random::<i32>();

            println!("Logs is down: {}", failure);

            let s = send_failure_logs_to_controller(failure, s);

            let (response, s) = recv_start_logs_from_controller(s)?;

            let mut rng = thread_rng();
            let failure: i32 = rng.gen_range(1..=6);

            if failure == 1 {
                recurs_logs(s, 1, loops - 1)
            } else {
                recurs_logs(s, response, loops - 1)
            }
        }
        _ => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromLtoA::Close,
                Branching0fromLtoC::Close,
                Branching0fromLtoS::Close, =>
                Api,
                Controller,
                Storage, =>
                Logs,
                SessionMpstFour,
                4,
                3
            );

            println!("Finish protocol");

            close_mpst_multi(s)
        }
    }
}

fn main() {
    println!("Starting protocol");

    let (thread_api, thread_controller, thread_logs, thread_storage) = fork_mpst(
        endpoint_api,
        endpoint_controller,
        endpoint_logs,
        endpoint_storage,
    );

    thread_api.join().unwrap();
    thread_controller.join().unwrap();
    thread_logs.join().unwrap();
    thread_storage.join().unwrap();
}
