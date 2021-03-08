use std::mem;
use std::net::{Shutdown, TcpStream};

/// Cancels a session. Always succeeds. If the partner calls `recv` or `close` after cancellation, those calls fail.
pub fn cancel<T>(x: T) {
    mem::drop(x);
}

/// Cancels a session. Always succeeds. If the partner calls `recv` or `close` after cancellation, those calls fail.
pub fn cancel_tcp<T>(x: T, stream: TcpStream) {
    mem::drop(x);
    stream.shutdown(Shutdown::Both).unwrap();
    mem::drop(stream);
}
