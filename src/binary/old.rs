use crossbeam_channel::{bounded, Receiver, Sender};
use std::error::Error;
use std::fmt;
use std::marker;
use std::str::FromStr;

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

impl Session for End {
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

#[derive(Debug, Clone)]
struct SessionStrError {
    details: String,
}

impl SessionStrError {
    fn new(expected: &str) -> SessionStrError {
        SessionStrError {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for SessionStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expected `End`, found {:?}", self.details)
    }
}

impl Error for SessionStrError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl FromStr for End {
    type Err = SessionStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "End" => Ok(End::new().0),
            result => Err("End", result),
        }
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

impl<T: FromStr + marker::Send, S: FromStr + Session> FromStr for Send<T, S> {
    type Err = SessionStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..4] {
            "Send" => {
                let payload_continuation = get_blocks(s.to_string())?;
                let payload = T::from_str(&payload_continuation[0])?;
                let continuation = S::from_str(&payload_continuation[1])?;
                Ok(Send::<T, S>::new().0)
            }
            result => Err("Send<T, S>", result),
        }
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

impl<T: FromStr + marker::Send, S: FromStr + Session> FromStr for Recv<T, S> {
    type Err = SessionStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..4] {
            "Recv" => {
                let payload_continuation = get_blocks(s.to_string())?;
                let payload = T::from_str(&payload_continuation[0])?;
                let continuation = S::from_str(&payload_continuation[1])?;
                Ok(Recv::<T, S>::new().0)
            }
            result => Err("Recv<T, S>", result),
        }
    }
}

/// Separate the different _fields_ of a stringified type.
#[doc(hidden)]
fn get_blocks(full_block: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::new();
    let mut temp = String::from("");
    let mut index = -1;

    for i in full_block.chars() {
        if i == '&' {
        } else if i == '>' && index == 0 {
            result.push(format!("{}{}", temp, i));
            temp = String::from("");
        } else if i == '<' && index >= 0 {
            temp = format!("{}{}", temp, i);
            index += 1;
        } else if i == '>' && index >= 0 {
            temp = format!("{}{}", temp, i);
            index -= 1;
        } else if i == ',' && index == 0 {
            result.push(temp);
            temp = String::from("");
        } else if index >= 0 {
            temp = format!("{}{}", temp, i);
        } else if i == '<' {
            index += 1;
        } else if i == '>' {
            index -= 1;
        }
    }

    if !temp.is_empty() {
        result.push(temp);
    }

    Ok(result)
}
