use crate::role::b::RoleB;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`mpstthree::sessionmpst::SessionMpst`] related to
/// the `Dual` of B.
///
/// This `struct` should only be used in the `stack` field
/// of the [`mpstthree::sessionmpst::SessionMpst`] related
/// to the `Dual` of B.
///
/// [`mpstthree::sessionmpst::SessionMpst`]:
/// ../sessionmpst/struct.SessionMpst.html
#[derive(Debug)]
pub struct RoleBDual<R: Role> {
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleBDual<R> {
    type Dual = RoleB<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleBDual {
                sender: sender_dual,
            },
            RoleB {
                sender: sender_normal,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleBDual")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

#[doc(hidden)]
pub fn next_b_dual<R>(r: RoleBDual<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
