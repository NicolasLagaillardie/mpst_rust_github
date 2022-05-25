//! This module contains the definition and associated functions and traits
//! for the Send structure.

use crate::binary::struct_trait::recv::Recv;
use crate::binary::struct_trait::session::Session;

use crossbeam_channel::{bounded, Sender};

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
    #[doc(hidden)]
    pub channel: Sender<(T, S::Dual)>,
}

impl<T: marker::Send, S: Session> Session for Send<T, S> {
    type Dual = Recv<T, S::Dual>;

    fn new() -> (Self, Self::Dual) {
        let (sender, receiver) = bounded::<(T, S::Dual)>(1);
        (Send { channel: sender }, Recv { channel: receiver })
    }

    fn head_str() -> String {
        "Send".to_string()
    }

    fn tail_str() -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }

    fn self_head_str(&self) -> String {
        "Send".to_string()
    }

    fn self_tail_str(&self) -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }
}
