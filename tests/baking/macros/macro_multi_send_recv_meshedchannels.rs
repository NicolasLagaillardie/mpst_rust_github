// Test for parametrisation on the number of roles

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::bundle_impl;
use mpstthree::role::end::RoleEnd;
use std::error::Error;

// Create new roles
bundle_impl!(MeshedChannels => A, B, C, D, E => fork_mpst);

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

fn send_d_to_b(s: SendMeshedChannelsD<i32>) -> Result<(), Box<dyn Error>> {
    s.send(0).close()
}

fn recv_b_to_d(s: RecvMeshedChannelsB<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    s.close()
}

fn pawn_a(s: PawnA) -> Result<(), Box<dyn Error>> {
    s.close()
}

fn pawn_c(s: PawnC) -> Result<(), Box<dyn Error>> {
    s.close()
}

fn pawn_e(s: PawnE) -> Result<(), Box<dyn Error>> {
    s.close()
}

////////////////////////////////////////

pub fn test_new_send() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_b, thread_c, thread_d, thread_e) =
                fork_mpst(pawn_a, recv_b_to_d, pawn_c, send_d_to_b, pawn_e);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
            assert!(thread_d.join().is_ok());
            assert!(thread_e.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
