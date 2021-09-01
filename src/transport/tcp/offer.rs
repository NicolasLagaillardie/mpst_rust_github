//! This module contains the macros for
//! receiving a choice
//! for a TCP connection.

/// Offer a choice between many different sessions wrapped
/// in an `enum`
#[macro_export]
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
