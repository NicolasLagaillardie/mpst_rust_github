//! This module contains the required definitions and
//! functions for the basic name A.

use crate::name::Name;
use crossbeam_channel::{bounded, Receiver, Sender};

/// Used for indicating that a [`MeshedChannels`] is related to role A.
///
/// This `struct` should only be used in the `name` field
/// of the [`MeshedChannels`] related to A.
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// # Example
///
/// ```
/// use mpstthree::name::a::NameA;
/// use mpstthree::name::Name; // Only used for example
///
/// let _ = NameA::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct NameA {
    #[doc(hidden)]
    pub sender: Sender<()>,
    #[doc(hidden)]
    pub receiver: Receiver<()>,
}

impl Name for NameA {
    type Dual = NameA;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        (
            NameA {
                sender: sender1,
                receiver: receiver2,
            },
            NameA {
                sender: sender2,
                receiver: receiver1,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        "NameA".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        "".to_string()
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "NameA".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        "".to_string()
    }
}

#[doc(hidden)]
impl NameA {
    #[doc(hidden)]
    pub fn field_names(self) -> (&'static [&'static str], NameA) {
        (&["()", "()"], self)
    }
}

impl NameA {
    /// Cancel the session
    pub fn cancel(self) {
        drop(self);
    }
}
