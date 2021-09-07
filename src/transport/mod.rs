#![cfg(feature = "transport")]

//! The module for interacting with different modes of transport
//! such as HTTP or TCP
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature.*

#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub mod http;

#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub mod tcp;
