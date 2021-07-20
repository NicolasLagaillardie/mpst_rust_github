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
    pub sender: Sender<()>,
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
        String::from("RoleEnd")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        String::from("")
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        String::from("RoleEnd")
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        String::from("")
    }
}
