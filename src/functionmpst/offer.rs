//! This module contains all the *offer* functions

use crate::binary::cancel::cancel;
use crate::binary::struct_trait::Session;
use crate::functionmpst::recv::{
    recv_mpst_a_all_to_b, recv_mpst_a_all_to_c, recv_mpst_b_all_to_a, recv_mpst_b_all_to_c,
    recv_mpst_c_all_to_a, recv_mpst_c_all_to_b,
};
use crate::functionmpst::OfferMpst;
use crate::role::a::RoleA;
use crate::role::all_to_a::RoleAlltoA;
use crate::role::all_to_b::RoleAlltoB;
use crate::role::all_to_c::RoleAlltoC;
use crate::role::b::RoleB;
use crate::role::c::RoleC;
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use std::error::Error;

type SessionMpstToAFromB<S1, S2, S3, S4, S5, R1, R2> = SessionMpst<
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleA<RoleEnd>>,
    S5,
    RoleAlltoB<RoleEnd, RoleEnd>,
    RoleA<RoleEnd>,
>;
type SessionMpstToAFromC<S1, S2, S3, S4, S5, R1, R2> = SessionMpst<
    S5,
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleA<RoleEnd>>,
    RoleAlltoC<RoleEnd, RoleEnd>,
    RoleA<RoleEnd>,
>;
type SessionMpstToBFromA<S1, S2, S3, S4, S5, R1, R2> = SessionMpst<
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleB<RoleEnd>>,
    S5,
    RoleAlltoA<RoleEnd, RoleEnd>,
    RoleB<RoleEnd>,
>;
type SessionMpstToBFromC<S1, S2, S3, S4, S5, R1, R2> = SessionMpst<
    S5,
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleB<RoleEnd>>,
    RoleAlltoC<RoleEnd, RoleEnd>,
    RoleB<RoleEnd>,
>;
type SessionMpstToCFromA<S1, S2, S3, S4, S5, R1, R2> = SessionMpst<
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleC<RoleEnd>>,
    S5,
    RoleAlltoA<RoleEnd, RoleEnd>,
    RoleC<RoleEnd>,
>;
type SessionMpstToCFromB<S1, S2, S3, S4, S5, R1, R2> = SessionMpst<
    S5,
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleC<RoleEnd>>,
    RoleAlltoB<RoleEnd, RoleEnd>,
    RoleC<RoleEnd>,
>;

