//! This module contains all the functions that are used
//! for consuming [`mpstthree::sessionmpst::SessionMpst`].
//!
//! They are required to check the protocols, as they check
//! the concordance between the different fields of
//! [`mpstthree::sessionmpst::Sessionmpst`].
//!
//! [`mpstthree::sessionmpst::Sessionmpst`]: ../sessionmpst/struct.SessionMpst.html

pub mod choose;
pub mod close;
pub mod offer;
pub mod recursion;
pub mod recv;
pub mod send;

use crate::binary::{End, Recv, Send, Session};
use crate::sessionmpst::SessionMpst;
use either::Either;

/// Offer a choice between two sessions `S1` and `S1`. Those sessions should be [`mpstthree::sessionmpst::SessionMpst`], as requested by functions
/// such as [`mpstthree::offer::offer_mpst_session_to_b_from_a`]. Implemented using [`mpstthree::binary::Recv`] and [`either::Either`].
/// 
/// [`mpstthree::sessionmpst::Sessionmpst`]: ../sessionmpst/struct.SessionMpst.html
/// [`mpstthree::offer::offer_mpst_session_b_to_a`]: ../offer/fn.offer_mpst_session_to_b_from_a.html
/// [`mpstthree::binary::Recv`]: ../binary/struct.Recv.html
/// [`either::Either`]: ../either/enum.Either.html
pub type OfferMpst<S0, S1, S2, S3, R0, R1, N0> =
    Recv<Either<SessionMpst<S0, S1, R0, N0>, SessionMpst<S2, S3, R1, N0>>, End>;

/// Choose between two sessions `S1` and `S2`. Those sessions should be [`mpstthree::sessionmpst::SessionMpst`], as requested by functions
/// such as [`mpstthree::choose::choose_right_mpst_session_c_to_all`]. Implemented using [`mpstthree::binary::Send`] and [`either::Either`].
/// 
/// [`mpstthree::sessionmpst::Sessionmpst`]: ../sessionmpst/struct.SessionMpst.html
/// [`mpstthree::choose::choose_right_mpst_session_c_to_all`]: ../choose/fn.choose_right_mpst_session_c_to_all.html
/// [`mpstthree::binary::Send`]: ../binary/struct.Send.html
/// [`either::Either`]: ../either/enum.Either.html
pub type ChooseMpst<S0, S1, S2, S3, R0, R1, N0> = Send<
    Either<
        <SessionMpst<S0, S1, R0, N0> as Session>::Dual,
        <SessionMpst<S2, S3, R1, N0> as Session>::Dual,
    >,
    End,
>;
