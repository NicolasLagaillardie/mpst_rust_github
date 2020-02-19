use crossbeam_channel::{bounded, Sender};
use role::b_to_all::RoleBtoAll;
use role::Role;

pub struct RoleAlltoB<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleAlltoB<R> {
    type Dual = RoleBtoAll<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        return (
            RoleAlltoB {
                sender: sender_dual,
            },
            RoleBtoAll { sender: sender },
        );
    }
}

pub fn next_all_to_b<R>(r: RoleAlltoB<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
