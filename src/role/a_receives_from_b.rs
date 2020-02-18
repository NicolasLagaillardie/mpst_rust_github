use crossbeam_channel::{bounded, Receiver};
use role::b_sends_to_a::RoleBSendToA;
use role::Role;
use std::error::Error;

pub struct RoleAReceiveFromB<R: Role> {
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleAReceiveFromB<R> {
    type Dual = RoleBSendToA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);

        return (
            RoleAReceiveFromB {
                receiver: receiver,
            },
            RoleBSendToA {
                sender: sender,
            },
        );
    }
}

pub fn next_a_receive_from_b<R>(r: RoleAReceiveFromB<R>) -> Result<R, Box<dyn Error>>
where
    R: Role,
{
    let r = r.receiver.recv()?;
    Ok(r)
}
