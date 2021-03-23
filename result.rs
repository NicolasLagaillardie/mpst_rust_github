#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    choose_mpst_multi_to_all, close_mpst, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, create_sessionmpst,
    fork_mpst_multi, offer_mpst,
};
use mpstthree::role::broadcast::RoleBroadcast;
use rand::{random, thread_rng, Rng};
use std::error::Error;
use std::marker;
#[must_use]
pub struct SessionMpstFour<S1, S2, S3, R, N>
where
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    S3: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
    N: mpstthree::role::Role,
{
    pub session1: S1,
    pub session2: S2,
    pub session3: S3,
    pub stack: R,
    pub name: N,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<
        S1: ::core::fmt::Debug,
        S2: ::core::fmt::Debug,
        S3: ::core::fmt::Debug,
        R: ::core::fmt::Debug,
        N: ::core::fmt::Debug,
    > ::core::fmt::Debug for SessionMpstFour<S1, S2, S3, R, N>
where
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    S3: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
    N: mpstthree::role::Role,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            SessionMpstFour {
                session1: ref __self_0_0,
                session2: ref __self_0_1,
                session3: ref __self_0_2,
                stack: ref __self_0_3,
                name: ref __self_0_4,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "SessionMpstFour");
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
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "session3",
                    &&(*__self_0_2),
                );
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "stack", &&(*__self_0_3));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "name", &&(*__self_0_4));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[doc(hidden)]
impl<
        S1: mpstthree::binary::struct_trait::Session,
        S2: mpstthree::binary::struct_trait::Session,
        S3: mpstthree::binary::struct_trait::Session,
        R: mpstthree::role::Role,
        N: mpstthree::role::Role,
    > mpstthree::binary::struct_trait::Session for SessionMpstFour<S1, S2, S3, R, N>
{
    type Dual = SessionMpstFour<
        <S1 as mpstthree::binary::struct_trait::Session>::Dual,
        <S2 as mpstthree::binary::struct_trait::Session>::Dual,
        <S3 as mpstthree::binary::struct_trait::Session>::Dual,
        <R as mpstthree::role::Role>::Dual,
        <N as mpstthree::role::Role>::Dual,
    >;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = S1::new();
        let (sender2, receiver2) = S2::new();
        let (sender3, receiver3) = S3::new();
        let (role_one, role_two) = R::new();
        let (name_one, name_two) = N::new();
        (
            SessionMpstFour {
                session1: sender1,
                session2: sender2,
                session3: sender3,
                stack: role_one,
                name: name_one,
            },
            SessionMpstFour {
                session1: receiver1,
                session2: receiver2,
                session3: receiver3,
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
/// The Role
struct Api<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for Api<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Api {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Api");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Dual
struct DualAPI<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for DualAPI<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            DualAPI {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "DualAPI");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Role functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for Api<R> {
    type Dual = DualAPI<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            Api {
                sender: sender_dual,
            },
            DualAPI {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("DualAPI")
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
fn next_api<R>(r: Api<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Dual functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for DualAPI<R> {
    type Dual = Api<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            DualAPI {
                sender: sender_dual,
            },
            Api {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("Api")
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
fn next_dual_api<R>(r: DualAPI<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Role
struct Controller<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for Controller<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Controller {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "Controller");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Dual
struct DualController<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for DualController<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            DualController {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "DualController");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Role functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for Controller<R> {
    type Dual = DualController<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            Controller {
                sender: sender_dual,
            },
            DualController {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("DualController")
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
fn next_controller<R>(r: Controller<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Dual functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for DualController<R> {
    type Dual = Controller<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            DualController {
                sender: sender_dual,
            },
            Controller {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("Controller")
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
fn next_dual_controller<R>(r: DualController<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Role
struct Logs<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for Logs<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Logs {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Logs");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Dual
struct DualLogs<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for DualLogs<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            DualLogs {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "DualLogs");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Role functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for Logs<R> {
    type Dual = DualLogs<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            Logs {
                sender: sender_dual,
            },
            DualLogs {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("DualLogs")
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
fn next_logs<R>(r: Logs<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Dual functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for DualLogs<R> {
    type Dual = Logs<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            DualLogs {
                sender: sender_dual,
            },
            Logs {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("Logs")
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
fn next_dual_logs<R>(r: DualLogs<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Role
struct Storage<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for Storage<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Storage {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Storage");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Dual
struct DualStorage<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
{
    sender: crossbeam_channel::Sender<R::Dual>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<R: ::core::fmt::Debug> ::core::fmt::Debug for DualStorage<R>
where
    R: mpstthree::role::Role,
    R::Dual: mpstthree::role::Role,
    R::Dual: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            DualStorage {
                sender: ref __self_0_0,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "DualStorage");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sender", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
/// The Role functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for Storage<R> {
    type Dual = DualStorage<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            Storage {
                sender: sender_dual,
            },
            DualStorage {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("DualStorage")
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
fn next_storage<R>(r: Storage<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
/// The Dual functions
impl<R: mpstthree::role::Role> mpstthree::role::Role for DualStorage<R> {
    type Dual = Storage<<R as mpstthree::role::Role>::Dual>;
    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
        (
            DualStorage {
                sender: sender_dual,
            },
            Storage {
                sender: sender_normal,
            },
        )
    }
    #[doc(hidden)]
    fn head_str() -> String {
        String::from("Storage")
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
fn next_dual_storage<R>(r: DualStorage<R>) -> R
where
    R: mpstthree::role::Role,
{
    let (here, there) = <R as mpstthree::role::Role>::new();
    r.sender.send(there).unwrap_or(());
    here
}
fn send_start_controller_to_logs<T, S1, S2, S3, R>(
    x: T,
    s: SessionMpstFour<
        S1,
        mpstthree::binary::struct_trait::Send<T, S2>,
        S3,
        Logs<R>,
        Controller<mpstthree::role::end::RoleEnd>,
    >,
) -> SessionMpstFour<S1, S2, S3, R, Controller<mpstthree::role::end::RoleEnd>>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    S3: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    {
        let new_session = mpstthree::binary::send::send(x, s.session2);
        let new_queue = {
            fn temp<R>(r: Logs<R>) -> R
            where
                R: mpstthree::role::Role,
            {
                let (here, there) = <R as mpstthree::role::Role>::new();
                r.sender.send(there).unwrap_or(());
                here
            }
            temp(s.stack)
        };
        SessionMpstFour {
            session1: s.session1,
            session2: new_session,
            session3: s.session3,
            stack: new_queue,
            name: s.name,
        }
    }
}
fn send_logs_to_api<T, S1, S2, S3, R>(
    x: T,
    s: SessionMpstFour<
        mpstthree::binary::struct_trait::Send<T, S1>,
        S2,
        S3,
        Api<R>,
        Logs<mpstthree::role::end::RoleEnd>,
    >,
) -> SessionMpstFour<S1, S2, S3, R, Logs<mpstthree::role::end::RoleEnd>>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    S3: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    {
        let new_session = mpstthree::binary::send::send(x, s.session1);
        let new_queue = {
            fn temp<R>(r: Api<R>) -> R
            where
                R: mpstthree::role::Role,
            {
                let (here, there) = <R as mpstthree::role::Role>::new();
                r.sender.send(there).unwrap_or(());
                here
            }
            temp(s.stack)
        };
        SessionMpstFour {
            session1: new_session,
            session2: s.session2,
            session3: s.session3,
            stack: new_queue,
            name: s.name,
        }
    }
}
fn send_failure_logs_to_controller<T, S1, S2, S3, R>(
    x: T,
    s: SessionMpstFour<
        S1,
        mpstthree::binary::struct_trait::Send<T, S2>,
        S3,
        Controller<R>,
        Logs<mpstthree::role::end::RoleEnd>,
    >,
) -> SessionMpstFour<S1, S2, S3, R, Logs<mpstthree::role::end::RoleEnd>>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    S3: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    {
        let new_session = mpstthree::binary::send::send(x, s.session2);
        let new_queue = {
            fn temp<R>(r: Controller<R>) -> R
            where
                R: mpstthree::role::Role,
            {
                let (here, there) = <R as mpstthree::role::Role>::new();
                r.sender.send(there).unwrap_or(());
                here
            }
            temp(s.stack)
        };
        SessionMpstFour {
            session1: s.session1,
            session2: new_session,
            session3: s.session3,
            stack: new_queue,
            name: s.name,
        }
    }
}
fn send_logs_to_storage<T, S1, S2, S3, R>(
    x: T,
    s: SessionMpstFour<
        S1,
        S2,
        mpstthree::binary::struct_trait::Send<T, S3>,
        Storage<R>,
        Logs<mpstthree::role::end::RoleEnd>,
    >,
) -> SessionMpstFour<S1, S2, S3, R, Logs<mpstthree::role::end::RoleEnd>>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    S3: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    {
        let new_session = mpstthree::binary::send::send(x, s.session3);
        let new_queue = {
            fn temp<R>(r: Storage<R>) -> R
            where
                R: mpstthree::role::Role,
            {
                let (here, there) = <R as mpstthree::role::Role>::new();
                r.sender.send(there).unwrap_or(());
                here
            }
            temp(s.stack)
        };
        SessionMpstFour {
            session1: s.session1,
            session2: s.session2,
            session3: new_session,
            stack: new_queue,
            name: s.name,
        }
    }
}
fn recv_api_from_logs<T, S1, S2, S3, R>(
    s: SessionMpstFour<
        S1,
        mpstthree::binary::struct_trait::Recv<T, S2>,
        S3,
        Logs<R>,
        Api<mpstthree::role::end::RoleEnd>,
    >,
) -> Result<
    (
        T,
        SessionMpstFour<S1, S2, S3, R, Api<mpstthree::role::end::RoleEnd>>,
    ),
    Box<dyn std::error::Error>,
>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    S3: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    (|| -> Result<_, Box<dyn std::error::Error>> {
        let (v, new_session) = mpstthree::binary::recv::recv(s.session2)?;
        let new_queue = {
            fn temp<R>(r: Logs<R>) -> R
            where
                R: mpstthree::role::Role,
            {
                let (here, there) = <R as mpstthree::role::Role>::new();
                r.sender.send(there).unwrap_or(());
                here
            }
            temp(s.stack)
        };
        Ok((
            v,
            SessionMpstFour {
                session1: s.session1,
                session2: new_session,
                session3: s.session3,
                stack: new_queue,
                name: s.name,
            },
        ))
    })()
}
fn recv_start_controller_from_logs<T, S1, S2, S3, R>(
    s: SessionMpstFour<
        S1,
        mpstthree::binary::struct_trait::Recv<T, S2>,
        S3,
        Logs<R>,
        Controller<mpstthree::role::end::RoleEnd>,
    >,
) -> Result<
    (
        T,
        SessionMpstFour<S1, S2, S3, R, Controller<mpstthree::role::end::RoleEnd>>,
    ),
    Box<dyn std::error::Error>,
>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    S3: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    (|| -> Result<_, Box<dyn std::error::Error>> {
        let (v, new_session) = mpstthree::binary::recv::recv(s.session2)?;
        let new_queue = {
            fn temp<R>(r: Logs<R>) -> R
            where
                R: mpstthree::role::Role,
            {
                let (here, there) = <R as mpstthree::role::Role>::new();
                r.sender.send(there).unwrap_or(());
                here
            }
            temp(s.stack)
        };
        Ok((
            v,
            SessionMpstFour {
                session1: s.session1,
                session2: new_session,
                session3: s.session3,
                stack: new_queue,
                name: s.name,
            },
        ))
    })()
}
fn recv_start_logs_from_controller<T, S1, S2, S3, R>(
    s: SessionMpstFour<
        S1,
        mpstthree::binary::struct_trait::Recv<T, S2>,
        S3,
        Controller<R>,
        Logs<mpstthree::role::end::RoleEnd>,
    >,
) -> Result<
    (
        T,
        SessionMpstFour<S1, S2, S3, R, Logs<mpstthree::role::end::RoleEnd>>,
    ),
    Box<dyn std::error::Error>,
>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    S3: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    (|| -> Result<_, Box<dyn std::error::Error>> {
        let (v, new_session) = mpstthree::binary::recv::recv(s.session2)?;
        let new_queue = {
            fn temp<R>(r: Controller<R>) -> R
            where
                R: mpstthree::role::Role,
            {
                let (here, there) = <R as mpstthree::role::Role>::new();
                r.sender.send(there).unwrap_or(());
                here
            }
            temp(s.stack)
        };
        Ok((
            v,
            SessionMpstFour {
                session1: s.session1,
                session2: new_session,
                session3: s.session3,
                stack: new_queue,
                name: s.name,
            },
        ))
    })()
}
fn recv_storage_from_logs<T, S1, S2, S3, R>(
    s: SessionMpstFour<
        S1,
        S2,
        mpstthree::binary::struct_trait::Recv<T, S3>,
        Logs<R>,
        Storage<mpstthree::role::end::RoleEnd>,
    >,
) -> Result<
    (
        T,
        SessionMpstFour<S1, S2, S3, R, Storage<mpstthree::role::end::RoleEnd>>,
    ),
    Box<dyn std::error::Error>,
>
where
    T: std::marker::Send,
    S1: mpstthree::binary::struct_trait::Session,
    S2: mpstthree::binary::struct_trait::Session,
    S3: mpstthree::binary::struct_trait::Session,
    R: mpstthree::role::Role,
{
    (|| -> Result<_, Box<dyn std::error::Error>> {
        let (v, new_session) = mpstthree::binary::recv::recv(s.session3)?;
        let new_queue = {
            fn temp<R>(r: Logs<R>) -> R
            where
                R: mpstthree::role::Role,
            {
                let (here, there) = <R as mpstthree::role::Role>::new();
                r.sender.send(there).unwrap_or(());
                here
            }
            temp(s.stack)
        };
        Ok((
            v,
            SessionMpstFour {
                session1: s.session1,
                session2: s.session2,
                session3: new_session,
                stack: new_queue,
                name: s.name,
            },
        ))
    })()
}
fn close_mpst_multi<R>(
    s: SessionMpstFour<
        mpstthree::binary::struct_trait::End,
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
    s.session3
        .sender
        .send(mpstthree::binary::struct_trait::Signal::Stop)
        .unwrap_or(());
    s.session1.receiver.recv()?;
    s.session2.receiver.recv()?;
    s.session3.receiver.recv()?;
    Ok(())
}
fn fork_mpst<S1, S2, S3, S4, S5, S6, R1, R2, R3, R4, N1, N2, N3, N4, F1, F2, F3, F4>(
    f1: F1,
    f2: F2,
    f3: F3,
    f4: F4,
) -> (
    std::thread::JoinHandle<()>,
    std::thread::JoinHandle<()>,
    std::thread::JoinHandle<()>,
    std::thread::JoinHandle<()>,
)
where
    R1: mpstthree::role::Role + 'static,
    R2: mpstthree::role::Role + 'static,
    R3: mpstthree::role::Role + 'static,
    R4: mpstthree::role::Role + 'static,
    N1: mpstthree::role::Role + 'static,
    N2: mpstthree::role::Role + 'static,
    N3: mpstthree::role::Role + 'static,
    N4: mpstthree::role::Role + 'static,
    S1: mpstthree::binary::struct_trait::Session + 'static,
    S2: mpstthree::binary::struct_trait::Session + 'static,
    S3: mpstthree::binary::struct_trait::Session + 'static,
    S4: mpstthree::binary::struct_trait::Session + 'static,
    S5: mpstthree::binary::struct_trait::Session + 'static,
    S6: mpstthree::binary::struct_trait::Session + 'static,
    F1: FnOnce(SessionMpstFour<S1, S2, S3, R1, N1>) -> Result<(), Box<dyn std::error::Error>>
        + std::marker::Send
        + 'static,
    F2: FnOnce(
            SessionMpstFour<<S1 as mpstthree::binary::struct_trait::Session>::Dual, S4, S5, R2, N2>,
        ) -> Result<(), Box<dyn std::error::Error>>
        + std::marker::Send
        + 'static,
    F3: FnOnce(
            SessionMpstFour<
                <S2 as mpstthree::binary::struct_trait::Session>::Dual,
                <S4 as mpstthree::binary::struct_trait::Session>::Dual,
                S6,
                R3,
                N3,
            >,
        ) -> Result<(), Box<dyn std::error::Error>>
        + std::marker::Send
        + 'static,
    F4: FnOnce(
            SessionMpstFour<
                <S3 as mpstthree::binary::struct_trait::Session>::Dual,
                <S5 as mpstthree::binary::struct_trait::Session>::Dual,
                <S6 as mpstthree::binary::struct_trait::Session>::Dual,
                R4,
                N4,
            >,
        ) -> Result<(), Box<dyn std::error::Error>>
        + std::marker::Send
        + 'static,
{
    let (channel_1_2, channel_2_1) = <S1 as mpstthree::binary::struct_trait::Session>::new();
    let (channel_1_3, channel_3_1) = <S2 as mpstthree::binary::struct_trait::Session>::new();
    let (channel_1_4, channel_4_1) = <S3 as mpstthree::binary::struct_trait::Session>::new();
    let (channel_2_3, channel_3_2) = <S4 as mpstthree::binary::struct_trait::Session>::new();
    let (channel_2_4, channel_4_2) = <S5 as mpstthree::binary::struct_trait::Session>::new();
    let (channel_3_4, channel_4_3) = <S6 as mpstthree::binary::struct_trait::Session>::new();
    let (role_1, _) = R1::new();
    let (role_2, _) = R2::new();
    let (role_3, _) = R3::new();
    let (role_4, _) = R4::new();
    let (name_1, _) = N1::new();
    let (name_2, _) = N2::new();
    let (name_3, _) = N3::new();
    let (name_4, _) = N4::new();
    let sessionmpst_1 = SessionMpstFour {
        session1: channel_1_2,
        session2: channel_1_3,
        session3: channel_1_4,
        stack: role_1,
        name: name_1,
    };
    let sessionmpst_2 = SessionMpstFour {
        session1: channel_2_1,
        session2: channel_2_3,
        session3: channel_2_4,
        stack: role_2,
        name: name_2,
    };
    let sessionmpst_3 = SessionMpstFour {
        session1: channel_3_1,
        session2: channel_3_2,
        session3: channel_3_4,
        stack: role_3,
        name: name_3,
    };
    let sessionmpst_4 = SessionMpstFour {
        session1: channel_4_1,
        session2: channel_4_2,
        session3: channel_4_3,
        stack: role_4,
        name: name_4,
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
        std::thread::spawn(move || {
            std::panic::set_hook(Box::new(|_info| {}));
            match f4(sessionmpst_4) {
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
type NameApi = Api<RoleEnd>;
type NameController = Controller<RoleEnd>;
type NameLogs = Logs<RoleEnd>;
type NameStorage = Storage<RoleEnd>;
type RecvLogsChoice = Logs<RoleEnd>;
enum Branching0fromLtoA<N: marker::Send> {
    Up(SessionMpstFour<End, RecursAtoL<N>, End, RecvLogsChoice, NameApi>),
    Down(SessionMpstFour<End, RecursAtoL<N>, End, RecvLogsChoice, NameApi>),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameApi>),
}
type RecursAtoL<N> = Recv<Branching0fromLtoA<N>, End>;
enum Branching0fromLtoC<N: marker::Send> {
    Up(SessionMpstFour<End, Recv<N, RecursCtoL<N>>, End, Logs<RecvLogsChoice>, NameController>),
    Down(
        SessionMpstFour<
            End,
            Recv<N, Send<N, RecursCtoL<N>>>,
            End,
            Logs<Logs<RecvLogsChoice>>,
            NameController,
        >,
    ),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameController>),
}
type RecursCtoL<N> = Recv<Branching0fromLtoC<N>, End>;
enum Branching0fromLtoS<N: marker::Send> {
    Up(SessionMpstFour<End, End, RecursStoL<N>, RecvLogsChoice, NameStorage>),
    Down(SessionMpstFour<End, End, RecursStoL<N>, RecvLogsChoice, NameStorage>),
    Close(SessionMpstFour<End, End, End, RoleEnd, NameStorage>),
}
type RecursStoL<N> = Recv<Branching0fromLtoS<N>, End>;
type Choose0fromLtoA<N> = Send<Branching0fromLtoA<N>, End>;
type Choose0fromLtoC<N> = Send<Branching0fromLtoC<N>, End>;
type Choose0fromLtoS<N> = Send<Branching0fromLtoS<N>, End>;
type EndpointApi<N> = SessionMpstFour<End, RecursAtoL<N>, End, RecvLogsChoice, NameApi>;
type EndpointController<N> =
    SessionMpstFour<End, RecursCtoL<N>, End, RecvLogsChoice, NameController>;
type EndpointControllerInit<N> =
    SessionMpstFour<End, Send<N, RecursCtoL<N>>, End, Logs<RecvLogsChoice>, NameController>;
type EndpointStorage<N> = SessionMpstFour<End, End, RecursStoL<N>, RecvLogsChoice, NameStorage>;
type EndpointLogs<N> = SessionMpstFour<
    Choose0fromLtoA<N>,
    Choose0fromLtoC<N>,
    Choose0fromLtoS<N>,
    RoleBroadcast,
    NameLogs,
>;
type EndpointLogsInit<N> = SessionMpstFour<
    Choose0fromLtoA<N>,
    Recv<N, Choose0fromLtoC<N>>,
    Choose0fromLtoS<N>,
    Controller<RoleBroadcast>,
    NameLogs,
>;
fn endpoint_api(s: EndpointApi<i32>) -> Result<(), Box<dyn Error>> {
    (move || -> Result<_, _> {
        let (l, s) = recv_api_from_logs(s)?;
        mpstthree::binary::cancel::cancel(s);
        match l {
            Branching0fromLtoA::Up(s) => endpoint_api(s),
            Branching0fromLtoA::Down(s) => endpoint_api(s),
            Branching0fromLtoA::Close(s) => close_mpst_multi(s),
        }
    })()
}
fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["Send start to Logs: ", "\n"],
            &match (&0,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
    let s = send_start_controller_to_logs(0, s);
    recurs_controller(s)
}
fn recurs_controller(s: EndpointController<i32>) -> Result<(), Box<dyn Error>> {
    (move || -> Result<_, _> {
        let (l, s) = recv_start_controller_from_logs(s)?;
        mpstthree::binary::cancel::cancel(s);
        match l {
            Branching0fromLtoC::Up(s) => {
                let (success, s) = recv_start_controller_from_logs(s)?;
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["Success from Logs: ", "\n"],
                        &match (&success,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                };
                recurs_controller(s)
            }
            Branching0fromLtoC::Down(s) => {
                let (failure, s) = recv_start_controller_from_logs(s)?;
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["Failure from Logs: ", "\n"],
                        &match (&failure,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                };
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["Send restart to Logs: ", "\n"],
                        &match (&0,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                };
                let s = send_start_controller_to_logs(0, s);
                recurs_controller(s)
            }
            Branching0fromLtoC::Close(s) => close_mpst_multi(s),
        }
    })()
}
fn endpoint_storage(s: EndpointStorage<i32>) -> Result<(), Box<dyn Error>> {
    (move || -> Result<_, _> {
        let (l, s) = recv_storage_from_logs(s)?;
        mpstthree::binary::cancel::cancel(s);
        match l {
            Branching0fromLtoS::Up(s) => endpoint_storage(s),
            Branching0fromLtoS::Down(s) => endpoint_storage(s),
            Branching0fromLtoS::Close(s) => close_mpst_multi(s),
        }
    })()
}
fn endpoint_logs(s: EndpointLogsInit<i32>) -> Result<(), Box<dyn Error>> {
    let (status, s) = recv_start_logs_from_controller(s)?;
    recurs_logs(s, status, 20)
}
fn recurs_logs(s: EndpointLogs<i32>, status: i32, loops: i32) -> Result<(), Box<dyn Error>> {
    match status {
        0 => {
            let s = {
                let (channel_1_2, channel_2_1) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_1_3, channel_3_1) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_1_4, channel_4_1) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_2_3, channel_3_2) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_2_4, channel_4_2) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_3_4, channel_4_3) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (stack_1, _) = <_ as mpstthree::role::Role>::new();
                let (stack_2, _) = <_ as mpstthree::role::Role>::new();
                let (stack_3, _) = <_ as mpstthree::role::Role>::new();
                let (stack_4, _) = <_ as mpstthree::role::Role>::new();
                let (name_1, _) =
                    <Api<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let (name_2, _) =
                    <Controller<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let (name_3, _) =
                    <Storage<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let (name_4, _) =
                    <Logs<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let mut s = s;
                s.session1 = mpstthree::binary::send::send(
                    Branching0fromLtoA::Up(SessionMpstFour {
                        session1: channel_1_2,
                        session2: channel_1_3,
                        session3: channel_1_4,
                        stack: stack_1,
                        name: name_1,
                    }),
                    s.session1,
                );
                s.session2 = mpstthree::binary::send::send(
                    Branching0fromLtoC::Up(SessionMpstFour {
                        session1: channel_2_1,
                        session2: channel_2_3,
                        session3: channel_2_4,
                        stack: stack_2,
                        name: name_2,
                    }),
                    s.session2,
                );
                s.session3 = mpstthree::binary::send::send(
                    Branching0fromLtoS::Up(SessionMpstFour {
                        session1: channel_4_1,
                        session2: channel_4_2,
                        session3: channel_4_3,
                        stack: stack_3,
                        name: name_3,
                    }),
                    s.session3,
                );
                let _ = {
                    fn temp(
                        r: &mpstthree::role::broadcast::RoleBroadcast,
                    ) -> Result<(), Box<dyn std::error::Error>> {
                        Ok(())
                    }
                    temp(&s.stack)
                };
                mpstthree::binary::cancel::cancel(s);
                SessionMpstFour {
                    session1: channel_3_1,
                    session2: channel_3_2,
                    session3: channel_3_4,
                    stack: stack_4,
                    name: name_4,
                }
            };
            let success = random::<i32>();
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["Logs is up: ", "\n"],
                    &match (&success,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
            };
            let s = send_failure_logs_to_controller(success, s);
            if loops < 0 {
                recurs_logs(s, 2, loops - 1)
            } else {
                let mut rng = thread_rng();
                let failure: i32 = rng.gen_range(1..=6);
                if failure == 1 {
                    recurs_logs(s, 1, loops - 1)
                } else {
                    recurs_logs(s, 0, loops - 1)
                }
            }
        }
        1 => {
            let s = {
                let (channel_1_2, channel_2_1) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_1_3, channel_3_1) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_1_4, channel_4_1) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_2_3, channel_3_2) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_2_4, channel_4_2) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_3_4, channel_4_3) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (stack_1, _) = <_ as mpstthree::role::Role>::new();
                let (stack_2, _) = <_ as mpstthree::role::Role>::new();
                let (stack_3, _) = <_ as mpstthree::role::Role>::new();
                let (stack_4, _) = <_ as mpstthree::role::Role>::new();
                let (name_1, _) =
                    <Api<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let (name_2, _) =
                    <Controller<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let (name_3, _) =
                    <Storage<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let (name_4, _) =
                    <Logs<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let mut s = s;
                s.session1 = mpstthree::binary::send::send(
                    Branching0fromLtoA::Down(SessionMpstFour {
                        session1: channel_1_2,
                        session2: channel_1_3,
                        session3: channel_1_4,
                        stack: stack_1,
                        name: name_1,
                    }),
                    s.session1,
                );
                s.session2 = mpstthree::binary::send::send(
                    Branching0fromLtoC::Down(SessionMpstFour {
                        session1: channel_2_1,
                        session2: channel_2_3,
                        session3: channel_2_4,
                        stack: stack_2,
                        name: name_2,
                    }),
                    s.session2,
                );
                s.session3 = mpstthree::binary::send::send(
                    Branching0fromLtoS::Down(SessionMpstFour {
                        session1: channel_4_1,
                        session2: channel_4_2,
                        session3: channel_4_3,
                        stack: stack_3,
                        name: name_3,
                    }),
                    s.session3,
                );
                let _ = {
                    fn temp(
                        r: &mpstthree::role::broadcast::RoleBroadcast,
                    ) -> Result<(), Box<dyn std::error::Error>> {
                        Ok(())
                    }
                    temp(&s.stack)
                };
                mpstthree::binary::cancel::cancel(s);
                SessionMpstFour {
                    session1: channel_3_1,
                    session2: channel_3_2,
                    session3: channel_3_4,
                    stack: stack_4,
                    name: name_4,
                }
            };
            let failure = random::<i32>();
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["Logs is down: ", "\n"],
                    &match (&failure,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
            };
            let s = send_failure_logs_to_controller(failure, s);
            let (response, s) = recv_start_logs_from_controller(s)?;
            let mut rng = thread_rng();
            let failure: i32 = rng.gen_range(1..=6);
            if failure == 1 {
                recurs_logs(s, 1, loops - 1)
            } else {
                recurs_logs(s, response, loops - 1)
            }
        }
        _ => {
            let s = {
                let (channel_1_2, channel_2_1) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_1_3, channel_3_1) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_1_4, channel_4_1) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_2_3, channel_3_2) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_2_4, channel_4_2) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (channel_3_4, channel_4_3) =
                    <_ as mpstthree::binary::struct_trait::Session>::new();
                let (stack_1, _) = <_ as mpstthree::role::Role>::new();
                let (stack_2, _) = <_ as mpstthree::role::Role>::new();
                let (stack_3, _) = <_ as mpstthree::role::Role>::new();
                let (stack_4, _) = <_ as mpstthree::role::Role>::new();
                let (name_1, _) =
                    <Api<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let (name_2, _) =
                    <Controller<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let (name_3, _) =
                    <Storage<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let (name_4, _) =
                    <Logs<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                let mut s = s;
                s.session1 = mpstthree::binary::send::send(
                    Branching0fromLtoA::Close(SessionMpstFour {
                        session1: channel_1_2,
                        session2: channel_1_3,
                        session3: channel_1_4,
                        stack: stack_1,
                        name: name_1,
                    }),
                    s.session1,
                );
                s.session2 = mpstthree::binary::send::send(
                    Branching0fromLtoC::Close(SessionMpstFour {
                        session1: channel_2_1,
                        session2: channel_2_3,
                        session3: channel_2_4,
                        stack: stack_2,
                        name: name_2,
                    }),
                    s.session2,
                );
                s.session3 = mpstthree::binary::send::send(
                    Branching0fromLtoS::Close(SessionMpstFour {
                        session1: channel_4_1,
                        session2: channel_4_2,
                        session3: channel_4_3,
                        stack: stack_3,
                        name: name_3,
                    }),
                    s.session3,
                );
                let _ = {
                    fn temp(
                        r: &mpstthree::role::broadcast::RoleBroadcast,
                    ) -> Result<(), Box<dyn std::error::Error>> {
                        Ok(())
                    }
                    temp(&s.stack)
                };
                mpstthree::binary::cancel::cancel(s);
                SessionMpstFour {
                    session1: channel_3_1,
                    session2: channel_3_2,
                    session3: channel_3_4,
                    stack: stack_4,
                    name: name_4,
                }
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["Finish protocol\n"],
                    &match () {
                        () => [],
                    },
                ));
            };
            close_mpst_multi(s)
        }
    }
}
#[allow(dead_code)]
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["Starting protocol\n"],
            &match () {
                () => [],
            },
        ));
    };
    let (thread_api, thread_controller, thread_logs, thread_storage) = fork_mpst(
        endpoint_api,
        endpoint_controller,
        endpoint_logs,
        endpoint_storage,
    );
    thread_api.join().unwrap();
    thread_controller.join().unwrap();
    thread_logs.join().unwrap();
    thread_storage.join().unwrap();
}
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
