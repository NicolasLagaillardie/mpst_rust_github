//! This module contains the functions for
//! selecting the first active session.

use crate::binary::struct_trait::{recv::Recv, session::Session};
use crossbeam_channel::Select;
use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::marker;

/// Error returned when `select` or `select_mut` are called
/// with an empty vector.
#[derive(Debug)]
enum SelectError {
    EmptyVec,
}

impl fmt::Display for SelectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SelectError::EmptyVec => {
                write!(f, "please use a vector with at least one element")
            }
        }
    }
}

impl Error for SelectError {
    fn description(&self) -> &str {
        match *self {
            SelectError::EmptyVec => "empty vectors not allowed",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SelectError::EmptyVec => None,
        }
    }
}

/// Selects the first active session. Receives from the
/// selected session, and removes the endpoint from the
/// input vector. Returns the received value and
/// the continuation of the selected session.
pub fn select_mut<T, S>(rs: &mut Vec<Recv<T, S>>) -> Result<(T, S), Box<dyn Error>>
where
    T: marker::Send,
    S: Session, {
    if rs.is_empty() {
        Err(Box::new(SelectError::EmptyVec))
    } else {
        let (index, res) = {
            let mut sel = Select::new();
            let iter = rs.iter();
            for r in iter {
                sel.recv(&r.channel);
            }
            loop {
                let index = sel.ready();
                let res = rs[index].channel.try_recv();

                if let Err(e) = res {
                    if e.is_empty() {
                        continue;
                    }
                }

                break (index, res);
            }
        };

        let _ = rs.swap_remove(index);
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(Box::new(e)),
        }
    }
}

type SelectType<T, S> = Result<(T, S), Box<dyn Error>>;

/// Selects the first active session. Receives from the
/// selected session. Returns the received value, the
/// continuation of the selected session, and a copy of the
/// input vector without the selected session.
pub fn select<T, S>(rs: Vec<Recv<T, S>>) -> (SelectType<T, S>, Vec<Recv<T, S>>)
where
    T: marker::Send,
    S: Session, {
    let mut rs = rs;
    let res = select_mut(&mut rs);
    (res, rs)
}
