extern crate mpstthree;
use mpstthree::checking::checker;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::sessionmpst::SessionMpst;

use std::marker;

use mpstthree::role::a_to_b::RoleAtoB;
use mpstthree::role::a_to_c::RoleAtoC;
use mpstthree::role::b_to_a::RoleBtoA;
use mpstthree::role::b_to_c::RoleBtoC;
use mpstthree::role::c_to_a::RoleCtoA;
use mpstthree::role::c_to_b::RoleCtoB;
use mpstthree::role::end::RoleEnd;

/// Test our usecase from Places 2020
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

/// A: A?C.A!C.μX.( 0 & A?C.A?C.A!B.A?B.A!C.X )
/// B: μX.( 0 & B?A.B!A.X )
/// C: C!A.C?A.μX.( 0 + C!A.C?A.X )

type AtoCClose = End;
type AtoBClose = End;
type AtoBVideo<N> = Send<N, Recv<N, End>>;

type InitA<N> = Recv<N, Send<N, RecursAtoC<N>>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type RecursAtoC<N> = Recv<CBranchesAtoC<N>, End>;
type RecursBtoC<N> = Recv<CBranchesBtoC<N>, End>;

enum CBranchesAtoC<N: marker::Send> {
    End(SessionMpst<AtoBClose, AtoCClose, QueueAEnd>),
    Video(SessionMpst<AtoBVideo<N>, Recv<N, Send<N, RecursAtoC<N>>>, QueueAVideo>),
}
enum CBranchesBtoC<N: marker::Send> {
    End(SessionMpst<BtoAClose, BtoCClose, QueueBEnd>),
    Video(SessionMpst<BtoAVideo<N>, RecursBtoC<N>, QueueBVideo>),
}
type ChooseCforAtoC<N> = Send<CBranchesAtoC<N>, End>;
type ChooseCforBtoC<N> = Send<CBranchesBtoC<N>, End>;

type InitC<N> = Send<N, Recv<N, ChooseCforAtoC<N>>>;

/// Queues
type QueueAEnd = RoleEnd;
type QueueAVideo = RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleAtoC<RoleEnd>>>>>;
type QueueAInit = RoleAtoC<RoleAtoC<RoleAtoC<RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleBtoA<RoleBtoA<RoleBtoC<RoleEnd>>>;
type QueueBRecurs = RoleBtoC<RoleEnd>;

type QueueCRecurs = RoleCtoA<RoleCtoB<RoleEnd>>;
type QueueCFull = RoleCtoA<RoleCtoA<QueueCRecurs>>;

/// Creating the MP sessions
/// For C

type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCforBtoC<N>, QueueCFull>;

/// For A
type EndpointAFull<N> = SessionMpst<End, InitA<N>, QueueAInit>;

/// For B
type EndpointBRecurs<N> = SessionMpst<End, RecursBtoC<N>, QueueBRecurs>;

#[test]
fn test_checker() {
    let (s1, _): (EndpointAFull<i32>, _) = SessionMpst::new();
    let (s2, _): (EndpointBRecurs<i32>, _) = SessionMpst::new();
    let (s3, _): (EndpointCFull<i32>, _) = SessionMpst::new();

    let result = checker(s1, s2, s3);

    assert!(result.is_ok());
}
