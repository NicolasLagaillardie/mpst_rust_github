// Test for parametrisation on the name of the roles
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::end::RoleEnd;
use std::error::Error;

use mpstthree::generate;

// Create new roles
generate!("basic", MeshedChannels, A, B, D);

type StackA = RoleA<RoleEnd>;
type StackD = RoleD<RoleEnd>;

type SendMeshedChannelsD<N> = MeshedChannels<Send<N, End>, End, StackA, NameD>;

type SendMeshedChannelsA<N> = MeshedChannels<End, Send<N, End>, StackD, NameA>;

type RecvMeshedChannelsD<N> = MeshedChannels<Recv<N, End>, End, StackA, NameD>;

type RecvMeshedChannelsA<N> = MeshedChannels<End, Recv<N, End>, StackD, NameA>;

// Create an B dummy
type Dummy = MeshedChannels<End, End, RoleEnd, NameB>;

// The functions for the basic exchanges
fn send_a_to_d(s: SendMeshedChannelsA<i32>) -> Result<(), Box<dyn Error>> {
    s.send(0).close()
}

fn send_d_to_a(s: SendMeshedChannelsD<i32>) -> Result<(), Box<dyn Error>> {
    s.send(0).close()
}

fn recv_a_to_d(s: RecvMeshedChannelsA<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv();
    s.close()
}

fn recv_d_to_a(s: RecvMeshedChannelsD<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv();
    s.close()
}

fn dummy(s: Dummy) -> Result<(), Box<dyn Error>> {
    s.close()
}

/////////////////////////////////////////

pub fn basic_macros_send() {
    assert!({
        {
            let (thread_a, thread_dummy, thread_d) = fork_mpst(send_a_to_d, dummy, recv_d_to_a);

            assert!(thread_a.join().is_ok());
            assert!(thread_dummy.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok::<(), Box<dyn Error>>(())
    }
    .is_ok());
}

pub fn basic_macros_recv() {
    assert!({
        {
            let (thread_a, thread_dummy, thread_d) = fork_mpst(recv_a_to_d, dummy, send_d_to_a);

            assert!(thread_a.join().is_ok());
            assert!(thread_dummy.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok::<(), Box<dyn Error>>(())
    }
    .is_ok());
}
