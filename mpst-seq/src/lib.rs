use proc_macro_hack::proc_macro_hack;

pub use mpst_seq_proc::seq;
pub use mpst_seq_proc::recv_aux_simple;

#[proc_macro_hack]
pub use mpst_seq_proc::e_seq;


#[proc_macro_hack]
pub use mpst_seq_proc::e_recv_aux_simple;