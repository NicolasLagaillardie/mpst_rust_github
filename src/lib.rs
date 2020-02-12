///#![feature(never_type)]
extern crate crossbeam_channel;
extern crate either;

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
pub struct Send<T, S: Session> {
    channel: Sender<(T, S::Dual)>,
}

/// Receive `T`, then continue as `S`.
#[must_use]
#[derive(Debug)]
pub struct Recv<T, S: Session> {
    channel: Receiver<(T, S)>,
}

/// End of communication.
#[must_use]
#[derive(Debug)]
pub struct End {
    sender: Sender<()>,
    receiver: Receiver<()>,
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
}

impl Session for End {
    type Dual = End;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        return (
            End {
                sender: sender1,
                receiver: receiver2,
            },
            End {
                sender: sender2,
                receiver: receiver1,
            },
        );
    }
}

impl<T: marker::Send, S: Session> Session for Send<T, S> {
    type Dual = Recv<T, S::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<(T, S::Dual)>(1);
        return (Send { channel: sender }, Recv { channel: receiver });
    }
}

impl<T: marker::Send, S: Session> Session for Recv<T, S> {
    type Dual = Send<T, S::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (there, here) = Self::Dual::new();
        return (here, there);
    }
}

/// Send a value of type `T`. Always succeeds. Returns the continuation of the
/// session `S`.
pub fn send<T, S>(x: T, s: Send<T, S>) -> S
where
    T: marker::Send,
    S: Session,
{
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
    P: FnOnce(S) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
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
    ($label:path, $session:expr) => {{
        let (here, there) = <_ as Session>::new();
        let s = send($label(there), $session);
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

/// Selects the first active session. Receives from the selected session.
/// Returns the received value, the continuation of the selected session, and a
/// copy of the input vector without the selected session.
pub fn select<T, S>(rs: Vec<Recv<T, S>>) -> (Result<(T, S), Box<dyn Error>>, Vec<Recv<T, S>>)
where
    T: marker::Send,
    S: Session,
{
    let mut rs = rs;
    let res = select_mut(&mut rs);
    (res, rs)
}

/// MPST Block from here...

///// Type Role
//#[derive(Debug)]
//pub enum Role {
//    one,
//    two,
//    end,
//}

//pub struct RoleQueue<R: Role, Q: Queue> {
//    main: (R, Q);
//}
//
//pub trait Queue: marker::Sized + marker::Send {
//    #[doc(hidden)]
//    fn new() -> Self; 
//}
//
//impl<R: Role, Q: Queue> Queue for RoleQueue<R, Q> {
//    #[doc(hidden)]
//    fn new() -> Self {
//        let result = (R, Q);
//        return RoleQueue { main: result, } ;
//    }
//}

//#[derive(Debug)]
//pub struct RoleOne<Q: Queue> {
////    sender: Sender<(Q)>,
////    receiver: Receiver<(Q)>,
//}
//
//#[derive(Debug)]
//pub struct RoleTwo<Q: Queue> {
////    sender: Sender<(Q)>,
////    receiver: Receiver<(Q)>,
//}
//
//#[derive(Debug)]
//pub struct RoleEnd {
////    sender: Sender<()>,
////    receiver: Receiver<()>,
//}

//pub struct SessionMpst<S1: Session, S2: Session, Q: Queue> {
pub struct SessionMpst<S1: Session, S2: Session> {
    pub session1: S1,
    pub session2: S2,
//    queue: Q,
}








//pub struct End {
//    sender: Sender<()>,
//    receiver: Receiver<()>,
//}
//
///// Trait for session types. Provides duality.
//pub trait Session: marker::Sized + marker::Send {
//    /// The session type dual to `Self`.
//    type Dual: Session<Dual = Self>;
//
//    /// Creates two new *dual* channels.
//    ///
//    /// *Here be dragons!*
//    ///
//    /// The `new` function is used internally in this library to define
//    /// functions such as `send` and `fork`. When combined with `thread::spawn`,
//    /// it can be used to construct deadlocks.
//    #[doc(hidden)]
//    fn new() -> (Self, Self::Dual);
//}
//
//pub trait Queue: marker::Sized + marker::Send {
//    fn new() -> Self;
//}
//
//impl Queue for RoleEnd {
//    #[doc(hidden)]
//    fn new() -> Self {
//        let (sender1, receiver1) = bounded::<()>(1);
//
//        return (
//            End {
//                sender: sender1,
//                receiver: receiver2,
//            },
//            End {
//                sender: sender2,
//                receiver: receiver1,
//            },
//        );
//    }
//}











impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session> Session
    for SessionMpst<Recv<T1, S1>, Recv<T2, S2>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Send<T2, S2::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

//        let queue_one = Vec::new();
//        let queue_two = Vec::new() ;

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
//                queue: queue_one,
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
//                queue: queue_two,
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session> Session
    for SessionMpst<Send<T1, S1>, Send<T2, S2>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Recv<T2, S2::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

//        let queue_one = Vec::new();
//        let queue_two = Vec::new();
 
        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
//                queue: queue_one,
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
//                queue: queue_two,
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session> Session
    for SessionMpst<Send<T1, S1>, Recv<T2, S2>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Send<T2, S2::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

//        let queue_one = Vec::new();
//        let queue_two = Vec::new();

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
//                queue: queue_one,
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
//                queue: queue_two,
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session> Session
    for SessionMpst<Recv<T1, S1>, Send<T2, S2>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Recv<T2, S2::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

//        let queue_one = Vec::new();
//        let queue_two = Vec::new();

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
//                queue: queue_one,
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
//                queue: queue_two,
            },
        );
    }
}

