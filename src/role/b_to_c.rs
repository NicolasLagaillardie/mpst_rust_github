use role::c_to_b::RoleCtoB;
use role::Role;
use crossbeam_channel::{bounded, Sender, Receiver};

pub struct RoleBtoC<R: Role> {
    pub sender: Sender<R::Dual>,
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleBtoC<R> {
    type Dual = RoleCtoB<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);
        let (sender_dual, receiver_dual) = bounded::<R::Dual>(1);

        return (
            RoleBtoC {
                sender: sender_dual,
                receiver: receiver,
            },
            RoleCtoB {
                sender: sender,
                receiver: receiver_dual,
            },
        );
    }
}

pub fn next_b_to_c<R>(r: RoleBtoC<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    r.receiver.recv();
    here
}
