use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_fork_multi, choose_mpst_multi_to_all, close_mpst, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, create_sessionmpst,
    offer_mpst,
};

use rand::{random, thread_rng, Rng};
use std::error::Error;
use std::marker;

// Create new SessionMpst for seven participants
create_sessionmpst!(SessionMpstFour, 4);

// global protocol ActyxOS2(role Api, role Controller, role Storage)
// {
//     Start(Int) from Controller to Storage;
//
//     rec Loop {
//         choice at Storage
//         {
//             Up(Int) from Storage to Controller;
//             Start(Int) from Controller to Api; // If Api already started, useless
//
//             Resquest(Int) from Api to Storage;
//
//             choice at Storage
//             {
//                 HardPing(Int) from Storage to Controller;
//                 Response(Int) from Storage to Api;
//             }
//             or
//             {
//                 Failure(Int) from Storage to Controller;
//                 Stop(Int) from Controller to Api;
//                 Restart(Int) from Controller to Storage;
//             }
//         }
//         or
//         {
//             Failure(Int) from Storage to Controller;
//             Stop(Int) from Controller to Api;
//             Restart(Int) from Controller to Storage;
//         }
//     }
// }

// Create Roles
create_multiple_normal_role!(
    Api, next_api, DualAPI, next_dual_api |
    Controller,
    next_controller,
    DualController,
    next_dual_controller |
    Logs, next_logs, DualLogs, next_dual_logs |
    Storage, next_storage, DualStorage, next_dual_storage |
);

// Create send
create_send_mpst_session_bundle!(
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
    send_start_controller_to_storage,
    Storage,
    next_storage,
    3, | =>
    Controller,
    SessionMpstFour,
    4
);
create_send_mpst_session_bundle!(
    send_response_storage_to_api,
    Api,
    next_api,
    1, |
    send_new_status_storage_to_controller,
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
    recv_response_api_from_storage,
    Storage,
    next_storage,
    3, | =>
    Api,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
    recv_new_status_controller_from_storage,
    Storage,
    next_storage,
    3, | =>
    Controller,
    SessionMpstFour,
    4
);
create_recv_mpst_session_bundle!(
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
    2, | =>
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
enum Branching0fromStoA<N: marker::Send> {
    Up(
        SessionMpstFour<
            Recv<N, End>,
            End,
            Send<N, Recurs1fromAtoS<N>>,
            Controller<Storage<RecvStorageChoice>>,
            NameApi,
        >,
    ),
    Down(
        SessionMpstFour<
            Recv<N, End>,
            End,
            Recurs0fromAtoS<N>,
            Controller<RecvStorageChoice>,
            NameApi,
        >,
    ),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameApi>),
}
type Recurs0fromAtoS<N> = Recv<Branching0fromStoA<N>, End>;
enum Branching1fromStoA<N: marker::Send> {
    Request(
        SessionMpstFour<End, End, Recv<N, Recurs0fromAtoS<N>>, Storage<RecvStorageChoice>, NameApi>,
    ),
    Down(
        SessionMpstFour<
            Recv<N, End>,
            End,
            Recurs0fromAtoS<N>,
            Controller<RecvStorageChoice>,
            NameApi,
        >,
    ),
}
type Recurs1fromAtoS<N> = Recv<Branching1fromStoA<N>, End>;
// Controller
enum Branching0fromStoC<N: marker::Send> {
    Up(
        SessionMpstFour<
            Send<N, End>,
            End,
            Recv<N, Recurs1fromCtoS<N>>,
            Storage<Api<RecvStorageChoice>>,
            NameController,
        >,
    ),
    Down(
        SessionMpstFour<
            Send<N, End>,
            End,
            Recv<N, Send<N, Recurs0fromCtoS<N>>>,
            Storage<Api<Storage<RecvStorageChoice>>>,
            NameController,
        >,
    ),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameController>),
}
type Recurs0fromCtoS<N> = Recv<Branching0fromStoC<N>, End>;
enum Branching1fromStoC<N: marker::Send> {
    Up(
        SessionMpstFour<
            End,
            End,
            Recv<N, Recurs0fromCtoS<N>>,
            Storage<RecvStorageChoice>,
            NameController,
        >,
    ),
    Down(
        SessionMpstFour<
            Send<N, End>,
            End,
            Recv<N, Send<N, Recurs0fromCtoS<N>>>,
            Storage<Api<Storage<RecvStorageChoice>>>,
            NameController,
        >,
    ),
}
type Recurs1fromCtoS<N> = Recv<Branching1fromStoC<N>, End>;
// Logs
enum Branching0fromStoL<N: marker::Send> {
    Up(SessionMpstFour<End, End, Recurs1fromLtoS<N>, RecvStorageChoice, NameLogs>),
    Down(SessionMpstFour<End, End, Recurs0fromLtoS<N>, RecvStorageChoice, NameLogs>),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameLogs>),
}
type Recurs0fromLtoS<N> = Recv<Branching0fromStoL<N>, End>;
enum Branching1fromStoL<N: marker::Send> {
    Up(SessionMpstFour<End, End, Recurs0fromLtoS<N>, RecvStorageChoice, NameLogs>),
    Down(SessionMpstFour<End, End, Recurs0fromLtoS<N>, RecvStorageChoice, NameLogs>),
}
type Recurs1fromLtoS<N> = Recv<Branching1fromStoL<N>, End>;
// Storage
type Choose0fromStoA<N> = Send<Branching0fromStoA<N>, End>;
type Choose0fromStoC<N> = Send<Branching0fromStoC<N>, End>;
type Choose0fromStoL<N> = Send<Branching0fromStoL<N>, End>;
type Choose1fromStoA<N> = Send<Branching1fromStoA<N>, End>;
type Choose1fromStoC<N> = Send<Branching1fromStoC<N>, End>;
type Choose1fromStoL<N> = Send<Branching1fromStoL<N>, End>;

