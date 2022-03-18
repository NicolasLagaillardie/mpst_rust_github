//! This module contains the functions for receiving
//! a payload for binary sessions.

use crate::binary::struct_trait::{recv::Recv, session::Session};

use std::boxed::Box;
use std::error::Error;
use std::marker;

/// Receive a value of type `T`. Can fail. Returns either a
/// pair of the received value and the continuation of the
/// session `S` or an error.
pub fn recv<T, S>(s: Recv<T, S>) -> Result<(T, S), Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    let (v, s) = s.channel.recv()?;
    Ok((v, s))
}
