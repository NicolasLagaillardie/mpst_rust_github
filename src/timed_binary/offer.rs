//! This module contains the functions and macros
//! for receiving
//! a choice for binary sessions.

/// Offer a choice between many different sessions wrapped
/// in an `enum`
#[macro_export]
macro_rules! timed_offer {
    ($session: expr, $all_clocks:expr, { $( $pat: pat => $result: expr , )+ }) => {
        (move || -> Result<_, _> {
            let (l, s) = mpstthree::timed_binary::recv::recv($session, $all_clocks)?;
            mpstthree::binary::cancel::cancel(s);
            match l {
                $(
                    $pat => $result,
                )+
                _ => panic!("Unexpected payload") ,
            }
        })()
    };
}
