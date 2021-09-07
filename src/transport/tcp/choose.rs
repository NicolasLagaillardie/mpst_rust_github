//! This module contains the macros for
//! choosing a specific branch
//! for a TCP connection.
//!
//! *This module is available only if mp-anon is built with
//! the `"transport"` feature.*

/// Choose between many different sessions wrapped in an
/// `enum`
///
/// *This macro is available only if mp-anon is built with
/// the `"transport"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
macro_rules! choose_tcp {
    ($label:path, $session:expr, $data:expr) => {{
        let (here, there) = <_ as Session>::new();
        let s = mpstthree::binary::send::send(($data, $label(there)), $session);
        mpstthree::binary::cancel::cancel(s);
        mpstthree::binary::cancel::cancel($data);
        here
    }};
}
