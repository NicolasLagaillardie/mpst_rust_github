use crossbeam_channel::{bounded, Sender};
use role::a_to_all::RoleAtoAll;
use role::Role;

pub struct RoleAlltoA<R1: Role, R2: Role> {
    pub sender1: Sender<R1::Dual>,
    pub sender2: Sender<R2::Dual>,
}

impl<R1: Role, R2: Role> Role for RoleAlltoA<R1, R2> {
    type Dual = RoleAtoAll<R1::Dual, R2::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender1, _) = bounded::<R1>(1);
        let (sender2, _) = bounded::<R2>(1);
        let (sender_dual1, _) = bounded::<R1::Dual>(1);
        let (sender_dual2, _) = bounded::<R2::Dual>(1);

        return (
            RoleAlltoA {
                sender1: sender_dual1,
                sender2: sender_dual2,
            },
            RoleAtoAll {
                sender1: sender1,
                sender2: sender2,
            },
        );
    }
}

pub fn next_all_to_a<R1, R2>(r: RoleAlltoA<R1, R2>) -> (R1, R2)
where
    R1: Role,
    R2: Role,
{
    let (here1, there1) = R1::new();
    let (here2, there2) = R2::new();
    r.sender1.send(there1).unwrap_or(());
    r.sender2.send(there2).unwrap_or(());
    (here1, here2)
}