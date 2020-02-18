use crossbeam_channel::{bounded, Sender};
use role::a_receives_from_b::RoleAReceiveFromB;
use role::Role;

pub struct RoleBSendToA<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleBSendToA<R> {
    type Dual = RoleAReceiveFromB<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R::Dual>(1);

        return (
            RoleBSendToA {
                sender: sender,
            },
            RoleAReceiveFromB {
                receiver: receiver,
            },
        );
    }
}

pub fn next_b_sends_to_a<R>(r: RoleBSendToA<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}