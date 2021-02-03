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

// Complex protocol: supervising API
// G = Storage?Controller[start].µX.(choice at Storage) {
//     starting: API?Controller[start].X // start and check new state
//     up: API!Storage[request].API?Storage[response].X // do stuff
//     down: Storage!Controller[failed].API?Controller[stop].Storage?Controller[start].X // Controller tries to start storage, new state = starting
//     close: 0 // Needed for overflow
// }

// Workaround for the Send<Recv<X>> bug
// G = Storage?Controller[start].µX.(choice at Storage) {
//     starting: API?Controller[start].X // start and check new state
//     up_request: API!Storage[request].X // do stuff
//     up_response: .API?Storage[response].X // do stuff
//     down: Storage!Controller[failed].API?Controller[stop].Storage?Controller[start].X // Controller tries to start storage, new state = starting
//     close: 0 // Needed for overflow
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
    send_failed_api,
    Controller,
    next_controller,
    1, |
    send_api_to_logs,
    Logs,
    next_logs,
    2, |
    send_request_storage,
    Storage,
    next_storage,
    3, | =>
    Api,
    SessionMpstFour,
    4
);
create_send_mpst_session_bundle!(
    send_start_controller_to_api,
    Api,
    next_api,
    1, |
    send_start_controller_to_logs,
    Logs,
    next_logs,
    2, |
    send_start_controller_to_storage,
    Storage,
    next_storage,
    3, | =>
    Controller,
    SessionMpstFour,
    4
);
create_send_mpst_session_bundle!(
    send_logs_to_api,
    Api,
    next_api,
    1, |
    send_logs_to_controller,
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
create_send_mpst_session_bundle!(
    send_response_storage_to_api,
    Api,
    next_api,
    1, |
    send_failure_storage_to_controller,
    Controller,
    next_controller,
    2, |
    send_storage_to_logs,
    Logs,
    next_logs,
    3, | =>
    Storage,
    SessionMpstFour,
    4
);

// Create recv
create_recv_mpst_session_bundle!(
    recv_start_api_from_controller,
    Controller,
    next_controller,
    1, |
    recv_api_from_logs,
    Logs,
    next_logs,
    2, |
    recv_response_api_from_storage,
    Storage,
    next_storage,
    3, | =>
    Api,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
    recv_start_controller_from_api,
    Api,
    next_api,
    1, |
    recv_start_controller_from_logs,
    Logs,
    next_logs,
    2, |
    recv_start_controller_from_storage,
    Storage,
    next_storage,
    3, | =>
    Controller,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
    recv_logs_from_api,
    Api,
    next_api,
    1, |
    recv_logs_from_controller,
    Controller,
    next_controller,
    2, |
    recv_logs_from_storage,
    Storage,
    next_storage,
    3, | =>
    Logs,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
    recv_request_storage_from_api,
    Api,
    next_api,
    1, |
    recv_start_storage_from_controller,
    Controller,
    next_controller,
    2, |
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
type RecvStorageChoice = Storage<RoleEnd>;

// Api
enum BranchingSforA<N: marker::Send> {
    Starting(
        SessionMpstFour<Recv<N, End>, End, RecursAtoS<N>, Controller<RecvStorageChoice>, NameApi>,
    ),
    UpRequest(
        SessionMpstFour<End, End, Send<N, RecursAtoS<N>>, Storage<RecvStorageChoice>, NameApi>,
    ),
    UpResponse(
        SessionMpstFour<End, End, Recv<N, RecursAtoS<N>>, Storage<RecvStorageChoice>, NameApi>,
    ),
    Down(SessionMpstFour<Recv<N, End>, End, RecursAtoS<N>, Controller<RecvStorageChoice>, NameApi>),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameApi>),
}
type RecursAtoS<N> = Recv<BranchingSforA<N>, End>;
// Controller
enum BranchingSforC<N: marker::Send> {
    Starting(
        SessionMpstFour<Send<N, End>, End, RecursCtoS<N>, Api<RecvStorageChoice>, NameController>,
    ),
    UpRequest(SessionMpstFour<End, End, RecursCtoS<N>, RecvStorageChoice, NameController>),
    UpResponse(SessionMpstFour<End, End, RecursCtoS<N>, RecvStorageChoice, NameController>),
    Down(
        SessionMpstFour<
            Send<N, End>,
            End,
            Recv<N, Send<N, RecursCtoS<N>>>,
            Storage<Api<Storage<RecvStorageChoice>>>,
            NameController,
        >,
    ),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameController>),
}
type RecursCtoS<N> = Recv<BranchingSforC<N>, End>;
// Logs
enum BranchingSforL<N: marker::Send> {
    Starting(SessionMpstFour<End, End, RecursLtoS<N>, RecvStorageChoice, NameLogs>),
    UpRequest(SessionMpstFour<End, End, RecursLtoS<N>, RecvStorageChoice, NameLogs>),
    UpResponse(SessionMpstFour<End, End, RecursLtoS<N>, RecvStorageChoice, NameLogs>),
    Down(SessionMpstFour<End, End, RecursLtoS<N>, RecvStorageChoice, NameLogs>),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameLogs>),
}
type RecursLtoS<N> = Recv<BranchingSforL<N>, End>;
// Storage
type ChooseSforAtoS<N> = Send<BranchingSforA<N>, End>;
type ChooseSforCtoS<N> = Send<BranchingSforC<N>, End>;
type ChooseSforLtoS<N> = Send<BranchingSforL<N>, End>;

