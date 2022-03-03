//! This module contains the required definitions and
//! functions for the basic role B when it is making
//! an internal choice.
//! Its dual is [RoleAlltoB](crate::role::all_to_b::RoleAlltoB).

use crate::role::all_to_b::RoleAlltoB;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`MeshedChannels`] related to B
/// to execute its [`Session`] fields
/// with every other processes.
///
/// This `struct` is used for branching without `enum`. See
/// the test `05_usecase.rs`.
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [`Session`]: crate::binary::struct_trait::session::Session
///
/// # Example
///
/// ```
/// use mpstthree::role::b_to_all::RoleBtoAll;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::role::Role; // Only used for example
///
/// type StackBtoAllDual = RoleBtoAll<RoleEnd, RoleEnd>;
///
/// let _ = StackBtoAllDual::new(); // Only used for example
/// ```
#[derive(Debug)]
pub struct RoleBtoAll<R1, R2>
where
    R1: Role,
    R2: Role,
    R1::Dual: Role,
    R2::Dual: Role,
{
    #[doc(hidden)]
    pub sender1: Sender<R1::Dual>,
    #[doc(hidden)]
    pub sender2: Sender<R2::Dual>,
}

impl<R1: Role, R2: Role> Role for RoleBtoAll<R1, R2> {
    type Dual = RoleAlltoB<R1::Dual, R2::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal_1, _) = bounded::<R1>(1);
        let (sender_normal_2, _) = bounded::<R2>(1);
        let (sender_dual_1, _) = bounded::<R1::Dual>(1);
        let (sender_dual_2, _) = bounded::<R2::Dual>(1);

        (
            RoleBtoAll {
                sender1: sender_dual_1,
                sender2: sender_dual_2,
            },
            RoleAlltoB {
                sender1: sender_normal_1,
                sender2: sender_normal_2,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        "RoleBtoAll".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!(
            "{}<{}> + {}<{}>",
            R1::head_str(),
            R1::tail_str(),
            R2::head_str(),
            R2::tail_str()
        )
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "RoleBtoAll".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        format!(
            "{}<{}> + {}<{}>",
            R1::head_str(),
            R1::tail_str(),
            R2::head_str(),
            R2::tail_str()
        )
    }
}

impl<R1: Role, R2: Role> RoleBtoAll<R1, R2> {
    /// Return the right continuation for RoleBtoAll
    pub fn continuation_left(&self) -> R1 {
        let (here, there) = R1::new();
        self.sender1.send(there).unwrap_or(());
        here
    }

    /// Return the left continuation for RoleBtoAll
    pub fn continuation_right(&self) -> R2 {
        let (here, there) = R2::new();
        self.sender2.send(there).unwrap_or(());
        here
    }
}
