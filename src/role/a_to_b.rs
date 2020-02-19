use crossbeam_channel::{bounded, Sender};
use role::b_to_a::RoleBtoA;
use role::Role;

pub struct RoleAtoB<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleAtoB<R> {
    type Dual = RoleBtoA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        return (
            RoleAtoB {
                sender: sender_dual,
            },
            RoleBtoA { sender: sender },
        );
    }
}

pub fn next_a_to_b<R>(r: RoleAtoB<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