// Creating the MP sessions
// Api
type NestedApi<N> = SessionMpstFour<End, End, Recurs1fromAtoS<N>, RecvStorageChoice, NameApi>;
type EndpointApi<N> = SessionMpstFour<End, End, Recurs0fromAtoS<N>, RecvStorageChoice, NameApi>;
// Controller
type NestedController<N> =
    SessionMpstFour<End, End, Recurs1fromCtoS<N>, RecvStorageChoice, NameController>;
type EndpointController<N> =
    SessionMpstFour<End, End, Recurs0fromCtoS<N>, RecvStorageChoice, NameController>;
type EndpointControllerInit<N> = SessionMpstFour<
    End,
    End,
    Send<N, Recurs0fromCtoS<N>>,
    Storage<RecvStorageChoice>,
    NameController,
>;
// Logs
type NestedLogs<N> = SessionMpstFour<End, End, Recurs1fromLtoS<N>, RecvStorageChoice, NameLogs>;
type EndpointLogs<N> = SessionMpstFour<End, End, Recurs0fromLtoS<N>, RecvStorageChoice, NameLogs>;
// Storage
type NestedStorage<N> = SessionMpstFour<
    Choose1fromStoA<N>,
    Choose1fromStoC<N>,
    Choose1fromStoL<N>,
    Api<Controller<Logs<RoleEnd>>>,
    NameStorage,
>;
type EndpointStorage<N> = SessionMpstFour<
    Choose0fromStoA<N>,
    Choose0fromStoC<N>,
    Choose0fromStoL<N>,
    Api<Controller<Logs<RoleEnd>>>,
    NameStorage,
>;
type EndpointStorageInit<N> = SessionMpstFour<
    Choose0fromStoA<N>,
    Recv<N, Choose0fromStoC<N>>,
    Choose0fromStoL<N>,
    Controller<Api<Controller<Logs<RoleEnd>>>>,
    NameStorage,
>;

/////////////////////////

fn endpoint_api(s: EndpointApi<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_response_api_from_storage, {
        Branching0fromStoA::Up(s) => {

            let (start, s) = recv_start_api_from_controller(s)?;

            println!("Start API: {}", start);

            let request = random::<i32>();

            println!("Request to Storage: {}", request);

            let s = send_request_storage(request, s);

            nested_api(s)
        },
        Branching0fromStoA::Down(s) => {

            let (stop, s) = recv_start_api_from_controller(s)?;

            println!("Stop API: {}", stop);

            endpoint_api(s)
        },
        Branching0fromStoA::Close(s) => {
            close_mpst_multi(s)
        },
    })
}

fn nested_api(s: NestedApi<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_response_api_from_storage, {
        Branching1fromStoA::Request(s) => {

            let (response, s) = recv_response_api_from_storage(s)?;

            println!("Response from Storage: {}", response);

            endpoint_api(s)
        },
        Branching1fromStoA::Down(s) => {

            let (stop, s) = recv_start_api_from_controller(s)?;

            println!("Stop API: {}", stop);

            endpoint_api(s)
        },
    })
}

fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
    println!("Send start to Storage: {}", 0);

    let s = send_start_controller_to_storage(0, s);

    recurs_controller(s)
}

fn recurs_controller(s: EndpointController<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_new_status_controller_from_storage, {
        Branching0fromStoC::Up(s) => {

            let (success, s) = recv_new_status_controller_from_storage(s)?;

            println!("Storage successfuly restarted: {}", success);

            let start = random::<i32>();

            println!("Send start to API: {}", start);

            let s = send_start_controller_to_api(start, s);

            nested_controller(s)
        },
        Branching0fromStoC::Down(s) => {

            let (failure, s) = recv_new_status_controller_from_storage(s)?;

            println!("Failure from Storage: {}", failure);

            let stop = random::<i32>();

            println!("Send stop to API: {}", stop);

            let s = send_start_controller_to_api(stop, s);

            println!("Send start to Storage: {}", 0);

            let s = send_start_controller_to_storage(0, s);
            recurs_controller(s)
        },
        Branching0fromStoC::Close(s) => {
            close_mpst_multi(s)
        },
    })
}

