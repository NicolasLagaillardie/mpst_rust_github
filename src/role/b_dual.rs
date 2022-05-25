//! This module contains the required definitions and
//! functions for the dual of the basic role B.

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
/// use mpstthree::role::Role; // Only used for example
///
/// type StackBDual = RoleBDual<RoleEnd>;
///
/// let _ = StackBDual::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct RoleBDual<R>
where
    R: Role,
    R::Dual: Role,
{
    #[doc(hidden)]
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleBDual<R> {
    type Dual = RoleB<R::Dual>;

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

    fn head_str() -> String {
        "RoleBDual".to_string()
    }

    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }

    fn self_head_str(&self) -> String {
        "RoleBDual".to_string()
    }

    fn self_tail_str(&self) -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

impl<R: Role> RoleBDual<R> {
    /// Return the continuation for RoleBDual
    pub fn continuation(&self) -> R {
        let (here, there) = R::new();
        self.sender.send(there).unwrap_or(());
        here
    }
}
