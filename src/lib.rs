///#![feature(never_type)]

extern crate crossbeam_channel;
extern crate either;

use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::marker;
use std::mem;
use std::thread::{JoinHandle, spawn};
use std::panic;
use crossbeam_channel::{Sender, Receiver, bounded, Select};
use either::Either;

/// Creation of the Role
#[must_use]
#[derive(Debug)]
pub struct StructRole {
    name: String,
}

/// Send `T`, then continue as `S`.
#[must_use]
#[derive(Debug)]
pub struct Send<T, RS: Role, RR: Role, S: Session> {
    channel: Sender<(T, S::Dual)>,
    sender: Role,
    receiver: Role,
}

/// Receive `T`, then continue as `S`.
#[must_use]
#[derive(Debug)]
pub struct Recv<T, RS: Role, RR: Role, S: Session> {
    channel: Receiver<(T, S)>,
    sender: Role,
    receiver: Role,
}

/// End of communication.
#[must_use]
#[derive(Debug)]
pub struct End {
    sender: Sender<()>,
    receiver: Receiver<()>,
}

pub trait Role: marker::Sized + marker::Send {
    fn new(name: String) -> StructRole;
}

impl Role for StructRole{
    fn new(name: String) -> StructRole {
        StructRole { name: name }
    }
}

pub trait Session: marker::Sized + marker::Send {
    type Dual: Session<Dual=Self>;

    fn new() -> (Self, Self::Dual);
}

impl Session for End {
    type Dual = End;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        return (End { sender: sender1, receiver: receiver2 },
                End { sender: sender2, receiver: receiver1 });
    }
}

impl<T: marker::Send, RS: Role, RR: Role, S: Session> Session for Send<T, RS, RR, S> {
    type Dual = Recv<T, RR, RS, S::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let basicRole = Role::new(String::from(""));

        return (Send {
            channel: sender,
            sender: basicRole,
            receiver: basicRole,
        }, Recv {
            channel: receiver,
            sender: basicRole,
            receiver: basicRole,
        });
    }
}

impl<T: marker::Send, RS: Role, RR: Role, S: Session> Session for Recv<T, RS, RR, S> {
    type Dual = Send<T, RR, RS, S::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (there, here) = Self::Dual::new();
        return (here, there);
    }
}

/// Send a value of type `T`. Always succeeds. Returns the continuation of the
/// session `S`.
pub fn send<T, RS, RR, S>(x: T, rs: RS, rr: RR, s: Send<T, RS, RR, S>) -> S
where
    T: marker::Send,
    RS: Role,
    RR: Role,
    S: Session,
{
    let (here, there) = S::new();
    s.channel.send((x, there)).unwrap_or(());
    here

}

/// Receive a value of type `T`. Can fail. Returns either a pair of the received    
/// value and the continuation of the session `S` or an error.    
pub fn recv<T, RS, RR, S>(rs: RS, rr: RR, s: Recv<T, RS, RR, S>) -> Result<(T, S), Box<dyn Error>>    
where    
    T: marker::Send,
    RS: Role,
    RR: Role,
    S: Session,    
{    
    let (v, s) = s.channel.recv()?;    
    Ok((v, s))    
}

/// Cancels a session. Always succeeds. If the partner calls `recv` or `close`    
/// after cancellation, those calls fail.    
pub fn cancel<T>(x: T) -> () {    
    mem::drop(x);    
}    
    
/// Closes a session. Synchronises with the partner, and fails if the partner    
/// has crashed.    
pub fn close(s: End) -> Result<(), Box<dyn Error>> {    
    s.sender.send(()).unwrap_or(());    
    s.receiver.recv()?;
    Ok(())    
}

