//! This module contains the macros
//! for creating fork functions for interleaved sessions.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"interleaved"` feature.*

use std::error::Error;
use std::marker;

use crate::binary::struct_trait::session::Session;
use crate::meshedchannels::MeshedChannels;
use crate::role::Role;
use crate::name::Name;

#[doc(hidden)]
pub fn fork_mpst_solo<S0, S1, S2, R0, R1, R2, N0, N1, N2, F>(f: F) -> Result<(), Box<dyn Error>>
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    R0: Role + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    N0: Name + 'static,
    N1: Name + 'static,
    N2: Name + 'static,
    F: FnOnce(
            MeshedChannels<S0, S1, R0, N0>,
            MeshedChannels<<S0 as Session>::Dual, S2, R1, N1>,
            MeshedChannels<<S1 as Session>::Dual, <S2 as Session>::Dual, R2, N2>,
        ) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    let (channel_ab, channel_ba) = S0::new();
    let (channel_ac, channel_ca) = S1::new();
    let (channel_bc, channel_cb) = S2::new();

    let (role_a, _) = R0::new();
    let (role_b, _) = R1::new();
    let (role_c, _) = R2::new();

    let (name_a, _) = N0::new();
    let (name_b, _) = N1::new();
    let (name_c, _) = N2::new();

    let a = MeshedChannels {
        session1: channel_ab,
        session2: channel_ac,
        stack: role_a,
        name: name_a,
    };
    let b = MeshedChannels {
        session1: channel_ba,
        session2: channel_bc,
        stack: role_b,
        name: name_b,
    };
    let c = MeshedChannels {
        session1: channel_ca,
        session2: channel_cb,
        stack: role_c,
        name: name_c,
    };

    f(a, b, c)
}

#[macro_export]
#[doc(hidden)]
macro_rules! fork_mpst_multi_solo {
    ($func_name:ident, $meshedchannels_name:ident, $n_sessions:literal) => {
        mpst_seq::fork_mpst_multi_solo!($func_name, $meshedchannels_name, $n_sessions);
    };
}

#[macro_export]
#[doc(hidden)]
// Indexes start at one
macro_rules! fork_mpst_multi_interleaved {
    (
        $func_name:ident,
        $meshedchannels_name_one:ident,
        $nsessions_one:literal,
        $index_tuple_one:literal,
        $meshedchannels_name_two:ident,
        $nsessions_two:literal,
        $index_tuple_two:literal
    ) => {
        mpst_seq::fork_mpst_multi_interleaved!(
            $func_name,
            $meshedchannels_name_one,
            $nsessions_one,
            $index_tuple_one,
            $meshedchannels_name_two,
            $nsessions_two,
            $index_tuple_two
        );
    };
}
