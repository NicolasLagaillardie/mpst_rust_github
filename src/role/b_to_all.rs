use crossbeam_channel::{bounded, Sender};
use role::all_to_b::RoleAlltoB;
use role::Role;

pub struct RoleBtoAll<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleBtoAll<R> {
    type Dual = RoleAlltoB<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        return (
            RoleBtoAll {
                sender: sender_dual,
            },
            RoleAlltoB { sender: sender },
        );
    }
}

pub fn next_b_to_all<R>(r: RoleBtoAll<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
