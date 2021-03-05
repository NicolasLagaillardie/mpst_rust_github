//! The main structure used for representing a participant,
//! also named a party, within a protocol.
//!
//! It contains 4 fields:
//! - **session1**: contains the first binary session type, which links the participant to the first
//!   participant in the alphanumerical order. It contains
//!   [`mpstthree::binary::struct_trait::Session`].
//! - **session2**: contains the second binary session type, which links the participant to the
//!   second participant in the alphanumerical order. It contains
//!   [`mpstthree::binary::struct_trait::Session`].
//! - **stack**: contains the ordering of the interactions between the participant and the others.
//!   It contains [`mpstthree::role::Role`].
//! - **name**: contains the name of the participant. It should look like `RoleA<RoleEnd>` or
//!   `RoleB<RoleEnd>`.
//!
//! [`mpstthree::binary::struct_trait::Session`]:
//! ../binary/trait.Session.html [`mpstthree::role::Role`]:
//! ../role/trait.Role.html

/// The structure which encapsulates two binary session
/// types, a stack and a name.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
///
/// use mpstthree::sessionmpst::SessionMpst;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// // Creating the binary sessions
/// type AtoB<N> = Send<N, End>;
/// type AtoC<N> = Recv<N, End>;
///
/// // Queues
/// type QueueA = RoleB<RoleC<RoleEnd>>;
///
/// // Creating the MP sessions
/// type EndpointA<N> = SessionMpst<AtoB<N>, AtoC<N>, QueueA, RoleA<RoleEnd>>;
/// ```
#[must_use]
#[derive(Debug)]
pub struct SessionMpst<S1, S2, R, N>
where
    S1: crate::binary::struct_trait::Session,
    S2: crate::binary::struct_trait::Session,
    R: crate::role::Role,
    N: crate::role::Role,
{
    pub session1: S1,
    pub session2: S2,
    pub stack: R,
    pub name: N,
}

#[doc(hidden)]
impl<
        S1: crate::binary::struct_trait::Session,
        S2: crate::binary::struct_trait::Session,
        R: crate::role::Role,
        N: crate::role::Role,
    > crate::binary::struct_trait::Session for SessionMpst<S1, S2, R, N>
{
    type Dual = SessionMpst<
        <S1 as crate::binary::struct_trait::Session>::Dual,
        <S2 as crate::binary::struct_trait::Session>::Dual,
        <R as crate::role::Role>::Dual,
        <N as crate::role::Role>::Dual,
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
