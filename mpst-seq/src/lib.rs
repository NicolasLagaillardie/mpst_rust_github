use proc_macro_hack::proc_macro_hack;

pub use mpst_seq_proc::baking;
pub use mpst_seq_proc::broadcast_cancel;
pub use mpst_seq_proc::close_mpst;
pub use mpst_seq_proc::close_mpst_check_cancel;
pub use mpst_seq_proc::create_broadcast_role_short;
pub use mpst_seq_proc::create_choose_type_multi;
pub use mpst_seq_proc::create_normal_role_short;
pub use mpst_seq_proc::fork_mpst_multi;
pub use mpst_seq_proc::recv_all_aux_simple;
pub use mpst_seq_proc::recv_aux_simple;
pub use mpst_seq_proc::send_aux_simple;
pub use mpst_seq_proc::send_cancel;
pub use mpst_seq_proc::seq;

#[proc_macro_hack]
pub use mpst_seq_proc::e_seq;

#[proc_macro_hack]
pub use mpst_seq_proc::e_recv_aux_simple;

#[proc_macro_hack]
pub use mpst_seq_proc::e_recv_all_aux_simple;

#[proc_macro_hack]
pub use mpst_seq_proc::e_send_aux_simple;

#[proc_macro_hack]
pub use mpst_seq_proc::e_send_cancel;

#[proc_macro_hack]
pub use mpst_seq_proc::e_broadcast_cancel;

#[proc_macro_hack]
pub use mpst_seq_proc::e_create_normal_role_short;

#[proc_macro_hack]
pub use mpst_seq_proc::e_create_broadcast_role_short;

#[proc_macro_hack]
pub use mpst_seq_proc::e_baking;

#[proc_macro_hack]
pub use mpst_seq_proc::e_close_mpst;

#[proc_macro_hack]
pub use mpst_seq_proc::e_fork_mpst_multi;

#[proc_macro_hack]
pub use mpst_seq_proc::e_create_choose_type_multi;
