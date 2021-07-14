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

type SendMeshedChannelsD<N> = MeshedChannels<Send<N, End>, End, TestA, TestD>;

type SendMeshedChannelsA<N> = MeshedChannels<End, Send<N, End>, TestD, TestA>;

type RecvMeshedChannelsD<N> = MeshedChannels<Recv<N, End>, End, TestA, TestD>;

type RecvMeshedChannelsA<N> = MeshedChannels<End, Recv<N, End>, TestD, TestA>;

// Create an B dummy
type Dummy = MeshedChannels<End, End, RoleEnd, TestB>;

// The functions for the basic exchanges
fn send_a_to_d(s: SendMeshedChannelsA<i32>) -> Result<(), Box<dyn Error>> {
    s.send(0).close()
}

fn send_d_to_a(s: SendMeshedChannelsD<i32>) -> Result<(), Box<dyn Error>> {
    s.send(0).close()
}

fn recv_a_to_d(s: RecvMeshedChannelsA<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    s.close()
}

fn recv_d_to_a(s: RecvMeshedChannelsD<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    s.close()
}

fn dummy(s: Dummy) -> Result<(), Box<dyn Error>> {
    s.close()
}

/////////////////////////////////////////

pub fn basic_macros_send() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_dummy, thread_d) = fork_mpst(send_a_to_d, dummy, recv_d_to_a);

            assert!(thread_a.join().is_ok());
            assert!(thread_dummy.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

pub fn basic_macros_recv() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_dummy, thread_d) = fork_mpst(recv_a_to_d, dummy, send_d_to_a);

            assert!(thread_a.join().is_ok());
            assert!(thread_dummy.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
