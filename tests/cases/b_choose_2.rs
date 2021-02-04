use mpstthree::checking::checker;

use std::boxed::Box;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::all_to_c::RoleAlltoC;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_dual::RoleBDual;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_to_all::RoleCtoAll;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_to_b;
use mpstthree::functionmpst::recv::recv_mpst_b_to_c;

use mpstthree::functionmpst::send::send_mpst_b_to_a;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::functionmpst::offer::offer_mpst_session_to_a_from_c;
use mpstthree::functionmpst::offer::offer_mpst_session_to_b_from_c;

use mpstthree::functionmpst::choose::choose_left_mpst_session_c_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_c_to_all;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// Test a simple storage server, implemented using binary choice.
/// Simple types
type BtoCNeg<N> = Recv<N, End>;
type BtoCAdd<N> = Recv<N, End>;

type BtoANeg<N> = Send<N, End>;
type BtoAAdd<N> = Send<N, End>;

type CtoBNeg<N> = <BtoCNeg<N> as Session>::Dual;
type CtoBAdd<N> = <BtoCAdd<N> as Session>::Dual;

type AtoBNeg<N> = <BtoANeg<N> as Session>::Dual;
type AtoBAdd<N> = <BtoAAdd<N> as Session>::Dual;

/// Queues
type QueueOfferB = RoleC<RoleA<RoleEnd>>;
type QueueOfferBDual = <QueueOfferB as Role>::Dual;
type QueueFullB = RoleAlltoC<RoleEnd, RoleEnd>;

type QueueChoiceC = RoleB<RoleEnd>;
type QueueFullC = RoleCtoAll<QueueChoiceC, QueueChoiceC>;

type QueueOfferA = RoleB<RoleEnd>;
type QueueOfferADual = <QueueOfferA as Role>::Dual;
type QueueFullA = RoleAlltoC<RoleEnd, RoleEnd>;

/// Creating the MP sessions
/// For B
type EndpointBAdd<N> = SessionMpst<BtoAAdd<N>, BtoCAdd<N>, QueueOfferB, RoleB<RoleEnd>>;
type EndpointBNeg<N> = SessionMpst<BtoANeg<N>, BtoCNeg<N>, QueueOfferB, RoleB<RoleEnd>>;

type OfferB<N> = OfferMpst<
    BtoAAdd<N>,
    BtoCAdd<N>,
    BtoANeg<N>,
    BtoCNeg<N>,
    QueueOfferB,
    QueueOfferB,
    RoleB<RoleEnd>,
>;
type EndpointChoiceB<N> = SessionMpst<End, OfferB<N>, QueueFullB, RoleB<RoleEnd>>;

/// For C
type ChooseCtoB<N> = ChooseMpst<
    AtoBAdd<N>,
    CtoBAdd<N>,
    AtoBNeg<N>,
    CtoBNeg<N>,
    QueueOfferBDual,
    QueueOfferBDual,
    RoleBDual<RoleEnd>,
>;
type ChooseCtoA<N> = ChooseMpst<
    BtoAAdd<N>,
    End,
    BtoANeg<N>,
    End,
    QueueOfferADual,
    QueueOfferADual,
    RoleADual<RoleEnd>,
>;
type EndpointChoiceC<N> = SessionMpst<ChooseCtoA<N>, ChooseCtoB<N>, QueueFullC, RoleC<RoleEnd>>;

/// For A
type EndpointAAdd<N> = SessionMpst<AtoBAdd<N>, End, QueueOfferA, RoleA<RoleEnd>>;
type EndpointANeg<N> = SessionMpst<AtoBNeg<N>, End, QueueOfferA, RoleA<RoleEnd>>;

type OfferA<N> =
    OfferMpst<AtoBAdd<N>, End, AtoBNeg<N>, End, QueueOfferA, QueueOfferA, RoleA<RoleEnd>>;
type EndpointChoiceA<N> = SessionMpst<End, OfferA<N>, QueueFullA, RoleA<RoleEnd>>;

/// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_b_from_c(
        s,
        |s: EndpointBAdd<i32>| {
            let (x, s) = recv_mpst_b_to_c(s)?;
            let s = send_mpst_b_to_a(x + 1, s);

            assert_eq!(x, 1);

            close_mpst(s)
        },
        |s: EndpointBNeg<i32>| {
            let (x, s) = recv_mpst_b_to_c(s)?;
            let s = send_mpst_b_to_a(x + 1, s);

            assert_eq!(x, 2);

            close_mpst(s)
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    let s = choose_left_mpst_session_c_to_all::<
        BtoAAdd<i32>,
        BtoANeg<i32>,
        End,
        CtoBAdd<i32>,
        End,
        CtoBNeg<i32>,
        QueueOfferADual,
        QueueOfferADual,
        QueueOfferBDual,
        QueueOfferBDual,
        QueueChoiceC,
        QueueChoiceC,
    >(s);
    let s = send_mpst_c_to_b(1, s);
    close_mpst(s)
}

fn simple_store_client_right(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    let s = choose_right_mpst_session_c_to_all::<
        BtoAAdd<i32>,
        BtoANeg<i32>,
        End,
        CtoBAdd<i32>,
        End,
        CtoBNeg<i32>,
        QueueOfferADual,
        QueueOfferADual,
        QueueOfferBDual,
        QueueOfferBDual,
        QueueChoiceC,
        QueueChoiceC,
    >(s);
    let s = send_mpst_c_to_b(2, s);
    close_mpst(s)
}

fn simple_store_pawn(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_a_from_c(
        s,
        |s: EndpointAAdd<i32>| {
            let (x, s) = recv_mpst_a_to_b(s)?;

            assert_eq!(x, 2);

            close_mpst(s)
        },
        |s: EndpointANeg<i32>| {
            let (x, s) = recv_mpst_a_to_b(s)?;

            assert_eq!(x, 3);

            close_mpst(s)
        },
    )
}

/////////////////////////////////////////

fn double_choice() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test the left branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_pawn,
                simple_store_server,
                simple_store_client_left,
            );

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        // Test the right branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_pawn,
                simple_store_server,
                simple_store_client_right,
            );

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

fn double_choice_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let s = RandomState::new();
            let hm: HashMap<String, &Vec<String>> = HashMap::with_hasher(s);

            let (s1, _): (EndpointChoiceA<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointChoiceB<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointChoiceC<i32>, _) = SessionMpst::new();

            let (a, b, c) = checker(s1, s2, s3, &hm, &HashMap::new())?;

            assert_eq!(a, "A: ( A?B.0 & A?B.0 )");
            assert_eq!(b, "B: ( B?C.B!A.0 & B?C.B!A.0 )");
            assert_eq!(c, "C: ( C!B.0 + C!B.0 )");
        }
        Ok(())
    }()
    .is_ok());
}

/////////////////////////////////////////

fn main() {
    double_choice();
    double_choice_checker();
}
