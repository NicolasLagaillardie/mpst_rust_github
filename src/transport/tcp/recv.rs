//! This module contains the functions for
//! receiving a payload
//! for a TCP connection.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_tcp"` feature.*

use crate::binary::struct_trait::{recv::Recv, session::Session};
// use std::boxed::Box;
use std::error::Error;
use std::io::Read;
use std::marker;
use std::net::TcpStream;

type TcpData = [u8; 128];

type TupleRecv<T, S> = (T, S, TcpData, TcpStream);

/// Receive a value of type `T`. Can fail. Returns either a
/// pair of the received value and the continuation of the
/// session `S` or an error.
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_tcp"` feature.*
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_tcp")))
)]
pub fn recv_tcp<T, S>(
    s: Recv<(T, TcpData), S>,
    mut stream: TcpStream,
    tcp: bool,
) -> Result<TupleRecv<T, S>, Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    let (v, s) = s.channel.recv()?;
    let mut data = [0_u8; 128];
    match tcp
    {
        true =>
        {
            // stream.shutdown(Shutdown::Write)?; // Force stream to be read only. Needed?
            stream.read_exact(&mut data)?
        }
        false => (),
    };
    Ok((v.0, s, data, stream))
}
