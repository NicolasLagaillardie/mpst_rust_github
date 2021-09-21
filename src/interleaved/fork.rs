//! This module contains the macros
//! for creating fork functions for interleaved sessions.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"interleaved"` feature.*

use std::error::Error;
use std::marker;
use std::panic::set_hook;
use std::thread::{spawn, JoinHandle};

use crate::binary::struct_trait::session::Session;
use crate::functionmpst::fork::fork_simple;
use crate::meshedchannels::MeshedChannels;
use crate::role::Role;

#[doc(hidden)]
// Spawn a thread to run a function `p` which expects two `MeshedChannels` as inputs
pub(crate) fn fork_simple_interleaved<S1, S2, S3, S4, R1, R2, N1, N2, P>(
    p: P,
    s_0: MeshedChannels<S1, S2, R1, N1>,
    s_1: MeshedChannels<S3, S4, R2, N2>,
) -> JoinHandle<()>
where
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    S4: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    N1: Role + 'static,
    N2: Role + 'static,
    P: FnOnce(
            MeshedChannels<S1, S2, R1, N1>,
            MeshedChannels<S3, S4, R2, N2>,
        ) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    spawn(move || {
        set_hook(Box::new(|_info| {
            // do nothing
        }));
        match p(s_0, s_1) {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    })
}

#[doc(hidden)]
pub fn fork_mpst_solo<S0, S1, S2, R0, R1, R2, N0, N1, N2, F>(f: F) -> Result<(), Box<dyn Error>>
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    R0: Role + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    N0: Role + 'static,
    N1: Role + 'static,
    N2: Role + 'static,
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

#[doc(hidden)]
// Fork 4 basic functions and 1 interleaved function.
// The interleaved function
pub fn fork_mpst_interleaved<
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    F0,
    F1,
    F2,
    F3,
    FDouble,
>(
    (f0, f1): (F0, F1),
    (f2, f3): (F2, F3),
    f_double: FDouble,
) -> (
    JoinHandle<()>,
    JoinHandle<()>,
    JoinHandle<()>,
    JoinHandle<()>,
    JoinHandle<()>,
)
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    S4: Session + 'static,
    S5: Session + 'static,
    R0: Role + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    R4: Role + 'static,
    R5: Role + 'static,
    N0: Role + 'static,
    N1: Role + 'static,
    N2: Role + 'static,
    N3: Role + 'static,
    N4: Role + 'static,
    N5: Role + 'static,
    ///////// First protocol
    F0: FnOnce(MeshedChannels<S0, S1, R0, N0>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F1: FnOnce(MeshedChannels<<S0 as Session>::Dual, S2, R1, N1>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    ///////// Second protocol
    F2: FnOnce(MeshedChannels<S3, S4, R2, N2>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F3: FnOnce(MeshedChannels<<S3 as Session>::Dual, S5, R3, N3>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    ///////// Combined function
    FDouble: FnOnce(
            MeshedChannels<<S1 as Session>::Dual, <S2 as Session>::Dual, R4, N4>,
            MeshedChannels<<S4 as Session>::Dual, <S5 as Session>::Dual, R5, N5>,
        ) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    ///////// First protocol
    let (channel_0_1, channel_1_0) = S0::new();
    let (channel_0_2, channel_2_0) = S1::new();
    let (channel_1_2, channel_2_1) = S2::new();

    let (role_0, _) = R0::new();
    let (role_1, _) = R1::new();
    let (role_2, _) = R4::new();

    let (name_0, _) = N0::new();
    let (name_1, _) = N1::new();
    let (name_2, _) = N4::new();

    let endpoint_0 = MeshedChannels {
        session1: channel_0_1,
        session2: channel_0_2,
        stack: role_0,
        name: name_0,
    };
    let endpoint_1 = MeshedChannels {
        session1: channel_1_0,
        session2: channel_1_2,
        stack: role_1,
        name: name_1,
    };
    let endpoint_double_0 = MeshedChannels {
        session1: channel_2_0,
        session2: channel_2_1,
        stack: role_2,
        name: name_2,
    };

    ///////// Second protocol
    let (channel_3_4, channel_4_3) = S3::new();
    let (channel_3_5, channel_5_3) = S4::new();
    let (channel_4_5, channel_5_4) = S5::new();

    let (role_3, _) = R2::new();
    let (role_4, _) = R3::new();
    let (role_5, _) = R5::new();

    let (name_3, _) = N2::new();
    let (name_4, _) = N3::new();
    let (name_5, _) = N5::new();

    let endpoint_2 = MeshedChannels {
        session1: channel_3_4,
        session2: channel_3_5,
        stack: role_3,
        name: name_3,
    };
    let endpoint_3 = MeshedChannels {
        session1: channel_4_3,
        session2: channel_4_5,
        stack: role_4,
        name: name_4,
    };
    let endpoint_double_1 = MeshedChannels {
        session1: channel_5_3,
        session2: channel_5_4,
        stack: role_5,
        name: name_5,
    };
    (
        fork_simple(f0, endpoint_0),
        fork_simple(f1, endpoint_1),
        fork_simple(f2, endpoint_2),
        fork_simple(f3, endpoint_3),
        fork_simple_interleaved(f_double, endpoint_double_0, endpoint_double_1),
    )
}

#[macro_export]
#[doc(hidden)]
macro_rules! fork_mpst_multi_solo {
    ($func_name:ident, $meshedchannels_name:ident, $nsessions:literal) => {
        mpst_seq::fork_mpst_multi_solo!($func_name, $meshedchannels_name, $nsessions);
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
