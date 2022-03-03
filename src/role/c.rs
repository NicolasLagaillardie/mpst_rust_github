//! This module contains the required definitions and
//! functions for the basic role C.
//! Its dual is [RoleCDual](crate::role::c_dual::RoleCDual).

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
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// # Example
///
/// ```
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::role::Role; // Only used for example
///
/// type StackC = RoleC<RoleEnd>;
///
/// let _ = StackC::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct RoleC<R>
where
    R: Role,
    R::Dual: Role,
{
    #[doc(hidden)]
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
        "RoleC".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "RoleC".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

impl<R: Role> RoleC<R> {
    /// Return the continuation for RoleC
    pub fn continuation(&self) -> R {
        let (here, there) = R::new();
        self.sender.send(there).unwrap_or(());
        here
    }
}
