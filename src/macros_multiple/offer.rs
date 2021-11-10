//! This module contains the macros
//! for creating offer functions for any number
//! of participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_multiple"` feature.*

/// Create the *OfferMpst* type to be used with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *OfferMpst* type
/// * The *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_meshedchannels, create_offer_type_multi};
///
/// create_meshedchannels!(MeshedChannels, 3);
/// create_offer_type_multi!(OfferMpstThree, MeshedChannels, 3);
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_offer_type_multi {
    ($type_name:ident, $meshedchannels_name:ident, $nsessions:literal) => {
        mpst_seq::create_offer_type_multi!($type_name, $meshedchannels_name, $nsessions);
    };
}

/// Creates an *OfferMpst* function to receive an offer on a given binary session type of a
/// MeshedChannels.
///
/// # Arguments
///
/// * The name of the new *OfferMpst* function
/// * The name of the *OfferMpst* type that will be used
/// * The name of the dual of the broadcasting sender. This one should contain *toAll* according to
///   the convention
/// * The name of the *recv* function that is related
/// * The name of the receiver
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the MeshedChannels for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::{
///     create_broadcast_role, create_meshedchannels, create_normal_role,
///     create_offer_mpst_session_multi, create_offer_type_multi,
/// };
///
/// create_normal_role!(RoleB, RoleBDual);
/// create_broadcast_role!(RoleAlltoD, RoleDtoAll);
///
/// create_meshedchannels!(MeshedChannels, 3);
/// create_offer_type_multi!(OfferMpstThree, MeshedChannels, 3);
///
/// create_offer_mpst_session_multi!(
///     offer_mpst_session_b_to_d,
///     OfferMpstThree,
///     RoleAlltoD,
///     RoleB,
///     MeshedChannels,
///     3,
///     2
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_offer_mpst_session_multi {
    (
        $func_name:ident,
        $type_name:ident,
        $role:ident,
        $name:ident,
        $meshedchannels_name:ident,
        $nsessions:literal,
        $exclusion:literal
    ) => {
        mpst_seq::create_offer_mpst_session_multi!(
            $func_name,
            $type_name,
            $role,
            $name,
            $meshedchannels_name,
            $nsessions,
            $exclusion
        );
    };
}

/// Offer a choice between many different sessions wrapped in an `enum`
///
/// # Arguments
///
/// * The session to be used
/// * \[Optional\] The *recv* function that will be used
/// * Each path, which are each variant of the enum which contains the new branches
/// * The block of code to process each new session
///
/// # Basic example
///
/// ```ignore
/// offer_mpst!(
///     s,
///     recv_mpst_a_from_d,
///     {
///         CBranchesAtoC::End(s) => {
///             close_mpst_multi(s)
///         },
///         CBranchesAtoC::Video(s) => {
///             let (request, s) = recv_mpst_a_from_d(s)?;
///             let s = send_mpst_a_to_b(request + 1, s);
///             let (video, s) = recv_mpst_a_from_b(s)?;
///             let s = send_mpst_a_to_d(video + 1, s);
///             authenticator_recurs(s)
///         },
///     }
/// )?;
/// ```
///
/// # Baking example
///
/// ```ignore
/// offer_mpst!(
///     s,
///     {
///         CBranchesAtoC::End(s) => {
///             close_mpst_multi(s)
///         },
///         CBranchesAtoC::Video(s) => {
///             let (request, s) = recv_mpst_a_from_d(s)?;
///             let s = send_mpst_a_to_b(request + 1, s);
///             let (video, s) = recv_mpst_a_from_b(s)?;
///             let s = send_mpst_a_to_d(video + 1, s);
///             authenticator_recurs(s)
///         },
///     }
/// )?;
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! offer_mpst {
    ($session: expr, $recv_mpst: ident, { $( $pat: pat => $result: expr, )+ }) => {
        (move || -> Result<_, _> {
            let (l, s) = $recv_mpst($session)?;
            mpstthree::binary::cancel::cancel(s);
            match l {
                $(
                    $pat => $result,
                )+
                _ => panic!("Unexpected payload") ,
            }
        })()
    };
    ($session: expr, { $( $pat: pat => $result: expr, )+ }) => {
        (move || -> Result<_, _> {
            let (l, s) = $session.recv()?;
            mpstthree::binary::cancel::cancel(s);
            match l {
                $(
                    $pat => $result,
                )+
                _ => panic!("Unexpected payload") ,
            }
        })()
    };
}

/// Offer a choice and send the session to the broadcaster
///
/// # Arguments
///
/// * The session to be used
/// * \[Optional\] The *recv* function that will be used
/// * Each path, which are each variant of the enum which contains the new branches
/// * The block of code to process each new session
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! offer_cancel_mpst {
    ($session: expr, $recv_mpst: ident, { $( $pat: pat => $result: expr, )+ }) => {
        (move || -> Result<_, _> {
            let ((session1, cont), s) = $recv_mpst($session)?;
            let s = s.session1.sender.send(mpstthree::binary::struct_trait::end::Signal::Offer(session1)).unwrap();
            mpstthree::binary::cancel::cancel(s);
            match cont {
                $(
                    $pat => $result,
                )+
                _ => panic!("Unexpected payload") ,
            }
        })()
    };
    ($session: expr, { $( $pat: pat => $result: expr, )+ }) => {
        (move || -> Result<_, _> {
            let ((session1, cont), s) = $session.recv()?;
            let s = s.session1.sender.send(mpstthree::binary::struct_trait::end::Signal::Offer(session1)).unwrap();
            mpstthree::binary::cancel::cancel(s);
            match cont {
                $(
                    $pat => $result,
                )+
                _ => panic!("Unexpected payload") ,
            }
        })()
    };
}

/// Offer a choice between many different sessions wrapped in an `enum`. Used with http functions
///
/// # Arguments
///
/// * The session to be used
/// * The *recv* function that will be used
/// * Each path, which are each variant of the enum which contains the new branches
/// * The block of code to process each new session
///
/// # Example
///
/// ```ignore
/// offer_http_mpst!(s, recv_mpst_a_from_d, {
///    CBranchesAtoC::End(s) => {
///        close_mpst_multi(s)
///    },
///    CBranchesAtoC::Video(s) => {
///        let (request, s) = recv_mpst_a_from_d(s)?;
///        let s = send_mpst_a_to_b(request + 1, s);
///        let (video, s) = recv_mpst_a_from_b(s)?;
///        let s = send_mpst_a_to_d(video + 1, s);
///        authenticator_recurs(s)
///    },
/// })?;
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! offer_http_mpst {
    ($session: expr, $recv_mpst: ident, { $( $pat: pat => $result: expr, )+ }) => {
        (move || -> Result<_, _> {
            let https = hyper_tls::HttpsConnector::new();
            let client = hyper::Client::builder().build::<_, hyper::Body>(https);
            let (l, s, req) = $recv_mpst($session, false, Vec::new())?;
            mpstthree::binary::cancel::cancel(s);
            mpstthree::binary::cancel::cancel(req);
            match l {
                $(
                    $pat => $result,
                )+
                _ => panic!("Unexpected payload") ,
            }
        })()
    };
}
