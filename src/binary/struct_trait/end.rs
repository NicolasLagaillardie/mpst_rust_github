//! This module contains the definition and associated functions and traits
//! for the End structure.

use crate::binary::struct_trait::session::Session;

use crossbeam_channel::{bounded, Receiver, Sender};

/// End of communication.
#[must_use]
#[derive(Debug)]
pub struct End {
    #[doc(hidden)]
    pub sender: Sender<Signal>,
    #[doc(hidden)]
    pub receiver: Receiver<Signal>,
}

#[doc(hidden)]
#[derive(Debug)]
pub enum Signal {
    #[doc(hidden)]
    Offer(End),
    #[doc(hidden)]
    Stop,
    #[doc(hidden)]
    Cancel,
}

impl Session for End {
    type Dual = End;

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

    fn head_str() -> String {
        "End".to_string()
    }

    fn tail_str() -> String {
        "".to_string()
    }

    fn self_head_str(&self) -> String {
        "End".to_string()
    }

    fn self_tail_str(&self) -> String {
        "".to_string()
    }
}
