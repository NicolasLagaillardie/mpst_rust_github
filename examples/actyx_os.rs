// use mpstthree::binary::{End, Recv, Send};
// use mpstthree::role::end::RoleEnd;
// use mpstthree::role::Role;
// use mpstthree::{
//     bundle_fork_multi, choose_mpst_multi_to_all, close_mpst, create_normal_role,
//     create_recv_mpst_session, create_recv_mpst_session_bundle, create_send_mpst_session,
//     create_send_mpst_session_bundle, create_sessionmpst, offer_mpst,
// };

// use std::error::Error;

// // Create new SessionMpst for seven participants
// create_sessionmpst!(SessionMpstSeven, 7);

// // Create Roles
// create_normal_role!(Api, next_api, DualAPI, next_dual_api); // #1
// create_normal_role!(ClientApi, next_client_api, DualClientApi, next_dual_client_api); // #2
// create_normal_role!(ClientController, next_client_controller, DualClientController, next_dual_client_controller); // #3
// create_normal_role!(ClientStorage, next_client_storage, DualClientStorage, next_dual_client_storage); // #4
// create_normal_role!(Controller, next_controller, DualController, next_dual_controller); // #5
// create_normal_role!(Logs, next_logs, DualLogs, next_dual_logs); // #6
// create_normal_role!(Storage, next_storage, DualStorage, next_dual_storage); // #7

// // Create send
// create_send_mpst_session_bundle!(
//     http_api_to_client_api,
//     ClientApi,
//     next_client_api,
//     1, |
//     send_api_to_client_controller,
//     ClientController,
//     next_client_controller,
//     2, |
//     send_api_to_client_storage,
//     ClientStorage,
//     next_client_storage,
//     3, |
//     failed_api,
//     Controller,
//     next_controller,
//     4, |
//     send_api_to_logs,
//     Logs,
//     next_logs,
//     5, |
//     request_storage,
//     Storage,
//     next_storage,
//     6, | =>
//     Api,
//     SessionMpstSeven,
//     7
// );
// create_send_mpst_session_bundle!(
//     send_client_api_to_api,
//     Api,
//     next_api,
//     1, |
//     send_client_api_to_client_controller,
//     ClientController,
//     next_client_controller,
//     2, |
//     send_client_api_to_client_storage,
//     ClientStorage,
//     next_client_storage,
//     3, |
//     send_client_api_to_controller,
//     Controller,
//     next_controller,
//     4, |
//     send_client_api_to_logs,
//     Logs,
//     next_logs,
//     5, |
//     send_client_api_to_storage,
//     Storage,
//     next_storage,
//     6, | =>
//     ClientApi,
//     SessionMpstSeven,
//     7
// );
// create_send_mpst_session_bundle!(
//     send_client_controller_to_api,
//     Api,
//     next_api,
//     1, |
//     send_client_controller_to_client_api,
//     ClientApi,
//     next_client_api,
//     2, |
//     send_client_controller_to_client_storage,
//     ClientStorage,
//     next_client_storage,
//     3, |
//     http_client_controller_to_controller,
//     Controller,
//     next_controller,
//     4, |
//     send_client_controller_to_logs,
//     Logs,
//     next_logs,
//     5, |
//     send_client_controller_to_storage,
//     Storage,
//     next_storage,
//     6, | =>
//     ClientController,
//     SessionMpstSeven,
//     7
// );
// create_send_mpst_session_bundle!(
//     send_client_storage_to_api,
//     Api,
//     next_api,
//     1, |
//     send_client_storage_to_client_api,
//     ClientApi,
//     next_client_api,
//     2, |
//     send_client_storage_to_client_controller,
//     ClientController,
//     next_client_controller,
//     3, |
//     send_client_storage_to_controller,
//     Controller,
//     next_controller,
//     4, |
//     send_client_storage_to_logs,
//     Logs,
//     next_logs,
//     5, |
//     ipfs_client_storage_to_storage,
//     Storage,
//     next_storage,
//     6, | =>
//     ClientStorage,
//     SessionMpstSeven,
//     7
// );
// create_send_mpst_session_bundle!(
//     start_controller_to_api,
//     Api,
//     next_api,
//     1, |
//     send_controller_to_client_api,
//     ClientApi,
//     next_client_api,
//     2, |
//     http_controller_to_client_controller,
//     ClientController,
//     next_client_controller,
//     3, |
//     send_controller_to_controller,
//     ClientStorage,
//     next_client_storage,
//     4, |
//     start_controller_to_logs,
//     Logs,
//     next_logs,
//     5, |
//     https_controller_to_storage,
//     Storage,
//     next_storage,
//     6, | =>
//     Controller,
//     SessionMpstSeven,
//     7
// );
// create_send_mpst_session_bundle!(
//     send_logs_to_api,
//     Api,
//     next_api,
//     1, |
//     send_logs_to_client_api,
//     ClientApi,
//     next_client_api,
//     2, |
//     send_logs_to_client_controller,
//     ClientController,
//     next_client_controller,
//     3, |
//     send_logs_to_controller,
//     ClientStorage,
//     next_client_storage,
//     4, |
//     send_logs_to_controller,
//     Controller,
//     next_controller,
//     5, |
//     send_logs_to_storage,
//     Storage,
//     next_storage,
//     6, | =>
//     Logs,
//     SessionMpstSeven,
//     7
// );

fn main() {}
