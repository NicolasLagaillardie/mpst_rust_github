////////////////////////////////////////////
/// OFFER

/// Create the *OfferMpst* type to be used with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *OfferMpst* type
/// * The *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_offer_type_multi, create_sessionmpst};
///
/// create_sessionmpst!(SessionMpst, 3);
/// create_offer_type_multi!(OfferMpstThree, SessionMpst, 3);
/// ```
#[macro_export]
macro_rules! create_offer_type_multi {
    ($type_name: ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            type $type_name<#(S#N:0,)2:0 R0, R1, N0> = mpstthree::binary::struct_trait::Recv<either::Either<$sessionmpst_name<#(S#N:0,)0:0 R0, N0>, $sessionmpst_name<#(S#N:0,)3:0 R1, N0>>, mpstthree::binary::struct_trait::End>;
        });
    }
}

/// Creates an *OfferMpst* function to receive an offer on a given binary session type of a
/// SessionMpst with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *OfferMpst* function
/// * The name of the *OfferMpst* type that will be used
/// * The name of the dual of the broadcasting sender. This one should contain *toAll* according to
///   the convention
/// * The name of the *recv* function that is related
/// * The name of the receiver
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the SessionMpst for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::{
///     create_broadcast_role, create_normal_role, create_offer_mpst_session_multi,
///     create_offer_type_multi, create_recv_mpst_all_session, create_sessionmpst,
/// };
///
/// create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
/// create_broadcast_role!(RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);
///
/// create_sessionmpst!(SessionMpst, 3);
/// create_offer_type_multi!(OfferMpstThree, SessionMpst, 3);
///
/// create_recv_mpst_all_session!(recv_mpst_b_all_to_d, RoleAlltoD, RoleB, SessionMpst, 3, 2);
///
/// create_offer_mpst_session_multi!(
///     offer_mpst_session_b_to_d,
///     OfferMpstThree,
///     RoleAlltoD,
///     recv_mpst_b_all_to_d,
///     RoleB,
///     SessionMpst,
///     3,
///     2
/// );
/// ```
#[macro_export]
macro_rules! create_offer_mpst_session_multi {
    ($func_name:ident, $type_name: ident, $role:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<'a, #(S#N:0,)2:0 F, G, R1, R2, U>(
                s: $sessionmpst_name<
                    %(
                        mpstthree::binary::struct_trait::End,
                    )(
                        $type_name<#(S#N:0,)2:0 R1, R2, $name<mpstthree::role::end::RoleEnd>>,
                    )0*
                    $role<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
                f: F,
                g: G,
            ) -> Result<U, Box<dyn std::error::Error + 'a>>
            where
                #(
                    S#N:0: mpstthree::binary::struct_trait::Session,
                )2:0
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
                F: FnOnce(
                    $sessionmpst_name<
                        #(S#N:0,)0:0
                        R1,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                G: FnOnce(
                    $sessionmpst_name<
                        #(S#N:0,)3:0
                        R2,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
            {
                let (e, s) = $recv_func(s)?;
                mpstthree::binary::cancel::cancel(s);
                e.either(f, g)
            }
        });
    }
}

/// Offer a choice between many different sessions wrapped in an `enum`
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
/// offer_mpst!(s, recv_mpst_a_from_d, {
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
#[macro_export]
macro_rules! offer_mpst {
    ($session:expr, $recv_mpst:ident, { $($pat:pat => $result:expr, )* }) => {
        (move || -> Result<_, _> {
            let (l, s) = $recv_mpst($session)?;
            mpstthree::binary::cancel::cancel(s);
            match l {
                $(
                    $pat => $result,
                )*
            }
        })()
    };
}

/// Offer a choice and send the session to the pawn
///
/// # Arguments
///
/// * The session to be used
/// * The *recv* function that will be used
/// * Each path, which are each variant of the enum which contains the new branches
/// * The block of code to process each new session
#[macro_export]
macro_rules! offer_cancel_mpst {
    ($session:expr, $recv_mpst:ident, { $($pat:pat => $result:expr, )* }) => {
        (move || -> Result<_, _> {
            let ((session1, cont), s) = $recv_mpst($session)?;
            let s = s.session1.sender.send(mpstthree::binary::struct_trait::Signal::Offer(session1)).unwrap();
            mpstthree::binary::cancel::cancel(s);
            match cont {
                $(
                    $pat => $result,
                )*
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
#[macro_export]
macro_rules! offer_http_mpst {
    ($session:expr, $recv_mpst:ident, { $($pat:pat => $result:expr, )* }) => {
        (move || -> Result<_, _> {
            let (l, s, req) = $recv_mpst($session, false, hyper::Request::default())?;
            mpstthree::binary::cancel::cancel(s);
            mpstthree::binary::cancel::cancel(req);
            match l {
                $(
                    $pat => $result,
                )*
            }
        })()
    };
}
