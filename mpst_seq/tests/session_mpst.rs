// Test for parametrisation on the number of roles
extern crate crossbeam_channel;
extern crate either;
extern crate mpstthree;
use mpst_seq::seq;
use mpstthree::binary::{End, Session};
use mpstthree::create_sessionmpst;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use std::error::Error;

macro_rules! create_sessionmpst {
    ($struct_name:ident, $nsession:literal) => {
        seq!(N in 1..$nsession {
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

macro_rules! impl_sessionmpst {
    ($struct_name:ident, $nsession:literal) => {
        seq!(N in 1..$nsession {
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

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstThree, 2);

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstFour, 3);

#[test]
fn basic_macros() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (sender1, _) = End::new();
            let (sender2, _) = End::new();
            let (role_one, _) = RoleEnd::new();

            let _test = SessionMpstThree {
                session1: sender1,
                session2: sender2,
                stack: role_one,
            };

            let (_test2, _) = SessionMpstThree::<End, End, RoleEnd>::new();
        }
        Ok(())
    }()
    .is_ok());

    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (sender1, _) = End::new();
            let (sender2, _) = End::new();
            let (sender3, _) = End::new();
            let (role_one, _) = RoleEnd::new();

            let _test = SessionMpstFour {
                session1: sender1,
                session2: sender2,
                session3: sender3,
                stack: role_one,
            };

            let (_test2, _) = SessionMpstFour::<End, End, End, RoleEnd>::new();
        }
        Ok(())
    }()
    .is_ok());
}
