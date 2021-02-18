//! The slightly modified binary session type library.
//!
//! [![github]](https://github.com/wenkokke/sesh)&ensp;[![crates-io]](https://crates.io/crates/sesh)&ensp;[![docs-rs]](https://docs.rs/sesh)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
use crossbeam_channel::{bounded, Receiver, Select, Sender};
use either::Either;
use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::marker;
use std::mem;
use std::panic;
use std::thread::{spawn, JoinHandle};

/// Send `T`, then continue as `S`.
#[must_use]
#[derive(Debug)]
pub struct Send<T, S>
where
    T: marker::Send,
    S: Session,
    S::Dual: Session,
{
    channel: Sender<(T, S::Dual)>,
}

/// Receive `T`, then continue as `S`.
#[must_use]
#[derive(Debug)]
pub struct Recv<T, S>
where
    T: marker::Send,
    S: Session,
{
    channel: Receiver<(T, S)>,
}

/// End of communication.
#[must_use]
#[derive(Debug)]
pub struct End {
    pub sender: Sender<()>,
    pub receiver: Receiver<()>,
}

/// Trait for session types. Provides duality.
pub trait Session: marker::Sized + marker::Send {
    /// The session type dual to `Self`.
    type Dual: Session<Dual = Self>;

    /// Creates two new *dual* channels.
    ///
    /// *Here be dragons!*
    ///
    /// The `new` function is used internally in this library to define
    /// functions such as `send` and `fork`. When combined with `thread::spawn`,
    /// it can be used to construct deadlocks.
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual);

    #[doc(hidden)]
    fn head_str() -> String;

    #[doc(hidden)]
    fn tail_str() -> String;
}

impl Session for End {
    type Dual = End;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        (
            End {
                sender: sender1,
                receiver: receiver2,
            },
            End {
                sender: sender2,
                receiver: receiver1,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("End")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        String::from("")
    }
}

impl<T: marker::Send, S: Session> Session for Send<T, S> {
    type Dual = Recv<T, S::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<(T, S::Dual)>(1);
        (Send { channel: sender }, Recv { channel: receiver })
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("Send")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }
}

impl<T: marker::Send, S: Session> Session for Recv<T, S> {
    type Dual = Send<T, S::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (there, here) = Self::Dual::new();
        (here, there)
    }

    #[doc(hidden)]
    fn head_str() -> String {
        String::from("Recv")
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }
}

/// Send a value of type `T`. Always succeeds. Returns the continuation of the
/// session `S`.
pub fn send<T, S>(x: T, s: Send<T, S>) -> S
where
    T: marker::Send,
    S: Session,
{
    // For TCP client
    // stream.write(&data[0..size]).unwrap();
    // Need to force next type: stream.shutdown(Shutdown::Write).unwrap(); but no way to do it twice 
    let (here, there) = S::new();
    s.channel.send((x, there)).unwrap_or(());
    here
}

/// Receive a value of type `T`. Can fail. Returns either a pair of the received
/// value and the continuation of the session `S` or an error.
pub fn recv<T, S>(s: Recv<T, S>) -> Result<(T, S), Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    // For TCP client
    // let mut data = [0 as u8; 50]; // using 50 byte buffer
    // match stream.read(&mut data) { // or stream.read_exact(&mut data)
    // Ok(size) =>
    // Err(e) =>
    // Need to force next type: stream.shutdown(Shutdown::Read).unwrap(); but no way to do it twice 
    let (v, s) = s.channel.recv()?;
    Ok((v, s))
}

/// Cancels a session. Always succeeds. If the partner calls `recv` or `close`
/// after cancellation, those calls fail.
pub fn cancel<T>(x: T) {
    mem::drop(x);
}

/// Closes a session. Synchronises with the partner, and fails if the partner
/// has crashed.
pub fn close(s: End) -> Result<(), Box<dyn Error>> {
    // For TCP client
    // Need to force closing type: stream.shutdown(Shutdown::Both).unwrap();
    s.sender.send(()).unwrap_or(());
    s.receiver.recv()?;
    Ok(())
}

#[doc(hidden)]
pub fn fork_with_thread_id<S, P>(p: P) -> (JoinHandle<()>, S::Dual)
where
    S: Session + 'static,
    P: FnOnce(S) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    // For TCP client
    // match TcpStream::connect("localhost:3333") {
    // Ok(result) =>
    // Err(e) =>
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

