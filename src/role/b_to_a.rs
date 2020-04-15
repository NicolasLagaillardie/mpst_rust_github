use crossbeam_channel::{bounded, Sender};
use role::a_to_b::RoleAtoB;
use role::Role;

/// Gives the order to the `SessionMpst` related to B to execute its `session` field with A.
///
/// This `struct` should only be used in the `queue` field of the `SessionMpst` related to B.
pub struct RoleBtoA<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleBtoA<R> {
    type Dual = RoleAtoB<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleBtoA {
                sender: sender_dual,
            },
            RoleAtoB {
                sender: sender_normal,
            },
        )
    }
}

/// Send a value of type `Role`. Always succeeds. Returns the continuation of the
/// queue `R`.
pub fn next_b_to_a<R>(r: RoleBtoA<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
