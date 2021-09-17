//! This module contains the macros
//! for creating fork functions for any number
//! of participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_multiple"` feature.*

/// Creates the _fork_ function to be used with more than 3
/// participants.
///
/// # Arguments
///
/// * The name of the new *fork* function
/// * The name of the *simple fork* function
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_meshedchannels, fork_mpst_multi};
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// fork_mpst_multi!(fork_mpst, MeshedChannels, 3);
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! fork_mpst_multi {
    ($func_name:ident, $meshedchannels_name:ident, $nsessions:literal) => {
        mpst_seq_macros_multiple::fork_mpst_multi!($func_name, $meshedchannels_name, $nsessions);
    };
}
