//! This module contains the macros for
//! choosing a specific branch
//! for a UDP connection.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_udp"` feature.*

/// Choose between many different sessions wrapped in an
/// `enum`
///
/// *This macro is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_udp"` feature.*
#[macro_export]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
macro_rules! choose_udp {
    ($label:path, $session:expr, $data:expr) => {{
        let (here, there) = <_ as mpstthree::binary::struct_trait::session::Session>::new();
        let s = mpstthree::binary::send::send(($data, $label(there)), $session);
        mpstthree::binary::cancel::cancel(s);
        mpstthree::binary::cancel::cancel($data);
        here
    }};
}
