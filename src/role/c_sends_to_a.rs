use crossbeam_channel::{bounded, Sender};
use role::a_receives_from_c::RoleAReceiveFromC;
use role::Role;

pub struct RoleCSendToA<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleCSendToA<R> {
    type Dual = RoleAReceiveFromC<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<R::Dual>(1);

        return (
            RoleCSendToA {
                sender: sender,
            },
            RoleAReceiveFromC {
                receiver: receiver,
            },
        );
    }
}

pub fn next_c_sends_to_a<R>(r: RoleCSendToA<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}