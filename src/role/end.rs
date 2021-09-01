//! This module contains the required definitions and
//! functions for closing a stack.
//! Its dual is it self.

use crossbeam_channel::{bounded, Receiver, Sender};

/// This structure is used to close an ordering or a name.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::end::End;
///
/// // Creating the binary sessions
/// type Close = End;
/// ```
#[derive(Debug)]
pub struct RoleEnd {
    #[doc(hidden)]
    pub sender: Sender<()>,
    #[doc(hidden)]
    pub receiver: Receiver<()>,
}

impl crate::role::Role for RoleEnd {
    type Dual = RoleEnd;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        (
            RoleEnd {
                sender: sender1,
                receiver: receiver2,
            },
            RoleEnd {
                sender: sender2,
                receiver: receiver1,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        "RoleEnd".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        "".to_string()
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "RoleEnd".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        "".to_string()
    }
}
