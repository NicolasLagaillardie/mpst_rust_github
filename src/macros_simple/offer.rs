//! This module contains the macros
//! for creating offer functions for three
//! of participants, whatever are their name.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_simple"` feature.*

/// Create an *offer* function to recv on the first binary
/// session from any kind of role.  Must be used with
/// [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *offer* function
/// * The name of the dual of the broadcasting sender
/// * The name of the receiver
///
/// # Example
///
/// ```
/// use mpstthree::functionmpst::OfferMpst;
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::{
///     create_broadcast_role, create_normal_name, create_normal_role, create_offer_mpst_session_1,
/// };
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_role!(RoleC, RoleCDual);
/// create_normal_name!(NameC);
/// create_broadcast_role!(RoleAlltoA, RoleAtoAll);
///
/// create_offer_mpst_session_1!(offer_mpst_session_c_to_a, RoleAlltoA, NameC);
/// ```
///
/// [`MeshedChannels`]:.crate::meshedchannels::MeshedChannels.
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_offer_mpst_session_1 {
    ($func_name:ident, $role:ident, $name:ident) => {
        mpst_seq_proc::create_offer_mpst_session_multi!(
            $func_name,
            OfferMpst,
            $role,
            $name,
            MeshedChannels,
            3,
            1
        );
    };
}

/// Create an *offer* function to recv on the second binary
/// session from any kind of role.  Must be used with
/// [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *offer* function
/// * The name of the dual of the broadcasting sender
/// * The name of the receiver
///
/// # Example
///
/// ```
/// use mpstthree::functionmpst::OfferMpst;
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::{
///     create_broadcast_role, create_normal_name, create_normal_role, create_offer_mpst_session_2,
/// };
///
/// create_normal_role!(RoleA, RoleADual);
/// create_normal_role!(RoleC, RoleCDual);
/// create_normal_name!(NameA);
/// create_broadcast_role!(RoleAlltoC, RoleCtoAll);
///
/// create_offer_mpst_session_2!(offer_mpst_session_a_to_c, RoleAlltoC, NameA);
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_offer_mpst_session_2 {
    ($func_name:ident, $role:ident, $name:ident) => {
        mpst_seq_proc::create_offer_mpst_session_multi!(
            $func_name,
            OfferMpst,
            $role,
            $name,
            MeshedChannels,
            3,
            2
        );
    };
}
