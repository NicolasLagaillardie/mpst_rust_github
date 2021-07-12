use std::mem;
use std::net::{Shutdown, TcpStream};

/// Cancels a session. Always succeeds. If the partner calls `recv` or `close` after cancellation,
/// those calls fail.
///
/// # Example
///
/// ```
/// use mpstthree::binary::cancel::cancel;
/// use mpstthree::binary::struct_trait::End;
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::binary::struct_trait::Session;
///
/// let (s, s_dual) = MeshedChannels::<End, End, RoleEnd, RoleA<RoleEnd>>::new();
/// cancel(s);
/// ```
pub fn cancel<T>(s: T) {
    mem::drop(s);
}

/// Cancels a session. Always succeeds. If the partner calls `recv` or `close` after cancellation,
/// those calls fail. Used for tcp transport.
///
/// Drops the session *s* and shutdowns the `TcpStream` *stream*
///
/// # Example
///
/// ```
/// use mpstthree::binary::cancel::cancel_tcp;
/// use mpstthree::binary::struct_trait::End;
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::end::RoleEnd;
/// use mpstthree::binary::struct_trait::Session;
/// use std::net::{TcpListener, TcpStream};
///
/// let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
/// let (s, s_dual) = MeshedChannels::<End, End, RoleEnd, RoleA<RoleEnd>>::new();
/// let stream = TcpStream::connect("localhost:3333").unwrap();
/// cancel_tcp(s, stream);
/// ```
pub fn cancel_tcp<T>(s: T, stream: TcpStream) {
    mem::drop(s);
    stream.shutdown(Shutdown::Both).unwrap();
    mem::drop(stream);
}
