use crossbeam_channel::{bounded, Receiver, Sender};
use role::Role;

/// End of communication.
#[derive(Debug)]
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
    fn head_str() -> String {
        String::from("RoleEnd")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        String::from("")
    }
}
