use mpstthree::binary::End;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_fork_multi, close_mpst, create_normal_role, create_recv_mpst_session,
    create_recv_mpst_session_bundle, create_send_mpst_session, create_send_mpst_session_bundle,
    create_sessionmpst,
};

use std::error::Error;

// Create new SessionMpst for seven participants
create_sessionmpst!(SessionMpstSeven, 7);

// Clients needed???
//
// Simple protocol: logging service
// G = Logs?Controller[start].µX.(choice at Logs) {
//     up: X // do stuff
//     down: Logs!Controller[failed].Logs?Controller[start].X // Logging service waits before starting again
// }
//
// Complex protocol: supervising API
// G = Storage?Controller[start].µX.(choice at Storage) {
//     starting: API?Controller[start].X // start and check new state
//     up: API!Storage[request].API?Storage[response].X // do stuff
//     down: Storage!Controller[failed].API?Controller[stop].Storage?Controller[start].X // Controller tries to start storage, new state = starting
// }

// Create Roles
create_normal_role!(Api, next_api, DualAPI, next_dual_api); // #1
create_normal_role!(
    ClientApi,
    next_client_api,
    DualClientApi,
    next_dual_client_api
); // #2
create_normal_role!(
    ClientController,
    next_client_controller,
    DualClientController,
    next_dual_client_controller
); // #3
create_normal_role!(
    ClientStorage,
    next_client_storage,
    DualClientStorage,
    next_dual_client_storage
); // #4
create_normal_role!(
    Controller,
    next_controller,
    DualController,
    next_dual_controller
); // #5
create_normal_role!(Logs, next_logs, DualLogs, next_dual_logs); // #6
create_normal_role!(Storage, next_storage, DualStorage, next_dual_storage); // #7

// Create send
create_send_mpst_session_bundle!(
    send_http_api_to_client_api,
    ClientApi,
    next_client_api,
    1, |
    send_api_to_client_controller,
    ClientController,
    next_client_controller,
    2, |
    send_api_to_client_storage,
    ClientStorage,
    next_client_storage,
    3, |
    send_failed_api,
    Controller,
    next_controller,
    4, |
    send_api_to_logs,
    Logs,
    next_logs,
    5, |
    send_request_storage,
    Storage,
    next_storage,
    6, | =>
    Api,
    SessionMpstSeven,
    7
);
create_send_mpst_session_bundle!(
    send_client_api_to_api,
    Api,
    next_api,
    1, |
    send_client_api_to_client_controller,
    ClientController,
    next_client_controller,
    2, |
    send_client_api_to_client_storage,
    ClientStorage,
    next_client_storage,
    3, |
    send_client_api_to_controller,
    Controller,
    next_controller,
    4, |
    send_client_api_to_logs,
    Logs,
    next_logs,
    5, |
    send_client_api_to_storage,
    Storage,
    next_storage,
    6, | =>
    ClientApi,
    SessionMpstSeven,
    7
);
create_send_mpst_session_bundle!(
    send_client_controller_to_api,
    Api,
    next_api,
    1, |
    send_client_controller_to_client_api,
    ClientApi,
    next_client_api,
    2, |
    send_client_controller_to_client_storage,
    ClientStorage,
    next_client_storage,
    3, |
    send_http_client_controller_to_controller,
    Controller,
    next_controller,
    4, |
    send_client_controller_to_logs,
    Logs,
    next_logs,
    5, |
    send_client_controller_to_storage,
    Storage,
    next_storage,
    6, | =>
    ClientController,
    SessionMpstSeven,
    7
);
create_send_mpst_session_bundle!(
    send_client_storage_to_api,
    Api,
    next_api,
    1, |
    send_client_storage_to_client_api,
    ClientApi,
    next_client_api,
    2, |
    send_client_storage_to_client_controller,
    ClientController,
    next_client_controller,
    3, |
    send_client_storage_to_controller,
    Controller,
    next_controller,
    4, |
    send_client_storage_to_logs,
    Logs,
    next_logs,
    5, |
    send_ipfs_client_storage_to_storage,
    Storage,
    next_storage,
    6, | =>
    ClientStorage,
    SessionMpstSeven,
    7
);
create_send_mpst_session_bundle!(
    send_start_controller_to_api,
    Api,
    next_api,
    1, |
    send_controller_to_client_api,
    ClientApi,
    next_client_api,
    2, |
    send_http_controller_to_client_controller,
    ClientController,
    next_client_controller,
    3, |
    send_controller_to_controller,
    ClientStorage,
    next_client_storage,
    4, |
    send_start_controller_to_logs,
    Logs,
    next_logs,
    5, |
    send_https_controller_to_storage,
    Storage,
    next_storage,
    6, | =>
    Controller,
    SessionMpstSeven,
    7
);
create_send_mpst_session_bundle!(
    send_logs_to_api,
    Api,
    next_api,
    1, |
    send_logs_to_client_api,
    ClientApi,
    next_client_api,
    2, |
    send_logs_to_client_controller,
    ClientController,
    next_client_controller,
    3, |
    send_logs_to_client_storage,
    ClientStorage,
    next_client_storage,
    4, |
    send_logs_to_controller,
    Controller,
    next_controller,
    5, |
    send_logs_to_storage,
    Storage,
    next_storage,
    6, | =>
    Logs,
    SessionMpstSeven,
    7
);
create_send_mpst_session_bundle!(
    send_response_storage_to_api,
    Api,
    next_api,
    1, |
    send_storage_to_client_api,
    ClientApi,
    next_client_api,
    2, |
    send_storage_to_client_controller,
    ClientController,
    next_client_controller,
    3, |
    send_ipfs_storage_to_client_storage,
    ClientStorage,
    next_client_storage,
    4, |
    send_storage_to_controller,
    Controller,
    next_controller,
    5, |
    send_storage_to_logs,
    Logs,
    next_logs,
    6, | =>
    Storage,
    SessionMpstSeven,
    7
);

