use crossbeam_channel::{bounded, Sender};
use role::c_to_b::RoleCtoB;
use role::Role;

/// Gives the order to the `SessionMpst` related to B to execute its `session` field with C.
///
/// This `struct` should only be used in the `queue` field of the `SessionMpst` related to B.
#[derive(Debug)]
pub struct RoleBtoC<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleBtoC<R> {
    type Dual = RoleCtoB<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleBtoC {
                sender: sender_dual,
            },
            RoleCtoB {
                sender: sender_normal,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleBtoC")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

/// Send a value of type `Role`. Always succeeds. Returns the continuation of the
/// queue `R`.
pub fn next_b_to_c<R>(r: RoleBtoC<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
