use crossbeam_channel::{bounded, Sender};
use role::a_to_all::RoleAtoAll;
use role::Role;

pub struct RoleAlltoA<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleAlltoA<R> {
    type Dual = RoleAtoAll<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        return (
            RoleAlltoA {
                sender: sender_dual,
            },
            RoleAtoAll { sender: sender },
        );
    }
}

pub fn next_all_to_a<R>(r: RoleAlltoA<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
