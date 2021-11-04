//! This module contains the functions for sending
//! a payload for binary sessions.

use crate::binary::cancel::cancel;
use crate::binary::struct_trait::{send::Send, session::Session};
use std::boxed::Box;
use std::error::Error;
use std::marker;
use std::panic;

/// Send a value of type `T`. Always succeeds. Returns the
/// continuation of the session `S`.
pub fn send<T, S>(x: T, s: Send<T, S>) -> S
where
    T: marker::Send,
    S: Session,
{
    let (here, there) = S::new();
    s.channel.send((x, there)).unwrap_or(());
    here
}

/// Send a value of type `T`. Always succeeds. Returns the
/// continuation of the session `S`.
pub fn send_canceled<T, S>(x: T, s: Send<T, S>) -> Result<S, Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    let (here, there) = S::new();
    let _ = s.channel.send((x, there))?;
    Ok(here)
}
