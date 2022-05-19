#![cfg(feature = "functionmpst")]

//! This module contains all the functions that are used
//! for consuming [`MeshedChannels`].
//!
//! They are required to check the protocols, as they check
//! the concordance between the different fields of
//! [`MeshedChannels`].
//!
//! [`MeshedChannels`]: crate::meshedchannels::MeshedChannels

pub mod choose;
pub mod close;
pub mod fork;
pub mod offer;
pub mod recursion;
pub mod recv;
pub mod send;

use crate::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use crate::meshedchannels::MeshedChannels;
use crate::role::Role;
use either::Either;

/// Offer a choice between two sessions `S1` and `S1`. Those
/// sessions should be [`MeshedChannels`],
/// as requested by functions such as
/// [`offer_mpst_session_to_b_from_a`].
/// Implemented using [`Recv`] and
/// [`Either`].
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [`offer_mpst_session_to_b_from_a`]: crate::functionmpst::offer::offer_mpst_session_to_b_from_a
/// [`Recv`]: crate::binary::struct_trait::recv::Recv
/// [`Either`]: either::Either
pub type OfferMpst<S0, S1, S2, S3, R0, R1, N0> =
    Recv<Either<MeshedChannels<S0, S1, R0, N0>, MeshedChannels<S2, S3, R1, N0>>, End>;

/// Choose between two sessions `S1` and `S2`. Those
/// sessions should be [`MeshedChannels`], as requested by functions
/// such as [`choose_right_mpst_session_c_to_all`].
/// Implemented using [`Send`] and [`Either`].
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [`choose_right_mpst_session_c_to_all`]: crate::functionmpst\
/// ::choose::choose_right_mpst_session_c_to_all
/// [`Send`]: crate::binary::struct_trait::send::Send
/// [`Either`]: either::Either
pub type ChooseMpst<S0, S1, S2, S3, R0, R1, N0> = Send<
    Either<
        MeshedChannels<<S0 as Session>::Dual, <S1 as Session>::Dual, <R0 as Role>::Dual, N0>,
        MeshedChannels<<S2 as Session>::Dual, <S3 as Session>::Dual, <R1 as Role>::Dual, N0>,
    >,
    End,
>;
