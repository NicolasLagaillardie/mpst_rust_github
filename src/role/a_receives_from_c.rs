use crossbeam_channel::{bounded, Receiver};
use role::c_sends_to_a::RoleCSendToA;
use role::Role;
use std::error::Error;

pub struct RoleAReceiveFromC<R: Role> {
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleAReceiveFromC<R> {
    type Dual = RoleCSendToA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);

        return (
            RoleAReceiveFromC {
                receiver: receiver,
            },
            RoleCSendToA {
                sender: sender,
            },
        );
    }
}

pub fn next_a_receive_from_c<R>(r: RoleAReceiveFromC<R>) -> Result<R, Box<dyn Error>>
where
    R: Role,
{
    let r = r.receiver.recv()?;
    Ok(r)
}