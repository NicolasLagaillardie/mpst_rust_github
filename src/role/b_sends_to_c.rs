use crossbeam_channel::{bounded, Sender};
use role::c_receives_from_b::RoleCReceiveFromB;
use role::Role;

pub struct RoleBSendToC<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleBSendToC<R> {
    type Dual = RoleCReceiveFromB<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R::Dual>(1);

        return (
            RoleBSendToC {
                sender: sender,
            },
            RoleCReceiveFromB {
                receiver: receiver,
            },
        );
    }
}

pub fn next_b_sends_to_c<R>(r: RoleBSendToC<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}