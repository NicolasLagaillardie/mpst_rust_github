use proc_macro_hack::proc_macro_hack;

pub use mpst_seq_proc_interleaved::close_mpst_interleaved;
pub use mpst_seq_proc_interleaved::fork_mpst_multi_interleaved;

#[proc_macro_hack]
pub use mpst_seq_proc_interleaved::e_fork_mpst_multi_interleaved;

#[proc_macro_hack]
pub use mpst_seq_proc_interleaved::e_close_mpst_interleaved;
