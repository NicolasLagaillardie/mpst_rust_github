pub mod a_to_all;
pub mod a_to_b;
pub mod a_to_c;
pub mod all_to_a;
pub mod all_to_b;
pub mod all_to_c;
pub mod b_to_a;
pub mod b_to_all;
pub mod b_to_c;
pub mod c_to_a;
pub mod c_to_all;
pub mod c_to_b;
pub mod end;
use std::marker;
// use downcast_rs::Downcast;

/// Trait for session types. Provides duality.
// pub trait Role: marker::Sized + marker::Send + Downcast {
pub trait Role: marker::Sized + marker::Send {
    /// The Role type dual to `Self`.
    type Dual: Role<Dual = Self>;

    /// Creates two new *dual* roles.
    ///
    /// The `new` function is used internally in this library to define
    /// functions such as `fork_simple`. The `Dual` is often unused,
    /// but may be necessary for specific cases, such as closing a connection.
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual);

    #[doc(hidden)]
    fn head() -> String;
}

pub trait RoleA: marker::Sized + marker::Send {
    /// The Role type dual to `Self`.
    type Dual: RoleA<Dual = Self>;

    /// Creates two new *dual* roles.
    ///
    /// The `new` function is used internally in this library to define
    /// functions such as `fork_simple`. The `Dual` is often unused,
    /// but may be necessary for specific cases, such as closing a connection.
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual);

    #[doc(hidden)]
    fn head() -> String;
}

pub trait RoleB: marker::Sized + marker::Send {
    /// The Role type dual to `Self`.
    type Dual: RoleB<Dual = Self>;

    /// Creates two new *dual* roles.
    ///
    /// The `new` function is used internally in this library to define
    /// functions such as `fork_simple`. The `Dual` is often unused,
    /// but may be necessary for specific cases, such as closing a connection.
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual);

    #[doc(hidden)]
    fn head() -> String;
}

pub trait RoleC: marker::Sized + marker::Send {
    /// The Role type dual to `Self`.
    type Dual: RoleC<Dual = Self>;

    /// Creates two new *dual* roles.
    ///
    /// The `new` function is used internally in this library to define
    /// functions such as `fork_simple`. The `Dual` is often unused,
    /// but may be necessary for specific cases, such as closing a connection.
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual);

    #[doc(hidden)]
    fn head() -> String;
}

// impl<R: RoleA> Role for R {
//     type Dual = RoleA<R::Dual>;

//     #[doc(hidden)]
//     fn new() -> (Self, Self::Dual) {
//         let (sender_normal, _) = bounded::<R>(1);
//         let (sender_dual, _) = bounded::<R::Dual>(1);

//         (
//             RoleCtoB {
//                 sender: sender_dual,
//             },
//             RoleBtoC {
//                 sender: sender_normal,
//             },
//         )
//     }

//     #[doc(hidden)]
//     fn head() -> String {
//         String::from("RoleA")
//     }
// }

// pub trait RoleAll: Role {}

// pub trait RoleEnd: Role {}
