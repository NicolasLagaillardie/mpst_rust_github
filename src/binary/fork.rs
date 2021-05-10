use crate::binary::struct_trait::Session;
use std::boxed::Box;
use std::error::Error;
use std::marker;
use std::net::TcpStream;
use std::panic;
use std::thread::{spawn, JoinHandle};

type TcpFork<T> = Result<(JoinHandle<()>, T, TcpStream), Box<dyn Error>>;

#[doc(hidden)]
pub fn fork_with_thread_id<S, P>(p: P) -> (JoinHandle<()>, S::Dual)
where
    S: Session + 'static,
    P: FnOnce(S) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    let (there, here) = Session::new();
    let other_thread = spawn(move || {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        match p(there) {
            Ok(()) => (),
            Err(e) => panic!("{}", e.to_string()),
        }
    });
    (other_thread, here)
}

/// Creates a child process, and a session with two dual
/// endpoints of type `S` and `S::Dual`. The first endpoint
/// is given to the child process. Returns the
/// second endpoint.
pub fn fork<S, P>(p: P) -> S::Dual
where
    S: Session + 'static,
    P: FnOnce(S) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    fork_with_thread_id(p).1
}

/// Creates a child process, and a session with two dual
/// endpoints of type `S` and `S::Dual`. The first endpoint
/// is given to the child process. Returns the
/// second endpoint.
pub fn fork_tcp<S, P>(p: P, address: &str) -> TcpFork<S::Dual>
where
    S: Session + 'static,
    P: FnOnce(S, TcpStream) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    let stream = TcpStream::connect(address)?;
    let copy_stream = stream.try_clone()?;
    let (there, here) = Session::new();
    let other_thread = spawn(move || {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        match p(there, copy_stream) {
            Ok(()) => (),
            Err(e) => panic!("{}", e.to_string()),
        }
    });
    Ok((other_thread, here, stream))
}
