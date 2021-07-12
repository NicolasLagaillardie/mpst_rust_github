// Test for parametrisation on the name of the roles
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use std::error::Error;

use mpstthree::bundle_impl;

// Create new roles
bundle_impl!(MeshedChannels => A, B, D => fork_mpst);

type TestA = RoleA<RoleEnd>;
type TestB = RoleB<RoleEnd>;
type TestD = RoleD<RoleEnd>;

type SendSessionMPSTD<N> = MeshedChannels<Send<N, End>, End, TestA, TestD>;

type SendSessionMPSTA<N> = MeshedChannels<End, Send<N, End>, TestD, TestA>;

type RecvSessionMPSTD<N> = MeshedChannels<Recv<N, End>, End, TestA, TestD>;

type RecvSessionMPSTA<N> = MeshedChannels<End, Recv<N, End>, TestD, TestA>;

// Create an B pawn
type Pawn = MeshedChannels<End, End, RoleEnd, TestB>;

// The functions for the basic exchanges
fn send_a_to_d(s: SendSessionMPSTA<i32>) -> Result<(), Box<dyn Error>> {
    s.send(0).close()
}

fn send_d_to_a(s: SendSessionMPSTD<i32>) -> Result<(), Box<dyn Error>> {
    s.send(0).close()
}

fn recv_a_to_d(s: RecvSessionMPSTA<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    s.close()
}

fn recv_d_to_a(s: RecvSessionMPSTD<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    s.close()
}

fn pawn(s: Pawn) -> Result<(), Box<dyn Error>> {
    s.close()
}

/////////////////////////////////////////

pub fn basic_macros_send() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(send_a_to_d, pawn, recv_d_to_a);

            assert!(thread_a.join().is_ok());
            assert!(thread_pawn.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

pub fn basic_macros_recv() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(recv_a_to_d, pawn, send_d_to_a);

            assert!(thread_a.join().is_ok());
            assert!(thread_pawn.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
