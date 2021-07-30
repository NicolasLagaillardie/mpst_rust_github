#![cfg(feature = "macros")]

//! This module contains all the macros that are used for
//! the parametrisation on the name and number of
//! participants.
//!
//! They main ones are[`create_normal_role`] and
//! [`create_meshedchannels`]
//!
//! [`create_normal_role`]: crate::create_normal_role
//! [`create_meshedchannels`]: crate::create_meshedchannels

pub mod multiple;
pub mod name;
pub mod simple;
