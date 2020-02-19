pub mod choose;
pub mod close;
pub mod offer;
pub mod recv;
pub mod send;

use binary::{End, Recv, Send, Session};
use either::Either;

pub type OfferMpst<S1, S2> = Recv<Either<S1, S2>, End>;

pub type ChooseMpst<S1, S2> = Send<Either<<S1 as Session>::Dual, <S2 as Session>::Dual>, End>;
//pub type ChooseMpst<S1, S2> = Send<Either<S1, S2>, End>;
