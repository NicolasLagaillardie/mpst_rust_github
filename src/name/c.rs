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
/// use mpstthree::name::a::NameC;
/// use mpstthree::role::Role; // Only used for example
///
/// type NameOfC = NameC;
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

    #[doc(hidden)]
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

    #[doc(hidden)]
    fn head_str() -> String {
        "NameC".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        "".to_string()
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "NameC".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        "".to_string()
    }
}

#[doc(hidden)]
impl NameC {
    #[doc(hidden)]
    pub fn field_names(self) -> (&'static [&'static str], NameC) {
        (&["()", "()"], self)
    }
}

impl NameC {
    /// Cancel the session
    pub fn cancel(self) {
        drop(self);
    }
}
