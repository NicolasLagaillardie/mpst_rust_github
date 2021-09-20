//! This module contains the macros
//! for creating close functions for interleaved sessions.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"interleaved"` feature.*

use crate::binary::struct_trait::end::End;
use crate::binary::struct_trait::end::Signal;
use crate::meshedchannels::MeshedChannels;
use crate::role::end::RoleEnd;
use crate::role::Role;

use std::error::Error;

#[doc(hidden)]
pub fn close_mpst_interleaved<R1, R2, R3>(
    s_1: MeshedChannels<End, End, RoleEnd, R1>,
    s_2: MeshedChannels<End, End, RoleEnd, R2>,
    s_3: MeshedChannels<End, End, RoleEnd, R3>,
) -> Result<(), Box<dyn Error>>
where
    R1: Role,
    R2: Role,
    R3: Role,
{
    s_1.session1.sender.send(Signal::Stop).unwrap_or(());
    s_1.session2.sender.send(Signal::Stop).unwrap_or(());
    s_2.session1.sender.send(Signal::Stop).unwrap_or(());
    s_2.session2.sender.send(Signal::Stop).unwrap_or(());
    s_3.session1.sender.send(Signal::Stop).unwrap_or(());
    s_3.session2.sender.send(Signal::Stop).unwrap_or(());

    s_1.session1.receiver.recv()?;
    s_1.session2.receiver.recv()?;
    s_2.session1.receiver.recv()?;
    s_2.session2.receiver.recv()?;
    s_3.session1.receiver.recv()?;
    s_3.session2.receiver.recv()?;

    Ok(())
}

#[macro_export]
#[doc(hidden)]
macro_rules! close_mpst_interleaved {
    ($func_name:ident, $meshedchannels_name:ident, $nsessions:literal) => {
        mpst_seq::close_mpst_interleaved!($func_name, $meshedchannels_name, $nsessions);
    };
}