impl<T: marker::Send, S: Session> Session for SessionMpst<Recv<T, S>, End> {
    type Dual = SessionMpst<Send<T, S::Dual>, End>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

//        let queue_one = Vec::new();
//        let queue_two = Vec::new();

        return (
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
//                queue: queue_one,
            },
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
//                queue: queue_two,
            },
        );
    }
}

impl<T: marker::Send, S: Session> Session for SessionMpst<End, Recv<T, S>> {
    type Dual = SessionMpst<End, Send<T, S::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

//        let queue_one = Vec::new();
//        let queue_two = Vec::new();

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Recv { channel: receiver },
//                queue: queue_one,
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Send { channel: sender },
//                queue: queue_two,
            },
        );
    }
}

impl<T: marker::Send, S: Session> Session for SessionMpst<End, Send<T, S>> {
    type Dual = SessionMpst<End, Recv<T, S::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

//        let queue_one = Vec::new();
//        let queue_two = Vec::new();

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Send { channel: sender },
//                queue: queue_one,
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Recv { channel: receiver },
//                queue: queue_two,
            },
        );
    }
}

//impl<T: marker::Send, S: Session, Q: Queue, R: RoleOne<Q>> Session for SessionMpst<Send<T, S>, End, R> {
impl<T: marker::Send, S: Session> Session for SessionMpst<Send<T, S>, End> {
//    type Dual = SessionMpst<Recv<T, S::Dual>, End, R>;
    type Dual = SessionMpst<Recv<T, S::Dual>, End>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

//        let queue_one = RoleOne<Q>;
//        let queue_two = RoleOne<Q>;

        return (
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
//                queue: queue_one,
            },
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
//                queue: queue_two,
            },
        );
    }
}

//impl<R: Role> Session for SessionMpst<End, End, RoleEnd> {
impl Session for SessionMpst<End, End> {
//    type Dual = SessionMpst<End, End, RoleEnd>;
    type Dual = SessionMpst<End, End>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1_one, receiver1_one) = bounded::<()>(1);
        let (sender2_one, receiver2_one) = bounded::<()>(1);
        let (sender1_two, receiver1_two) = bounded::<()>(1);
        let (sender2_two, receiver2_two) = bounded::<()>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1_one,
                    receiver: receiver2_one,
                },
                session2: End {
                    sender: sender1_two,
                    receiver: receiver2_two,
                },
//                queue: RoleEnd,
            },
            SessionMpst {
                session1: End {
                    sender: sender2_one,
                    receiver: receiver1_one,
                },
                session2: End {
                    sender: sender2_two,
                    receiver: receiver1_two,
                },
//                queue: RoleEnd,
            },
        );
    }
}

///// Comparing Roles
//impl PartialEq for Role {
//    fn eq(&self, other: &Self) -> bool {
//        self.name == other.name
//    }
//}

/// Sending on session 1
//pub fn send_mpst_session_one<T, S1, S2, Q>(
pub fn send_mpst_session_one<T, S1, S2>(
    x: T,
//    s: SessionMpst<Send<T, S1>, S2, RoleOne<Q>>,
    s: SessionMpst<Send<T, S1>, S2>,
//) -> SessionMpst<S1, S2, Q>
) -> SessionMpst<S1, S2>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
//    Q: Queue,
{
    let new_session = send(x, s.session1);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
//        queue: s.queue.tail,
    };

    result
}

