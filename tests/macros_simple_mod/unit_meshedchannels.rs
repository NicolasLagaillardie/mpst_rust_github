use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::meshedchannels::MeshedChannels;

use mpstthree::name::a::NameA;
use mpstthree::name::Name;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;

// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

// Stacks
type StackA = RoleB<RoleC<RoleEnd>>;

// Creating the MP sessions
type Endpoint<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, NameA>;

pub fn meshedchannels_fields() {
    let (meshedchannels_1, meshedchannels_2) = Endpoint::<i32>::new();

    // meshedchannels_1
    let (here1_stack, there1_stack) = RoleC::<RoleEnd>::new();
    let (here1_name, there1_name) = RoleEnd::new();
    meshedchannels_1
        .stack
        .sender
        .send(there1_stack)
        .unwrap_or(());
    meshedchannels_1.name.sender.send(()).unwrap_or(());

    assert!(here1_stack.sender.send(RoleEnd::new().1).is_err());
    assert!(here1_name.sender.send(()).is_ok());
    assert!(there1_name.sender.send(()).is_ok());

    // meshedchannels_2
    let (here2_stack, there2_stack) = RoleC::<RoleEnd>::new();
    let (here2_name, there2_name) = RoleEnd::new();
    meshedchannels_2
        .stack
        .sender
        .send(here2_stack)
        .unwrap_or(());
    meshedchannels_2.name.sender.send(()).unwrap_or(());

    assert!(there2_stack.sender.send(RoleEnd::new().1).is_err());
    assert!(here2_name.sender.send(()).is_ok());
    assert!(there2_name.sender.send(()).is_ok());
}

pub fn meshedchannels_methods() {
    assert_eq!(
        Endpoint::<i32>::head_str(),
        "Send\nRecv\nRoleB\nNameA".to_string()
    );

    assert_eq!(
        Endpoint::<i32>::tail_str(),
        "Send<End<>>\nRecv<End<>>\nRoleB<RoleC<RoleEnd<>>>\nNameA<>".to_string()
    );

    assert_eq!(AtoB::<i32>::head_str(), "Send".to_string());
    assert_eq!(AtoB::<i32>::tail_str(), "End<>".to_string());

    assert_eq!(AtoC::<i32>::head_str(), "Recv".to_string());
    assert_eq!(AtoC::<i32>::tail_str(), "End<>".to_string());

    assert_eq!(End::head_str(), "End".to_string());
    assert!(End::tail_str().is_empty());

    assert_eq!(
        Endpoint::<i32>::head_str(),
        format!(
            "{}\n{}\n{}\n{}",
            AtoB::<i32>::head_str(),
            AtoC::<i32>::head_str(),
            StackA::head_str(),
            NameA::head_str()
        )
    );

    assert_eq!(
        Endpoint::<i32>::tail_str(),
        format!(
            "{}<{}>\n{}<{}>\n{}<{}>\n{}<{}>",
            AtoB::<i32>::head_str(),
            AtoB::<i32>::tail_str(),
            AtoC::<i32>::head_str(),
            AtoC::<i32>::tail_str(),
            StackA::head_str(),
            StackA::tail_str(),
            NameA::head_str(),
            NameA::tail_str()
        )
    );
}

pub fn meshedchannels_self_methods() {
    let (meshedchannels_1, meshedchannels_2) = Endpoint::<i32>::new();

    ////////////////////////////////

    assert_eq!(
        meshedchannels_1.self_head_str(),
        "Send\nRecv\nRoleB\nNameA".to_string()
    );

    assert_eq!(
        meshedchannels_2.self_head_str(),
        "Recv\nSend\nRoleBDual\nNameA".to_string()
    );

    assert_eq!(
        meshedchannels_1.session1.self_head_str(),
        "Send".to_string()
    );

    assert_eq!(
        meshedchannels_1.session2.self_head_str(),
        "Recv".to_string()
    );

    assert_eq!(
        meshedchannels_2.session1.self_head_str(),
        "Recv".to_string()
    );

    assert_eq!(
        meshedchannels_2.session2.self_head_str(),
        "Send".to_string()
    );

    ////////////////////////////////

    assert_eq!(
        meshedchannels_1.self_tail_str(),
        "Send<End<>>\nRecv<End<>>\nRoleB<RoleC<RoleEnd<>>>\nNameA<>".to_string()
    );

    assert_eq!(
        meshedchannels_2.self_tail_str(),
        "Recv<End<>>\nSend<End<>>\nRoleBDual<RoleCDual<RoleEnd<>>>\nNameA<>".to_string()
    );

    assert_eq!(
        meshedchannels_1.session1.self_tail_str(),
        "End<>".to_string()
    );

    assert_eq!(
        meshedchannels_1.session2.self_tail_str(),
        "End<>".to_string()
    );

    assert_eq!(
        meshedchannels_2.session1.self_tail_str(),
        "End<>".to_string()
    );

    assert_eq!(
        meshedchannels_2.session2.self_tail_str(),
        "End<>".to_string()
    );

    ////////////////////////////////

    let (end_1, end_2) = End::new();

    assert_eq!(end_1.self_head_str(), "End".to_string());

    assert!(end_1.self_tail_str().is_empty());

    assert_eq!(end_2.self_head_str(), "End".to_string());

    assert!(end_2.self_tail_str().is_empty());
}
