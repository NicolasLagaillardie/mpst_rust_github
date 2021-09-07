//! This module contains the functions and macros
//! for sending
//! a choice for binary sessions.

use crate::binary::cancel::cancel;
use crate::binary::send::send;
use crate::binary::struct_trait::{end::End, send::Send, session::Session};
use either::Either;

/// Choose between two sessions `S1` and `S2`. Implemented
/// using `Send` and `Either`.
pub type Choose<S1, S2> = Send<Either<<S1 as Session>::Dual, <S2 as Session>::Dual>, End>;

/// Given a choice between sessions `S1` and `S1`, choose
/// the first option.
pub fn choose_left<'a, S1, S2>(s: Choose<S1, S2>) -> S1
where
    S1: Session + 'a,
    S2: Session + 'a,
{
    let (here, there) = S1::new();
    let s = send(Either::Left(there), s);
    cancel(s);
    here
}

/// Given a choice between sessions `S1` and `S1`, choose
/// the second option.
pub fn choose_right<'a, S1, S2>(s: Choose<S1, S2>) -> S2
where
    S1: Session + 'a,
    S2: Session + 'a,
{
    let (here, there) = S2::new();
    let s = send(Either::Right(there), s);
    cancel(s);
    here
}

/// Choose between many different sessions wrapped in an
/// `enum`
#[macro_export]
macro_rules! choose {
    ($label:path, $session:expr) => {{
        let (here, there) = <_ as mpstthree::binary::struct_trait::session::Session>::new();
        let s = mpstthree::binary::send::send($label(there), $session);
        mpstthree::binary::cancel::cancel(s);
        here
    }};
}
