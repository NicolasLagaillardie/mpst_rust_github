extern crate crossbeam_channel;
extern crate dot;
extern crate either;

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
pub fn fork_simple<S1, S2, R, P>(p: P, s: SessionMpst<S1, S2, R>) -> JoinHandle<()>
where
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
    P: FnOnce(SessionMpst<S1, S2, R>) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
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
pub fn fork_mpst<S1, S2, S3, R1, R2, R3, F1, F2, F3>(
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
    F1: FnOnce(SessionMpst<S1, <S3 as Session>::Dual, R1>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F2: FnOnce(SessionMpst<<S1 as Session>::Dual, S2, R2>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F3: FnOnce(SessionMpst<S3, <S2 as Session>::Dual, R3>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    let (channel_ab, channel_ba) = S1::new();
    let (channel_ca, channel_ac) = S3::new();
    let (channel_bc, channel_cb) = S2::new();

    let (role_a, _) = R1::new();
    let (role_b, _) = R2::new();
    let (role_c, _) = R3::new();

    let a = SessionMpst {
        session1: channel_ab,
        session2: channel_ac,
        stack: role_a,
    };
    let b = SessionMpst {
        session1: channel_ba,
        session2: channel_bc,
        stack: role_b,
    };
    let c = SessionMpst {
        session1: channel_ca,
        session2: channel_cb,
        stack: role_c,
    };

    let thread_a = fork_simple(f1, a);
    let thread_b = fork_simple(f2, b);
    let thread_c = fork_simple(f3, c);

    (thread_a.join(), thread_b.join(), thread_c.join())
}
