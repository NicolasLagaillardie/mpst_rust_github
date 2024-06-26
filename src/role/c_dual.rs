//! This module contains the required definitions and
//! functions for the dual of the basic role C.

use crate::role::c::RoleC;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`MeshedChannels`] related to
/// the `Dual` of C.
///
/// This `struct` should only be used in the `stack` field
/// of the [`MeshedChannels`] related
/// to the `Dual` of C.
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// # Example
///
/// ```
/// use mpstthree::role::c_dual::RoleCDual;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::role::Role; // Only used for example
///
/// type StackCDual = RoleCDual<RoleEnd>;
///
/// let _ = StackCDual::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct RoleCDual<R>
where
    R: Role,
    R::Dual: Role,
{
    #[doc(hidden)]
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleCDual<R> {
    type Dual = RoleC<R::Dual>;

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

    fn head_str() -> String {
        "RoleCDual".to_string()
    }

    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }

    fn self_head_str(&self) -> String {
        "RoleCDual".to_string()
    }

    fn self_tail_str(&self) -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

impl<R: Role> RoleCDual<R> {
    /// Return the continuation for RoleCDual
    pub fn continuation(&self) -> R {
        let (here, there) = R::new();
        self.sender.send(there).unwrap_or(());
        here
    }
}
