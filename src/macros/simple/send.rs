////////////////////////////////////////////
/// SEND

/// Create a *send* function to send on the first binary
/// session from any kind of role.  Must be used with
/// [`mpstthree::sessionmpst::SessionMpst`].
///
/// # Arguments
///  
/// * The name of the new *send* function
/// * The name of the receiver
/// * The name of related *next* function
/// * The name of the sender
///  
/// # Example
///  
/// ```
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::{create_normal_role, create_send_mpst_session_1};
///
/// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
/// create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
///
/// create_send_mpst_session_1!(send_mpst_c_to_a, RoleA, next_a, RoleC);
/// ```
///
/// [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_send_mpst_session_1 {
    ($func_name:ident, $role:ident, $next:ident, $name:ident) => {
        mpstthree::create_send_mpst_session!($func_name, $role, $next, $name, SessionMpst, 3, 1);
    };
}

/// Create a *send* function to send on the second binary
/// session from any kind of role.  Must be used with
/// [`mpstthree::sessionmpst::SessionMpst`].
///
/// # Arguments
///  
/// * The name of the new *send* function
/// * The name of the receiver
/// * The name of related *next* function
/// * The name of the sender
///  
/// # Example
///  
/// ```
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::{create_normal_role, create_send_mpst_session_2};
///
/// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
/// create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
///
/// create_send_mpst_session_2!(send_mpst_a_to_c, RoleC, next_c, RoleA);
/// ```
///
/// [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_send_mpst_session_2 {
    ($func_name:ident, $role:ident, $next:ident, $name:ident) => {
        mpstthree::create_send_mpst_session!($func_name, $role, $next, $name, SessionMpst, 3, 2);
    };
}
