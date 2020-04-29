use crossbeam_channel::{bounded, Receiver, Sender};
use role::Role;

use std::fmt;

/// End of communication.
pub struct RoleEnd {
    pub sender: Sender<()>,
    pub receiver: Receiver<()>,
}

impl Role for RoleEnd {
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
    fn head() -> String {
        String::from("RoleEnd")
    }
}

impl fmt::Display for RoleEnd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RoleEnd")
    }
}
