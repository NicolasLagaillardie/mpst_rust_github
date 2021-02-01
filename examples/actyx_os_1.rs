use mpstthree::binary::End;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_fork_multi, close_mpst, create_normal_role, create_recv_mpst_session,
    create_recv_mpst_session_bundle, create_send_mpst_session, create_send_mpst_session_bundle,
    create_sessionmpst,
};

use std::error::Error;

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
    send_https_controller_to_storage,
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
    send_storage_to_controller,
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
    recv_failed_api,
    Controller,
    next_controller,
    1, |
    recv_api_from_logs,
    Logs,
    next_logs,
    2, |
    recv_request_storage,
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
    recv_https_controller_from_storage,
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
    recv_response_storage_from_api,
    Api,
    next_api,
    1, |
    recv_storage_from_controller,
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

// Creating the MP sessions
type EndpointApi = SessionMpstFour<End, End, End, RoleEnd, NameApi>;
type EndpointController = SessionMpstFour<End, End, End, RoleEnd, NameController>;
type EndpointLogs = SessionMpstFour<End, End, End, RoleEnd, NameLogs>;
type EndpointStorage = SessionMpstFour<End, End, End, RoleEnd, NameStorage>;

fn simple_five_endpoint_api(s: EndpointApi) -> Result<(), Box<dyn Error>> {
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
    let (thread_api, thread_controller, thread_logs, thread_storage) = fork_mpst(
        simple_five_endpoint_api,
        simple_five_endpoint_controller,
        simple_five_endpoint_logs,
        simple_five_endpoint_storage,
    );

    thread_api.join().unwrap();
    thread_controller.join().unwrap();
    thread_logs.join().unwrap();
    thread_storage.join().unwrap();
}
