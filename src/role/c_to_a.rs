use crossbeam_channel::{bounded, Receiver, Sender};
use role::a_to_c::RoleAtoC;
use role::Role;

pub struct RoleCtoA<R: Role> {
    pub sender: Sender<R::Dual>,
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleCtoA<R> {
    type Dual = RoleAtoC<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);
        let (sender_dual, receiver_dual) = bounded::<R::Dual>(1);

        return (
            RoleCtoA {
                sender: sender_dual,
                receiver: receiver,
            },
            RoleAtoC {
                sender: sender,
                receiver: receiver_dual,
            },
        );
    }
}

pub fn next_c_to_a<R>(r: RoleCtoA<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    r.receiver.recv();
    here
}
