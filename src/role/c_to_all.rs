use crossbeam_channel::{bounded, Sender};
use role::all_to_c::RoleAlltoC;
use role::Role;

pub struct RoleCtoAll<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleCtoAll<R> {
    type Dual = RoleAlltoC<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        return (
            RoleCtoAll {
                sender: sender_dual,
            },
            RoleAlltoC { sender: sender },
        );
    }
}

pub fn next_c_to_all<R>(r: RoleCtoAll<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
