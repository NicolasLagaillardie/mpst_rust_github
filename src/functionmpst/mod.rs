pub mod choose;
pub mod close;
pub mod offer;
pub mod recv;
pub mod send;

use binary::{End, Recv, Send, Session};
use either::Either;
use sessionmpst::SessionMpst;

/// Offer a choice between two sessions `S1` and `S1`. Those sessions should be `SessionMpst`, as requested by functions
/// such as `offer_mpst_session_b_to_a`. Implemented using `Recv` and `Either`.
pub type OfferMpst<S0, S1, S2, S3, R0, R1> =
    Recv<Either<SessionMpst<S0, S1, R0>, SessionMpst<S2, S3, R1>>, End>;

/// Choose between two sessions `S1` and `S2`. Those sessions should be `SessionMpst`, as requested by functions
/// such as `choose_right_mpst_session_c_to_all`. Implemented using `Send` and `Either`.
pub type ChooseMpst<S0, S1, S2, S3, R0, R1> = Send<
    Either<<SessionMpst<S0, S1, R0> as Session>::Dual, <SessionMpst<S2, S3, R1> as Session>::Dual>,
    End,
>;

