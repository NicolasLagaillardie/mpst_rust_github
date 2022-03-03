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
/// use mpstthree::name::a::NameB;
/// use mpstthree::role::Role; // Only used for example
///
/// type NameOfB = NameB;
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

    #[doc(hidden)]
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

    #[doc(hidden)]
    fn head_str() -> String {
        "NameB".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        "".to_string()
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "NameB".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        "".to_string()
    }
}

#[doc(hidden)]
impl NameB {
    #[doc(hidden)]
    pub fn field_names(self) -> (&'static [&'static str], NameB) {
        (&["()", "()"], self)
    }
}

impl NameB {
    /// Cancel the session
    pub fn cancel(self) {
        drop(self);
    }
}
