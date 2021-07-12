use crate::role::a_to_all::RoleAtoAll;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// This structure is used by any participant other than A
/// to receive a choice given by A.
///
/// This `struct` is used for branching without `enum`. See
/// the test `05_usecase.rs`.
#[derive(Debug)]
pub struct RoleAlltoA<R1, R2>
where
    R1: Role,
    R2: Role,
    R1::Dual: Role,
    R2::Dual: Role,
{
    pub sender1: Sender<R1::Dual>,
    pub sender2: Sender<R2::Dual>,
}

impl<R1: Role, R2: Role> Role for RoleAlltoA<R1, R2> {
    type Dual = RoleAtoAll<R1::Dual, R2::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal_1, _) = bounded::<R1>(1);
        let (sender_normal_2, _) = bounded::<R2>(1);
        let (sender_dual_1, _) = bounded::<R1::Dual>(1);
        let (sender_dual_2, _) = bounded::<R2::Dual>(1);

        (
            RoleAlltoA {
                sender1: sender_dual_1,
                sender2: sender_dual_2,
            },
            RoleAtoAll {
                sender1: sender_normal_1,
                sender2: sender_normal_2,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleAlltoA")
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
}
