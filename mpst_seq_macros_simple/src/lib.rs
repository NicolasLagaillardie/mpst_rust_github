use proc_macro_hack::proc_macro_hack;

pub use mpst_seq_proc_macros_simple::create_broadcast_role_short;
pub use mpst_seq_proc_macros_simple::create_normal_role_short;
pub use mpst_seq_proc_macros_simple::create_offer_mpst_session_multi;
pub use mpst_seq_proc_macros_simple::create_recv_mpst_session;
pub use mpst_seq_proc_macros_simple::create_send_mpst_session;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_simple::e_create_broadcast_role_short;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_simple::e_create_normal_role_short;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_simple::e_create_offer_mpst_session_multi;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_simple::e_create_recv_mpst_session;

#[proc_macro_hack]
pub use mpst_seq_proc_macros_simple::e_create_send_mpst_session;