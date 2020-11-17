////////////////////////////////////////////
/// OFFER

///  Create the *OfferMpst* type to be used with more than 3 participants.
///  
///  # Arguments
///  
///  * The name of the new *OfferMpst* type
///  * The *SessionMpst* type that will be used
///  * The number of participants (all together)
///  
///  # Examples
///  
///  ```
///  use mpstthree::{create_sessionmpst, create_offer_type_multi};
///
///  create_sessionmpst!(SessionMpst, 3);
///  create_offer_type_multi!(OfferMpstThree, SessionMpst, 3);
///  ```
#[macro_export]
macro_rules! create_offer_type_multi {
    ($type_name: ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            type $type_name<#(S#N:0,)2:0 R0, R1, N0> = mpstthree::binary::Recv<either::Either<$sessionmpst_name<#(S#N:0,)0:0 R0, N0>, $sessionmpst_name<#(S#N:0,)3:0 R1, N0>>, mpstthree::binary::End>;
        });
    }
}

/// Get an offer on $exclusion
#[macro_export]
macro_rules! create_offer_mpst_session_multi {
    ($func_name:ident, $type_name: ident, $role:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<'a, #(S#N:0,)2:0 F, G, R1, R2, U>(
                s: $sessionmpst_name<
                    %(
                        mpstthree::binary::End,
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
                #(S#N:0: mpstthree::binary::Session,)2:0
                #(
                    R#N:0: mpstthree::role::Role,
                )0:0
                F: FnOnce(
                    $sessionmpst_name<
                        #(S#N:0,)0:0
                        R1,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                G: FnOnce(
                    $sessionmpst_name<
                        #(S#N:0,)0:2
                        R2,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
            {
                let (e, s) = $recv_func(s)?;
                mpstthree::binary::cancel(s);
                e.either(f, g)
            }
        });
    }
}

/// Get a mpst offer
#[macro_export]
macro_rules! offer_mpst {
    ($session:expr, $recv_mpst:ident, { $($pat:pat => $result:block, )* }) => {
        (move || -> Result<_, _> {
            let (l, s) = $recv_mpst($session)?;
            mpstthree::binary::cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
}
