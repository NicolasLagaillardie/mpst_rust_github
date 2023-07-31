//! This feature only includes a message structure.
//! It is parameterised with a label and a payload.

use std::marker::Send;

/// A structure to represent a message as it should be
/// in session types, with a label and a payload.
/// The types of those parameters are not fixed but
/// they must be trasnferable through threads.
/// Nothing else is implemented with this structure.
///
/// # Example
///
/// ```
/// use mpstthree::message::Message;
///
/// let _ = Message {
///     label: String::from("label"),
///     payload: 0,
/// };
/// ```
#[must_use]
#[derive(Debug)]
pub struct Message<L, T>
where
    T: Send,
    L: Send,
{
    #[doc(hidden)]
    pub label: L,
    #[doc(hidden)]
    pub payload: T,
}
