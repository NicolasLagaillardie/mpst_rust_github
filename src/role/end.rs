use std::error::Error;
use crossbeam_channel::{bounded, Sender, Receiver};
use role::Role;

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

        return (
            RoleEnd {
                sender: sender1,
                receiver: receiver2,
            },
            RoleEnd {
                sender: sender2,
                receiver: receiver1,
            },
        );
    }
}

pub fn next_end(r: RoleEnd) -> Result<(), Box<dyn Error>> {
    r.sender.send(()).unwrap_or(());
    r.receiver.recv()?;
    Ok(())
}
