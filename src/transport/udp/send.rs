//! This module contains the functions for
//! sending a payload
//! for a UDP connection.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_udp"` feature.*

use crate::binary::struct_trait::{send::Send, session::Session};
// use std::boxed::Box;
use std::error::Error;
use std::marker;
use std::net::UdpSocket;
use std::panic;

type UdpData = [u8; 128];

/// Send a value of type `T` over UDP. Returns the
/// continuation of the session `S` and the UdpSocket. May fail.
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_udp"` feature.*
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub fn send_udp<T, S>(
    x: T, // Need to force x and data to be of the same type every time but for choice/offer
    data: &UdpData,
    s: Send<(T, UdpData), S>,
    socket: UdpSocket,
    udp: bool,
) -> Result<(S, usize, UdpSocket), Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    let (here, there) = S::new();
    match s.channel.send(((x, *data), there)) {
        Ok(()) => match udp {
            true => {
                let result = socket.send(data)?;
                Ok((here, result, socket))
            }
            false => Ok((here, 0, socket)),
        },
        Err(e) => panic!("{}", e.to_string()),
    }
}
