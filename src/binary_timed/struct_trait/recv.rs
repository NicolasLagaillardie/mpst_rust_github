#![allow(clippy::collapsible_else_if)]

//! This module contains the definition and associated functions and traits
//! for the Recv structure.

use crate::binary::struct_trait::session::Session;
use crate::binary_timed::struct_trait::send::SendTimed;

use crossbeam_channel::Receiver;

use std::marker;

/// Receive `T`, then continue as `S`.
/// 6 additional parameters are present:
/// * CLOCK: a char to indicate which clock will be tested under the time constraints (in *seconds*)
/// * START: an i128 which represents the lower bound for the time constraint
/// * INCLUDE_START: a bool which indicates whether START is included in the bound
/// * END: an i128 which represents the upper bound for the time constraint
/// * INCLUDE_END: a bool which indicates whether END is included in the bound
/// * RESET: a char which whether CLOCK needs to be reset after the receive. Currently limited to
///   CLOCK, future feature will use a Vec<char>
#[must_use]
#[derive(Debug)]
pub struct RecvTimed<
    T,
    const CLOCK: char,
    const START: i128,
    const INCLUDE_START: bool,
    const END: i128,
    const INCLUDE_END: bool,
    const RESET: char,
    S,
> where
    T: marker::Send,
    S: Session,
{
    #[doc(hidden)]
    pub channel: Receiver<(T, S)>,
    #[doc(hidden)]
    pub clock: char,
    #[doc(hidden)]
    pub start: i128,
    #[doc(hidden)]
    pub include_start: bool,
    #[doc(hidden)]
    pub end: i128,
    #[doc(hidden)]
    pub include_end: bool,
    #[doc(hidden)]
    pub reset: char,
}

impl<
        T: marker::Send,
        const CLOCK: char,
        const START: i128,
        const INCLUDE_START: bool,
        const END: i128,
        const INCLUDE_END: bool,
        const RESET: char,
        S: Session,
    > RecvTimed<T, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET, S>
{
    #[doc(hidden)]
    pub fn constraint(&self) -> String {
        if self.start < 0 {
            if self.end >= 0 {
                if self.include_end {
                    format!("{} <= {}", self.clock, self.end)
                } else {
                    format!("{} < {}", self.clock, self.end)
                }
            } else {
                panic!("Both start and end parameters are negative")
            }
        } else {
            if self.end < 0 {
                if self.include_start {
                    format!("{} <= {}", self.start, self.clock)
                } else {
                    format!("{} < {}", self.start, self.clock)
                }
            } else {
                match (self.include_start, self.include_end) {
                    (true, true) => format!("{} <= {} <= {}", self.start, self.clock, self.end),
                    (true, false) => format!("{} <= {} < {}", self.start, self.clock, self.end),
                    (false, true) => format!("{} < {} <= {}", self.start, self.clock, self.end),
                    (false, false) => format!("{} < {} < {}", self.start, self.clock, self.end),
                }
            }
        }
    }
}

impl<
        T: marker::Send,
        const CLOCK: char,
        const START: i128,
        const INCLUDE_START: bool,
        const END: i128,
        const INCLUDE_END: bool,
        const RESET: char,
        S: Session,
    > Session for RecvTimed<T, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET, S>
{
    type Dual = SendTimed<T, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET, S::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = Self::Dual::new();
        (receiver, sender)
    }

    fn head_str() -> String {
        "Recv".to_string()
    }

    fn tail_str() -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }

    fn self_head_str(&self) -> String {
        "Recv".to_string()
    }

    fn self_tail_str(&self) -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }
}
