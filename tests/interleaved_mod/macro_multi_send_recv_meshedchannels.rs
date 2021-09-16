// Test for parametrisation on the number of roles

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    close_mpst_interleaved, create_meshedchannels, create_multiple_normal_role,
    create_recv_mpst_session, create_send_mpst_session, fork_mpst_multi_interleaved,
};
use std::error::Error;

// Create new MeshedChannels for five participants
create_meshedchannels!(MeshedChannels, 5);

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
    RoleD, RoleDDual |
    RoleE, RoleEDual |
);

// Create new send functions
create_send_mpst_session!(send_mpst_d_to_b, RoleB, RoleD, MeshedChannels, 5, 2);

// Create new recv functions and related types
create_recv_mpst_session!(recv_mpst_b_from_d, RoleD, RoleB, MeshedChannels, 5, 3);

close_mpst_interleaved!(close_mpst_multi, MeshedChannels, 5);

fork_mpst_multi_interleaved!(fork_mpst, MeshedChannels, 5);

type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;

type SendMeshedChannelsD<N> = MeshedChannels<End, Send<N, End>, End, End, NameB, NameD>;

type RecvMeshedChannelsB<N> = MeshedChannels<End, End, Recv<N, End>, End, NameD, NameB>;

type PawnA = MeshedChannels<End, End, End, End, RoleEnd, NameA>;
type PawnC = MeshedChannels<End, End, End, End, RoleEnd, NameC>;
type PawnE = MeshedChannels<End, End, End, End, RoleEnd, NameE>;

fn full(
    s_1: PawnA,
    s_2: RecvMeshedChannelsB<i32>,
    s_3: PawnC,
    s_4: SendMeshedChannelsD<i32>,
    s_5: PawnE,
) -> Result<(), Box<dyn Error>> {
    let s_4 = send_mpst_d_to_b(0, s_4);
    let (_, s_2) = recv_mpst_b_from_d(s_2)?;
    close_mpst_multi(s_1, s_2, s_3, s_4, s_5)
}

////////////////////////////////////////

pub fn interleaved_main() {
    assert!(fork_mpst(full).is_ok());
}
