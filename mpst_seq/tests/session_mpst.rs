// Test for parametrisation on the number of roles
extern crate crossbeam_channel;
extern crate either;
extern crate mpstthree;
use mpst_seq::eseq;
use mpstthree::binary::{End, Session};
use mpstthree::create_sessionmpst;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use std::error::Error;

macro_rules! create_struct_sessionmpst {
    ($struct_name:ident, $nsession:literal) => {
        eseq!(N in 1..$nsession {
            #[must_use]
            #[derive(Debug)]
            pub struct $struct_name<
                #(
                    session#N: S#N,
                )* R: Role
            > {
                #(
                    pub session#N: S#N,
                )*
                pub stack: R,
            }
        })
    }
}

macro_rules! create_impl_sessionmpst {
    ($struct_name:ident, $nsession:literal) => {
        eseq!(N in 1..$nsession {
            #[doc(hidden)]
            impl<#(session#N: Session,)* R: Role> Session for $struct_name<#(session#N, )* R> {
                type Dual =
                    $struct_name<#(<session#N as Session>::Dual, )* <R as Role>::Dual>;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {

                    let (role_one, role_two) = R::new();

                    // Issue with no link between the two new SessionMpst
                    (
                        $struct_name {
                            #(
                                session#N: {
                                    let (sender, _) = $session_type::new();
                                    sender
                                },
                            )*
                            stack: role_one,
                        },
                        $struct_name {
                            #(
                                session#N: {
                                    let (_, receiver) = $session_type::new();
                                    receiver
                                },
                            )*
                            stack: role_two,
                        }
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    format!(
                        "{} + {} + {}",
                        S1::head_str(),
                        S2::head_str(),
                        R::head_str()
                    )
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{} + {} + {}",
                        S1::tail_str(),
                        S2::tail_str(),
                        R::tail_str()
                    )
                }
            }
        })
    }
}

macro_rules! create_sessionmpst {
    ($struct_name:ident, $nsession:literal) => {
        create_struct_sessionmpst!($struct_name, $nsession);
        create_impl_sessionmpst!($struct_name, $nsession);
    };
}


