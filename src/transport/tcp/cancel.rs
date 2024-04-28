//! This module contains the functions
//! for cancelling binary sessions and
//! shutdown the related TcpStream.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_tcp"` feature.*

use std::mem;
use std::net::{Shutdown, TcpStream};

/// Cancels a session. Always succeeds. If the partner calls
/// `send`, `recv` or `close` after cancellation,
/// those calls fail. Used for TCP transport.
///
/// Drops the session *s* and shutdowns the `TcpStream` *stream*
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::end::End;
/// use mpstthree::binary::struct_trait::session::Session;
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::name::a::NameA;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::transport::tcp::cancel::cancel_tcp;
/// use std::net::{TcpListener, TcpStream};
///
/// let _listener = TcpListener::bind("0.0.0.0:3333").unwrap();
/// let (s, _s_dual) = MeshedChannels::<End, End, RoleEnd, NameA>::new();
/// let stream = TcpStream::connect("localhost:3333").unwrap();
/// cancel_tcp(s, stream);
/// ```
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_tcp"` feature.*
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_tcp")))
)]
pub fn cancel_tcp<T>(s: T, stream: TcpStream) {
    mem::drop(s);
    stream.shutdown(Shutdown::Both).unwrap();
    mem::drop(stream);
}
