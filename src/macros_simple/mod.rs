#![cfg(feature = "macros_simple")]

//! This module contains all the macros that are used for
//! the parametrisation on the name of the participants.
//!
//! They main ones are[`create_normal_role`] and
//! [`create_meshedchannels`]
//!
//! [`create_normal_role`]: crate::create_normal_role
//! [`create_meshedchannels`]: crate::create_meshedchannels
//!
//! This module is available only if MultiCrusty is built with the "macros_simple" feature.

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
pub mod choose;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
pub mod name;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
pub mod offer;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
pub mod recv;

#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
pub mod send;
