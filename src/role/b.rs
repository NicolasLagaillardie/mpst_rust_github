//! This module contains the required definitions and
//! functions for the basic role B.
//! Its dual is [RoleBDual](crate::role::b_dual::RoleBDual).

use crate::role::b_dual::RoleBDual;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`MeshedChannels`] related to B.
///
/// This `struct` should only be used in the `stack` field
/// of the [`MeshedChannels`] related
/// to B.
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// # Example
///
/// ```
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::role::Role; // Only used for example
///
/// type StackB = RoleB<RoleEnd>;
///
/// let _ = StackB::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct RoleB<R>
where
    R: Role,
    R::Dual: Role,
{
    #[doc(hidden)]
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleB<R> {
    type Dual = RoleBDual<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleB {
                sender: sender_dual,
            },
            RoleBDual {
                sender: sender_normal,
            },
        )
    }

    fn head_str() -> String {
        "RoleB".to_string()
    }

    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }

    fn self_head_str(&self) -> String {
        "RoleB".to_string()
    }

    fn self_tail_str(&self) -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

impl<R: Role> RoleB<R> {
    /// Return the continuation for RoleB
    pub fn continuation(&self) -> R {
        let (here, there) = R::new();
        self.sender.send(there).unwrap_or(());
        here
    }
}
