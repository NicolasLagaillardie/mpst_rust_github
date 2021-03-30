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
/// use mpstthree::functionmpst::OfferMpst;
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::{
///     create_broadcast_role, create_normal_role, create_offer_mpst_session_1
/// };
///
/// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
/// create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
/// create_broadcast_role!(RoleAlltoA, next_all_to_a, RoleAtoAll, next_a_to_all);
///
/// create_offer_mpst_session_1!(
///     offer_mpst_session_c_to_a,
///     RoleAlltoA,
///     RoleC
/// );
/// ```
///
/// [`mpstthree::sessionmpst::SessionMpst`]:../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_offer_mpst_session_1 {
    ($func_name:ident, $role:ident, $name:ident) => {
        mpstthree::create_offer_mpst_session_multi!(
            $func_name,
            OfferMpst,
            $role,
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
/// use mpstthree::functionmpst::OfferMpst;
/// use mpstthree::sessionmpst::SessionMpst;
/// use mpstthree::{
///     create_broadcast_role, create_normal_role, create_offer_mpst_session_2
/// };
///
/// create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
/// create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
/// create_broadcast_role!(RoleAlltoC, next_all_to_c, RoleCtoAll, next_c_to_all);
///
/// create_offer_mpst_session_2!(
///     offer_mpst_session_a_to_c,
///     RoleAlltoC,
///     RoleA
/// );
/// ```
///
/// [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_offer_mpst_session_2 {
    ($func_name:ident, $role:ident, $name:ident) => {
        mpstthree::create_offer_mpst_session_multi!(
            $func_name,
            OfferMpst,
            $role,
            $name,
            SessionMpst,
            3,
            2
        );
    };
}