/// Creates a child process, and a session with two dual endpoints of type `S`
/// and `S::Dual`. The first endpoint is given to the child process. Returns the
/// second endpoint.
pub fn fork<S, P>(p: P) -> S::Dual
where
    S: Session + 'static,
    P: FnOnce(S) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    fork_with_thread_id(p).1
}

/// Offer a choice between two sessions `S1` and `S1`. Implemented using `Recv`
/// and `Either`.
pub type Offer<S1, S2> = Recv<Either<S1, S2>, End>;

/// Choose between two sessions `S1` and `S2`. Implemented using `Send` and
/// `Either`.
pub type Choose<S1, S2> = Send<Either<<S1 as Session>::Dual, <S2 as Session>::Dual>, End>;

/// Offer a choice between two sessions `S1` and `S2`.
pub fn offer_either<'a, S1, S2, F, G, R>(
    s: Offer<S1, S2>,
    f: F,
    g: G,
) -> Result<R, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    F: FnOnce(S1) -> Result<R, Box<dyn Error + 'a>>,
    G: FnOnce(S2) -> Result<R, Box<dyn Error + 'a>>,
{
    let (e, s) = recv(s)?;
    cancel(s);
    e.either(f, g)
}

/// Given a choice between sessions `S1` and `S1`, choose the first option.
pub fn choose_left<'a, S1, S2>(s: Choose<S1, S2>) -> S1
where
    S1: Session + 'a,
    S2: Session + 'a,
{
    let (here, there) = S1::new();
    let s = send(Either::Left(there), s);
    cancel(s);
    here
}

/// Given a choice between sessions `S1` and `S1`, choose the second option.
pub fn choose_right<'a, S1, S2>(s: Choose<S1, S2>) -> S2
where
    S1: Session + 'a,
    S2: Session + 'a,
{
    let (here, there) = S2::new();
    let s = send(Either::Right(there), s);
    cancel(s);
    here
}

/// Offer a choice between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer {
    ($session:expr, { $($pat:pat => $result:expr,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = recv($session)?;
            mpstthree::binary::cancel(s);
            match l {
                $(
                    $pat => $result,
                )*
            }
        })()
    };
}

/// Choose between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose {
    ($label:path, $session:expr) => {{
        let (here, there) = <_ as Session>::new();
        let s = send($label(there), $session);
        mpstthree::binary::cancel(s);
        here
    }};
}

/// Error returned when `select` or `select_mut` are called with an empty vector.
#[derive(Debug)]
enum SelectError {
    EmptyVec,
}

impl fmt::Display for SelectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SelectError::EmptyVec => write!(f, "please use a vector with at least one element"),
        }
    }
}

impl Error for SelectError {
    fn description(&self) -> &str {
        match *self {
            SelectError::EmptyVec => "empty vectors not allowed",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SelectError::EmptyVec => None,
        }
    }
}

/// Selects the first active session. Receives from the selected session, and
/// removes the endpoint from the input vector. Returns the received value and
/// the continuation of the selected session.
pub fn select_mut<T, S>(rs: &mut Vec<Recv<T, S>>) -> Result<(T, S), Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    if rs.is_empty() {
        Err(Box::new(SelectError::EmptyVec))
    } else {
        let (index, res) = {
            let mut sel = Select::new();
            let iter = rs.iter();
            for r in iter {
                sel.recv(&r.channel);
            }
            loop {
                let index = sel.ready();
                let res = rs[index].channel.try_recv();

                if let Err(e) = res {
                    if e.is_empty() {
                        continue;
                    }
                }

                break (index, res);
            }
        };

        let _ = rs.swap_remove(index);
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(Box::new(e)),
        }
    }
}

type SelectType<T, S> = Result<(T, S), Box<dyn Error>>;

/// Selects the first active session. Receives from the selected session.
/// Returns the received value, the continuation of the selected session, and a
/// copy of the input vector without the selected session.
pub fn select<T, S>(rs: Vec<Recv<T, S>>) -> (SelectType<T, S>, Vec<Recv<T, S>>)
where
    T: marker::Send,
    S: Session,
{
    let mut rs = rs;
    let res = select_mut(&mut rs);
    (res, rs)
}
