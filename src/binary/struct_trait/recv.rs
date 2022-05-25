//! This module contains the definition and associated functions and traits
//! for the Recv structure.

use crate::binary::struct_trait::send::Send;
use crate::binary::struct_trait::session::Session;

use crossbeam_channel::Receiver;

use std::error::Error;
use std::fmt;
use std::marker;

/// Receive `T`, then continue as `S`.
#[must_use]
#[derive(Debug)]
pub struct Recv<T, S>
where
    T: marker::Send,
    S: Session,
{
    #[doc(hidden)]
    pub channel: Receiver<(T, S)>,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct RecvError {
    details: String,
}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expected `Recv`, found {:?}", self.details)
    }
}

impl Error for RecvError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl<T: marker::Send, S: Session> Session for Recv<T, S> {
    type Dual = Send<T, S::Dual>;

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
