use crate::role::all_to_b::RoleAlltoB;
use crate::role::Role;
use crossbeam_channel::{bounded, Sender};

/// Gives the order to the
/// [`mpstthree::sessionmpst::SessionMpst`] related to B
/// to execute its [`mpstthree::binary::Session`] fields
/// with every other processes.
///
/// This `struct` is used for branching without `enum`. See
/// the test `05_usecase.rs`.
///
/// [`mpstthree::sessionmpst::SessionMpst`]:
/// ../sessionmpst/struct.SessionMpst.html
/// [`mpstthree::binary::Session`]: ../binary/trait.Session.
/// html
#[derive(Debug)]
pub struct RoleBtoAll<R1: Role, R2: Role> {
    pub sender1: Sender<R1::Dual>,
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
        String::from("RoleBtoAll")
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

#[doc(hidden)]
pub fn next_b_to_all<R1, R2>(r: RoleBtoAll<R1, R2>) -> (R1, R2)
where
    R1: Role,
    R2: Role,
{
    let (here1, there1) = R1::new();
    let (here2, there2) = R2::new();
    r.sender1.send(there1).unwrap_or(());
    r.sender2.send(there2).unwrap_or(());
    (here1, here2)
}
