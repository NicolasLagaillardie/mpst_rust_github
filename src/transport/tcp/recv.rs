//! This module contains the functions for
//! receiving a payload
//! for a TCP connection.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature.*

use crate::binary::struct_trait::{recv::Recv, session::Session};
use std::boxed::Box;
use std::error::Error;
use std::io::Read;
use std::marker;
use std::net::TcpStream;

type TcpData = [u8; 128];

type TupleRecv<T, S> = (T, S, TcpData, usize, TcpStream);

/// Receive a value of type `T`. Can fail. Returns either a
/// pair of the received value and the continuation of the
/// session `S` or an error.
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature.*
#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub fn recv_tcp<T, S>(
    s: Recv<(T, TcpData), S>,
    mut stream: TcpStream,
    tcp: bool,
) -> Result<TupleRecv<T, S>, Box<dyn Error>>
where
    T: marker::Send,
    S: Session, {
    let (v, s) = s.channel.recv()?;
    let mut data = [0_u8; 128];
    let r = match tcp {
        true => {
            // stream.shutdown(Shutdown::Write)?; // Force stream to be read only. Needed?
            stream.read(&mut data)?
        }
        false => 0_usize,
    };
    Ok((v.0, s, data, r, stream))
}
