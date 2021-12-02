//! This module contains the macros
//! for creating offer functions for any number
//! of participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"interleaved"` feature.*

#[macro_export]
#[doc(hidden)]
macro_rules! offer_mpst_interleaved {
    (
        $(
            $session: expr ,
            $recv_mpst: ident ,
            $pat: path
        ),+ $(,)?
    ) => {
        (
            $(
                {
                    let (l, s) = $recv_mpst($session)?;
                    mpstthree::binary::cancel::cancel(s);
                    match l {
                        $pat(s) => s,
                        _ => panic!("Unexpected payload") ,
                    }
                },
            )+
        )
    };
    (
        $(
            $session: expr ,
            $pat: path
        ),+ $(,)?
    ) => {
        (
            $(
                {
                    let (l, s) = $session.recv()?;
                    mpstthree::binary::cancel::cancel(s);
                    match l {
                        $pat(s) => s,
                        _ => panic!("Wrong payload"),
                    }
                },
            )+
        )
    };
}
