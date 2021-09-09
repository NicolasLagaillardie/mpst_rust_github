//! This module contains the functions for
//! forking binary sessions and
//! the related TcpStream.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_tcp"` feature.*

use crate::binary::struct_trait::session::Session;
use std::boxed::Box;
use std::error::Error;
use std::marker;
use std::net::TcpStream;
use std::panic;
use std::thread::{spawn, JoinHandle};

type TcpFork<T> = Result<(JoinHandle<()>, T, TcpStream), Box<dyn Error>>;

/// Creates a child process, and a session with two dual
/// endpoints of type `S` and `S::Dual`. The first endpoint
/// is given to the child process. Returns the
/// second endpoint.
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_tcp"` feature.*
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_tcp")))
)]
pub fn fork_tcp<S, P>(p: P, address: &str) -> TcpFork<S::Dual>
where
    S: Session + 'static,
    P: FnOnce(S, TcpStream) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    let stream = TcpStream::connect(address)?;
    let copy_stream = stream.try_clone()?;
    let (there, here) = Session::new();
    let other_thread = spawn(move || {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        match p(there, copy_stream) {
            Ok(()) => (),
            Err(e) => panic!("{}", e.to_string()),
        }
    });
    Ok((other_thread, here, stream))
}
