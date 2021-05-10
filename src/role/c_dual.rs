use crate::role::c::RoleC;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`mpstthree::sessionmpst::SessionMpst`] related to
/// the `Dual` of C.
///
/// This `struct` should only be used in the `stack` field
/// of the [`mpstthree::sessionmpst::SessionMpst`] related
/// to the `Dual` of C.
///
/// [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html
#[derive(Debug)]
pub struct RoleCDual<R>
where
    R: Role,
    R::Dual: Role,
{
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleCDual<R> {
    type Dual = RoleC<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleCDual {
                sender: sender_dual,
            },
            RoleC {
                sender: sender_normal,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleCDual")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

#[doc(hidden)]
pub fn next_c_dual<R>(r: RoleCDual<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send(there).unwrap_or(());
    here
}
