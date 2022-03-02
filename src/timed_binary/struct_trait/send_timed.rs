//! This module contains the definition and associated functions and traits
//! for the Send structure.

use crate::binary::struct_trait::get_blocks;
use crate::binary::struct_trait::recv::Recv;
use crate::binary::struct_trait::session::Session;
use crossbeam_channel::{bounded, Sender};
use std::error::Error;
use std::fmt;
use std::marker;
use std::str::FromStr;

/// Send `T`, then continue as `S`.
#[must_use]
#[derive(Debug)]
pub struct Send<
    T,
    S,
    const CLOCK: char,
    const START: i128,
    const INCLUDE_START: bool,
    const END: i128,
    const INCLUDE_END: bool,
    const RESET: Vec<char>,
> where
    T: marker::Send,
    S: Session,
    S::Dual: Session,
{
    #[doc(hidden)]
    pub channel: Sender<(T, S::Dual)>,
    #[doc(hidden)]
    pub clock: CLOCK,
    #[doc(hidden)]
    pub start: START,
    #[doc(hidden)]
    pub include_start: INCLUDE_START,
    #[doc(hidden)]
    pub end: END,
    #[doc(hidden)]
    pub include_end: INCLUDE_END,
    #[doc(hidden)]
    pub reset: RESET,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct SendError {
    details: String,
}

impl SendError {
    fn new(details: &str) -> SendError {
        SendError {
            details: details.to_string(),
        }
    }
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

impl<T: marker::Send, S: Session> Session for Send<T, S> {
    type Dual = Recv<T, S::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<(T, S::Dual)>(1);
        (Send { channel: sender }, Recv { channel: receiver })
    }

    #[doc(hidden)]
    fn clock_str(&self) -> String {
        String::from(self.clock)
    }

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
                    format!("{} >= {}", self.clock, self.end)
                } else {
                    format!("{} > {}", self.clock, self.end)
                }
            } else {
                match (self.include_start, include_end) {
                    (true, true) => format!("{} >= {} >= {}", self.start, self.clock, self.end),
                    (true, false) => format!("{} >= {} > {}", self.start, self.clock, self.end),
                    (false, true) => format!("{} > {} >= {}", self.start, self.clock, self.end),
                    (false, false) => format!("{} > {} > {}", self.start, self.clock, self.end),
                }
            }
        }
    }

    #[doc(hidden)]
    fn reset_clock(&self) {
        self.reset.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", ")
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
    fn self_head_str() -> String {
        "Send".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str() -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }
}

impl<T: FromStr + marker::Send, S: FromStr + Session> FromStr for Send<T, S>
where
    <T as FromStr>::Err: fmt::Debug,
    <S as FromStr>::Err: fmt::Debug,
{
    type Err = SendError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..4] {
            "Send" => {
                let payload_continuation = get_blocks(s.to_string()).unwrap();
                let _ = T::from_str(&payload_continuation[0]).unwrap();
                let _ = S::from_str(&payload_continuation[1]).unwrap();
                Ok(Send::<T, S>::new().0)
            }
            result => Err(SendError::new(result)),
        }
    }
}
