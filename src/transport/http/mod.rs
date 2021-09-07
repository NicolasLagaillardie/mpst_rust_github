//! This module contains the functions
//! and macros for handling an HTTP connection.
//!
//! *This module is available only if mp-anon is built with
//! the `"transport"` feature.*

#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub mod recv;

#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub mod send;
