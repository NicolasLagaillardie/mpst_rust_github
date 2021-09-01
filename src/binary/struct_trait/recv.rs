use crate::binary::struct_trait::get_blocks;
use crate::binary::struct_trait::send::Send;
use crate::binary::struct_trait::session::Session;
use crossbeam_channel::Receiver;
use std::error::Error;
use std::fmt;
use std::marker;
use std::str::FromStr;

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

#[derive(Debug, Clone)]
pub struct RecvError {
    details: String,
}

impl RecvError {
    fn new(details: &str) -> RecvError {
        RecvError {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expected `Recv`, found {:?}", self.details)
    }
}

impl Error for RecvError {
    fn description(&self) -> &str {
        &self.details
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
        "Recv".to_string()
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        "Recv".to_string()
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        format!("{}<{}>", S::head_str(), S::tail_str())
    }
}

impl<T: FromStr + marker::Send, S: FromStr + Session> FromStr for Recv<T, S>
where
    <T as FromStr>::Err: fmt::Debug,
    <S as FromStr>::Err: fmt::Debug,
{
    type Err = RecvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..4] {
            "Recv" => {
                let payload_continuation = get_blocks(s.to_string()).unwrap();
                let _ = T::from_str(&payload_continuation[0]).unwrap();
                let _ = S::from_str(&payload_continuation[1]).unwrap();
                Ok(Recv::<T, S>::new().0)
            }
            result => Err(RecvError::new(result)),
        }
    }
}
