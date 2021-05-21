use proc_macro_hack::proc_macro_hack;

pub use mpst_seq_proc::broadcast_cancel;
pub use mpst_seq_proc::create_broadcast_role_short;
pub use mpst_seq_proc::create_normal_role_short;
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
