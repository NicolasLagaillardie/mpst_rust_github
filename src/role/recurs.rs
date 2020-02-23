use crossbeam_channel::{bounded, Sender, Receiver};
use role::Role;
use std::marker;
use std::error::Error;

pub struct RoleRecurs<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T: marker::Send> Role for RoleRecurs<T> {
    type Dual = RoleRecurs<T>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<T>(1);
        let (sender_dual, receiver_dual) = bounded::<T>(1);

        return (
            RoleRecurs {
                sender: sender_dual,
                receiver: receiver_dual,
            },
            RoleRecurs {
                sender: sender,
                receiver: receiver,
            },
        );
    }
}

pub fn next_recurs<T>(r: RoleRecurs<T>) -> Result<T, Box<dyn Error>>
where
    T: marker::Send,
{
    let x = r.receiver.recv()?;
    Ok(x)
}
