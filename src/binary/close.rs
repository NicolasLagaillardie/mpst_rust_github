//! This module contains the functions for closing
//! binary sessions.

use crate::binary::struct_trait::{end::End, end::Signal};

// use std::boxed::Box;
use std::error::Error;

/// Closes a session. Synchronises with the partner, and
/// fails if the partner has crashed.
pub fn close(s: End) -> Result<(), Box<dyn Error>> {
    s.sender.send(Signal::Stop)?;
    match s.receiver.recv()? {
        Signal::Stop => {}
        err => panic!("Unexpected label, expected Signal::Stop, got {:?}", err),
    }
    drop(s);
    Ok(())
}
