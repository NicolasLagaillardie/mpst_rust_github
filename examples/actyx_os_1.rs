use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose_mpst_multi_to_all, close_mpst, create_normal_role,
    create_recv_mpst_session, create_recv_mpst_session_bundle, create_send_mpst_session,
    create_send_mpst_session_bundle, create_sessionmpst, offer_mpst,
};

use rand::{random, thread_rng, Rng};
use std::error::Error;
use std::marker;

// Create new SessionMpst for seven participants
create_sessionmpst!(SessionMpstFour, 4);

// Simple protocol: logging service
// G = Logs?Controller[start].µX.(choice at Logs) {
//     up: X // do stuff
//     down: Logs!Controller[failed].Logs?Controller[start].X // Logging service waits before starting again
// }

// Create Roles
create_normal_role!(Api, next_api, DualAPI, next_dual_api); // #1
create_normal_role!(
    Controller,
    next_controller,
    DualController,
    next_dual_controller
); // #2
create_normal_role!(Logs, next_logs, DualLogs, next_dual_logs); // #3
create_normal_role!(Storage, next_storage, DualStorage, next_dual_storage); // #4

// Create send
create_send_mpst_session_bundle!(
    send_start_controller_to_logs,
    Logs,
    next_logs,
    2, | =>
    Controller,
    SessionMpstFour,
    4
);
create_send_mpst_session_bundle!(
    send_logs_to_api,
    Api,
    next_api,
    1, |
    send_failure_logs_to_controller,
    Controller,
    next_controller,
    2, |
    send_logs_to_storage,
    Storage,
    next_storage,
    3, | =>
    Logs,
    SessionMpstFour,
    4
);

// Create recv
create_recv_mpst_session_bundle!(
    recv_api_from_logs,
    Logs,
    next_logs,
    2, | =>
    Api,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
    recv_start_controller_from_logs,
    Logs,
    next_logs,
    2, | =>
    Controller,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
    recv_start_logs_from_controller,
    Controller,
    next_controller,
    2, | =>
    Logs,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
    recv_storage_from_logs,
    Logs,
    next_logs,
    3, | =>
    Storage,
    SessionMpstFour,
    4
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstFour, 4);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstFour, 4);

// Names
type NameApi = Api<RoleEnd>;
type NameController = Controller<RoleEnd>;
type NameLogs = Logs<RoleEnd>;
type NameStorage = Storage<RoleEnd>;

// Receive choice from Storage
type RecvLogsChoice = Logs<RoleEnd>;

// Api
enum BranchingLforA<N: marker::Send> {
    Up(SessionMpstFour<End, RecursAtoL<N>, End, RecvLogsChoice, NameApi>),
    Down(SessionMpstFour<End, RecursAtoL<N>, End, RecvLogsChoice, NameApi>),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameApi>),
}
type RecursAtoL<N> = Recv<BranchingLforA<N>, End>;
// Controller
enum BranchingLforC<N: marker::Send> {
    Up(SessionMpstFour<End, RecursCtoL<N>, End, RecvLogsChoice, NameController>),
    Down(
        SessionMpstFour<
            End,
            Recv<N, Send<N, RecursCtoL<N>>>,
            End,
            Logs<Logs<RecvLogsChoice>>,
            NameController,
        >,
    ),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameController>),
}
type RecursCtoL<N> = Recv<BranchingLforC<N>, End>;
// Storage
enum BranchingLforS<N: marker::Send> {
    Up(SessionMpstFour<End, End, RecursStoL<N>, RecvLogsChoice, NameStorage>),
    Down(SessionMpstFour<End, End, RecursStoL<N>, RecvLogsChoice, NameStorage>),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameStorage>),
}
type RecursStoL<N> = Recv<BranchingLforS<N>, End>;
// Storage
type ChooseLforAtoL<N> = Send<BranchingLforA<N>, End>;
type ChooseLforCtoL<N> = Send<BranchingLforC<N>, End>;
type ChooseLforStoL<N> = Send<BranchingLforS<N>, End>;

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
    ChooseLforAtoL<N>,
    ChooseLforCtoL<N>,
    ChooseLforStoL<N>,
    Api<Controller<Storage<RoleEnd>>>,
    NameLogs,
>;
type EndpointLogsInit<N> = SessionMpstFour<
    ChooseLforAtoL<N>,
    Recv<N, ChooseLforCtoL<N>>,
    ChooseLforStoL<N>,
    Controller<Api<Controller<Storage<RoleEnd>>>>,
    NameLogs,
>;

fn endpoint_api(s: EndpointApi<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_api_from_logs, {
        BranchingLforA::Up(s) => {
            endpoint_api(s)
        },
        BranchingLforA::Down(s) => {
            endpoint_api(s)
        },
        BranchingLforA::Close(s) => {
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
        BranchingLforC::Up(s) => {
            recurs_controller(s)
        },
        BranchingLforC::Down(s) => {

            let (failure, s) = recv_start_controller_from_logs(s)?;

            println!("Failure from Logs: {}", failure);

            println!("Send restart to Logs: {}", 0);

            let s = send_start_controller_to_logs(0, s);

            recurs_controller(s)
        },
        BranchingLforC::Close(s) => {
            close_mpst_multi(s)
        },
    })
}

fn endpoint_storage(s: EndpointStorage<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_storage_from_logs, {
        BranchingLforS::Up(s) => {
            endpoint_storage(s)
        },
        BranchingLforS::Down(s) => {
            endpoint_storage(s)
        },
        BranchingLforS::Close(s) => {
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
                send_logs_to_api,
                send_failure_logs_to_controller,
                send_logs_to_storage, =>
                BranchingLforA::Up,
                BranchingLforC::Up,
                BranchingLforS::Up, =>
                Api,
                Controller,
                Storage, =>
                Logs,
                SessionMpstFour,
                4,
                3
            );

            println!("Logs is up: {}", random::<i32>());

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
                send_logs_to_api,
                send_failure_logs_to_controller,
                send_logs_to_storage, =>
                BranchingLforA::Down,
                BranchingLforC::Down,
                BranchingLforS::Down, =>
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
                send_logs_to_api,
                send_failure_logs_to_controller,
                send_logs_to_storage, =>
                BranchingLforA::Close,
                BranchingLforC::Close,
                BranchingLforS::Close, =>
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
