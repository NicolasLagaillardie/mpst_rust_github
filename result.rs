#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
/// An example which mixes both the usual way of creating recv/send functions
/// with create_recv_mpst_session_bundle/create_send_mpst_session_bundle and the short way to
/// call the code within those functions with recv_mpst/send_mpst
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_normal_role_short, create_recv_mpst_session_bundle, create_send_mpst_session_bundle,
    offer_mpst, recv_mpst, send_mpst,
};
use std::error::Error;
#[must_use]
pub struct SessionMpstThree<S1, S2, R, N>
where
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
    N: mpstthree::role::Role,
{
    pub session1: S1,
    pub session2: S2,
    pub stack: R,
    pub name: N,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<
        S1: ::core::fmt::Debug,
        S2: ::core::fmt::Debug,
        R: ::core::fmt::Debug,
        N: ::core::fmt::Debug,
    > ::core::fmt::Debug for SessionMpstThree<S1, S2, R, N>
where
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
    N: mpstthree::role::Role,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            SessionMpstThree {
                session1: ref __self_0_0,
                session2: ref __self_0_1,
                stack: ref __self_0_2,
                name: ref __self_0_3,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "SessionMpstThree");
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "session1",
                    &&(*__self_0_0),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "session2",
                    &&(*__self_0_1),
                );
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "stack", &&(*__self_0_2));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0_3));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[doc(hidden)]
impl<
        S1: mpstthree::binary::struct_trait::Session,
        S2: mpstthree::binary::struct_trait::Session,
        R: mpstthree::role::Role,
        N: mpstthree::role::Role,
    > mpstthree::binary::struct_trait::Session for SessionMpstThree<S1, S2, R, N>
{
    type Dual = SessionMpstThree<
        <S1 as mpstthree::binary::struct_trait::Session>::Dual,
        <S2 as mpstthree::binary::struct_trait::Session>::Dual,
        <R as mpstthree::role::Role>::Dual,
        <N as mpstthree::role::Role>::Dual,
    >;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = S1::new();
        let (sender2, receiver2) = S2::new();
        let (role_one, role_two) = R::new();
        let (name_one, name_two) = N::new();
        (
            SessionMpstThree {
                session1: sender1,
                session2: sender2,
                stack: role_one,
                name: name_one,
            },
            SessionMpstThree {
                session1: receiver1,
                session2: receiver2,
                stack: role_two,
                name: name_two,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", " + ", " + ", " + "],
                &match (
                    &S1::head_str(),
                    &S2::head_str(),
                    &R::head_str(),
                    &N::head_str(),
                ) {
                    (arg0, arg1, arg2, arg3) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg3, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        }
    }
    #[doc(hidden)]
    fn tail_str() -> String {
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", " + ", " + ", " + "],
                &match (
                    &S1::tail_str(),
                    &S2::tail_str(),
                    &R::tail_str(),
                    &N::head_str(),
                ) {
                    (arg0, arg1, arg2, arg3) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg3, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        }
    }
}
fn close_mpst_multi<R>(
    s: SessionMpstThree<
        mpstthree::binary::struct_trait::End,
        mpstthree::binary::struct_trait::End,
        mpstthree::role::end::RoleEnd,
        R,
    >,
) -> Result<(), Box<dyn std::error::Error>>
where
    R: mpstthree::role::Role,
{
    s.session1
        .sender
        .send(mpstthree::binary::struct_trait::Signal::Stop)
        .unwrap_or(());
    s.session2
        .sender
        .send(mpstthree::binary::struct_trait::Signal::Stop)
        .unwrap_or(());
    s.session1.receiver.recv()?;
    s.session2.receiver.recv()?;
    Ok(())
}
fn fork_mpst<S1, S2, S3, R1, R2, R3, N1, N2, N3, F1, F2, F3>(
    f1: F1,
    f2: F2,
    f3: F3,
) -> (
    std::thread::JoinHandle<()>,
    std::thread::JoinHandle<()>,
    std::thread::JoinHandle<()>,
)
where
    R1: mpstthree::role::Role + 'static,
    R2: mpstthree::role::Role + 'static,
    R3: mpstthree::role::Role + 'static,
    N1: mpstthree::role::Role + 'static,
    N2: mpstthree::role::Role + 'static,
    N3: mpstthree::role::Role + 'static,
    S1: mpstthree::binary::struct_trait::Session + 'static,
    S2: mpstthree::binary::struct_trait::Session + 'static,
    S3: mpstthree::binary::struct_trait::Session + 'static,
    F1: FnOnce(SessionMpstThree<S1, S2, R1, N1>) -> Result<(), Box<dyn std::error::Error>>
        + std::marker::Send
        + 'static,
    F2: FnOnce(
            SessionMpstThree<<S1 as mpstthree::binary::struct_trait::Session>::Dual, S3, R2, N2>,
        ) -> Result<(), Box<dyn std::error::Error>>
        + std::marker::Send
        + 'static,
    F3: FnOnce(
            SessionMpstThree<
                <S2 as mpstthree::binary::struct_trait::Session>::Dual,
                <S3 as mpstthree::binary::struct_trait::Session>::Dual,
                R3,
                N3,
            >,
        ) -> Result<(), Box<dyn std::error::Error>>
        + std::marker::Send
        + 'static,
{
    let (channel_1_2, channel_2_1) = <S1 as mpstthree::binary::struct_trait::Session>::new();
    let (channel_1_3, channel_3_1) = <S2 as mpstthree::binary::struct_trait::Session>::new();
    let (channel_2_3, channel_3_2) = <S3 as mpstthree::binary::struct_trait::Session>::new();
    let (role_1, _) = R1::new();
    let (role_2, _) = R2::new();
    let (role_3, _) = R3::new();
    let (name_1, _) = N1::new();
    let (name_2, _) = N2::new();
    let (name_3, _) = N3::new();
    let sessionmpst_1 = SessionMpstThree {
        session1: channel_1_2,
        session2: channel_1_3,
        stack: role_1,
        name: name_1,
    };
    let sessionmpst_2 = SessionMpstThree {
        session1: channel_2_1,
        session2: channel_2_3,
        stack: role_2,
        name: name_2,
    };
    let sessionmpst_3 = SessionMpstThree {
        session1: channel_3_1,
        session2: channel_3_2,
        stack: role_3,
        name: name_3,
    };
    (
        std::thread::spawn(move || {
            std::panic::set_hook(Box::new(|_info| {}));
            match f1(sessionmpst_1) {
                Ok(()) => (),
                Err(e) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &[""],
                    &match (&e,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                    },
                )),
            }
        }),
        std::thread::spawn(move || {
            std::panic::set_hook(Box::new(|_info| {}));
            match f2(sessionmpst_2) {
                Ok(()) => (),
                Err(e) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &[""],
                    &match (&e,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                    },
                )),
            }
        }),
        std::thread::spawn(move || {
            std::panic::set_hook(Box::new(|_info| {}));
            match f3(sessionmpst_3) {
                Ok(()) => (),
                Err(e) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &[""],
                    &match (&e,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                    },
                )),
            }
        }),
    )
}
/// The Role
struct RoleA<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for RoleA<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            RoleA {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "RoleA");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Dual
struct RoleADual<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for RoleADual<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            RoleADual {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "RoleADual");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Role functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for RoleA<R> {
    type Dual = RoleADual<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            RoleA {
                sender: sender_dual,
            },
            RoleADual {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleA")
    }
    #[doc(hidden)]
    fn tail_str() -> String {
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", "<", ">"],
                &match (
                    &<R as mpstthree::role::Role>::head_str(),
                    &<R as mpstthree::role::Role>::tail_str(),
                ) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        }
    }
}
fn next_a<R>(r: RoleA<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Dual functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for RoleADual<R> {
    type Dual = RoleA<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            RoleADual {
                sender: sender_dual,
            },
            RoleA {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleADual")
    }
    #[doc(hidden)]
    fn tail_str() -> String {
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", "<", ">"],
                &match (
                    &<R as mpstthree::role::Role>::head_str(),
                    &<R as mpstthree::role::Role>::tail_str(),
                ) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        }
    }
}
fn next_a_dual<R>(r: RoleADual<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Role
struct RoleB<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for RoleB<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            RoleB {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "RoleB");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Dual
struct RoleBDual<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for RoleBDual<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            RoleBDual {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "RoleBDual");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Role functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for RoleB<R> {
    type Dual = RoleBDual<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            RoleB {
                sender: sender_dual,
            },
            RoleBDual {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleB")
    }
    #[doc(hidden)]
    fn tail_str() -> String {
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", "<", ">"],
                &match (
                    &<R as mpstthree::role::Role>::head_str(),
                    &<R as mpstthree::role::Role>::tail_str(),
                ) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        }
    }
}
fn next_b<R>(r: RoleB<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Dual functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for RoleBDual<R> {
    type Dual = RoleB<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            RoleBDual {
                sender: sender_dual,
            },
            RoleB {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleBDual")
    }
    #[doc(hidden)]
    fn tail_str() -> String {
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", "<", ">"],
                &match (
                    &<R as mpstthree::role::Role>::head_str(),
                    &<R as mpstthree::role::Role>::tail_str(),
                ) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        }
    }
}
fn next_b_dual<R>(r: RoleBDual<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Role
struct RoleC<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for RoleC<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            RoleC {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "RoleC");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Dual
struct RoleCDual<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for RoleCDual<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            RoleCDual {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "RoleCDual");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Role functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for RoleC<R> {
    type Dual = RoleCDual<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            RoleC {
                sender: sender_dual,
            },
            RoleCDual {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleC")
    }
    #[doc(hidden)]
    fn tail_str() -> String {
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", "<", ">"],
                &match (
                    &<R as mpstthree::role::Role>::head_str(),
                    &<R as mpstthree::role::Role>::tail_str(),
                ) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        }
    }
}
fn next_c<R>(r: RoleC<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Dual functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for RoleCDual<R> {
    type Dual = RoleC<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            RoleCDual {
                sender: sender_dual,
            },
            RoleC {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("RoleCDual")
    }
    #[doc(hidden)]
    fn tail_str() -> String {
        {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", "<", ">"],
                &match (
                    &<R as mpstthree::role::Role>::head_str(),
                    &<R as mpstthree::role::Role>::tail_str(),
                ) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        }
    }
}
fn next_c_dual<R>(r: RoleCDual<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type RS = Recv<(), Send<(), End>>;
type SR = Send<(), Recv<(), End>>;
type R2A<R> = RoleA<RoleA<R>>;
type R2B<R> = RoleB<RoleB<R>>;
type R2C<R> = RoleC<RoleC<R>>;
type StackRecurs = RoleA<RoleB<RoleEnd>>;
enum Branching0fromCtoA {
    More(SessionMpstThree<RS, Recv<(), Send<(), RecursAtoC>>, R2C<R2B<RoleC<RoleEnd>>>, NameA>),
    Done(SessionMpstThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = Recv<Branching0fromCtoA, End>;
enum Branching0fromCtoB {
    More(SessionMpstThree<SR, Recv<(), Send<(), RecursBtoC>>, R2C<R2A<RoleC<RoleEnd>>>, NameB>),
    Done(SessionMpstThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = Recv<Branching0fromCtoB, End>;
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoB = Send<Branching0fromCtoB, End>;
type EndpointA = SessionMpstThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = SessionMpstThree<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = SessionMpstThree<Choose0fromCtoA, Choose0fromCtoB, StackRecurs, NameC>;
fn send_mpst_c_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpstThree<
        mpstthree::binary::struct_trait::Send<T, S1>,
        S2,
        RoleA<R>,
        RoleC<mpstthree::role::end::RoleEnd>,
    >,
) -> SessionMpstThree<S1, S2, R, RoleC<mpstthree::role::end::RoleEnd>>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    {
        let new_session = mpstthree::binary::send::send(x, s.session1);
        let new_queue = next_a(s.stack);
        SessionMpstThree {
            session1: new_session,
            session2: s.session2,
            stack: new_queue,
            name: s.name,
        }
    }
}
fn send_mpst_c_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpstThree<
        S1,
        mpstthree::binary::struct_trait::Send<T, S2>,
        RoleB<R>,
        RoleC<mpstthree::role::end::RoleEnd>,
    >,
) -> SessionMpstThree<S1, S2, R, RoleC<mpstthree::role::end::RoleEnd>>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    {
        let new_session = mpstthree::binary::send::send(x, s.session2);
        let new_queue = next_b(s.stack);
        SessionMpstThree {
            session1: s.session1,
            session2: new_session,
            stack: new_queue,
            name: s.name,
        }
    }
}
fn recv_mpst_a_from_c<T, S1, S2, R>(
    s: SessionMpstThree<
        S1,
        mpstthree::binary::struct_trait::Recv<T, S2>,
        RoleC<R>,
        RoleA<mpstthree::role::end::RoleEnd>,
    >,
) -> Result<
    (
        T,
        SessionMpstThree<S1, S2, R, RoleA<mpstthree::role::end::RoleEnd>>,
    ),
    Box<dyn std::error::Error>,
>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    (|| -> Result<_, Box<dyn std::error::Error>> {
        let (v, new_session) = mpstthree::binary::recv::recv(s.session2)?;
        let new_queue = next_c(s.stack);
        let result = SessionMpstThree {
            session1: s.session1,
            session2: new_session,
            stack: new_queue,
            name: s.name,
        };
        Ok((v, result))
    })()
}
fn recv_mpst_b_from_c<T, S1, S2, R>(
    s: SessionMpstThree<
        S1,
        mpstthree::binary::struct_trait::Recv<T, S2>,
        RoleC<R>,
        RoleB<mpstthree::role::end::RoleEnd>,
    >,
) -> Result<
    (
        T,
        SessionMpstThree<S1, S2, R, RoleB<mpstthree::role::end::RoleEnd>>,
    ),
    Box<dyn std::error::Error>,
>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    (|| -> Result<_, Box<dyn std::error::Error>> {
        let (v, new_session) = mpstthree::binary::recv::recv(s.session2)?;
        let new_queue = next_c(s.stack);
        let result = SessionMpstThree {
            session1: s.session1,
            session2: new_session,
            stack: new_queue,
            name: s.name,
        };
        Ok((v, result))
    })()
}
type EndpointDoneC = SessionMpstThree<End, End, RoleEnd, NameC>;
type EndpointMoreC = SessionMpstThree<
    Send<(), Recv<(), Choose0fromCtoA>>,
    Send<(), Recv<(), Choose0fromCtoB>>,
    R2A<R2B<StackRecurs>>,
    NameC,
