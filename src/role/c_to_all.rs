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
/// [`MeshedChannels`]: RoleBroadcastmeshedchannels/struct.MeshedChannels.html
/// [`Session`]: RoleBroadcastbinary/trait.Session.
///
/// # Example
///
/// ```
/// use mpstthree::role::c_to_all::RoleCtoAll;
/// use mpstthree::role::end::RoleEnd;
///
/// type NameCtoAllDual = RoleCtoAll<RoleEnd, RoleEnd>;
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
    pub sender1: Sender<R1::Dual>,
    pub sender2: Sender<R2::Dual>,
}

impl<R1: Role, R2: Role> Role for RoleCtoAll<R1, R2> {
    type Dual = RoleAlltoC<R1::Dual, R2::Dual>;

    #[doc(hidden)]
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

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleCtoAll")
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
        String::from("RoleCtoAll")
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
