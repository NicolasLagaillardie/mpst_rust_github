use crossbeam_channel::{bounded, Sender};
use role::b_to_c::RoleBtoC;
use role::Role;

use std::fmt;

/// Gives the order to the `SessionMpst` related to C to execute its `session` field with B.
///
/// This `struct` should only be used in the `queue` field of the `SessionMpst` related to C.
pub struct RoleCtoB<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleCtoB<R> {
    type Dual = RoleBtoC<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleCtoB {
                sender: sender_dual,
            },
            RoleBtoC {
                sender: sender_normal,
            },
        )
    }

    #[doc(hidden)]
    fn head() -> String {
        String::from("RoleCtoB")
    }
}

impl<R: Role> fmt::Display for RoleCtoB<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RoleCtoB")
    }
}

/// Send a value of type `Role`. Always succeeds. Returns the continuation of the
/// queue `R`.
pub fn next_c_to_b<R>(r: RoleCtoB<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
