use crate::role::b::RoleB;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`MeshedChannels`] related to
/// the `Dual` of B.
///
/// This `struct` should only be used in the `stack` field
/// of the [`MeshedChannels`] related
/// to the `Dual` of B.
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// # Example
///
/// ```
/// use mpstthree::role::b_dual::RoleBDual;
/// use mpstthree::role::end::RoleEnd;
///
/// type NameBDual = RoleBDual<RoleEnd>;
/// ```
#[derive(Debug)]
pub struct RoleBDual<R>
where
    R: Role,
    R::Dual: Role,
{
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
        "RoleBDual".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "RoleBDual".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}
