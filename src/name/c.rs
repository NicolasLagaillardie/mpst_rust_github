//! This module contains the required definitions and
//! functions for the basic name C.

use crate::name::Name;
use crossbeam_channel::{bounded, Receiver, Sender};

/// Used for indicating that a [`MeshedChannels`] is related to role C.
///
/// This `struct` should only be used in the `name` field
/// of the [`MeshedChannels`] related to C.
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// # Example
///
/// ```
/// use mpstthree::name::c::NameC;
/// use mpstthree::name::Name; // Only used for example
///
/// let _ = NameC::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct NameC {
    #[doc(hidden)]
    pub sender: Sender<()>,
    #[doc(hidden)]
    pub receiver: Receiver<()>,
}

impl Name for NameC {
    type Dual = NameC;

    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        (
            NameC {
                sender: sender1,
                receiver: receiver2,
            },
            NameC {
                sender: sender2,
                receiver: receiver1,
            },
        )
    }

    fn head_str() -> String {
        "NameC".to_string()
    }

    fn tail_str() -> String {
        "".to_string()
    }

    fn self_head_str(&self) -> String {
        "NameC".to_string()
    }

    fn self_tail_str(&self) -> String {
        "".to_string()
    }
}

impl NameC {
    /// Cancel the session
    pub fn cancel(self) {
        drop(self);
    }
}
