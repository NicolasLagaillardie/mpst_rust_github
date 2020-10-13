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

    if let Err(err) = sessionmpst_1.name.sender.send(RoleEnd::new().1) {
        assert_eq!(err.into_inner().sender.send(()), Ok(()));
    }

    if let Err(err) = sessionmpst_2.name.sender.send(RoleEnd::new().1) {
        assert_eq!(err.into_inner().sender.send(()), Ok(()));
    }
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
