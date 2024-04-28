//! This module contains the functions for
//! receiving a payload
//! for a UDP connection.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_udp"` feature.*

use crate::binary::struct_trait::{recv::Recv, session::Session};
use std::error::Error;
use std::marker;
use std::net::UdpSocket;

type UdpData = [u8; 128];

type TupleRecv<T, S> = (T, S, UdpData, usize, UdpSocket);

/// Receive a value of type `T`. Can fail. Returns either a
/// pair of the received value and the continuation of the
/// session `S` or an error.
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_udp"` feature.*
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_udp")))
)]
pub fn recv_udp<T, S>(
    s: Recv<(T, UdpData), S>,
    socket: UdpSocket,
    udp: bool,
) -> Result<TupleRecv<T, S>, Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    let (v, s) = s.channel.recv()?;
    let mut data = [0_u8; 128];
    let r = match udp {
        true => socket.recv(&mut data)?,
        false => 0_usize,
    };
    Ok((v.0, s, data, r, socket))
}