/// Offer a choice to A from B (on its session field related
/// to B) between two
/// [`mpstthree::sessionmpst::SessionMpst`](../sessionmpst/
/// struct.SessionMpst. html), `SessionMpst<S1, S2, R1, N1>`
/// and `SessionMpst<S3, S4, R2, N2>`.
///
/// # Example
///
/// // Please check the tests  `usecase`
///
/// ```ignore
/// offer_mpst_session_to_a_from_b(
///    s,
///    |s: EndpointARecv<i32>| {
///        let (request, s) = recv_mpst_a_from_b(s)?;
///        close_mpst(s)
///    },
///    |s: EndpointAEnd| {
///        close_mpst(s)
///    },
/// )
/// ```
pub fn offer_mpst_session_to_a_from_b<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: SessionMpstToAFromB<S1, S2, S3, S4, S5, R1, R2>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    F: FnOnce(SessionMpst<S1, S2, R1, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_a_all_to_b(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to B from C (on its session field related
/// to A) between two
/// [`mpstthree::sessionmpst::SessionMpst`](../sessionmpst/
/// struct.SessionMpst. html), `SessionMpst<S1, S2, R1, N1>`
/// and `SessionMpst<S3, S4, R2, N2>`.
///
/// # Example
///
/// // Please check the tests  `usecase`
///
/// ```ignore
/// offer_mpst_session_to_a_from_c(
///    s,
///    |s: EndpointARecv<i32>| {
///        let (request, s) = recv_mpst_a_from_c(s)?;
///        close_mpst(s)
///    },
///    |s: EndpointAEnd| {
///        close_mpst(s)
///    },
/// )
/// ```
pub fn offer_mpst_session_to_a_from_c<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: SessionMpstToAFromC<S1, S2, S3, S4, S5, R1, R2>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    F: FnOnce(SessionMpst<S1, S2, R1, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_a_all_to_c(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to A from B (on its session field related
/// to B) between two
/// [`mpstthree::sessionmpst::SessionMpst`](../sessionmpst/
/// struct.SessionMpst. html), `SessionMpst<S1, S2, R1, N1>`
/// and `SessionMpst<S3, S4, R2, N2>`.
///
/// # Example
///
/// // Please check the tests  `usecase`
///
/// ```ignore
/// offer_mpst_session_to_b_from_a(
///    s,
///    |s: EndpointBRecv<i32>| {
///        let (request, s) = recv_mpst_b_from_a(s)?;
///        close_mpst(s)
///    },
///    |s: EndpointBEnd| {
///        close_mpst(s)
///    },
/// )
/// ```
pub fn offer_mpst_session_to_b_from_a<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: SessionMpstToBFromA<S1, S2, S3, S4, S5, R1, R2>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    F: FnOnce(SessionMpst<S1, S2, R1, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_b_all_to_a(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to A from B (on its session field related
/// to B) between two
/// [`mpstthree::sessionmpst::SessionMpst`](../sessionmpst/
/// struct.SessionMpst. html), `SessionMpst<S1, S2, R1, N1>`
/// and `SessionMpst<S3, S4, R2, N2>`.
///
/// # Example
///
/// // Please check the tests  `usecase`
///
/// ```ignore
/// offer_mpst_session_to_b_from_c(
///    s,
///    |s: EndpointBRecv<i32>| {
///        let (request, s) = recv_mpst_b_from_c(s)?;
///        close_mpst(s)
///    },
///    |s: EndpointBEnd| {
///        close_mpst(s)
///    },
/// )
/// ```
pub fn offer_mpst_session_to_b_from_c<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: SessionMpstToBFromC<S1, S2, S3, S4, S5, R1, R2>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    F: FnOnce(SessionMpst<S1, S2, R1, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_b_all_to_c(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to A from C (on its session field related
/// to C) between two
/// [`mpstthree::sessionmpst::SessionMpst`](../sessionmpst/
/// struct.SessionMpst. html), `SessionMpst<S1, S2, R1, N1>`
/// and `SessionMpst<S3, S4, R2, N2>`.
///
/// # Example
///
/// // Please check the tests  `usecase`
///
/// ```ignore
/// offer_mpst_session_to_c_from_a(
///    s,
///    |s: EndpointCRecv<i32>| {
///        let (request, s) = recv_mpst_c_from_a(s)?;
///        close_mpst(s)
///    },
///    |s: EndpointCEnd| {
///        close_mpst(s)
///    },
/// )
/// ```
pub fn offer_mpst_session_to_c_from_a<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: SessionMpstToCFromA<S1, S2, S3, S4, S5, R1, R2>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    F: FnOnce(SessionMpst<S1, S2, R1, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_c_all_to_a(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to A from C (on its session field related
/// to C) between two
/// [`mpstthree::sessionmpst::SessionMpst`](../sessionmpst/
/// struct.SessionMpst. html), `SessionMpst<S1, S2, R1, N1>`
/// and `SessionMpst<S3, S4, R2, N2>`.
///
/// # Example
///
/// // Please check the tests  `usecase`
///
/// ```ignore
/// offer_mpst_session_to_c_from_b(
///    s,
///    |s: EndpointCRecv<i32>| {
///        let (request, s) = recv_mpst_c_from_b(s)?;
///        close_mpst(s)
///    },
///    |s: EndpointCEnd| {
///        close_mpst(s)
///    },
/// )
/// ```
pub fn offer_mpst_session_to_c_from_b<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: SessionMpstToCFromB<S1, S2, S3, S4, S5, R1, R2>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    F: FnOnce(SessionMpst<S1, S2, R1, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_c_all_to_b(s)?;
    cancel(s);
    e.either(f, g)
}
