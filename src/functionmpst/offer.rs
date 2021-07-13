//! This module contains all the *offer* functions

use crate::binary::cancel::cancel;
use crate::binary::struct_trait::Session;
use crate::functionmpst::recv::{
    recv_mpst_a_all_from_b, recv_mpst_a_all_from_c, recv_mpst_b_all_from_a, recv_mpst_b_all_from_c,
    recv_mpst_c_all_from_a, recv_mpst_c_all_from_b,
};
use crate::functionmpst::OfferMpst;
use crate::meshedchannels::MeshedChannels;
use crate::role::a::RoleA;
use crate::role::all_to_a::RoleAlltoA;
use crate::role::all_to_b::RoleAlltoB;
use crate::role::all_to_c::RoleAlltoC;
use crate::role::b::RoleB;
use crate::role::c::RoleC;
use crate::role::end::RoleEnd;
use crate::role::Role;
use std::error::Error;

type MeshedChannelsToAFromB<S1, S2, S3, S4, S5, R1, R2> = MeshedChannels<
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleA<RoleEnd>>,
    S5,
    RoleAlltoB<RoleEnd, RoleEnd>,
    RoleA<RoleEnd>,
>;
type MeshedChannelsToAFromC<S1, S2, S3, S4, S5, R1, R2> = MeshedChannels<
    S5,
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleA<RoleEnd>>,
    RoleAlltoC<RoleEnd, RoleEnd>,
    RoleA<RoleEnd>,
>;
type MeshedChannelsToBFromA<S1, S2, S3, S4, S5, R1, R2> = MeshedChannels<
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleB<RoleEnd>>,
    S5,
    RoleAlltoA<RoleEnd, RoleEnd>,
    RoleB<RoleEnd>,
>;
type MeshedChannelsToBFromC<S1, S2, S3, S4, S5, R1, R2> = MeshedChannels<
    S5,
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleB<RoleEnd>>,
    RoleAlltoC<RoleEnd, RoleEnd>,
    RoleB<RoleEnd>,
>;
type MeshedChannelsToCFromA<S1, S2, S3, S4, S5, R1, R2> = MeshedChannels<
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleC<RoleEnd>>,
    S5,
    RoleAlltoA<RoleEnd, RoleEnd>,
    RoleC<RoleEnd>,
>;
type MeshedChannelsToCFromB<S1, S2, S3, S4, S5, R1, R2> = MeshedChannels<
    S5,
    OfferMpst<S1, S2, S3, S4, R1, R2, RoleC<RoleEnd>>,
    RoleAlltoB<RoleEnd, RoleEnd>,
    RoleC<RoleEnd>,
>;

/// Offer a choice to A from B (on its session field related
/// to B) between two [`MeshedChannels`],
/// `MeshedChannels<S1, S2, R1, N1>` and `MeshedChannels<S3, S4, R2, N2>`.
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
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
pub fn offer_mpst_session_to_a_from_b<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: MeshedChannelsToAFromB<S1, S2, S3, S4, S5, R1, R2>,
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
    F: FnOnce(MeshedChannels<S1, S2, R1, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(MeshedChannels<S3, S4, R2, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_a_all_from_b(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to B from C (on its session field related to A) between two
/// [`MeshedChannels`],
/// `MeshedChannels<S1, S2, R1, N1>` and `MeshedChannels<S3, S4, R2, N2>`.
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
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
pub fn offer_mpst_session_to_a_from_c<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: MeshedChannelsToAFromC<S1, S2, S3, S4, S5, R1, R2>,
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
    F: FnOnce(MeshedChannels<S1, S2, R1, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(MeshedChannels<S3, S4, R2, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_a_all_from_c(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to A from B (on its session field related to B) between two
/// [`MeshedChannels`],
/// `MeshedChannels<S1, S2, R1, N1>` and `MeshedChannels<S3, S4, R2, N2>`.
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
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
pub fn offer_mpst_session_to_b_from_a<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: MeshedChannelsToBFromA<S1, S2, S3, S4, S5, R1, R2>,
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
    F: FnOnce(MeshedChannels<S1, S2, R1, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(MeshedChannels<S3, S4, R2, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_b_all_from_a(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to A from B (on its session field related to B) between two
/// [`MeshedChannels`],
/// `MeshedChannels<S1, S2, R1, N1>` and `MeshedChannels<S3, S4, R2, N2>`.
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
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
pub fn offer_mpst_session_to_b_from_c<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: MeshedChannelsToBFromC<S1, S2, S3, S4, S5, R1, R2>,
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
    F: FnOnce(MeshedChannels<S1, S2, R1, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(MeshedChannels<S3, S4, R2, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_b_all_from_c(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to A from C (on its session field related to C) between two
/// [`MeshedChannels`],
/// `MeshedChannels<S1, S2, R1, N1>` and `MeshedChannels<S3, S4, R2, N2>`.
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
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
pub fn offer_mpst_session_to_c_from_a<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: MeshedChannelsToCFromA<S1, S2, S3, S4, S5, R1, R2>,
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
    F: FnOnce(MeshedChannels<S1, S2, R1, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(MeshedChannels<S3, S4, R2, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_c_all_from_a(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice to A from C (on its session field related to C) between two
/// [`MeshedChannels`],
/// `MeshedChannels<S1, S2, R1, N1>` and `MeshedChannels<S3, S4, R2, N2>`.
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
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
pub fn offer_mpst_session_to_c_from_b<'a, S1, S2, S3, S4, S5, F, G, R1, R2, U>(
    s: MeshedChannelsToCFromB<S1, S2, S3, S4, S5, R1, R2>,
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
    F: FnOnce(MeshedChannels<S1, S2, R1, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(MeshedChannels<S3, S4, R2, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_c_all_from_b(s)?;
    cancel(s);
    e.either(f, g)
}
