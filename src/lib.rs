use std::any::Any;
use std::error::Error;
use std::marker;
use std::panic;
use std::thread::{spawn, JoinHandle};

pub mod binary;
pub mod checking;
pub mod functionmpst;
pub mod role;
pub mod sessionmpst;

use binary::Session;
use role::Role;
use sessionmpst::SessionMpst;

type ResultAnySend = Result<(), Box<(dyn Any + marker::Send + 'static)>>;

#[doc(hidden)]
pub fn fork_simple<S1, S2, R, N, P>(p: P, s: SessionMpst<S1, S2, R, N>) -> JoinHandle<()>
where
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
    N: Role + 'static,
    P: FnOnce(SessionMpst<S1, S2, R, N>) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    spawn(move || {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        match p(s) {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    })
}

/// Creates and returns three child processes for three `SessionMpst` linked together.
///
/// Creates 3 pairs of endpoints, each pair of type `S` and `S::Dual`.
/// Creates 3 `Role` for each queue.
/// Creates 3 `SessionMpst`, linked together with the pairs of endpoints, and get the related child processes from `fork_simple`.
/// Returns the tuple of the 3 child processes.
pub fn fork_mpst<S1, S2, S3, R1, R2, R3, N1, N2, N3, F1, F2, F3>(
    f1: F1,
    f2: F2,
    f3: F3,
) -> (ResultAnySend, ResultAnySend, ResultAnySend)
where
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    N1: Role + 'static,
    N2: Role + 'static,
    N3: Role + 'static,
    F1: FnOnce(SessionMpst<S1, <S3 as Session>::Dual, R1, N1>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F2: FnOnce(SessionMpst<<S1 as Session>::Dual, S2, R2, N2>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F3: FnOnce(SessionMpst<S3, <S2 as Session>::Dual, R3, N3>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    let (channel_ab, channel_ba) = S1::new();
    let (channel_ca, channel_ac) = S3::new();
    let (channel_bc, channel_cb) = S2::new();

    let (role_a, _) = R1::new();
    let (role_b, _) = R2::new();
    let (role_c, _) = R3::new();

    let (name_a, _) = N1::new();
    let (name_b, _) = N2::new();
    let (name_c, _) = N3::new();

    let a = SessionMpst {
        session1: channel_ab,
        session2: channel_ac,
        stack: role_a,
        name: name_a,
    };
    let b = SessionMpst {
        session1: channel_ba,
        session2: channel_bc,
        stack: role_b,
        name: name_b,
    };
    let c = SessionMpst {
        session1: channel_ca,
        session2: channel_cb,
        stack: role_c,
        name: name_c,
    };

    let thread_a = fork_simple(f1, a);
    let thread_b = fork_simple(f2, b);
    let thread_c = fork_simple(f3, c);

    (thread_a.join(), thread_b.join(), thread_c.join())
}

#[macro_export]
macro_rules! fork_mpst_multi {
    ($struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(K in 1..=$nsessions {
            fn fork_simple_multi<#(S#K,)* R, N, P>(p: P, s: $struct_name<#(S#K,)* R, N>) -> std::thread::JoinHandle<()>
            where
                #(
                    S#K: mpstthree::binary::Session + 'static,
                )*
                R: mpstthree::role::Role + 'static,
                N: mpstthree::role::Role + 'static,
                P: FnOnce($struct_name<#(S#K,)* R, N>) -> Result<(), Box<dyn std::error::Error>> + std::marker::Send + 'static,
            {
                std::thread::spawn(move || {
                    panic::set_hook(Box::new(|_info| {
                        // do nothing
                    }));
                    match p(s) {
                        Ok(()) => (),
                        Err(e) => panic!("{:?}", e),
                    }
                })
            }

            fn fork_mpst_multi<#(S#K,)* #(R#K,)* #(N#K,)* #(F#K,)*>(
                #(
                    f#K: F#K,
                )*
            ) -> (
                #(
                    Result<(), Box<(dyn std::any::Any + std::marker::Send + 'static)>>
                )*
            )
            where
                #(
                    S#K: mpstthree::binary::Session + 'static,
                )*
                #(
                    R#K: mpstthree::binary::Session + 'static,
                )*
                #(
                    N#K: mpstthree::binary::Session + 'static,
                )*

                F1: FnOnce($struct_name<S1, <S3 as Session>::Dual, R1, N1>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                F2: FnOnce($struct_name<<S1 as Session>::Dual, S2, R2, N2>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                F3: FnOnce($struct_name<S3, <S2 as Session>::Dual, R3, N3>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
            {
                let (channel_ab, channel_ba) = S1::new();
                let (channel_bc, channel_cb) = S2::new();
                let (channel_ca, channel_ac) = S3::new();

                #(
                    let (role_#K, _) = R#K::new();
                    let (name_#K, _) = N#K::new();
                )*

                let a = $struct_name {
                    session1: channel_ab,
                    session2: channel_ac,
                    stack: role_a,
                    name: name_a,
                };
                let b = $struct_name {
                    session1: channel_ba,
                    session2: channel_bc,
                    stack: role_b,
                    name: name_b,
                };
                let c = $struct_name {
                    session1: channel_ca,
                    session2: channel_cb,
                    stack: role_c,
                    name: name_c,
                };

                #(
                    let thread_#K = fork_simple_multi(f1, sessionmpst_#K);
                )*

                #(
                    thread_#K.join(),
                )*
            }
        });
    }
}
