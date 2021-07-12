//! This module contains all the functions that are used
//! for consuming [`mpstthree::meshedchannels::MeshedChannels`].
//!
//! They are required to check the protocols, as they check
//! the concordance between the different fields of
//! [`mpstthree::meshedchannels::Sessionmpst`].
//!
//! [`mpstthree::meshedchannels::Sessionmpst`]: ../meshedchannels/struct.MeshedChannels.html

pub mod choose;
pub mod close;
pub mod offer;
pub mod recursion;
pub mod recv;
pub mod send;

use crate::binary::struct_trait::{End, Recv, Send, Session};
use crate::meshedchannels::MeshedChannels;
use crate::role::Role;
use either::Either;

/// Offer a choice between two sessions `S1` and `S1`. Those
/// sessions should be [`mpstthree::meshedchannels::MeshedChannels`],
/// as requested by functions such as
/// [`mpstthree::functionmpst::offer::offer_mpst_session_to_b_from_a`].
/// Implemented using [`mpstthree::binary::struct_trait::Recv`] and
/// [`either::Either`].
///
/// [`mpstthree::meshedchannels::Sessionmpst`]: ../meshedchannels/struct.MeshedChannels.html
/// [`mpstthree::functionmpst::offer::offer_mpst_session_to_b_from_a`]: ../functionmpst/offer/fn.offer_mpst_session_to_b_from_a.html
/// [`mpstthree::binary::struct_trait::Recv`]: ../binary/struct.Recv.html
/// [`either::Either`]: ../either/enum.Either.html
pub type OfferMpst<S0, S1, S2, S3, R0, R1, N0> =
    Recv<Either<MeshedChannels<S0, S1, R0, N0>, MeshedChannels<S2, S3, R1, N0>>, End>;

/// Choose between two sessions `S1` and `S2`. Those
/// sessions should be [`mpstthree::meshedchannels::MeshedChannels`], as requested by functions
/// such as [`mpstthree::choose::choose_right_mpst_session_c_to_all`]. Implemented using
/// [`mpstthree::binary::struct_trait::Send`] and [`either::Either`].
///
/// [`mpstthree::meshedchannels::Sessionmpst`]: ../meshedchannels/struct.MeshedChannels.html
/// [`mpstthree::choose::choose_right_mpst_session_c_to_all`]: ../binary/choose/fn.choose_right_mpst_session_c_to_all.html
/// [`mpstthree::binary::struct_trait::Send`]: ../binary/struct_trait/struct.Send.html
/// [`either::Either`]: ../either/enum.Either.html
pub type ChooseMpst<S0, S1, S2, S3, R0, R1, N0> = Send<
    Either<
        MeshedChannels<
            <S0 as Session>::Dual,
            <S1 as Session>::Dual,
            <R0 as Role>::Dual,
            <N0 as Role>::Dual,
        >,
        MeshedChannels<
            <S2 as Session>::Dual,
            <S3 as Session>::Dual,
            <R1 as Role>::Dual,
            <N0 as Role>::Dual,
        >,
    >,
    End,
>;
