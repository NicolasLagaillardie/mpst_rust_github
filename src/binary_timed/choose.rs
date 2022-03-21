//! This module contains the functions and macros
//! for sending
//! a choice for binary sessions.

use crate::binary::cancel::cancel;
use crate::binary::struct_trait::{end::End, session::Session};
use crate::binary_timed::send::send;
use crate::binary_timed::struct_trait::send::SendTimed;

use either::Either;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

/// Choose between two sessions `S1` and `S2`. Implemented
/// using `Send` and `Either`.
pub type ChooseTimed<
    S1,
    S2,
    const CLOCK: char,
    const START: i128,
    const INCLUDE_START: bool,
    const END: i128,
    const INCLUDE_END: bool,
    const RESET: bool,
> = SendTimed<
    Either<<S1 as Session>::Dual, <S2 as Session>::Dual>,
    End,
    CLOCK,
    START,
    INCLUDE_START,
    END,
    INCLUDE_END,
    RESET,
>;

/// Given a choice between sessions `S1` and `S1`, choose
/// the first option.
pub fn choose_left<
    'a,
    S1,
    S2,
    const CLOCK: char,
    const START: i128,
    const INCLUDE_START: bool,
    const END: i128,
    const INCLUDE_END: bool,
    const RESET: bool,
>(
    all_clocks: &mut HashMap<char, Instant>,
    s: ChooseTimed<S1, S2, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET>,
) -> Result<S1, Box<dyn Error>>
where
    S1: Session + 'a,
    S2: Session + 'a,
{
    let (here, there) = S1::new();
    let s = send(Either::Left(there), all_clocks, s)?;
    cancel(s);
    Ok(here)
}

/// Given a choice between sessions `S1` and `S1`, choose
/// the second option.
pub fn choose_right<
    'a,
    S1,
    S2,
    const CLOCK: char,
    const START: i128,
    const INCLUDE_START: bool,
    const END: i128,
    const INCLUDE_END: bool,
    const RESET: bool,
>(
    all_clocks: &mut HashMap<char, Instant>,
    s: ChooseTimed<S1, S2, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET>,
) -> Result<S2, Box<dyn Error>>
where
    S1: Session + 'a,
    S2: Session + 'a,
{
    let (here, there) = S2::new();
    let s = send(Either::Right(there), all_clocks, s)?;
    cancel(s);
    Ok(here)
}

/// Choose between many different sessions wrapped in an
/// `enum`
#[macro_export]
macro_rules! timed_choose {
    ($label:path, $session:expr, $all_clocks:expr) => {{
        let (here, there) = <_ as mpstthree::binary::struct_trait::session::Session>::new();
        let s = mpstthree::binary_timed::send_timed::send($label(there), $session, $all_clocks);
        mpstthree::binary::cancel::cancel(s);
        here
    }};
}
