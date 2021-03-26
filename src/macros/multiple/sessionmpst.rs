////////////////////////////////////////////
/// SESSIONMPST

/// Creates a SessionMpst for more than 3 participants.
///
/// # Arguments
///
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::create_sessionmpst;
/// use mpstthree::role::Role;
///
/// create_sessionmpst!(SessionMpst, 3);
/// ```
#[macro_export]
macro_rules! create_sessionmpst {
    ($struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            #[must_use]
            #[derive(Debug)]
            pub struct $struct_name<
                #(
                    S#N:0,
                )0:0
                R,
                N
            >
            where
                #(
                    S#N:0: mpstthree::binary::struct_trait::Session,
                )0:0
                R: mpstthree::role::Role,
                N: mpstthree::role::Role
            {
                #(
                    pub session#N:0: S#N:0,
                )0:0
                pub stack: R,
                pub name: N,
            }

            #[doc(hidden)]
            impl<#(S#N:0: mpstthree::binary::struct_trait::Session,)0:0 R: mpstthree::role::Role, N: mpstthree::role::Role> mpstthree::binary::struct_trait::Session for $struct_name<#(S#N:0, )0:0 R, N> {
                type Dual =
                $struct_name<#(<S#N:0 as mpstthree::binary::struct_trait::Session>::Dual, )0:0 <R as mpstthree::role::Role>::Dual, <N as mpstthree::role::Role>::Dual, >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    #(
                        let (sender#N:0, receiver#N:0) = S#N:0::new();
                    )0:0

                    let (role_one, role_two) = R::new();
                    let (name_one, name_two) = N::new();

                    (
                        $struct_name {
                            #(
                                session#N:0: sender#N:0,
                            )0:0
                            stack: role_one,
                            name: name_one,
                        },
                        $struct_name {
                            #(
                                session#N:0: receiver#N:0,
                            )0:0
                            stack: role_two,
                            name: name_two,
                        }
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    let mut result = String::from("");
                    #(
                        result = format!("{} + {}", result, S#N:0::head_str());
                    )0:0
                    format!(
                        "{} + {} + {}",
                        result,
                        R::head_str(),
                        N::head_str()
                    )
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    let mut result = String::from("");
                    #(
                        result = format!("{} + {}", result, S#N:0::head_str());
                    )0:0
                    format!(
                        " {} + {} + {}",
                        result,
                        R::tail_str(),
                        N::head_str()
                    )
                }
            }
        });
    }
}
