use crossbeam_channel::{bounded, Receiver};
use role::c_sends_to_b::RoleCSendToB;
use role::Role;
use std::error::Error;

pub struct RoleBReceiveFromC<R: Role> {
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleBReceiveFromC<R> {
    type Dual = RoleCSendToB<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);

        return (
            RoleBReceiveFromC {
                receiver: receiver,
            },
            RoleCSendToB {
                sender: sender,
            },
        );
    }
}

pub fn next_b_receive_from_c<R>(r: RoleBReceiveFromC<R>) -> Result<R, Box<dyn Error>>
where
    R: Role,
{
    let r = r.receiver.recv()?;
    Ok(r)
}