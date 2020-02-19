use crossbeam_channel::{bounded, Sender};
use role::c_to_all::RoleCtoAll;
use role::Role;

pub struct RoleAlltoC<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleAlltoC<R> {
    type Dual = RoleCtoAll<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        return (
            RoleAlltoC {
                sender: sender_dual,
            },
            RoleCtoAll { sender: sender },
        );
    }
}

pub fn next_all_to_c<R>(r: RoleAlltoC<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
