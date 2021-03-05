////////////////////////////////////////////
/// OFFER

/// Create an *offer* function to recv on the first binary
/// session from any kind of role.  Must be used with
/// [`mpstthree::sessionmpst::SessionMpst`].
///
/// # Arguments
///  
/// * The name of the new *offer* function
/// * The name of the dual of the broadcasting sender
/// * The name of related *next* function
/// * The name of the receiver
///  
/// # Example
///  
/// ```
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::functionmpst::OfferMpst;
/// use mpstthree::{create_normal_role, create_broadcast_role, create_recv_mpst_all_session_1, create_offer_mpst_session_1};
///
/// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
/// create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
/// create_broadcast_role!(RoleAlltoA, next_all_to_a, RoleAtoAll, next_a_to_all);
///
/// create_recv_mpst_all_session_1!(recv_mpst_c_all_to_a, RoleAlltoA, next_all_to_a, RoleC);
///
/// create_offer_mpst_session_1!(
///     offer_mpst_session_c_to_a,
///     RoleAlltoA,
///     recv_mpst_c_all_to_a,
///     RoleC
/// );
/// ```
///
/// [`mpstthree::sessionmpst::SessionMpst`]:../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_offer_mpst_session_1 {
    ($func_name:ident, $role:ident, $recv_func:ident, $name:ident) => {
        mpstthree::create_offer_mpst_session_multi!(
            $func_name,
            OfferMpst,
            $role,
            $recv_func,
            $name,
            SessionMpst,
            3,
            1
        );
    };
}

/// Create an *offer* function to recv on the second binary
/// session from any kind of role.  Must be used with
/// [`mpstthree::sessionmpst::SessionMpst`].
///
/// # Arguments
///  
/// * The name of the new *offer* function
/// * The name of the dual of the broadcasting sender
/// * The name of related *next* function
/// * The name of the receiver
///  
/// # Example
///  
/// ```
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::functionmpst::OfferMpst;
/// use mpstthree::{create_normal_role, create_broadcast_role, create_recv_mpst_all_session_2, create_offer_mpst_session_2};
///
/// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
/// create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
/// create_broadcast_role!(RoleAlltoC, next_all_to_c, RoleCtoAll, next_c_to_all);
///
/// create_recv_mpst_all_session_2!(recv_mpst_a_all_to_c, RoleAlltoC, next_all_to_c, RoleA);
///
/// create_offer_mpst_session_2!(
///     offer_mpst_session_a_to_c,
///     RoleAlltoC,
///     recv_mpst_a_all_to_c,
///     RoleA
/// );
/// ```
///
/// [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_offer_mpst_session_2 {
    ($func_name:ident, $role:ident, $recv_func:ident, $name:ident) => {
        mpstthree::create_offer_mpst_session_multi!(
            $func_name,
            OfferMpst,
            $role,
            $recv_func,
            $name,
            SessionMpst,
            3,
            2
        );
    };
}
