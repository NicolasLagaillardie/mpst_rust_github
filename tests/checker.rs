extern crate mpstthree;
use mpstthree::checking::checker;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;

use mpstthree::role::a_to_b::RoleAtoB;
use mpstthree::role::a_to_c::RoleAtoC;
use mpstthree::role::b_to_a::RoleBtoA;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c_to_a::RoleCtoA;
use mpstthree::role::c_to_b::RoleCtoB;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// Test a simple storage server, implemented using binary choice.
/// Simple types
type AtoBNeg<N> = Recv<N, End>;
type AtoBAdd<N> = Recv<N, End>;

type AtoCNeg<N> = Send<N, End>;
type AtoCAdd<N> = Send<N, End>;

type BtoANeg<N> = <AtoBNeg<N> as Session>::Dual;
type BtoAAdd<N> = <AtoBAdd<N> as Session>::Dual;

type CtoANeg<N> = <AtoCNeg<N> as Session>::Dual;
type CtoAAdd<N> = <AtoCAdd<N> as Session>::Dual;

/// Queues
type QueueOfferA = RoleAtoB<RoleAtoC<RoleEnd>>;
type QueueOfferADual = <QueueOfferA as Role>::Dual;
type QueueFullA = RoleAtoB<QueueOfferA>;

type QueueChoiceB = RoleBtoA<RoleEnd>;
type QueueFullB = RoleBtoAll<QueueChoiceB, QueueChoiceB>;

type QueueOfferC = RoleCtoA<RoleEnd>;
type QueueOfferCDual = <QueueOfferC as Role>::Dual;
type QueueFullC = RoleCtoB<QueueOfferC>;

/// Creating the MP sessions
/// For A
type OfferA<N> =
    OfferMpst<AtoBAdd<N>, AtoCAdd<N>, AtoBNeg<N>, AtoCNeg<N>, QueueOfferA, QueueOfferA>;
type EndpointChoiceA<N> = SessionMpst<OfferA<N>, End, QueueFullA>;

/// For B
type ChooseBtoA<N> =
    ChooseMpst<BtoAAdd<N>, CtoAAdd<N>, BtoANeg<N>, CtoANeg<N>, QueueOfferADual, QueueOfferADual>;
type ChooseBtoC<N> = ChooseMpst<AtoCAdd<N>, End, AtoCNeg<N>, End, QueueOfferCDual, QueueOfferCDual>;
type EndpointChoiceB<N> = SessionMpst<ChooseBtoA<N>, ChooseBtoC<N>, QueueFullB>;

/// For C

type OfferC<N> = OfferMpst<CtoAAdd<N>, End, CtoANeg<N>, End, QueueOfferC, QueueOfferC>;
type EndpointChoiceC<N> = SessionMpst<End, OfferC<N>, QueueFullC>;

#[test]
fn test_checker() {
    let (s1, _): (EndpointChoiceA<i32>, _) = SessionMpst::new();
    let (s2, _): (EndpointChoiceB<i32>, _) = SessionMpst::new();
    let (s3, _): (EndpointChoiceC<i32>, _) = SessionMpst::new();

    let result = checker(s1, s2, s3);

    assert!(result.is_ok());
}
