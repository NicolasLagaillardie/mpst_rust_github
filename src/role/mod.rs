//! The main trait used for representing an ordering or the
//! name of a participant.
//!
//! Every structure that relies on this trait, such as
//! [`mpstthree::role::a::RoleA`] and
//! [`mpstthree::role::b::RoleB`], contains at
//! least a parameter, which is a [`mpstthree::role::Role`]
//! itself. The only exception is
//! [`mpstthree::role::end::RoleEnd`], which represents
//! the end of any ordering.
//!
//! [`mpstthree::role::Role`]: ../role/trait.Role.html
//! [`mpstthree::role::end::RoleEnd`]: ../role/end/struct.RoleEnd.html
//! [`mpstthree::role::a::RoleA`]: ../role/a/struct.RoleA.html
//! [`mpstthree::role::b::RoleB`]:  /role/b/struct.RoleB.html

pub mod a;
pub mod a_dual;
pub mod a_to_all;
pub mod all_to_a;
pub mod all_to_b;
pub mod all_to_c;
pub mod b;
pub mod b_dual;
pub mod b_to_all;
pub mod broadcast;
pub mod c;
pub mod c_dual;
pub mod c_to_all;
pub mod end;
use std::marker;

/// Trait for session types. Provides duality.
pub trait Role: marker::Sized + marker::Send {
    /// The Role type dual to `Self`.
    type Dual: Role<Dual = Self>;

    // Creates two new *dual* roles.
    //
    // The `new` function is used internally in this library to
    // define functions such as
    // [`mpstthree::fork::fork_simple`]. The `Dual` is often
    // unused, but may be necessary for specific cases, such as
    // closing a connection.
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual);

    #[doc(hidden)]
    fn head_str() -> String;

    #[doc(hidden)]
    fn tail_str() -> String;
}
