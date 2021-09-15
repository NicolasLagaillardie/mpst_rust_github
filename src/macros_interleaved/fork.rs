//! This module contains the macros
//! for creating fork functions for interleaved sessions.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_interleaved"` feature.*

#[macro_export]
#[doc(hidden)]
macro_rules! fork_mpst_multi_interleaved {
    ($func_name:ident, $meshedchannels_name:ident, $nsessions:literal) => {
        mpst_seq::fork_mpst_multi_interleaved!($func_name, $meshedchannels_name, $nsessions);
    };
}
