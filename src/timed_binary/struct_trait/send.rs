#![allow(clippy::collapsible_else_if)]

//! This module contains the definition and associated functions and traits
//! for the Send structure.

use crate::binary::struct_trait::session::Session;
use crate::timed_binary::struct_trait::recv::RecvTimed;
use crossbeam_channel::{bounded, Sender};
use std::error::Error;
use std::fmt;
use std::marker;

/// Send `T`, then continue as `S`.
/// 6 additional parameters are present:
/// * CLOCK: a char to indicate which clock will be tested under the time constraints (in *seconds*)
/// * START: an i128 which represents the lower bound for the time constraint
/// * INCLUDE_START: a bool which indicates whether START is included in the bound
/// * END: an i128 which represents the upper bound for the time constraint
/// * INCLUDE_END: a bool which indicates whether END is included in the bound
/// * RESET: a char which whether CLOCK needs to be reset after the send. Currently limited to CLOCK, future feature will use a Vec<char>
#[derive(Debug)]
pub struct SendTimed<
    T,
    S,
    const CLOCK: char,
    const START: i128,
    const INCLUDE_START: bool,
    const END: i128,
    const INCLUDE_END: bool,
    const RESET: bool,
> where
    T: marker::Send,
    S: Session,
    S::Dual: Session,
{
    #[doc(hidden)]
    pub channel: Sender<(T, S::Dual)>,
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
    pub reset: bool,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct SendError {
    details: String,
}

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expected `Send`, found {:?}", self.details)
    }
}

impl Error for SendError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl<
        T: marker::Send,
        S: Session,
        const CLOCK: char,
        const START: i128,
        const INCLUDE_START: bool,
        const END: i128,
        const INCLUDE_END: bool,
        const RESET: bool,
    > SendTimed<T, S, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET>
{
    #[doc(hidden)]
    fn constraint(&self) -> String {
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
                    format!("{} >= {}", self.clock, self.start)
                } else {
                    format!("{} > {}", self.clock, self.start)
                }
            } else {
                match (self.include_start, self.include_end) {
                    (true, true) => format!("{} <= {} <= {}", self.start, self.clock, self.end),
                    (true, false) => format!("{} <= {} < {}", self.start, self.clock, self.end),
                    (false, true) => format!("{} <={} <= {}", self.start, self.clock, self.end),
                    (false, false) => format!("{}  {} < {}", self.start, self.clock, self.end),
                }
            }
        }
    }
}

impl<
        T: marker::Send,
        S: Session,
        const CLOCK: char,
        const START: i128,
        const INCLUDE_START: bool,
        const END: i128,
        const INCLUDE_END: bool,
        const RESET: bool,
    > Session for SendTimed<T, S, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET>
{
    type Dual = RecvTimed<T, S::Dual, CLOCK, START, INCLUDE_START, END, INCLUDE_END, RESET>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<(T, S::Dual)>(1);
        (
            SendTimed {
                channel: sender,
                clock: CLOCK,
                start: START,
                include_start: INCLUDE_START,
                end: END,
                include_end: INCLUDE_END,
                reset: RESET,
            },
            RecvTimed {
                channel: receiver,
                clock: CLOCK,
                start: START,
                include_start: INCLUDE_START,
                end: END,
                include_end: INCLUDE_END,
                reset: RESET,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        "Send".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "Send".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }
}
