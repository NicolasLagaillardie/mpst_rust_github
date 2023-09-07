// Test for parametrisation on the number of roles

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    close_mpst, create_meshedchannels, create_multiple_normal_name, create_multiple_normal_role,
    create_recv_mpst_session, create_send_mpst_session, fork_mpst_multi,
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

// Create new names
create_multiple_normal_name!(NameA, NameB, NameC, NameD, NameE);

// Create new send functions
create_send_mpst_session!(send_mpst_d_to_b, RoleB, NameD, MeshedChannels, 5, 2);

// Create new recv functions and related types
create_recv_mpst_session!(recv_mpst_b_from_d, RoleD, NameB, MeshedChannels, 5, 3);

close_mpst!(close_mpst_multi, MeshedChannels, 5);

fork_mpst_multi!(fork_mpst, MeshedChannels, 5);

type SendMeshedChannelsD<N> = MeshedChannels<End, Send<N, End>, End, End, RoleB<RoleEnd>, NameD>;

type RecvMeshedChannelsB<N> = MeshedChannels<End, End, Recv<N, End>, End, RoleD<RoleEnd>, NameB>;

type PawnA = MeshedChannels<End, End, End, End, RoleEnd, NameA>;
type PawnC = MeshedChannels<End, End, End, End, RoleEnd, NameC>;
type PawnE = MeshedChannels<End, End, End, End, RoleEnd, NameE>;

fn send_d_to_b(s: SendMeshedChannelsD<i32>) -> Result<(), Box<dyn Error>> {
    let (size, s) = s.field_names();
    assert_eq!(size.len(), 4);
    let s = send_mpst_d_to_b(0, s);
    close_mpst_multi(s)
}

fn recv_b_to_d(s: RecvMeshedChannelsB<i32>) -> Result<(), Box<dyn Error>> {
    let (size, s) = s.field_names();
    assert_eq!(size.len(), 4);
    let (_, s) = recv_mpst_b_from_d(s)?;
    close_mpst_multi(s)
}

fn pawn_a(s: PawnA) -> Result<(), Box<dyn Error>> {
    let (size, s) = s.field_names();
    assert_eq!(size.len(), 4);
    close_mpst_multi(s)
}

fn pawn_c(s: PawnC) -> Result<(), Box<dyn Error>> {
    let (size, s) = s.field_names();
    assert_eq!(size.len(), 4);
    close_mpst_multi(s)
}

fn pawn_e(s: PawnE) -> Result<(), Box<dyn Error>> {
    let (size, s) = s.field_names();
    assert_eq!(size.len(), 4);
    close_mpst_multi(s)
}

////////////////////////////////////////

pub fn test_new_send() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e) =
        fork_mpst(pawn_a, recv_b_to_d, pawn_c, send_d_to_b, pawn_e);

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_d.join().is_ok());
    assert!(thread_e.join().is_ok());
}
