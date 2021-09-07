//! This module contains the macros for
//! receiving a choice
//! for a TCP connection.
//!
//! *This module is available only if mp-anon is built with
//! the `"transport"` feature.*

/// Offer a choice between many different sessions wrapped
/// in an `enum`
///
/// *This macro is available only if mp-anon is built with
/// the `"transport"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
macro_rules! offer_tcp {
    ($session: expr, { $( $pat: pat => $result: expr , )* }) => {
        (move || -> Result<_, _> {
            let ((data, cont), s) = mpstthree::binary::recv::recv($session)?;
            mpstthree::binary::cancel::cancel(s);
            mpstthree::binary::cancel::cancel(data);

            match cont {
                $(
                    $pat => $result,
                )*
            }
        })()
    };
}
