#![cfg(feature = "interleaved")]

//! This module contains all the macros that are used for
//! the interleaved sessions.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"interleaved"` feature.*

#[cfg_attr(doc_cfg, doc(cfg(feature = "interleaved")))]
pub mod fork;

#[cfg_attr(doc_cfg, doc(cfg(feature = "interleaved")))]
pub mod close;

#[cfg_attr(doc_cfg, doc(cfg(feature = "interleaved")))]
pub mod offer;

#[macro_export]
#[doc(hidden)]
macro_rules! bundle_struct_fork_close_multi_interleaved {
    (
        $func_name_close:ident,
        $func_name_fork:ident,
        $meshedchannels_name:ident,
        $nsessions:literal
    ) => {
        mpstthree::create_meshedchannels!($meshedchannels_name, $nsessions);
        mpstthree::close_mpst_interleaved!($func_name_close, $meshedchannels_name, $nsessions);
        mpstthree::fork_mpst_multi_solo!($func_name_fork, $meshedchannels_name, $nsessions);
    };
}
