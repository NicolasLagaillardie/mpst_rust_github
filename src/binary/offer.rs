use crate::binary::cancel::cancel;
use crate::binary::recv::recv;
use crate::binary::struct_trait::{End, Recv, Session};
use either::Either;
use std::boxed::Box;
use std::error::Error;

/// Offer a choice between two sessions `S1` and `S1`.
/// Implemented using `Recv` and `Either`.
pub type Offer<S1, S2> = Recv<Either<S1, S2>, End>;

/// Offer a choice between two sessions `S1` and `S2`.
pub fn offer_either<'a, S1, S2, F, G, R>(
    s: Offer<S1, S2>,
    f: F,
    g: G,
) -> Result<R, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    F: FnOnce(S1) -> Result<R, Box<dyn Error + 'a>>,
    G: FnOnce(S2) -> Result<R, Box<dyn Error + 'a>>,
{
    let (e, s) = recv(s)?;
    cancel(s);
    e.either(f, g)
}

/// Offer a choice between many different sessions wrapped
/// in an `enum`
#[macro_export]
macro_rules! offer {
    ($session: expr, { $( $pat: pat => $result: expr , )* }) => {
        (move || -> Result<_, _> {
            let (l, s) = mpstthree::binary::recv::recv($session)?;
            mpstthree::binary::cancel::cancel(s);
            match l {
                $(
                    $pat => $result,
                )*
            }
        })()
    };
}

/// Offer a choice between many different sessions wrapped
/// in an `enum`
#[macro_export]
macro_rules! offer_tcp {
    ($session: expr, { $( $pat: pat => $result: expr , )* }) => {
        (move || -> Result<_, _> {
            let ((data, cont), s) = mpstthree::binary::recv::recv($session)?;
            mpstthree::binary::cancel::cancel(s);
            mpstthree::binary::cancel::cancel(data);

            match cont {
                $(
                    $pat => $result,
                )*
            }
        })()
    };
}
