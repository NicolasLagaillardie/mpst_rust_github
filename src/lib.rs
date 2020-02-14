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
pub struct RoleAtoB<R: Role> {
    sender: Sender<(R::Dual)>,
    receiver: Receiver<(R)>,
}
pub struct RoleAtoC<R: Role> {
    sender: Sender<(R::Dual)>,
    receiver: Receiver<(R)>,
}
pub struct RoleBtoA<R: Role> {
    sender: Sender<(R::Dual)>,
    receiver: Receiver<(R)>,
}
pub struct RoleBtoC<R: Role> {
    sender: Sender<(R::Dual)>,
    receiver: Receiver<(R)>,
}
pub struct RoleCtoA<R: Role> {
    sender: Sender<(R::Dual)>,
    receiver: Receiver<(R)>,
}
pub struct RoleCtoB<R: Role> {
    sender: Sender<(R::Dual)>,
    receiver: Receiver<(R)>,
}
pub struct RoleEnd {
    sender: Sender<()>,
    receiver: Receiver<()>,
}

pub trait Role: marker::Sized + marker::Send {
    type Dual: Role<Dual = Self>;

    fn new() -> (Self, Self::Dual);
}

impl Role for RoleEnd {
    type Dual = RoleEnd;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        return (
            RoleEnd {
                sender: sender1,
                receiver: receiver2,
            },
            RoleEnd {
                sender: sender2,
                receiver: receiver1,
            },
        );
    }
}

impl<R: Role> Role for RoleAtoB<R> {
    type Dual = RoleBtoA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<(R)>(1);
        let (sender_dual, receiver_dual) = bounded::<(R::Dual)>(1);

        return (
            RoleAtoB {
                sender: sender_dual,
                receiver: receiver,
            },
            RoleBtoA {
                sender: sender,
                receiver: receiver_dual,
            },
        );
    }
}

impl<R: Role> Role for RoleBtoA<R> {
    type Dual = RoleAtoB<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (there, here) = Self::Dual::new();
        return (here, there);
    }
}

impl<R: Role> Role for RoleBtoC<R> {
    type Dual = RoleCtoB<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<(R)>(1);
        let (sender_dual, receiver_dual) = bounded::<(R::Dual)>(1);

        return (
            RoleBtoC {
                sender: sender_dual,
                receiver: receiver,
            },
            RoleCtoB {
                sender: sender,
                receiver: receiver_dual,
            },
        );
    }
}

impl<R: Role> Role for RoleCtoB<R> {
    type Dual = RoleBtoC<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (there, here) = Self::Dual::new();
        return (here, there);
    }
}

impl<R: Role> Role for RoleCtoA<R> {
    type Dual = RoleAtoC<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<(R)>(1);
        let (sender_dual, receiver_dual) = bounded::<(R::Dual)>(1);

        return (
            RoleCtoA {
                sender: sender_dual,
                receiver: receiver,
            },
            RoleAtoC {
                sender: sender,
                receiver: receiver_dual,
            },
        );
    }
}

impl<R: Role> Role for RoleAtoC<R> {
    type Dual = RoleCtoA<R::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (there, here) = Self::Dual::new();
        return (here, there);
    }
}

pub fn nextAtoB<R>(r: RoleAtoB<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send((there)).unwrap_or(());
    r.receiver.recv();
    here
}

pub fn nextBtoA<R>(r: RoleBtoA<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send((there)).unwrap_or(());
    r.receiver.recv();
    here
}

pub fn nextAtoC<R>(r: RoleAtoC<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send((there)).unwrap_or(());
    r.receiver.recv();
    here
}

pub fn nextCtoA<R>(r: RoleCtoA<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send((there)).unwrap_or(());
    r.receiver.recv();
    here
}

pub fn nextBtoC<R>(r: RoleBtoC<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send((there)).unwrap_or(());
    r.receiver.recv();
    here
}

pub fn nextCtoB<R>(r: RoleCtoB<R>) -> R
where
    R: Role,
{
    let (here, there) = R::new();
    r.sender.send((there)).unwrap_or(());
    r.receiver.recv();
    here
}

pub fn nextEnd(r: RoleEnd) -> Result<(), Box<dyn Error>> {
    r.sender.send(()).unwrap_or(());
    r.receiver.recv()?;
    Ok(())
}