// Creating the MP sessions
// Api
type EndpointApi<N> = SessionMpstFour<End, End, RecursAtoS<N>, RecvStorageChoice, NameApi>;
// Controller
type EndpointController<N> =
    SessionMpstFour<End, End, RecursCtoS<N>, RecvStorageChoice, NameController>;
type EndpointControllerInit<N> =
    SessionMpstFour<End, End, Send<N, RecursCtoS<N>>, Storage<RecvStorageChoice>, NameController>;
// Logs
type EndpointLogs<N> = SessionMpstFour<End, End, RecursLtoS<N>, RecvStorageChoice, NameLogs>;
// Storage
type EndpointStorage<N> = SessionMpstFour<
    ChooseSforAtoS<N>,
    ChooseSforCtoS<N>,
    ChooseSforLtoS<N>,
    Api<Controller<Logs<RoleEnd>>>,
    NameStorage,
>;
type EndpointStorageInit<N> = SessionMpstFour<
    ChooseSforAtoS<N>,
    Recv<N, ChooseSforCtoS<N>>,
    ChooseSforLtoS<N>,
    Controller<Api<Controller<Logs<RoleEnd>>>>,
    NameStorage,
>;

fn endpoint_api(s: EndpointApi<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_response_api_from_storage, {
        BranchingSforA::Starting(s) => {

            let (start, s) = recv_start_api_from_controller(s)?;

            println!("Start API: {}", start);

            endpoint_api(s)
        },
        BranchingSforA::UpRequest(s) => {

            let request = random::<i32>();

            println!("Request to Storage: {}", request);

            let s = send_request_storage(request, s);

            endpoint_api(s)
        },
        BranchingSforA::UpResponse(s) => {

            let (response, s) = recv_response_api_from_storage(s)?;

            println!("Response from Storage: {}", response);

            endpoint_api(s)
        },
        BranchingSforA::Down(s) => {

            let (stop, s) = recv_start_api_from_controller(s)?;

            println!("Stop API: {}", stop);

            endpoint_api(s)
        },
        BranchingSforA::Close(s) => {
            close_mpst_multi(s)
        },
    })
}

fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
    println!("Send start to Storage: {}", 0);

    let s = send_start_controller_to_storage(0, s);

    recurs_controller(s)
}

fn recurs_controller(s: EndpointController<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_start_controller_from_storage, {
        BranchingSforC::Starting(s) => {
            let start = random::<i32>();

            println!("Send start to API: {}", start);

            let s = send_start_controller_to_api(start, s);

            recurs_controller(s)
        },
        BranchingSforC::UpRequest(s) => {
            recurs_controller(s)
        },
        BranchingSforC::UpResponse(s) => {
            recurs_controller(s)
        },
        BranchingSforC::Down(s) => {

            let (failure, s) = recv_start_controller_from_storage(s)?;

            println!("Failure from Storage: {}", failure);

            let stop = random::<i32>();

            println!("Send stop to API: {}", stop);

            let s = send_start_controller_to_api(stop, s);

            println!("Send start to Storage: {}", 0);

            let s = send_start_controller_to_storage(0, s);
            recurs_controller(s)
        },
        BranchingSforC::Close(s) => {
            close_mpst_multi(s)
        },
    })
}

