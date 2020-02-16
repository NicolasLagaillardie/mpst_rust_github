use crossbeam_channel::{bounded, Receiver, Sender};
use role::b_to_c::RoleBtoC;
use role::Role;

pub struct RoleCtoB<R: Role> {
    pub sender: Sender<R::Dual>,
    pub receiver: Receiver<R>,
}

impl<R: Role> Role for RoleCtoB<R> {
    type Dual = RoleBtoC<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R>(1);
        let (sender_dual, receiver_dual) = bounded::<R::Dual>(1);

        return (
            RoleCtoB {
                sender: sender_dual,
                receiver: receiver,
            },
            RoleBtoC {
                sender: sender,
                receiver: receiver_dual,
            },
        );
    }
}

pub fn next_c_to_b<R>(r: RoleCtoB<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    r.receiver.recv();
    here
}
