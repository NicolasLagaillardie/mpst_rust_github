//! This module contains the macros
//! for creating close functions for interleaved sessions.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"interleaved"` feature.*

use crate::binary::struct_trait::end::End;
use crate::binary::struct_trait::end::Signal;
use crate::meshedchannels::MeshedChannels;
use crate::name::Name;
use crate::role::end::RoleEnd;

use std::error::Error;

#[doc(hidden)]
pub fn close_mpst_interleaved<N1, N2, N3>(
    s_1: MeshedChannels<End, End, RoleEnd, N1>,
    s_2: MeshedChannels<End, End, RoleEnd, N2>,
    s_3: MeshedChannels<End, End, RoleEnd, N3>,
) -> Result<(), Box<dyn Error>>
where
    N1: Name,
    N2: Name,
    N3: Name,
{
    s_1.session1.sender.send(Signal::Stop)?;
    s_1.session2.sender.send(Signal::Stop)?;
    s_2.session1.sender.send(Signal::Stop)?;
    s_2.session2.sender.send(Signal::Stop)?;
    s_3.session1.sender.send(Signal::Stop)?;
    s_3.session2.sender.send(Signal::Stop)?;

    match s_1.session1.receiver.recv()? {
        Signal::Stop => {}
        err => panic!("Unexpected label, expected Signal::Stop, got {:?}", err),
    }
    match s_1.session2.receiver.recv()? {
        Signal::Stop => {}
        err => panic!("Unexpected label, expected Signal::Stop, got {:?}", err),
    }
    match s_2.session1.receiver.recv()? {
        Signal::Stop => {}
        err => panic!("Unexpected label, expected Signal::Stop, got {:?}", err),
    }
    match s_2.session2.receiver.recv()? {
        Signal::Stop => {}
        err => panic!("Unexpected label, expected Signal::Stop, got {:?}", err),
    }
    match s_3.session1.receiver.recv()? {
        Signal::Stop => {}
        err => panic!("Unexpected label, expected Signal::Stop, got {:?}", err),
    }
    match s_3.session2.receiver.recv()? {
        Signal::Stop => {}
        err => panic!("Unexpected label, expected Signal::Stop, got {:?}", err),
    }

    Ok(())
}

#[macro_export]
#[doc(hidden)]
macro_rules! close_mpst_interleaved {
    ($func_name:ident, $meshedchannels_name:ident, $n_sessions:literal) => {
        mpst_seq_proc::close_mpst_interleaved!($func_name, $meshedchannels_name, $n_sessions);
    };
}
