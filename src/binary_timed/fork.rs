//! This module contains the functions for
//! forking binary sessions.

use crate::binary::struct_trait::session::Session;

use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::marker;
use std::panic;
use std::thread::{Builder, JoinHandle};
use std::time::Instant;

#[doc(hidden)]
pub fn fork_with_thread_id<S, P>(p: P) -> (JoinHandle<()>, S::Dual)
where
    S: Session + 'static,
    P: FnOnce(S, &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    let (there, here) = Session::new();
    let other_thread = Builder::new()
        .name(String::from("Thread P"))
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            panic::set_hook(Box::new(|_info| {
                // do nothing
            }));

            let mut all_clocks = HashMap::<char, Instant>::new();

            match p(there, &mut all_clocks) {
                Ok(()) => (),
                Err(e) => panic!("{}", e.to_string()),
            }
        })
        .unwrap();
    (other_thread, here)
}

/// Creates a child process, and a session with two dual
/// endpoints of type `S` and `S::Dual`. The first endpoint
/// is given to the child process. Returns the
/// second endpoint.
pub fn fork<S, P>(p: P) -> S::Dual
where
    S: Session + 'static,
    P: FnOnce(S, &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    fork_with_thread_id(p).1
}
