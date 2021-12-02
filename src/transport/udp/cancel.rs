//! This module contains the functions
//! for cancelling binary sessions and
//! shutdown the related UdpSocket.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_udp"` feature.*

use std::mem;
use std::net::UdpSocket;

/// Cancels a session. Always succeeds. If the partner calls
/// `send`, `recv` or `close` after cancellation,
/// those calls fail. Used for UDP transport.
///
/// Drops the session *s* and shutdowns the `UdpSocket` *socket*
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::end::End;
/// use mpstthree::binary::struct_trait::session::Session;
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::transport::udp::cancel::cancel_udp;
/// use std::net::UdpSocket;
///
/// let socket = UdpSocket::bind("0.0.0.0:3333").unwrap();
/// let (s, _s_dual) = MeshedChannels::<End, End, RoleEnd, RoleA<RoleEnd>>::new();
/// cancel_udp(s, socket);
/// ```
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_udp"` feature.*
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub fn cancel_udp<T>(s: T, socket: UdpSocket) {
    mem::drop(s);
    mem::drop(socket);
}
