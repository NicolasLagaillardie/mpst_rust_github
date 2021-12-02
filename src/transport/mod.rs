//! The module for interacting with different modes of transport
//! such as HTTP or TCP
//!
//! *This module is available only if MultiCrusty is built with
//! either the `"transport"` feature, the
//! `"transport_udp"` feature, the
//! `"transport_tcp"` feature or the
//! `"transport_http"` feature.*

#[cfg(feature = "transport_http")]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_http")))
)]
pub mod http;

#[cfg(feature = "transport_tcp")]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_tcp")))
)]
pub mod tcp;

#[cfg(feature = "transport_udp")]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub mod udp;
