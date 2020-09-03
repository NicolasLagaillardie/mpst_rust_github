use binary::Session;
use role::Role;

/// A `struct` which encapsulates two binary session types and a queue.
///
/// This `struct` is the main one used in this library.
/// Each process is linked to the others with one `Session`,
/// and the order of the operations is given by the queue composed of `Role`.
#[must_use]
#[derive(Debug)]
pub struct SessionMpst<S1: Session, S2: Session, R: Role> {
    pub session1: S1,
    pub session2: S2,
    pub stack: R,
}

#[doc(hidden)]
impl<S1: Session, S2: Session, R: Role> Session for SessionMpst<S1, S2, R> {
    type Dual = SessionMpst<<S1 as Session>::Dual, <S2 as Session>::Dual, <R as Role>::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = S1::new();
        let (sender_two, receiver_two) = S2::new();

        let (role_one, role_two) = R::new();

        (
            SessionMpst {
                session1: sender_one,
                session2: sender_two,
                stack: role_one,
            },
            SessionMpst {
                session1: receiver_one,
                session2: receiver_two,
                stack: role_two,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        format!(
            "{} + {} + {}",
            S1::head_str(),
            S2::head_str(),
            R::head_str()
        )
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!(
            "{} + {} + {}",
            S1::tail_str(),
            S2::tail_str(),
            R::tail_str()
        )
    }
}

// Macro doesn't work: current created functions expect mpstthree::sessionmpst::SessionMpst, and not SessionMpst
#[macro_export]
macro_rules! create_sessionmpst {
    ($struct_name:ident, $($session_name: ident, $session_type: ident, )*) => {
        ////////////////////////////////////////////
        /// The SessionMpst

        #[must_use]
        #[derive(Debug)]
        struct $struct_name<$($session_type: Session, )* R: Role> {
            $(
                pub $session_name: $session_type,
            )*
            pub stack: R,
        }

        ////////////////////////////////////////////
        /// The SessionMpst functions

        #[doc(hidden)]
        impl<$($session_type: Session, )* R: Role> Session for $struct_name<$($session_type, )* R> {
            type Dual =
                $struct_name<$(<$session_type as Session>::Dual, )* <R as Role>::Dual>;

            #[doc(hidden)]
            fn new() -> (Self, Self::Dual) {

                let (role_one, role_two) = R::new();

                // Issue with no link between the two new SessionMpst
                (
                    $struct_name {
                        $(
                            $session_name: {
                                let (sender, _) = $session_type::new();
                                sender
                            },
                        )*
                        stack: role_one,
                    },
                    $struct_name {
                        $(
                            $session_name: {
                                let (_, receiver) = $session_type::new();
                                receiver
                            },
                        )*
                        stack: role_two,
                    }
                )
            }

            #[doc(hidden)]
            fn head_str() -> String {
                format!(
                    "{} + {} + {}",
                    S1::head_str(),
                    S2::head_str(),
                    R::head_str()
                )
            }

            #[doc(hidden)]
            fn tail_str() -> String {
                format!(
                    "{} + {} + {}",
                    S1::tail_str(),
                    S2::tail_str(),
                    R::tail_str()
                )
            }
        }
    };
}

// macro_rules! new_struct {
//     // input is empty: time to output
//     (@munch () -> {$(#[$attr:meta])* struct $name:ident $(($id:ident: $ty:ty))*}) => {
//         $(#[$attr])* struct $name { $($id: $ty),* }
//     };

//     // branch off to generate an inner struct
//     (@munch ($id:ident: struct $name:ident {$($inner:tt)*} $($next:tt)*) -> {$(#[$attr:meta])* struct $($output:tt)*}) => {
//         new_struct!(@munch ($($inner)*) -> {$(#[$attr])* struct $name});
//         new_struct!(@munch ($($next)*) -> {$(#[$attr])* struct $($output)* ($id: $name)});
//     };

//     // throw on the last field
//     (@munch ($id:ident: $ty:ty) -> {$($output:tt)*}) => {
//         new_struct!(@munch () -> {$($output)* ($id: $ty)});
//     };

//     // throw on another field (not the last one)
//     (@munch ($id:ident: $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
//         new_struct!(@munch ($($next)*) -> {$($output)* ($id: $ty)});
//     };

//     // entry point (this is where a macro call starts)
//     ($(#[$attr:meta])* struct $name:ident { $($input:tt)*} ) => {
//         new_struct!(@munch ($($input)*) -> {$(#[$attr])* struct $name});
//         //                 ^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
//         //                     input       output
//     }
// }

// new_struct! {
//     #[derive(Debug)]
//     struct Foo {
//         foo: i32,
//         bar: struct Bar {
//             bar: i32,
//             foobar: i64
//         }
//     }
// }

// fn main() {
//     println!("{:#?}", Foo { foo: 1, bar: Bar { bar: 2, foobar: 3 } });
// }
