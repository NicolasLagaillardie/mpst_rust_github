//! This module contains the functions for
//! sending a payload
//! for a TCP connection.

use crate::binary::struct_trait::{send::Send, session::Session};
use std::boxed::Box;
use std::error::Error;
use std::io::Write;
use std::marker;
use std::net::TcpStream;
use std::panic;

type TcpData = [u8; 128];

/// Send a value of type `T` over tcp. Returns the
/// continuation of the session `S`. May fail.
pub fn send_tcp<T, S>(
    x: T, // Need to force x and data to be of the same type but for choice/offer
    data: &TcpData,
    s: Send<(T, TcpData), S>,
    mut stream: TcpStream,
    tcp: bool,
) -> Result<(S, TcpStream), Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    let (here, there) = S::new();
    match s.channel.send(((x, *data), there)) {
        Ok(_) => {
            match tcp {
                true => {
                    // stream.shutdown(Shutdown::Read)?; // Force stream to be write only. Needed?
                    stream.write_all(data)?;
                    Ok((here, stream))
                }
                false => Ok((here, stream)),
            }
        }
        Err(e) => panic!("{}", e.to_string()),
    }
}
