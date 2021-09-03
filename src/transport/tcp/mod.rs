//! This module contains the functions
//! and macros for handling a TCP connection.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature.*

#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub mod choose;

#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub mod offer;

#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub mod recv;

#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub mod send;