/// Definition of MPST

pub struct SessionMpst<S1: Session, S2: Session, R: Role> {
    pub session1: S1,
    pub session2: S2,
    pub queue: R,
}

/// All Recv/Recv

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Recv<T2, S2>, RoleAtoB<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Send<T2, S2::Dual>, RoleBtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleAtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleBtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Recv<T2, S2>, RoleBtoA<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Send<T2, S2::Dual>, RoleAtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleBtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleAtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Recv<T2, S2>, RoleAtoC<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Send<T2, S2::Dual>, RoleCtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleAtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleCtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Recv<T2, S2>, RoleCtoA<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Send<T2, S2::Dual>, RoleAtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleCtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleAtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Recv<T2, S2>, RoleBtoC<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Send<T2, S2::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Recv<T2, S2>, RoleCtoB<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Send<T2, S2::Dual>, RoleBtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

/// All Send/Send

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Send<T2, S2>, RoleAtoB<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleBtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleAtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleBtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Send<T2, S2>, RoleBtoA<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleAtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleBtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleAtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Send<T2, S2>, RoleAtoC<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleCtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleAtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleCtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Send<T2, S2>, RoleCtoA<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleAtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleCtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleAtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Send<T2, S2>, RoleBtoC<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Send<T2, S2>, RoleCtoB<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleBtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

///All Send/Recv

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Recv<T2, S2>, RoleAtoB<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Send<T2, S2::Dual>, RoleBtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleAtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleBtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Recv<T2, S2>, RoleBtoA<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Send<T2, S2::Dual>, RoleAtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleBtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleAtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Recv<T2, S2>, RoleAtoC<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Send<T2, S2::Dual>, RoleCtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleAtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleCtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Recv<T2, S2>, RoleCtoA<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Send<T2, S2::Dual>, RoleAtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleCtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleAtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Recv<T2, S2>, RoleBtoC<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Send<T2, S2::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Recv<T2, S2>, RoleCtoB<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Send<T2, S2::Dual>, RoleBtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

/// ALl Recv/Send

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Send<T2, S2>, RoleAtoB<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleBtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleAtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleBtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Send<T2, S2>, RoleBtoA<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleAtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleBtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleAtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Send<T2, S2>, RoleAtoC<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleCtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleAtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleCtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Send<T2, S2>, RoleCtoA<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleAtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleCtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleAtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Send<T2, S2>, RoleBtoC<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Send<T2, S2>, RoleCtoB<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleBtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

