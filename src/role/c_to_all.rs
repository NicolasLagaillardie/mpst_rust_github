//! This module contains the required definitions and
//! functions for the basic role C when it is making
//! an internal choice.
//! Its dual is [RoleAlltoC](crate::role::all_to_c::RoleAlltoC).

use crate::role::all_to_c::RoleAlltoC;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`MeshedChannels`] related to C
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
/// use mpstthree::role::c_to_all::RoleCtoAll;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::role::Role; // Only used for example
///
/// type StackCtoAllDual = RoleCtoAll<RoleEnd, RoleEnd>;
///
/// let _ = StackCtoAllDual::new(); // Only used for example
/// ```
/// html
#[derive(Debug)]
pub struct RoleCtoAll<R1, R2>
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

impl<R1: Role, R2: Role> Role for RoleCtoAll<R1, R2> {
    type Dual = RoleAlltoC<R1::Dual, R2::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender_normal_1, _) = bounded::<R1>(1);
        let (sender_normal_2, _) = bounded::<R2>(1);
        let (sender_dual_1, _) = bounded::<R1::Dual>(1);
        let (sender_dual_2, _) = bounded::<R2::Dual>(1);

        (
            RoleCtoAll {
                sender1: sender_dual_1,
                sender2: sender_dual_2,
            },
            RoleAlltoC {
                sender1: sender_normal_1,
                sender2: sender_normal_2,
            },
        )
    }

    fn head_str() -> String {
        "RoleCtoAll".to_string()
    }

    fn tail_str() -> String {
        format!(
            "{}<{}> + {}<{}>",
            R1::head_str(),
            R1::tail_str(),
            R2::head_str(),
            R2::tail_str()
        )
    }

    fn self_head_str(&self) -> String {
        "RoleCtoAll".to_string()
    }

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

impl<R1: Role, R2: Role> RoleCtoAll<R1, R2> {
    /// Return the right continuation for RoleCtoAll
    pub fn continuation_left(&self) -> R1 {
        let (here, there) = R1::new();
        self.sender1.send(there).unwrap_or(());
        here
    }

    /// Return the left continuation for RoleCtoAll
    pub fn continuation_right(&self) -> R2 {
        let (here, there) = R2::new();
        self.sender2.send(there).unwrap_or(());
        here
    }
}
