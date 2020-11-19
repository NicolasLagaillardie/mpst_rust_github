extern crate mpstthree;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::sessionmpst::SessionMpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;

/// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

/// Queues
type QueueA = RoleB<RoleC<RoleEnd>>;

/// Creating the MP sessions
type Endpoint<N> = SessionMpst<AtoB<N>, AtoC<N>, QueueA, RoleA<RoleEnd>>;

#[test]
fn sessionmpst_fields() {
    let (sessionmpst_1, sessionmpst_2) = Endpoint::<i32>::new();

    // sessionmpst_1
    let (here1_stack, there1_stack) = RoleC::<RoleEnd>::new();
    let (here1_name, there1_name) = RoleEnd::new();
    sessionmpst_1.stack.sender.send(there1_stack).unwrap_or(());
    sessionmpst_1.name.sender.send(there1_name).unwrap_or(());

    assert_eq!(here1_stack.sender.send(RoleEnd::new().1).unwrap_or(()), ());
    assert_eq!(here1_name.sender.send(()).unwrap_or(()), ());

    // sessionmpst_2
    let (here2_stack, there2_stack) = RoleC::<RoleEnd>::new();
    let (here2_name, there2_name) = RoleEnd::new();
    sessionmpst_2.stack.sender.send(here2_stack).unwrap_or(());
    sessionmpst_2.name.sender.send(here2_name).unwrap_or(());

    assert_eq!(there2_stack.sender.send(RoleEnd::new().1).unwrap_or(()), ());
    assert_eq!(there2_name.sender.send(()).unwrap_or(()), ());
}

#[test]
fn sessionmpst_methods() {
    assert_eq!(
        Endpoint::<i32>::head_str(),
        format!(
            "{} + {} + {} + {}",
            AtoB::<i32>::head_str(),
            AtoC::<i32>::head_str(),
            QueueA::head_str(),
            RoleA::<RoleEnd>::head_str()
        )
    );

    assert_eq!(
        Endpoint::<i32>::tail_str(),
        format!(
            "{} + {} + {} + {}",
            AtoB::<i32>::tail_str(),
            AtoC::<i32>::tail_str(),
            QueueA::tail_str(),
            RoleA::<RoleEnd>::tail_str()
        )
    );
}
