//! This module contains the functions and macros
//! for sending
//! a choice for binary sessions.

/// Choose between many different sessions wrapped in an
/// `enum`
#[macro_export]
macro_rules! timed_choose {
    ($label:path, $session:expr, $all_clocks:expr) => {{
        let (here, there) = <_ as mpstthree::binary::struct_trait::session::Session>::new();
        let s = mpstthree::timed_binary::send_timed::send($label(there), $session, $all_clocks);
        mpstthree::binary::cancel::cancel(s);
        here
    }};
}
