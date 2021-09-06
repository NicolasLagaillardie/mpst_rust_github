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
pub struct Send<T, S>
where
    T: marker::Send,
    S: Session,
    S::Dual: Session, {
    #[doc(hidden)]
    pub channel: Sender<(T, S::Dual)>,
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
