//! This module contains the functions and macros
//! for receiving
//! a choice for binary sessions.

use crate::binary::cancel::cancel;
use crate::binary::struct_trait::{end::End, session::Session};
use crate::binary_timed::recv::recv;
use crate::binary_timed::struct_trait::recv::RecvTimed;

use either::Either;

use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

/// Offer a choice between two sessions `S1` and `S1`.
/// Implemented using `Recv` and `Either`.
pub type OfferTimed<
    S1,
    S2,
    const CLOCK: char,
    const START: i128,
    const INCLUDE_START: bool,
    const END: i128,
    const INCLUDE_END: bool,
    const RESET: bool,
> = RecvTimed<Either<S1, S2>, End, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET>;

/// Offer a choice between two sessions `S1` and `S2`.
pub fn offer_either<
    'a,
    S1,
    S2,
    F,
    G,
    R,
    const CLOCK: char,
    const START: i128,
    const INCLUDE_START: bool,
    const END: i128,
    const INCLUDE_END: bool,
    const RESET: bool,
>(
    all_clocks: &mut HashMap<char, Instant>,
    s: OfferTimed<S1, S2, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET>,
    f: F,
    g: G,
) -> Result<R, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    F: FnOnce(&mut HashMap<char, Instant>, S1) -> Result<R, Box<dyn Error + 'a>>,
    G: FnOnce(&mut HashMap<char, Instant>, S2) -> Result<R, Box<dyn Error + 'a>>,
{
    let (e, s) = recv(all_clocks, s)?;
    cancel(s);
    e.either_with(all_clocks, f, g)
}

/// Offer a choice between many different sessions wrapped
/// in an `enum`
#[macro_export]
macro_rules! offer_timed {
    ($session: expr, $all_clocks:expr, { $( $pat: pat => $result: expr , )+ }) => {
        (move || -> Result<_, _> {
            let (l, s) = mpstthree::binary_timed::recv::recv($all_clocks, $session)?;
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
