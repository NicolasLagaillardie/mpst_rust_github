//! This module contains the functions
//! and macros for handling a UDP connection.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_udp"` feature.*

#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub mod choose;

#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub mod offer;

#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub mod recv;

#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub mod send;

#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub mod fork;

#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub mod cancel;
