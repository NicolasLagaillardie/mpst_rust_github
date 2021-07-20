use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::meshedchannels::MeshedChannels;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;

/// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

/// Stacks
type StackA = RoleB<RoleC<RoleEnd>>;

/// Creating the MP sessions
type Endpoint<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, RoleA<RoleEnd>>;

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
    meshedchannels_1.name.sender.send(there1_name).unwrap_or(());

    assert_eq!(here1_stack.sender.send(RoleEnd::new().1).unwrap_or(()), ());
    assert_eq!(here1_name.sender.send(()).unwrap_or(()), ());

    // meshedchannels_2
    let (here2_stack, there2_stack) = RoleC::<RoleEnd>::new();
    let (here2_name, there2_name) = RoleEnd::new();
    meshedchannels_2
        .stack
        .sender
        .send(here2_stack)
        .unwrap_or(());
    meshedchannels_2.name.sender.send(here2_name).unwrap_or(());

    assert_eq!(there2_stack.sender.send(RoleEnd::new().1).unwrap_or(()), ());
    assert_eq!(there2_name.sender.send(()).unwrap_or(()), ());
}

pub fn meshedchannels_methods() {
    assert_eq!(
        Endpoint::<i32>::head_str(),
        format!(
            "{}\n{}\n{}\n{}",
            AtoB::<i32>::head_str(),
            AtoC::<i32>::head_str(),
            StackA::head_str(),
            RoleA::<RoleEnd>::head_str()
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
            RoleA::<RoleEnd>::head_str(),
            RoleA::<RoleEnd>::tail_str()
        )
    );

    assert_eq!(
        Endpoint::<i32>::tail_str(),
        format!("Send<End<>>\nRecv<End<>>\nRoleB<RoleC<RoleEnd<>>>\nRoleA<RoleEnd<>>")
    );

    assert_eq!(AtoB::<i32>::head_str(), format!("Send"));
    assert_eq!(AtoB::<i32>::tail_str(), format!("End<>"));

    assert_eq!(AtoC::<i32>::head_str(), format!("Recv"));
    assert_eq!(AtoC::<i32>::tail_str(), format!("End<>"));

    assert_eq!(End::head_str(), format!("End"));
    assert_eq!(End::tail_str(), format!(""));

    assert_eq!(RoleA::<RoleEnd>::head_str(), format!("RoleA"));
    assert_eq!(RoleA::<RoleEnd>::tail_str(), format!("RoleEnd<>"));

    assert_eq!(RoleB::<RoleEnd>::head_str(), format!("RoleB"));
    assert_eq!(RoleB::<RoleEnd>::tail_str(), format!("RoleEnd<>"));

    assert_eq!(RoleC::<RoleEnd>::head_str(), format!("RoleC"));
    assert_eq!(RoleC::<RoleEnd>::tail_str(), format!("RoleEnd<>"));
}
