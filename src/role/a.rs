//! This module contains the required definitions and
//! functions for the basic role A.
//! Its dual is [RoleADual](crate::role::a_dual::RoleADual).

use crate::role::a_dual::RoleADual;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`MeshedChannels`] related to A.
///
/// This `struct` should only be used in the `stack` field
/// of the [`MeshedChannels`] related
/// to A.
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// # Example
///
/// ```
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::role::Role; // Only used for example
///
/// type StackA = RoleA<RoleEnd>;
///
/// let _ = StackA::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct RoleA<R>
where
    R: Role,
    R::Dual: Role,
{
    #[doc(hidden)]
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleA<R> {
    type Dual = RoleADual<R::Dual>;

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

    fn head_str() -> String {
        "RoleA".to_string()
    }

    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }

    fn self_head_str(&self) -> String {
        "RoleA".to_string()
    }

    fn self_tail_str(&self) -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

impl<R: Role> RoleA<R> {
    /// Return the continuation for RoleA
    pub fn continuation(&self) -> R {
        let (here, there) = R::new();
        self.sender.send(there).unwrap_or(());
        here
    }
}
