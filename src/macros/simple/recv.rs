////////////////////////////////////////////
/// RECV

/// Create a *recv* function to recv on the first binary
/// session from any kind of role.  Must be used with
/// [`mpstthree::sessionmpst::SessionMpst`].
///
/// # Arguments
///
/// * The name of the new *recv* function
/// * The name of the sender
/// * The name of related *next* function
/// * The name of the receiver
///
/// # Example
///
/// ```
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::{create_normal_role, create_recv_mpst_session_1};
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_role!(RoleC, RoleCDual);
///
/// create_recv_mpst_session_1!(recv_mpst_c_from_a, RoleA, RoleC);
/// ```
///
/// [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_recv_mpst_session_1 {
    ($func_name:ident, $role:ident, $name:ident) => {
        mpstthree::create_recv_mpst_session!($func_name, $role, $name, SessionMpst, 3, 1);
    };
}

/// Create a *recv* function to recv on the second binary
/// session from any kind of role.  Must be used with
/// [`mpstthree::sessionmpst::SessionMpst`].
///
/// # Arguments
///
/// * The name of the new *recv* function
/// * The name of the sender
/// * The name of related *next* function
/// * The name of the receiver
///
/// # Example
///
/// ```
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::{create_normal_role, create_recv_mpst_session_2};
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_role!(RoleC, RoleCDual);
///
/// create_recv_mpst_session_2!(recv_mpst_a_from_c, RoleC, RoleA);
/// ```
///
/// [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_recv_mpst_session_2 {
    ($func_name:ident, $role:ident, $name:ident) => {
        mpstthree::create_recv_mpst_session!($func_name, $role, $name, SessionMpst, 3, 2);
    };
}