#[doc(hidden)]    
pub fn fork_with_thread_id<S, P>(p: P) -> (JoinHandle<()>, S::Dual)    
where    
    S: Session + 'static,    
    P: FnOnce(S) -> Result<(), Box<dyn Error>> + marker::Send + 'static    
{    
    let (there, here) = Session::new();    
    let other_thread = spawn(move || {    
        panic::set_hook(Box::new(|_info| {    
            // do nothing    
        }));    
        match p(there) {    
            Ok(()) => (),    
            Err(e) => panic!("{}", e.description()),    
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
    P: FnOnce(S) -> Result<(), Box<dyn Error>> + marker::Send + 'static
{
    fork_with_thread_id(p).1
}

/// Offer a choice between two sessions `S1` and `S1`. Implemented using `Recv`
/// and `Either`.
pub type Offer<S1, RS, RR, S2> =
    Recv<Either<S1, S2>, RS, RR, End>;

/// Choose between two sessions `S1` and `S2`. Implemented using `Send` and
/// `Either`.
pub type Choose<S1, RS, RR, S2> =
    Send<Either<<S1 as Session>::Dual, <S2 as Session>::Dual>, RS, RR, End>;

/// Offer a choice between two sessions `S1` and `S2`.
pub fn offer_either<'a, S1, S2, F, G, R, RS, RR>(rs: RS, rr: RR, s: Offer<S1, RS, RR, S2>, f: F, g: G)
                                         -> Result<R, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    RS: Role,
    RR: Role,
    F: FnOnce(S1) -> Result<R, Box<dyn Error + 'a>>,
    G: FnOnce(S2) -> Result<R, Box<dyn Error + 'a>>,
{
    let (e, s) = recv(rs, rr, s)?;
    cancel(s);
    e.either(f, g)
}

/// Given a choice between sessions `S1` and `S1`, choose the first option.
pub fn choose_left<'a, S1, RS, RR, S2>(rs: RS, rr: RR, s: Choose<S1, RS, RR, S2>) -> S1
where
    S1: Session + 'a,
    S2: Session + 'a,
    RS: Role,
    RR: Role,
{
    let (here, there) = S1::new();
    let s = send(Either::Left(there), rs, rr, s);
    cancel(s);
    here
}

/// Given a choice between sessions `S1` and `S1`, choose the second option.
pub fn choose_right<'a, S1, RS, RR, S2>(rs: RS, rr: RR, s: Choose<S1, RS, RR, S2>) -> S2
where
    S1: Session + 'a,
    S2: Session + 'a,
    RS: Role,
    RR: Role,
{
    let (here, there) = S2::new();
    let s = send(Either::Right(there), rs, rr, s);
    cancel(s);
    here
}

/// Offer a choice between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! offer {
    ($session:expr, $sender:expr, $receiver:expr, { $($pat:pat => $result:expr,)* }) => {
        (move || -> Result<_, _> {
            let (l, s) = recv($sender, $receiver, $session)?;
            cancel(s);
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
    ($label:path, $sender:expr, $receiver:expr, $session:expr) => {{
        let (here, there) = <_ as Session>::new();
        let s = send($label(there), $sender, $receiver, $session);
        cancel(s);
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
            SelectError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
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
pub fn select_mut<T, RS, RR, S>(rs: RS, rr: RR, result: &mut Vec<Recv<T, RS, RR, S>>) -> Result<(T, S), Box<dyn Error>>
where
    T: marker::Send,
    RS: Role,
    RR: Role,
    S: Session,
{
    if result.is_empty() {
        Err(Box::new(SelectError::EmptyVec))
    }
    else {
        let (index, res) = {
            let mut sel = Select::new();
            let iter = result.iter();
            for r in iter {
                sel.recv(&r.channel);
            }
            loop {                                                                                                
                let index = sel.ready();                                                                         
                let res = result[index].channel.try_recv();

                if let Err(e) = res {
                    if e.is_empty() {
                        continue;
                    }
                }

                break (index, res);
            }
        };                                                                                                            
       let _ = result.swap_remove(index);
       match res {
           Ok(res) => Ok(res),
           Err(e)  => Err(Box::new(e)),
       }
    }
}

/// Selects the first active session. Receives from the selected session.
/// Returns the received value, the continuation of the selected session, and a
/// copy of the input vector without the selected session.
pub fn select<T, RS, RR, S>(rs: RS, rr: RR, result: Vec<Recv<T, RS, RR, S>>) -> (Result<(T, S), Box<dyn Error>>, Vec<Recv<T, RS, RR, S>>)
where
    T: marker::Send,
    RS: Role,
    RR: Role,
    S: Session,
{
    let mut result = result;
    let res = select_mut(rs, rr, &mut result);
    (res, result)
}
