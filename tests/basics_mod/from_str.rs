use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::meshedchannels::MeshedChannels;
use std::str::FromStr;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;

use mpstthree::name::a::NameA;

// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

// Stacks
type StackA = RoleB<RoleC<RoleEnd>>;

// Creating the MP sessions
type Endpoint<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, NameA>;

pub fn binary_sessions() {
    // Test Send and End
    let right_send = AtoB::<i32>::from_str("Send<0, End>").unwrap();
    let left_send = AtoB::<i32>::new().0;
    assert_eq!(left_send.self_head_str(), right_send.self_head_str());
    assert_eq!(left_send.self_tail_str(), right_send.self_tail_str());

    // Test Recv and End
    let right_recv = AtoC::<i32>::from_str("Recv<0, End>").unwrap();
    let left_recv = AtoC::<i32>::new().0;
    assert_eq!(left_recv.self_head_str(), right_recv.self_head_str());
    assert_eq!(left_recv.self_tail_str(), right_recv.self_tail_str());
}

pub fn roles() {
    // Test RoleA and RoleEnd
    let right_role_a = RoleA::<RoleEnd>::new().0;
    assert_eq!("RoleA".to_string(), right_role_a.self_head_str());
    assert_eq!("RoleEnd<>".to_string(), right_role_a.self_tail_str());

    // Test RoleB and RoleEnd
    let right_role_b = RoleB::<RoleEnd>::new().0;
    assert_eq!("RoleB".to_string(), right_role_b.self_head_str());
    assert_eq!("RoleEnd<>".to_string(), right_role_b.self_tail_str());

    // Test RoleC and RoleEnd
    let right_role_c = RoleC::<RoleEnd>::new().0;
    assert_eq!("RoleC".to_string(), right_role_c.self_head_str());
    assert_eq!("RoleEnd<>".to_string(), right_role_c.self_tail_str());
}

pub fn meshedchannels() {
    let meshedchannels = Endpoint::<i32>::new().0;

    assert_eq!(
        "Send\nRecv\nRoleB\nRoleA".to_string(),
        meshedchannels.self_head_str()
    );
    assert_eq!(
        "Send<End<>>\nRecv<End<>>\nRoleB<RoleC<RoleEnd<>>>\nRoleA<RoleEnd<>>".to_string(),
        meshedchannels.self_tail_str()
    );
}
