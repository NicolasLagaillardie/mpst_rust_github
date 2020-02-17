///#![feature(never_type)]
extern crate crossbeam_channel;
extern crate either;

use std::error::Error;
use std::marker;
use std::panic;
use std::thread::{spawn, JoinHandle};

pub mod binary;
pub mod functionmpst;
pub mod role;
pub mod sessionmpst;

use binary::Session;
use role::Role;
use sessionmpst::SessionMpst;

pub fn fork_simple<S1, S2, R, P>(p: P, s: SessionMpst<S1, S2, R>) -> JoinHandle<()>
where
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
    P: FnOnce(SessionMpst<S1, S2, R>) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    let other_thread = spawn(move || {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        match p(s) {
            Ok(()) => (),
            Err(e) => panic!("{}", e.description()),
        }
    });
    other_thread
}
