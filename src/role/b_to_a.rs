use crossbeam_channel::{bounded, Receiver, Sender};
use role::a_to_b::RoleAtoB;
use role::Role;

pub struct RoleBtoA<R: Role> {
    pub sender: Sender<R::Dual>,
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleBtoA<R> {
    type Dual = RoleAtoB<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);
        let (sender_dual, receiver_dual) = bounded::<R::Dual>(1);

        return (
            RoleBtoA {
                sender: sender_dual,
                receiver: receiver,
            },
            RoleAtoB {
                sender: sender,
                receiver: receiver_dual,
            },
        );
    }
}

pub fn next_b_to_a<R>(r: RoleBtoA<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    r.receiver.recv();
    here
}
