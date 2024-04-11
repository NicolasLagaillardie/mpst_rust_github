//! This module contains the functions for closing
//! binary sessions and
//! shutdown the related TcpStream.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_tcp"` feature.*

use crate::binary::struct_trait::{end::End, end::Signal};

use std::error::Error;
use std::mem;
use std::net::{Shutdown, TcpStream};

/// Closes a Tcp session. Synchronises with the partner, and
/// fails if the partner has crashed.
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_tcp"` feature.*
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_tcp")))
)]
pub fn close_tcp(s: End, stream: TcpStream, tcp: bool) -> Result<(), Box<dyn Error>> {
    s.sender.send(Signal::Stop)?;
    s.receiver.recv()?;
    match tcp {
        true => {
            stream.shutdown(Shutdown::Both).unwrap_or(()); // Stop any operation on stream. Cannot fail as stream may already been stopped.
            mem::drop(stream); // close stream
            Ok(())
        }
        false => Ok(()),
    }
}
