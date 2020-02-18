use crossbeam_channel::{bounded, Receiver};
use role::a_sends_to_b::RoleASendToB;
use role::Role;
use std::error::Error;

pub struct RoleBReceiveFromA<R: Role> {
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleBReceiveFromA<R> {
    type Dual = RoleASendToB<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);

        return (
            RoleBReceiveFromA {
                receiver: receiver,
            },
            RoleASendToB {
                sender: sender,
            },
        );
    }
}

pub fn next_b_receive_from_a<R>(r: RoleBReceiveFromA<R>) -> Result<R, Box<dyn Error>>
where
    R: Role,
{
    let r = r.receiver.recv()?;
    Ok(r)
}