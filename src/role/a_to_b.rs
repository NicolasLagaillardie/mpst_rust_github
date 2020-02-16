use role::b_to_a::RoleBtoA;
use role::Role;
use crossbeam_channel::{bounded, Sender, Receiver};

pub struct RoleAtoB<R: Role> {
    pub sender: Sender<R::Dual>,
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleAtoB<R> {
    type Dual = RoleBtoA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);
        let (sender_dual, receiver_dual) = bounded::<R::Dual>(1);

        return (
            RoleAtoB {
                sender: sender_dual,
                receiver: receiver,
            },
            RoleBtoA {
                sender: sender,
                receiver: receiver_dual,
            },
        );
    }
}

pub fn next_a_to_b<R>(r: RoleAtoB<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    r.receiver.recv();
    here
}
