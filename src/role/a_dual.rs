//! This module contains the required definitions and
//! functions for the dual of the basic role A.

use crate::role::a::RoleA;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`MeshedChannels`] related to
/// the `Dual` of A.
///
/// This `struct` should only be used in the `stack` field
/// of the [`MeshedChannels`] related
/// to the `Dual` of A.
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// # Example
///
/// ```
/// use mpstthree::role::a_dual::RoleADual;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::role::Role; // Only used for example
///
/// type StackADual = RoleADual<RoleEnd>;
///
/// let _ = StackADual::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct RoleADual<R>
where
    R: Role,
    R::Dual: Role,
{
    #[doc(hidden)]
    pub sender: Sender<R::Dual>,
}

impl<R: Role> Role for RoleADual<R> {
    type Dual = RoleA<R::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = bounded::<R>(1);
        let (sender_dual, _) = bounded::<R::Dual>(1);

        (
            RoleADual {
                sender: sender_dual,
            },
            RoleA {
                sender: sender_normal,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        "RoleADual".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "RoleADual".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        format!("{}<{}>", R::head_str(), R::tail_str())
    }
}

impl<R: Role> RoleADual<R> {
    /// Return the continuation for RoleADual
    pub fn continuation(&self) -> R {
        let (here, there) = R::new();
        self.sender.send(there).unwrap_or(());
        here
    }
}