>;
fn done_from_c_to_all(
    s: SessionMpstThree<
        Send<Branching0fromCtoA, mpstthree::binary::struct_trait::End>,
        Send<Branching0fromCtoB, mpstthree::binary::struct_trait::End>,
        RoleA<RoleB<mpstthree::role::end::RoleEnd>>,
        RoleC<mpstthree::role::end::RoleEnd>,
    >,
) -> EndpointDoneC {
    let (channel_1_2, channel_2_1) = <_ as mpstthree::binary::struct_trait::Session>::new();
    let (channel_1_3, channel_3_1) = <_ as mpstthree::binary::struct_trait::Session>::new();
    let (channel_2_3, channel_3_2) = <_ as mpstthree::binary::struct_trait::Session>::new();
    let (stack_1, _) = <_ as mpstthree::role::Role>::new();
    let (stack_2, _) = <_ as mpstthree::role::Role>::new();
    let (stack_3, _) = <_ as mpstthree::role::Role>::new();
    let (name_1, _) = <RoleA<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
    let (name_2, _) = <RoleB<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
    let (name_3, _) = <RoleC<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
    let s = send_mpst_c_to_a(
        Branching0fromCtoA::Done(SessionMpstThree {
            session1: channel_1_2,
            session2: channel_1_3,
            stack: stack_1,
            name: name_1,
        }),
        s,
    );
    let s = send_mpst_c_to_b(
        Branching0fromCtoB::Done(SessionMpstThree {
            session1: channel_2_1,
            session2: channel_2_3,
            stack: stack_2,
            name: name_2,
        }),
        s,
    );
    mpstthree::binary::cancel::cancel(s);
    SessionMpstThree {
        session1: channel_3_1,
        session2: channel_3_2,
        stack: stack_3,
        name: name_3,
    }
}
fn more_from_c_to_all(
    s: SessionMpstThree<
        Send<Branching0fromCtoA, mpstthree::binary::struct_trait::End>,
        Send<Branching0fromCtoB, mpstthree::binary::struct_trait::End>,
        RoleA<RoleB<mpstthree::role::end::RoleEnd>>,
        RoleC<mpstthree::role::end::RoleEnd>,
    >,
) -> EndpointMoreC {
    let (channel_1_2, channel_2_1) = <_ as mpstthree::binary::struct_trait::Session>::new();
    let (channel_1_3, channel_3_1) = <_ as mpstthree::binary::struct_trait::Session>::new();
    let (channel_2_3, channel_3_2) = <_ as mpstthree::binary::struct_trait::Session>::new();
    let (stack_1, _) = <_ as mpstthree::role::Role>::new();
    let (stack_2, _) = <_ as mpstthree::role::Role>::new();
    let (stack_3, _) = <_ as mpstthree::role::Role>::new();
    let (name_1, _) = <RoleA<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
    let (name_2, _) = <RoleB<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
    let (name_3, _) = <RoleC<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
    let s = send_mpst_c_to_a(
        Branching0fromCtoA::More(SessionMpstThree {
            session1: channel_1_2,
            session2: channel_1_3,
            stack: stack_1,
            name: name_1,
        }),
        s,
    );
    let s = send_mpst_c_to_b(
        Branching0fromCtoB::More(SessionMpstThree {
            session1: channel_2_1,
            session2: channel_2_3,
            stack: stack_2,
            name: name_2,
        }),
        s,
    );
    mpstthree::binary::cancel::cancel(s);
    SessionMpstThree {
        session1: channel_3_1,
        session2: channel_3_2,
        stack: stack_3,
        name: name_3,
    }
}
fn simple_five_endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    (move || -> Result<_, _> {
        let (l, s) = recv_mpst_a_from_c(s)?;
        mpstthree::binary::cancel::cancel(s);
        match l {
            Branching0fromCtoA::Done(s) => close_mpst_multi(s),
            Branching0fromCtoA::More(s) => {
                let (_, s) = (|| -> Result<_, Box<dyn std::error::Error>> {
                    let (v, new_session) = mpstthree::binary::recv::recv(s.session2)?;
                    let new_queue = next_c(s.stack);
                    let result = SessionMpstThree {
                        session1: s.session1,
                        session2: new_session,
                        stack: new_queue,
                        name: s.name,
                    };
                    Ok((v, result))
                })()?;
                let s = {
                    let new_session = mpstthree::binary::send::send((), s.session2);
                    let new_queue = next_c(s.stack);
                    SessionMpstThree {
                        session1: s.session1,
                        session2: new_session,
                        stack: new_queue,
                        name: s.name,
                    }
                };
                let (_, s) = (|| -> Result<_, Box<dyn std::error::Error>> {
                    let (v, new_session) = mpstthree::binary::recv::recv(s.session1)?;
                    let new_queue = next_b(s.stack);
                    let result = SessionMpstThree {
                        session1: new_session,
                        session2: s.session2,
                        stack: new_queue,
                        name: s.name,
                    };
                    Ok((v, result))
                })()?;
                let s = {
                    let new_session = mpstthree::binary::send::send((), s.session1);
                    let new_queue = next_b(s.stack);
                    SessionMpstThree {
                        session1: new_session,
                        session2: s.session2,
                        stack: new_queue,
                        name: s.name,
                    }
                };
                simple_five_endpoint_a(s)
            }
        }
    })()
}
fn simple_five_endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    (move || -> Result<_, _> {
        let (l, s) = recv_mpst_b_from_c(s)?;
        mpstthree::binary::cancel::cancel(s);
        match l {
            Branching0fromCtoB::Done(s) => close_mpst_multi(s),
            Branching0fromCtoB::More(s) => {
                let (_, s) = (|| -> Result<_, Box<dyn std::error::Error>> {
                    let (v, new_session) = mpstthree::binary::recv::recv(s.session2)?;
                    let new_queue = next_c(s.stack);
                    let result = SessionMpstThree {
                        session1: s.session1,
                        session2: new_session,
                        stack: new_queue,
                        name: s.name,
                    };
                    Ok((v, result))
                })()?;
                let s = {
                    let new_session = mpstthree::binary::send::send((), s.session2);
                    let new_queue = next_c(s.stack);
                    SessionMpstThree {
                        session1: s.session1,
                        session2: new_session,
                        stack: new_queue,
                        name: s.name,
                    }
                };
                let s = {
                    let new_session = mpstthree::binary::send::send((), s.session1);
                    let new_queue = next_a(s.stack);
                    SessionMpstThree {
                        session1: new_session,
                        session2: s.session2,
                        stack: new_queue,
                        name: s.name,
                    }
                };
                let (_, s) = (|| -> Result<_, Box<dyn std::error::Error>> {
                    let (v, new_session) = mpstthree::binary::recv::recv(s.session1)?;
                    let new_queue = next_a(s.stack);
                    let result = SessionMpstThree {
                        session1: new_session,
                        session2: s.session2,
                        stack: new_queue,
                        name: s.name,
                    };
                    Ok((v, result))
                })()?;
                simple_five_endpoint_b(s)
            }
        }
    })()
}
fn simple_five_endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, SIZE)
}
fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_c_to_all(s);
            close_mpst_multi(s)
        }
        i => {
            let s = more_from_c_to_all(s);
            let s = {
                let new_session = mpstthree::binary::send::send((), s.session1);
                let new_queue = next_a(s.stack);
                SessionMpstThree {
                    session1: new_session,
                    session2: s.session2,
                    stack: new_queue,
                    name: s.name,
                }
            };
            let (_, s) = (|| -> Result<_, Box<dyn std::error::Error>> {
                let (v, new_session) = mpstthree::binary::recv::recv(s.session1)?;
                let new_queue = next_a(s.stack);
                let result = SessionMpstThree {
                    session1: new_session,
                    session2: s.session2,
                    stack: new_queue,
                    name: s.name,
                };
                Ok((v, result))
            })()?;
            let s = {
                let new_session = mpstthree::binary::send::send((), s.session2);
                let new_queue = next_b(s.stack);
                SessionMpstThree {
                    session1: s.session1,
                    session2: new_session,
                    stack: new_queue,
                    name: s.name,
                }
            };
            let (_, s) = (|| -> Result<_, Box<dyn std::error::Error>> {
                let (v, new_session) = mpstthree::binary::recv::recv(s.session2)?;
                let new_queue = next_b(s.stack);
                let result = SessionMpstThree {
                    session1: s.session1,
                    session2: new_session,
                    stack: new_queue,
                    name: s.name,
                };
                Ok((v, result))
            })()?;
            recurs_c(s, i - 1)
        }
    }
}
fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        simple_five_endpoint_a,
        simple_five_endpoint_b,
        simple_five_endpoint_c,
    );
    thread_a.join()?;
    thread_b.join()?;
    thread_c.join()?;
    Ok(())
}
static SIZE: i64 = 15;
#[allow(dead_code)]
fn main() {
    if !all_mpst().is_ok() {
        ::core::panicking::panic("assertion failed: all_mpst().is_ok()")
    };
}
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
