use crossbeam_channel::{bounded, Sender};
use role::b_receives_from_a::RoleBReceiveFromA;
use role::Role;

pub struct RoleASendToB<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleASendToB<R> {
    type Dual = RoleBReceiveFromA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R::Dual>(1);

        return (
            RoleASendToB {
                sender: sender,
            },
            RoleBReceiveFromA {
                receiver: receiver,
            },
        );
    }
}

pub fn next_a_sends_to_b<R>(r: RoleASendToB<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}