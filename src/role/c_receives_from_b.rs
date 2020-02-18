use crossbeam_channel::{bounded, Receiver};
use role::b_sends_to_c::RoleBSendToC;
use role::Role;
use std::error::Error;

pub struct RoleCReceiveFromB<R: Role> {
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleCReceiveFromB<R> {
    type Dual = RoleBSendToC<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);

        return (
            RoleCReceiveFromB {
                receiver: receiver,
            },
            RoleBSendToC {
                sender: sender,
            },
        );
    }
}

pub fn next_c_receive_from_b<R>(r: RoleCReceiveFromB<R>) -> Result<R, Box<dyn Error>>
where
    R: Role,
{
    let r = r.receiver.recv()?;
    Ok(r)
}