/// Sending on session 2
//pub fn send_mpst_session_two<T, S1, S2, Q>(
pub fn send_mpst_session_two<T, S1, S2>(
    x: T,
//    s: SessionMpst<S1, Send<T, S2>, RoleTwo<Q>>,
    s: SessionMpst<S1, Send<T, S2>>,
//) -> SessionMpst<S1, S2, Q>
) -> SessionMpst<S1, S2>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
//    Q: Queue,
{
    let new_session = send(x, s.session2);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
//        queue: s.queue.tail,
    };

    result
}

/// Recving on session 1
//pub fn recv_mpst_session_one<T, S1, S2, Q>(
pub fn recv_mpst_session_one<T, S1, S2>(
//    s: SessionMpst<Recv<T, S1>, S2, RoleOne<Q>>,
    s: SessionMpst<Recv<T, S1>, S2>,
//) -> Result<(T, SessionMpst<S1, S2, RQ>), Box<dyn Error>>
) -> Result<(T, SessionMpst<S1, S2>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
//    Q: Queue,
{
    let (v, new_session) = recv(s.session1)?;
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
//        queue: s.queue.tail, 
    };

    Ok((v, result))
}

/// Recving on session 2
//pub fn recv_mpst_session_two<T, S1, S2, Q>(
pub fn recv_mpst_session_two<T, S1, S2>(
//    s: SessionMpst<S1, Recv<T, S2>, RoleTwo<Q>>,
    s: SessionMpst<S1, Recv<T, S2>>,
//) -> Result<(T, SessionMpst<S1, S2, Q>), Box<dyn Error>>
) -> Result<(T, SessionMpst<S1, S2>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
//    Q: Queue,
{
    let (v, new_session) = recv(s.session2)?;
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
//        queue: s.queue.tail,
    };

    Ok((v, result))
}

/// Closes session one. Synchronises with the partner, and fails if the partner
/// has crashed.
//pub fn close_mpst(s: SessionMpst<End, End, RoleEnd>) -> Result<(), Box<dyn Error>> {
pub fn close_mpst(s: SessionMpst<End, End>) -> Result<(), Box<dyn Error>> {
    close(s.session1)?;
    close(s.session2)?;

    Ok(())
}

#[doc(hidden)]
pub fn fork_with_thread_id_mpst<S1, S2, P1, P2>(
    p_one: P1,
    p_two: P2,
) -> (
    JoinHandle<()>,
    JoinHandle<()>,
//    SessionMpst<S1::Dual, S2::Dual, RoleQueue>,
    SessionMpst<S1::Dual, S2::Dual>,
)
where
    S1: Session + 'static,
    P1: FnOnce(S1) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
    S2: Session + 'static,
    P2: FnOnce(S2) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    let (there_one, here_one) = Session::new();
    let (there_two, here_two) = Session::new();
//    let queue = RoleQueue::new();

    let other_thread_one = spawn(move || {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));

        match p_one(there_one) {
            Ok(()) => (),
            Err(e) => panic!("{}", e.description()),
        }
    });

    let other_thread_two = spawn(move || {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));

        match p_two(there_two) {
            Ok(()) => (),
            Err(e) => panic!("{}", e.description()),
        }
    });

    let result = SessionMpst {
        session1: here_one,
        session2: here_two,
//        queue: queue, 
    };

    (other_thread_one, other_thread_two, result)
}

/// Creates a child process, and a session with two dual endpoints of type `S`
/// and `S::Dual`. The first endpoint is given to the child process. Returns the
/// second endpoint.
//pub fn fork_mpst<S1, S2, P1, P2>(p_one: P1, p_two: P2) -> SessionMpst<S1::Dual, S2::Dual, RoleQueue>
pub fn fork_mpst<S1, S2, P1, P2>(p_one: P1, p_two: P2) -> SessionMpst<S1::Dual, S2::Dual>
where
    S1: Session + 'static,
    P1: FnOnce(S1) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
    S2: Session + 'static,
    P2: FnOnce(S2) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    fork_with_thread_id_mpst(p_one, p_two).2
}









pub fn fork_simple<S1, S2, P>(p: P, s: SessionMpst<S1, S2>) -> JoinHandle<()>
where
    S1: Session + 'static,
    S2: Session + 'static,
    P: FnOnce(SessionMpst<S1, S2>) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    let other_thread = spawn(move || {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));    
        match p(s) {
            Ok(()) => (),
            Err(e) => panic!("{}", e.description()),
        }    
    });    
    other_thread
}
// ... to there
