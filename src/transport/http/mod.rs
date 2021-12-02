#![cfg(feature = "transport_http")]

//! This module contains the functions
//! and macros for handling an HTTP connection.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_http"` feature.*

#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_http")))
)]
pub mod recv;

#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_http")))
)]
pub mod send;

#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_http")))
)]
pub mod choose;
