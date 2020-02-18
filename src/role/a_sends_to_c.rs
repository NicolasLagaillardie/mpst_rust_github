use crossbeam_channel::{bounded, Sender};
use role::c_receives_from_a::RoleCReceiveFromA;
use role::Role;

pub struct RoleASendToC<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleASendToC<R> {
    type Dual = RoleCReceiveFromA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R::Dual>(1);

        return (
            RoleASendToC {
                sender: sender,
            },
            RoleCReceiveFromA {
                receiver: receiver,
            },
        );
    }
}

pub fn next_a_sends_to_c<R>(r: RoleASendToC<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}