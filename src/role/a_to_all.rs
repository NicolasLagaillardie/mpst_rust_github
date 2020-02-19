use crossbeam_channel::{bounded, Sender};
use role::all_to_a::RoleAlltoA;
use role::Role;

pub struct RoleAtoAll<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleAtoAll<R> {
    type Dual = RoleAlltoA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        return (
            RoleAtoAll {
                sender: sender_dual,
            },
            RoleAlltoA { sender: sender },
        );
    }
}

pub fn next_a_to_all<R>(r: RoleAtoAll<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