// Create recv
create_recv_mpst_session_bundle!(
    recv_http_api_from_client_api,
    ClientApi,
    next_client_api,
    1, |
    recv_api_from_client_controller,
    ClientController,
    next_client_controller,
    2, |
    recv_api_from_client_storage,
    ClientStorage,
    next_client_storage,
    3, |
    recv_failed_api,
    Controller,
    next_controller,
    4, |
    recv_api_from_logs,
    Logs,
    next_logs,
    5, |
    recv_request_storage,
    Storage,
    next_storage,
    6, | =>
    Api,
    SessionMpstSeven,
    7
);
create_recv_mpst_session_bundle!(
    recv_client_api_from_api,
    Api,
    next_api,
    1, |
    recv_client_api_from_client_controller,
    ClientController,
    next_client_controller,
    2, |
    recv_client_api_from_client_storage,
    ClientStorage,
    next_client_storage,
    3, |
    recv_client_api_from_controller,
    Controller,
    next_controller,
    4, |
    recv_client_api_from_logs,
    Logs,
    next_logs,
    5, |
    recv_client_api_from_storage,
    Storage,
    next_storage,
    6, | =>
    ClientApi,
    SessionMpstSeven,
    7
);
create_recv_mpst_session_bundle!(
    recv_client_controller_from_api,
    Api,
    next_api,
    1, |
    recv_client_controller_from_client_api,
    ClientApi,
    next_client_api,
    2, |
    recv_client_controller_from_client_storage,
    ClientStorage,
    next_client_storage,
    3, |
    recv_http_client_controller_from_controller,
    Controller,
    next_controller,
    4, |
    recv_client_controller_from_logs,
    Logs,
    next_logs,
    5, |
    recv_client_controller_from_storage,
    Storage,
    next_storage,
    6, | =>
    ClientController,
    SessionMpstSeven,
    7
);
create_recv_mpst_session_bundle!(
    recv_client_storage_from_api,
    Api,
    next_api,
    1, |
    recv_client_storage_from_client_api,
    ClientApi,
    next_client_api,
    2, |
    recv_client_storage_from_client_controller,
    ClientController,
    next_client_controller,
    3, |
    recv_client_storage_from_controller,
    Controller,
    next_controller,
    4, |
    recv_client_storage_from_logs,
    Logs,
    next_logs,
    5, |
    recv_ipfs_client_storage_from_storage,
    Storage,
    next_storage,
    6, | =>
    ClientStorage,
    SessionMpstSeven,
    7
);
create_recv_mpst_session_bundle!(
    recv_start_controller_from_api,
    Api,
    next_api,
    1, |
    recv_controller_from_client_api,
    ClientApi,
    next_client_api,
    2, |
    recv_http_controller_from_client_controller,
    ClientController,
    next_client_controller,
    3, |
    recv_controller_from_controller,
    ClientStorage,
    next_client_storage,
    4, |
    recv_start_controller_from_logs,
    Logs,
    next_logs,
    5, |
    recv_https_controller_from_storage,
    Storage,
    next_storage,
    6, | =>
    Controller,
    SessionMpstSeven,
    7
);
create_recv_mpst_session_bundle!(
    recv_logs_from_api,
    Api,
    next_api,
    1, |
    recv_logs_from_client_api,
    ClientApi,
    next_client_api,
    2, |
    recv_logs_from_client_controller,
    ClientController,
    next_client_controller,
    3, |
    recv_logs_from_client_storage,
    ClientStorage,
    next_client_storage,
    4, |
    recv_logs_from_controller,
    Controller,
    next_controller,
    5, |
    recv_logs_from_storage,
    Storage,
    next_storage,
    6, | =>
    Logs,
    SessionMpstSeven,
    7
);
create_recv_mpst_session_bundle!(
    recv_response_storage_from_api,
    Api,
    next_api,
    1, |
    recv_storage_from_client_api,
    ClientApi,
    next_client_api,
    2, |
    recv_storage_from_client_controller,
    ClientController,
    next_client_controller,
    3, |
    recv_ipfs_storage_from_client_storage,
    ClientStorage,
    next_client_storage,
    4, |
    recv_storage_from_controller,
    Controller,
    next_controller,
    5, |
    recv_storage_from_logs,
    Logs,
    next_logs,
    6, | =>
    Storage,
    SessionMpstSeven,
    7
);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstSeven, 7);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstSeven, 7);

