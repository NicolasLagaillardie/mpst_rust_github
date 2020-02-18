use crossbeam_channel::{bounded, Sender};
use role::b_receives_from_c::RoleBReceiveFromC;
use role::Role;

pub struct RoleCSendToB<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleCSendToB<R> {
    type Dual = RoleBReceiveFromC<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R::Dual>(1);

        return (
            RoleCSendToB {
                sender: sender,
            },
            RoleBReceiveFromC {
                receiver: receiver,
            },
        );
    }
}

pub fn next_c_sends_to_b<R>(r: RoleCSendToB<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}