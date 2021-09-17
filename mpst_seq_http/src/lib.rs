use proc_macro_hack::proc_macro_hack;

pub use mpst_seq_proc_http::choose_mpst_multi_http_to_all;
pub use mpst_seq_proc_http::create_recv_http_session;
pub use mpst_seq_proc_http::create_send_http_session;

#[proc_macro_hack]
pub use mpst_seq_proc_http::e_choose_mpst_multi_http_to_all;

#[proc_macro_hack]
pub use mpst_seq_proc_http::e_create_send_http_session;

#[proc_macro_hack]
pub use mpst_seq_proc_http::e_create_recv_http_session;