// Names
type NameApi = Api<RoleEnd>;
type NameClientApi = ClientApi<RoleEnd>;
type NameClientController = ClientController<RoleEnd>;
type NameClientStorage = ClientStorage<RoleEnd>;
type NameController = Controller<RoleEnd>;
type NameLogs = Logs<RoleEnd>;
type NameStorage = Storage<RoleEnd>;

// Creating the MP sessions
type EndpointApi = SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameApi>;
type EndpointClientApi = SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameClientApi>;
type EndpointClientController =
    SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameClientController>;
type EndpointClientStorage =
    SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameClientStorage>;
type EndpointController = SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameController>;
type EndpointLogs = SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameLogs>;
type EndpointStorage = SessionMpstSeven<End, End, End, End, End, End, RoleEnd, NameStorage>;

fn simple_five_endpoint_api(s: EndpointApi) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn simple_five_endpoint_client_api(s: EndpointClientApi) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn simple_five_endpoint_client_controller(
    s: EndpointClientController,
) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn simple_five_endpoint_client_storage(s: EndpointClientStorage) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn simple_five_endpoint_controller(s: EndpointController) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn simple_five_endpoint_logs(s: EndpointLogs) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn simple_five_endpoint_storage(s: EndpointStorage) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn main() {
    let (
        thread_api,
        thread_client_api,
        thread_client_controller,
        thread_client_storage,
        thread_controller,
        thread_logs,
        thread_storage,
    ) = fork_mpst(
        simple_five_endpoint_api,
        simple_five_endpoint_client_api,
        simple_five_endpoint_client_controller,
        simple_five_endpoint_client_storage,
        simple_five_endpoint_controller,
        simple_five_endpoint_logs,
        simple_five_endpoint_storage,
    );

    thread_api.join().unwrap();
    thread_client_api.join().unwrap();
    thread_client_controller.join().unwrap();
    thread_client_storage.join().unwrap();
    thread_controller.join().unwrap();
    thread_logs.join().unwrap();
    thread_storage.join().unwrap();
}
