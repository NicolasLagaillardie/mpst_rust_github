#![cfg(feature = "macros_multiple")]

//! This module contains all the macros that are used for
//! the parametrisation on the number of
//! participants.
//!
//! They main ones are[`create_normal_role`] and
//! [`create_meshedchannels`]
//!
//! [`create_normal_role`]: crate::create_normal_role
//! [`create_meshedchannels`]: crate::create_meshedchannels
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_multiple"` feature.*

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
pub mod cancel;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
pub mod choose;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
pub mod close;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
pub mod fork;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
pub mod meshedchannels;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
pub mod offer;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
pub mod recv;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
pub mod send;

/// Creates the structure MeshedChannels
/// [`create_meshedchannels`],
/// the [`close_mpst`] and [`fork_mpst_multi`].
///
/// # Arguments
///
/// * The name of the new *close* function
/// * The name of the new *fork* function
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::bundle_struct_fork_close_multi;
///
/// bundle_struct_fork_close_multi!(close_mpst, fork_mpst, MeshedChannels, 3);
/// ```
///
/// [`create_meshedchannels`]: crate::create_meshedchannels
/// [`close_mpst`]: crate::close_mpst
/// [`fork_mpst_multi`]: crate::fork_mpst_multi
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! bundle_struct_fork_close_multi {
    (
        $func_name_close:ident,
        $func_name_fork:ident,
        $meshedchannels_name:ident,
        $n_sessions:literal
    ) => {
        mpstthree::create_meshedchannels!($meshedchannels_name, $n_sessions);
        mpstthree::close_mpst!($func_name_close, $meshedchannels_name, $n_sessions);
        mpstthree::fork_mpst_multi!($func_name_fork, $meshedchannels_name, $n_sessions);
    };
}

/// Creates the structure MeshedChannels
/// [`create_meshedchannels`],
/// the [`close_mpst_cancel`] and
/// [`fork_mpst_multi`]
/// functions to be used with more than 2 participants.
/// It checks the send sides of the channels along the recv sides.
///
/// # Arguments
///
/// * The name of the new *close* function
/// * The name of the new *fork* function
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::bundle_struct_fork_close_multi_cancel;
///
/// bundle_struct_fork_close_multi_cancel!(close_mpst, fork_mpst, MeshedChannels, 3);
/// ```
///
/// [`create_meshedchannels`]: crate::create_meshedchannels
/// [`close_mpst_cancel`]: crate::close_mpst_cancel
/// [`fork_mpst_multi`]: crate::fork_mpst_multi
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! bundle_struct_fork_close_multi_cancel {
    (
        $func_name_close:ident,
        $func_name_fork:ident,
        $meshedchannels_name:ident,
        $n_sessions:literal
    ) => {
        mpstthree::create_meshedchannels!($meshedchannels_name, $n_sessions);
        mpstthree::close_mpst_cancel!($func_name_close, $meshedchannels_name, $n_sessions);
        mpstthree::fork_mpst_multi!($func_name_fork, $meshedchannels_name, $n_sessions);
    };
}
