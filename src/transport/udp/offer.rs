//! This module contains the macros for
//! receiving a choice
//! for a UDP connection.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_udp"` feature.*

/// Offer a choice between many different sessions wrapped
/// in an `enum`.
///
/// *This macro is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_udp"` feature.*
#[macro_export]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
macro_rules! offer_udp {
    ($session: expr, { $( $pat: pat => $result: expr , )+ }) => {
        (move || -> Result<_, _> {
            let ((data, cont), s) = mpstthree::binary::recv::recv($session)?;
            mpstthree::binary::cancel::cancel(s);
            mpstthree::binary::cancel::cancel(data);

            match cont {
                $(
                    $pat => $result,
                )+
            }
        })()
    };
}
