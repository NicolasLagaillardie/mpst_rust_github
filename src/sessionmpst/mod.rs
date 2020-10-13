use crate::binary::Session;
use crate::role::Role;

/// A `struct` which encapsulates two binary session types and a stack.
///
/// This `struct` is the main one used in this library.
/// Each process is linked to the others with one `Session`,
/// and the order of the operations is given by the stack composed of `Role`.
#[must_use]
#[derive(Debug)]
pub struct SessionMpst<S1: Session, S2: Session, R: Role, N: Role> {
    pub session1: S1,
    pub session2: S2,
    pub stack: R,
    pub name: N,
}

#[doc(hidden)]
impl<S1: Session, S2: Session, R: Role, N: Role> Session for SessionMpst<S1, S2, R, N> {
    type Dual = SessionMpst<
        <S1 as Session>::Dual,
        <S2 as Session>::Dual,
        <R as Role>::Dual,
        <N as Role>::Dual,
    >;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = S1::new();
        let (sender_two, receiver_two) = S2::new();

        let (role_one, role_two) = R::new();
        let (name_one, name_two) = N::new();

        (
            SessionMpst {
                session1: sender_one,
                session2: sender_two,
                stack: role_one,
                name: name_one,
            },
            SessionMpst {
                session1: receiver_one,
                session2: receiver_two,
                stack: role_two,
                name: name_two,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        format!(
            "{} + {} + {} + {}",
            S1::head_str(),
            S2::head_str(),
            R::head_str(),
            N::head_str()
        )
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!(
            "{} + {} + {} + {}",
            S1::tail_str(),
            S2::tail_str(),
            R::tail_str(),
            N::tail_str()
        )
    }
}

#[macro_export]
macro_rules! create_sessionmpst {
    ($struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            #[must_use]
            #[derive(Debug)]
            pub struct $struct_name<
                #(
                    S#N: mpstthree::binary::Session,
                )* R: mpstthree::role::Role,
                N: mpstthree::role::Role
            > {
                #(
                    pub session#N: S#N,
                )*
                pub stack: R,
                pub name: N,
            }

            #[doc(hidden)]
            impl<#(S#N: mpstthree::binary::Session,)* R: mpstthree::role::Role, N: mpstthree::role::Role> mpstthree::binary::Session for $struct_name<#(S#N, )* R, N> {
                type Dual =
                $struct_name<#(<S#N as mpstthree::binary::Session>::Dual, )* <R as mpstthree::role::Role>::Dual, <N as mpstthree::role::Role>::Dual, >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    #(
                        let (sender#N, receiver#N) = S#N::new();
                    )*

                    let (role_one, role_two) = R::new();
                    let (name_one, name_two) = N::new();

                    (
                        $struct_name {
                            #(
                                session#N: sender#N,
                            )*
                            stack: role_one,
                            name: name_one,
                        },
                        $struct_name {
                            #(
                                session#N: receiver#N,
                            )*
                            stack: role_two,
                            name: name_two,
                        }
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    format!(
                        "{} + {} + {} + {}",
                        S1::head_str(),
                        S2::head_str(),
                        R::head_str(),
                        N::head_str()
                    )
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{} + {} + {} + {}",
                        S1::tail_str(),
                        S2::tail_str(),
                        R::tail_str(),
                        N::head_str()
                    )
                }
            }
        });
    }
}
