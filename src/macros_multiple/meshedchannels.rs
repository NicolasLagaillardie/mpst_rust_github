//! This module contains the macros
//! for creating MeshedChannels for any number
//! of participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_multiple"` feature.*

/// Creates a MeshedChannels for more than 3 participants.
///
/// # Arguments
///
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::create_meshedchannels;
///
/// create_meshedchannels!(MeshedChannels, 3);
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_meshedchannels {
    ($meshedchannels_name: ident, $n_sessions: literal) => {
        mpst_seq_proc::create_meshedchannels!($meshedchannels_name, $n_sessions);
    };
}
