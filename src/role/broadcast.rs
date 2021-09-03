//! This module contains the required definitions and
//! functions for a role to broadcast multiple choices.
//! Its dual is it self.

use crossbeam_channel::{bounded, Receiver, Sender};

/// This structure is used for closing a stack for an active role
/// which is about to broadcast a choice with an `enum`.
///
/// # Example
///
/// ```
/// use mpstthree::role::Role; // Only used for example
/// use mpstthree::role::broadcast::RoleBroadcast;
/// use mpstthree::role::a::RoleA;
///
/// type BroadcastA = RoleA<RoleBroadcast>;
///
/// let _ = BroadcastA::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct RoleBroadcast {
    #[doc(hidden)]
    pub sender: Sender<()>,
    #[doc(hidden)]
    pub receiver: Receiver<()>,
}

impl crate::role::Role for RoleBroadcast {
    type Dual = RoleBroadcast;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        (
            RoleBroadcast {
                sender: sender1,
                receiver: receiver2,
            },
            RoleBroadcast {
                sender: sender2,
                receiver: receiver1,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        "RoleBroadcast".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        "".to_string()
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "RoleBroadcast".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        "".to_string()
    }
}
