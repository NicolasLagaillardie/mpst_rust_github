use proc_macro_hack::proc_macro_hack;

pub use mpst_seq_proc_baking::baking;
pub use mpst_seq_proc_baking::baking_with_cancel;
pub use mpst_seq_proc_baking::baking_with_enum;
pub use mpst_seq_proc_baking::baking_with_enum_and_cancel;

#[proc_macro_hack]
pub use mpst_seq_proc_baking::e_baking;

#[proc_macro_hack]
pub use mpst_seq_proc_baking::e_baking_with_enum;

#[proc_macro_hack]
pub use mpst_seq_proc_baking::e_baking_with_cancel;

#[proc_macro_hack]
pub use mpst_seq_proc_baking::e_baking_with_enum_and_cancel;
