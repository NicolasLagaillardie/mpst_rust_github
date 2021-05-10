use crossbeam_channel::{bounded, Receiver, Sender};
use std::marker;

/// Send `T`, then continue as `S`.
#[must_use]
#[derive(Debug)]
pub struct Send<T, S>
where
    T: marker::Send,
    S: Session,
    S::Dual: Session,
{
    pub channel: Sender<(T, S::Dual)>,
}

/// Receive `T`, then continue as `S`.
#[must_use]
#[derive(Debug)]
pub struct Recv<T, S>
where
    T: marker::Send,
    S: Session,
{
    pub channel: Receiver<(T, S)>,
}

/// End of communication.
#[must_use]
#[derive(Debug)]
pub struct End {
    pub sender: Sender<Signal>,
    pub receiver: Receiver<Signal>,
}

#[derive(Debug)]
pub enum Signal {
    Offer(End),
    Stop,
    Cancel,
}

/// Trait for session types. Provides duality.
pub trait Session: marker::Sized + marker::Send {
    /// The session type dual to `Self`.
    type Dual: Session<Dual = Self>;

    /// Creates two new *dual* channels.
    ///
    /// *Here be dragons!*
    ///
    /// The `new` function is used internally in this
    /// library to define functions such as `send` and
    /// `fork`. When combined with `thread::spawn`,
    /// it can be used to construct deadlocks.
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual);

    #[doc(hidden)]
    fn head_str() -> String;

    #[doc(hidden)]
    fn tail_str() -> String;
}

impl crate::binary::struct_trait::Session for End {
    type Dual = End;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<Signal>(1);
        let (sender2, receiver2) = bounded::<Signal>(1);

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

impl<T: marker::Send, S: crate::binary::struct_trait::Session> crate::binary::struct_trait::Session
    for Send<T, S>
{
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

impl<T: marker::Send, S: crate::binary::struct_trait::Session> crate::binary::struct_trait::Session
    for Recv<T, S>
{
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
