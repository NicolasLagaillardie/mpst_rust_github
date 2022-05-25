//! This module contains the required definitions and
//! functions for the basic name B.

use crate::name::Name;
use crossbeam_channel::{bounded, Receiver, Sender};

/// Used for indicating that a [`MeshedChannels`] is related to role B.
///
/// This `struct` should only be used in the `name` field
/// of the [`MeshedChannels`] related to B.
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// # Example
///
/// ```
/// use mpstthree::name::b::NameB;
/// use mpstthree::name::Name; // Only used for example
///
/// let _ = NameB::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct NameB {
    #[doc(hidden)]
    pub sender: Sender<()>,
    #[doc(hidden)]
    pub receiver: Receiver<()>,
}

impl Name for NameB {
    type Dual = NameB;

    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        (
            NameB {
                sender: sender1,
                receiver: receiver2,
            },
            NameB {
                sender: sender2,
                receiver: receiver1,
            },
        )
    }

    fn head_str() -> String {
        "NameB".to_string()
    }

    fn tail_str() -> String {
        "".to_string()
    }

    fn self_head_str(&self) -> String {
        "NameB".to_string()
    }

    fn self_tail_str(&self) -> String {
        "".to_string()
    }
}

impl NameB {
    /// Cancel the session
    pub fn cancel(self) {
        drop(self);
    }
}
