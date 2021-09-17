use proc_macro_hack::proc_macro_hack;

pub use mpst_seq_proc_macros_multiple::broadcast_cancel;
pub use mpst_seq_proc_macros_multiple::choose_mpst_create_multi_to_all;
pub use mpst_seq_proc_macros_multiple::choose_mpst_multi_cancel_to_all;
pub use mpst_seq_proc_macros_multiple::close_mpst;
pub use mpst_seq_proc_macros_multiple::close_mpst_cancel;
pub use mpst_seq_proc_macros_multiple::close_mpst_check_cancel;
pub use mpst_seq_proc_macros_multiple::create_broadcast_role_short;
pub use mpst_seq_proc_macros_multiple::create_choose_mpst_session_multi_left;
pub use mpst_seq_proc_macros_multiple::create_choose_mpst_session_multi_right;
pub use mpst_seq_proc_macros_multiple::create_choose_type_multi;
pub use mpst_seq_proc_macros_multiple::create_fn_choose_mpst_cancel_multi_to_all_bundle;
pub use mpst_seq_proc_macros_multiple::create_fn_choose_mpst_multi_to_all_bundle;
pub use mpst_seq_proc_macros_multiple::create_meshedchannels;
pub use mpst_seq_proc_macros_multiple::create_normal_role_short;
pub use mpst_seq_proc_macros_multiple::create_offer_mpst_session_multi;
pub use mpst_seq_proc_macros_multiple::create_offer_type_multi;
pub use mpst_seq_proc_macros_multiple::create_recv_mpst_session;
pub use mpst_seq_proc_macros_multiple::create_send_check_cancel;
pub use mpst_seq_proc_macros_multiple::create_send_mpst_cancel;
pub use mpst_seq_proc_macros_multiple::create_send_mpst_session;
pub use mpst_seq_proc_macros_multiple::fork_mpst_multi;
pub use mpst_seq_proc_macros_multiple::recv_mpst;
pub use mpst_seq_proc_macros_multiple::send_cancel;
pub use mpst_seq_proc_macros_multiple::send_mpst;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_send_cancel;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_broadcast_cancel;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_normal_role_short;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_broadcast_role_short;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_close_mpst;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_fork_mpst_multi;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_choose_type_multi;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_choose_mpst_session_multi_left;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_choose_mpst_session_multi_right;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_choose_mpst_multi_cancel_to_all;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_fn_choose_mpst_multi_to_all_bundle;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_fn_choose_mpst_cancel_multi_to_all_bundle;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_offer_type_multi;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_offer_mpst_session_multi;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_recv_mpst;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_recv_mpst_session;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_send_check_cancel;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_send_mpst;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_send_mpst_session;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_send_mpst_cancel;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_create_meshedchannels;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_close_mpst_cancel;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_multiple::e_choose_mpst_create_multi_to_all;
