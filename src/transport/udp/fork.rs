//! This module contains the functions for
//! forking binary sessions and
//! the related UdpSocket.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_udp"` feature.*

use crate::binary::struct_trait::session::Session;
use std::boxed::Box;
use std::error::Error;
use std::marker;
use std::net::UdpSocket;
use std::panic;
use std::thread::{Builder, JoinHandle};

type UdpFork<T> = Result<(JoinHandle<()>, T, UdpSocket), Box<dyn Error>>;

/// Creates a child process, and a session with two dual
/// endpoints of type `S` and `S::Dual`. The first endpoint
/// is given to the child process. Returns the
/// second endpoint and the created *bound* and
/// *connected* socket.
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_udp"` feature.*
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub fn fork_udp<S, P>(p: P, bind: &str, connect: &str) -> UdpFork<S::Dual>
where
    S: Session + 'static,
    P: FnOnce(S, UdpSocket) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    let socket = UdpSocket::bind(bind)?;
    socket.connect(connect)?;
    let copy_socket = socket.try_clone()?;
    let (there, here) = Session::new();
    let other_thread = Builder::new()
        .name(String::from(bind))
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            panic::set_hook(Box::new(|_info| {
                // do nothing
            }));
            match p(there, copy_socket) {
                Ok(()) => (),
                Err(e) => panic!("{}", e.to_string()),
            }
        })
        .unwrap();
    Ok((other_thread, here, socket))
}