fn endpoint_logs(s: EndpointLogs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_logs_from_storage, {
        BranchingSforL::Starting(s) => {
            endpoint_logs(s)
        },
        BranchingSforL::UpRequest(s) => {
            endpoint_logs(s)
        },
        BranchingSforL::UpResponse(s) => {
            endpoint_logs(s)
        },
        BranchingSforL::Down(s) => {
            endpoint_logs(s)
        },
        BranchingSforL::Close(s) => {
            close_mpst_multi(s)
        },
    })
}

fn endpoint_storage(s: EndpointStorageInit<i32>) -> Result<(), Box<dyn Error>> {
    let (status, s) = recv_start_storage_from_controller(s)?;
    recurs_storage(s, status, 20, 0)
}

fn recurs_storage(
    s: EndpointStorage<i32>,
    status: i32,
    loops: i32,
    payload: i32,
) -> Result<(), Box<dyn Error>> {
    match status {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_response_storage_to_api,
                send_failure_storage_to_controller,
                send_storage_to_logs, =>
                BranchingSforA::Starting,
                BranchingSforC::Starting,
                BranchingSforL::Starting, =>
                Api,
                Controller,
                Logs, =>
                Storage,
                SessionMpstFour,
                4,
                4
            );

            recurs_storage(s, 1, loops - 1, payload)
        }
        1 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_response_storage_to_api,
                send_failure_storage_to_controller,
                send_storage_to_logs, =>
                BranchingSforA::UpRequest,
                BranchingSforC::UpRequest,
                BranchingSforL::UpRequest, =>
                Api,
                Controller,
                Logs, =>
                Storage,
                SessionMpstFour,
                4,
                4
            );

            let (request, s) = recv_request_storage_from_api(s)?;

            if loops <= 0 {
                recurs_storage(s, 4, loops - 1, request)
            } else {
                let mut rng = thread_rng();
                let failure: i32 = rng.gen_range(1..=6);

                if failure == 1 {
                    recurs_storage(s, 3, loops - 1, request)
                } else {
                    recurs_storage(s, 2, loops - 1, request)
                }
            }
        }
        2 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_response_storage_to_api,
                send_failure_storage_to_controller,
                send_storage_to_logs, =>
                BranchingSforA::UpResponse,
                BranchingSforC::UpResponse,
                BranchingSforL::UpResponse, =>
                Api,
                Controller,
                Logs, =>
                Storage,
                SessionMpstFour,
                4,
                4
            );

            let s = send_response_storage_to_api(-payload, s);

            if loops <= 0 {
                recurs_storage(s, 4, loops - 1, payload)
            } else {
                let mut rng = thread_rng();
                let failure: i32 = rng.gen_range(1..=6);

                if failure == 1 {
                    recurs_storage(s, 3, loops - 1, payload)
                } else {
                    recurs_storage(s, 1, loops - 1, payload)
                }
            }
        }
        3 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_response_storage_to_api,
                send_failure_storage_to_controller,
                send_storage_to_logs, =>
                BranchingSforA::Down,
                BranchingSforC::Down,
                BranchingSforL::Down, =>
                Api,
                Controller,
                Logs, =>
                Storage,
                SessionMpstFour,
                4,
                4
            );

            let failure = random::<i32>();

            println!("Failure of Storage: {}", failure);

            let s = send_failure_storage_to_controller(failure, s);

            let (start, s) = recv_start_storage_from_controller(s)?;

            println!("Receive restart from controller: {}", start);

            recurs_storage(s, start, loops - 1, payload)
        }
        _ => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_response_storage_to_api,
                send_failure_storage_to_controller,
                send_storage_to_logs, =>
                BranchingSforA::Close,
                BranchingSforC::Close,
                BranchingSforL::Close, =>
                Api,
                Controller,
                Logs, =>
                Storage,
                SessionMpstFour,
                4,
                4
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
