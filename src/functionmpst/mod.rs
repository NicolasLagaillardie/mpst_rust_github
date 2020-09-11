pub mod choose;
pub mod close;
pub mod macros_multiple;
pub mod macros_simple;
pub mod offer;
pub mod recv;
pub mod send;

use crate::binary::{End, Recv, Send, Session};
use crate::sessionmpst::SessionMpst;
use either::Either;

/// Offer a choice between two sessions `S1` and `S1`. Those sessions should be `SessionMpst`, as requested by functions
/// such as `offer_mpst_session_b_to_a`. Implemented using `Recv` and `Either`.
pub type OfferMpst<S0, S1, S2, S3, R0, R1, N0> =
    Recv<Either<SessionMpst<S0, S1, R0, N0>, SessionMpst<S2, S3, R1, N0>>, End>;

/// Choose between two sessions `S1` and `S2`. Those sessions should be `SessionMpst`, as requested by functions
/// such as `choose_right_mpst_session_c_to_all`. Implemented using `Send` and `Either`.
pub type ChooseMpst<S0, S1, S2, S3, R0, R1, N0> = Send<
    Either<
        <SessionMpst<S0, S1, R0, N0> as Session>::Dual,
        <SessionMpst<S2, S3, R1, N0> as Session>::Dual,
    >,
    End,
>;
