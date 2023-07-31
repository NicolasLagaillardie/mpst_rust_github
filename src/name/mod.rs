#![cfg(feature = "mpst")]

//! The main trait used for representing the
//! name of a participant.
//!
//! Every structure that relies on this trait, such as
//! [`RoleA`] and
//! [`RoleB`], contains at
//! least a parameter, which is a [`Role`]
//! itself. The only exception is
//! [`RoleEnd`], which represents
//! the end of any ordering.
//!
//! [`Role`]: crate::role::Role
//! [`RoleEnd`]: crate::role::end::RoleEnd
//! [`RoleA`]: crate::role::a::RoleA
//! [`RoleB`]: crate::role::b::RoleB

pub mod a;
pub mod b;
pub mod c;
use std::marker;

/// Trait for session types. Provides duality.
pub trait Name: marker::Sized + marker::Send {
    /// The Name type dual to `Self`.
    type Dual: Name;

    // Creates two new *dual* roles.
    //
    // The `new` function is used internally in this library to
    // define functions such as
    // [`fork_simple`](crate::fork::fork_simple). The `Dual` is often
    // unused, but may be necessary for specific cases, such as
    // closing a connection.
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
