//! This module contains the trait for session types. Provides duality.

use std::marker;

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

    #[doc(hidden)]
    fn self_head_str(&self) -> String;

    #[doc(hidden)]
    fn self_tail_str(&self) -> String;
}
