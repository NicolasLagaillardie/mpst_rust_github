use crossbeam_channel::{bounded, Receiver, Sender};

/// This structure is used for closing a stack for an active role
/// which is about to broadcast a choice with an `enum`.
///
/// # Example
///
/// ```
/// use mpstthree::role::broadcast::RoleBroadcast;
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::end::RoleEnd;
///
/// type BroadcastA = RoleA<RoleBroadcast>;
/// ```
#[derive(Debug)]
pub struct RoleBroadcast {
    pub sender: Sender<()>,
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