/// All Recv/End

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Recv<T, S>, End, RoleAtoB<R>> {
    type Dual = SessionMpst<Send<T, S::Dual>, End, RoleBtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleAtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleBtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Recv<T, S>, End, RoleBtoA<R>> {
    type Dual = SessionMpst<Send<T, S::Dual>, End, RoleAtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleBtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleAtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Recv<T, S>, End, RoleAtoC<R>> {
    type Dual = SessionMpst<Send<T, S::Dual>, End, RoleCtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleAtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleCtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Recv<T, S>, End, RoleCtoA<R>> {
    type Dual = SessionMpst<Send<T, S::Dual>, End, RoleAtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleCtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleAtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Recv<T, S>, End, RoleBtoC<R>> {
    type Dual = SessionMpst<Send<T, S::Dual>, End, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Recv<T, S>, End, RoleCtoB<R>> {
    type Dual = SessionMpst<Send<T, S::Dual>, End, RoleBtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleCtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleBtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

/// All End/Recv

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Recv<T, S>, RoleAtoB<R>> {
    type Dual = SessionMpst<End, Send<T, S::Dual>, RoleBtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Recv { channel: receiver },
                queue: RoleAtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Send { channel: sender },
                queue: RoleBtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Recv<T, S>, RoleBtoA<R>> {
    type Dual = SessionMpst<End, Send<T, S::Dual>, RoleAtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Recv { channel: receiver },
                queue: RoleBtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Send { channel: sender },
                queue: RoleAtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Recv<T, S>, RoleAtoC<R>> {
    type Dual = SessionMpst<End, Send<T, S::Dual>, RoleCtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Recv { channel: receiver },
                queue: RoleAtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Send { channel: sender },
                queue: RoleCtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Recv<T, S>, RoleCtoA<R>> {
    type Dual = SessionMpst<End, Send<T, S::Dual>, RoleAtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Recv { channel: receiver },
                queue: RoleCtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Send { channel: sender },
                queue: RoleAtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Recv<T, S>, RoleBtoC<R>> {
    type Dual = SessionMpst<End, Send<T, S::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Recv { channel: receiver },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Send { channel: sender },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Recv<T, S>, RoleCtoB<R>> {
    type Dual = SessionMpst<End, Send<T, S::Dual>, RoleBtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Recv { channel: receiver },
                queue: RoleCtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Send { channel: sender },
                queue: RoleBtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

/// All End/Send

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Send<T, S>, RoleAtoB<R>> {
    type Dual = SessionMpst<End, Recv<T, S::Dual>, RoleBtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Send { channel: sender },
                queue: RoleAtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Recv { channel: receiver },
                queue: RoleBtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Send<T, S>, RoleBtoA<R>> {
    type Dual = SessionMpst<End, Recv<T, S::Dual>, RoleAtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Send { channel: sender },
                queue: RoleBtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Recv { channel: receiver },
                queue: RoleAtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Send<T, S>, RoleAtoC<R>> {
    type Dual = SessionMpst<End, Recv<T, S::Dual>, RoleCtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Send { channel: sender },
                queue: RoleAtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Recv { channel: receiver },
                queue: RoleCtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Send<T, S>, RoleCtoA<R>> {
    type Dual = SessionMpst<End, Recv<T, S::Dual>, RoleAtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Send { channel: sender },
                queue: RoleCtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Recv { channel: receiver },
                queue: RoleAtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Send<T, S>, RoleBtoC<R>> {
    type Dual = SessionMpst<End, Recv<T, S::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Send { channel: sender },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Recv { channel: receiver },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Send<T, S>, RoleCtoB<R>> {
    type Dual = SessionMpst<End, Recv<T, S::Dual>, RoleBtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Send { channel: sender },
                queue: RoleCtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Recv { channel: receiver },
                queue: RoleBtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

/// All Send/End

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Send<T, S>, End, RoleAtoB<R>> {
    type Dual = SessionMpst<Recv<T, S::Dual>, End, RoleBtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleAtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleBtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Send<T, S>, End, RoleBtoA<R>> {
    type Dual = SessionMpst<Recv<T, S::Dual>, End, RoleAtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleBtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleAtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Send<T, S>, End, RoleAtoC<R>> {
    type Dual = SessionMpst<Recv<T, S::Dual>, End, RoleCtoA<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleAtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleCtoA {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Send<T, S>, End, RoleCtoA<R>> {
    type Dual = SessionMpst<Recv<T, S::Dual>, End, RoleAtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleCtoA {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleAtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Send<T, S>, End, RoleBtoC<R>> {
    type Dual = SessionMpst<Recv<T, S::Dual>, End, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Send<T, S>, End, RoleCtoB<R>> {
    type Dual = SessionMpst<Recv<T, S::Dual>, End, RoleBtoC<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<(R::Dual)>(1);
        let (sender_role_two, receiver_role_two) = bounded::<(R)>(1);

        return (
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleCtoB {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleBtoC {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl Session for SessionMpst<End, End, RoleEnd> {
    type Dual = SessionMpst<End, End, RoleEnd>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1_one, receiver1_one) = bounded::<()>(1);
        let (sender2_one, receiver2_one) = bounded::<()>(1);
        let (sender1_two, receiver1_two) = bounded::<()>(1);
        let (sender2_two, receiver2_two) = bounded::<()>(1);

        let (sender_role_one, receiver_role_one) = bounded::<()>(1);
        let (sender_role_two, receiver_role_two) = bounded::<()>(1);

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
                queue: RoleEnd {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
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
                queue: RoleEnd {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

/// Sending on session 1
pub fn send_mpst_session_one_A_to_B<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleAtoB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = nextAtoB(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_one_B_to_A<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleBtoA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = nextBtoA(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_one_A_to_C<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleAtoC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = nextAtoC(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_one_C_to_A<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleCtoA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = nextCtoA(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_one_C_to_B<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleCtoB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = nextCtoB(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_one_B_to_C<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleBtoC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = nextBtoC(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

/// Sending on session 2
pub fn send_mpst_session_two_A_to_B<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleAtoB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = nextAtoB(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_two_B_to_A<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleBtoA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = nextBtoA(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_two_A_to_C<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleAtoC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = nextAtoC(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_two_C_to_A<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleCtoA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = nextCtoA(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_two_B_to_C<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleBtoC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = nextBtoC(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

pub fn send_mpst_session_two_C_to_B<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleCtoB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = nextCtoB(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    result
}

/// Recving on session 1
pub fn recv_mpst_session_one_A_to_B<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleAtoB<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = nextAtoB(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_one_B_to_A<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleBtoA<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = nextBtoA(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_one_A_to_C<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleAtoC<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = nextAtoC(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_one_C_to_A<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleCtoA<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = nextCtoA(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_one_B_to_C<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleBtoC<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = nextBtoC(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_one_C_to_B<T, S1, S2, R>(
    s: SessionMpst<Recv<T, S1>, S2, RoleCtoB<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session1)?;
    let new_queue = nextCtoB(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    Ok((v, result))
}

/// Recving on session 2
pub fn recv_mpst_session_two_A_to_B<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleAtoB<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = nextAtoB(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_two_B_to_A<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleBtoA<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = nextBtoA(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_two_A_to_C<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleAtoC<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = nextAtoC(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_two_C_to_A<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleCtoA<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = nextCtoA(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_two_B_to_C<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleBtoC<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = nextBtoC(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

pub fn recv_mpst_session_two_C_to_B<T, S1, S2, R>(
    s: SessionMpst<S1, Recv<T, S2>, RoleCtoB<R>>,
) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let (v, new_session) = recv(s.session2)?;
    let new_queue = nextCtoB(s.queue);
    let result = SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    };

    Ok((v, result))
}

//pub type Offer_Mpst_one<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>> = Recv<Either<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>>, End>;

//pub type Choose_Mpst_one<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>> = Send<Either<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>>, End>;

pub fn offer_mpst_session_one_B_to_A<'a, S1, S2, S3, S4, F, G, R, R1, R2, U, C1, C2>(
    s: SessionMpst<Offer_Mpst_one<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>>, S4, RoleBtoA<R>>,
    f: F,
    g: G,
) -> Result<(U), Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    R1: Role,
    R2: Role,
    R: Role,
    F: FnOnce(SessionMpst<S1, S3, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S2, S3, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, new_session) = recv_mpst_session_one_B_to_A(s)?;
    cancel(new_session);
    e.either(f, g)
}

pub fn choose_left_mpst_session_one_A_to_B<S1, S2, S3, R, R1, R2>(
    s: SessionMpst<Choose_Mpst_one<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>>, S4, RoleAtoB<R>>,
) -> SessionMpst<S1, S3, R1>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    R: Role,
    R1: Role,
    R2: Role,
{
    let (here, there) = S1::new();
    let new_session = send(Either::Left(there), s.session1);
    cancel(new_session);
    let new_queue = nextAtoB(s.queue);

    let result = SessionMpst {
        session1: here,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

pub fn choose_right_mpst_session_one_A_to_B<S1, S2, S3, R>(
    s: SessionMpst<Choose<S1, S2>, S3, RoleAtoB<R>>,
) -> SessionMpst<S2, S3, R>
where
    S1: Session,
    S2: Session,
    S3: Session,
    R: Role,
{
    let (here, there) = S2::new();
    let new_session = send(Either::Right(there), s.session1);
    cancel(new_session);
    let new_queue = nextAtoB(s.queue);

    let result = SessionMpst {
        session1: here,
        session2: s.session2,
        queue: new_queue,
    };

    result
}

/// Closes session one. Synchronises with the partner, and fails if the partner
/// has crashed.
pub fn close_mpst(s: SessionMpst<End, End, RoleEnd>) -> Result<(), Box<dyn Error>> {
    close(s.session1)?;
    close(s.session2)?;
    nextEnd(s.queue);

    Ok(())
}

pub fn fork_simple<S1, S2, R, P>(p: P, s: SessionMpst<S1, S2, R>) -> JoinHandle<()>
where
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
    P: FnOnce(SessionMpst<S1, S2, R>) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
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
