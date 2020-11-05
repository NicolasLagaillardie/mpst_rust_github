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
    let (here1, there1) = RoleC::new();
    sessionmpst_1.stack.sender.send(there1).unwrap_or(());

    assert_eq!(here1.sender.send(RoleEnd::new().1).unwrap_or(()), ());

    // sessionmpst_2
    let (here2, there2) = RoleC::new();
    sessionmpst_2.stack.sender.send(here2).unwrap_or(());

    assert_eq!(there2.sender.send(RoleEnd::new().1).unwrap_or(()), ());
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
