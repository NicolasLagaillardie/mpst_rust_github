use crate::binary::struct_trait::session::Session;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// End of communication.
#[must_use]
#[derive(Debug)]
pub struct End {
    pub sender: Sender<Signal>,
    pub receiver: Receiver<Signal>,
}

#[derive(Debug)]
pub enum Signal {
    Offer(End),
    Stop,
    Cancel,
}

impl Session for End {
    type Dual = End;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<Signal>(1);
        let (sender2, receiver2) = bounded::<Signal>(1);

        (
            End {
                sender: sender1,
                receiver: receiver2,
            },
            End {
                sender: sender2,
                receiver: receiver1,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("End")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone)]
pub struct EndError {
    details: String,
}

impl EndError {
    fn new(details: &str) -> EndError {
        EndError {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for EndError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expected `End`, found {:?}", self.details)
    }
}

impl Error for EndError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl FromStr for End {
    type Err = EndError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "End" => Ok(End::new().0),
            result => Err(EndError::new(result)),
        }
    }
}
