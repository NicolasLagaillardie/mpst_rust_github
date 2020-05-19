use crossbeam_channel::{bounded, Sender};
use role::a_to_c::RoleAtoC;
use role::Role;

/// Gives the order to the `SessionMpst` related to C to execute its `session` field with A.
///
/// This `struct` should only be used in the `queue` field of the `SessionMpst` related to C.
#[derive(Debug)]
pub struct RoleCtoA<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleCtoA<R> {
    type Dual = RoleAtoC<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleCtoA {
                sender: sender_dual,
            },
            RoleAtoC {
                sender: sender_normal,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleCtoA")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

/// Send a value of type `Role`. Always succeeds. Returns the continuation of the
/// queue `R`.
pub fn next_c_to_a<R>(r: RoleCtoA<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
