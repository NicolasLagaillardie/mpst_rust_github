use role::c_to_a::RoleCtoA;
use role::Role;
use crossbeam_channel::{bounded, Sender, Receiver};

pub struct RoleAtoC<R: Role> {
    pub sender: Sender<R::Dual>,
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleAtoC<R> {
    type Dual = RoleCtoA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);
        let (sender_dual, receiver_dual) = bounded::<R::Dual>(1);

        return (
            RoleAtoC {
                sender: sender_dual,
                receiver: receiver,
            },
            RoleCtoA {
                sender: sender,
                receiver: receiver_dual,
            },
        );
    }
}

pub fn next_a_to_c<R>(r: RoleAtoC<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    r.receiver.recv();
    here
}
