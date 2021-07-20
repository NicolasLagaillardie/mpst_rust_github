use crate::role::c_dual::RoleCDual;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`MeshedChannels`] related to C.
///
/// This `struct` should only be used in the `stack` field
/// of the [`MeshedChannels`] related
/// to C.
///
/// [`MeshedChannels`]: RoleBroadcastmeshedchannels/struct.MeshedChannels.html
///
/// # Example
///
/// ```
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// type NameC = RoleC<RoleEnd>;
/// ```
#[derive(Debug)]
pub struct RoleC<R>
where
    R: Role,
    R::Dual: Role,
{
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleC<R> {
    type Dual = RoleCDual<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleC {
                sender: sender_dual,
            },
            RoleCDual {
                sender: sender_normal,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleC")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        String::from("RoleC")
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}