fn nested_controller(s: NestedController<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_new_status_controller_from_storage, {
        Branching1fromStoC::Up(s) => {

            let (success, s) = recv_new_status_controller_from_storage(s)?;

            println!("Storage successfuly restarted: {}", success);

            recurs_controller(s)
        },
        Branching1fromStoC::Down(s) => {

            let (failure, s) = recv_new_status_controller_from_storage(s)?;

            println!("Failure from Storage: {}", failure);

            let stop = random::<i32>();

            println!("Send stop to API: {}", stop);

            let s = send_start_controller_to_api(stop, s);

            println!("Send start to Storage: {}", 0);

            let s = send_start_controller_to_storage(0, s);

            recurs_controller(s)
        },
    })
}

fn endpoint_logs(s: EndpointLogs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_logs_from_storage, {
        Branching0fromStoL::Up(s) => {
            nested_logs(s)
        },
        Branching0fromStoL::Down(s) => {
            endpoint_logs(s)
        },
        Branching0fromStoL::Close(s) => {
            close_mpst_multi(s)
        },
    })
}

fn nested_logs(s: NestedLogs<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_logs_from_storage, {
        Branching1fromStoL::Up(s) => {
            endpoint_logs(s)
        },
        Branching1fromStoL::Down(s) => {
            endpoint_logs(s)
        },
    })
}

fn endpoint_storage(s: EndpointStorageInit<i32>) -> Result<(), Box<dyn Error>> {
    let (status, s) = recv_start_storage_from_controller(s)?;
    recurs_storage(s, status, 5, 0)
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
                send_new_status_storage_to_controller,
                send_storage_to_logs, =>
                Branching0fromStoA::Up,
                Branching0fromStoC::Up,
                Branching0fromStoL::Up, =>
                Api,
                Controller,
                Logs, =>
                Storage,
                SessionMpstFour,
                4,
                4
            );

            let success = random::<i32>();

            println!("Storage restarted: {}", success);

            let s = send_new_status_storage_to_controller(success, s);

            let (request, s) = recv_request_storage_from_api(s)?;

            if loops < 0 {
                nested_storage(s, 1, loops - 1, request)
            } else {
                let mut rng = thread_rng();
                let failure: i32 = rng.gen_range(1..=6);

                if failure == 1 {
                    nested_storage(s, 1, loops - 1, 0)
                } else {
                    nested_storage(s, 0, loops - 1, request)
                }
            }
        }
        1 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_response_storage_to_api,
                send_new_status_storage_to_controller,
                send_storage_to_logs, =>
                Branching0fromStoA::Down,
                Branching0fromStoC::Down,
                Branching0fromStoL::Down, =>
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

            let s = send_new_status_storage_to_controller(failure, s);

            let (restart, s) = recv_start_storage_from_controller(s)?;

            println!("Receive restart Storage from controller: {}", restart);

            if loops < 0 {
                recurs_storage(s, 2, loops - 1, payload)
            } else {
                let mut rng = thread_rng();
                let failure: i32 = rng.gen_range(1..=6);

                if failure == 1 {
                    recurs_storage(s, 1, loops - 1, 0)
                } else {
                    recurs_storage(s, restart, loops - 1, payload)
                }
            }
        }
        _ => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_response_storage_to_api,
                send_new_status_storage_to_controller,
                send_storage_to_logs, =>
                Branching0fromStoA::Close,
                Branching0fromStoC::Close,
                Branching0fromStoL::Close, =>
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

fn nested_storage(
    s: NestedStorage<i32>,
    status: i32,
    loops: i32,
    payload: i32,
) -> Result<(), Box<dyn Error>> {
    match status {
        0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_response_storage_to_api,
                send_new_status_storage_to_controller,
                send_storage_to_logs, =>
                Branching1fromStoA::Request,
                Branching1fromStoC::Up,
                Branching1fromStoL::Up, =>
                Api,
                Controller,
                Logs, =>
                Storage,
                SessionMpstFour,
                4,
                4
            );

            let ping = random::<i32>();

            println!("Send hard ping: {}", ping);

            let s = send_new_status_storage_to_controller(ping, s);

            let s = send_response_storage_to_api(-payload, s);

            recurs_storage(s, 0, loops - 1, payload)
        }
        _ => {
            let s = choose_mpst_multi_to_all!(
                s,
                send_response_storage_to_api,
                send_new_status_storage_to_controller,
                send_storage_to_logs, =>
                Branching1fromStoA::Down,
                Branching1fromStoC::Down,
                Branching1fromStoL::Down, =>
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

            let s = send_new_status_storage_to_controller(failure, s);

            let (start, s) = recv_start_storage_from_controller(s)?;

            println!("Receive restart Storage from controller: {}", start);

            if loops < 0 {
                recurs_storage(s, 2, loops - 1, payload)
            } else {
                let mut rng = thread_rng();
                let failure: i32 = rng.gen_range(1..=6);

                if failure == 1 {
                    recurs_storage(s, 1, loops - 1, 0)
                } else {
                    recurs_storage(s, start, loops - 1, payload)
                }
            }
        }
    }
}

/////////////////////////

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
