use crossbeam_channel::{bounded, Sender};
use role::c_to_a::RoleCtoA;
use role::Role;

pub struct RoleAtoC<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleAtoC<R> {
    type Dual = RoleCtoA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        return (
            RoleAtoC {
                sender: sender_dual,
            },
            RoleCtoA { sender: sender },
        );
    }
}

pub fn next_a_to_c<R>(r: RoleAtoC<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
