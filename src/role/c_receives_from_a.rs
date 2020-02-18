use crossbeam_channel::{bounded, Receiver};
use role::a_sends_to_c::RoleASendToC;
use role::Role;
use std::error::Error;

pub struct RoleCReceiveFromA<R: Role> {
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleCReceiveFromA<R> {
    type Dual = RoleASendToC<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);

        return (
            RoleCReceiveFromA {
                receiver: receiver,
            },
            RoleASendToC {
                sender: sender,
            },
        );
    }
}

pub fn next_c_receive_from_a<R>(r: RoleCReceiveFromA<R>) -> Result<R, Box<dyn Error>>
where
    R: Role,
{
    let r = r.receiver.recv()?;
    Ok(r)
}