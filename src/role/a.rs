use crate::role::a_dual::RoleADual;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`mpstthree::meshedchannels::MeshedChannels`] related to A.
///
/// This `struct` should only be used in the `stack` field
/// of the [`mpstthree::meshedchannels::MeshedChannels`] related
/// to A.
///
/// [`mpstthree::meshedchannels::MeshedChannels`]: ../meshedchannels/struct.MeshedChannels.html
#[derive(Debug)]
pub struct RoleA<R>
where
    R: Role,
    R::Dual: Role,
{
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleA<R> {
    type Dual = RoleADual<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleA {
                sender: sender_dual,
            },
            RoleADual {
                sender: sender_normal,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleA")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}
