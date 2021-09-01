use crate::binary::struct_trait::{end::End, end::Signal};
use std::boxed::Box;
use std::error::Error;
use std::mem;
use std::net::{Shutdown, TcpStream};

/// Closes a session. Synchronises with the partner, and
/// fails if the partner has crashed.
pub fn close(s: End) -> Result<(), Box<dyn Error>> {
    s.sender.send(Signal::Stop).unwrap_or(());
    s.receiver.recv()?;
    Ok(())
}

/// Closes a Tcp session. Synchronises with the partner, and
/// fails if the partner has crashed.
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
