// A → X → B
// B → Y → C
// C → Z → A

extern crate mpstthree;
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

use mpstthree::role::a::RoleB;
use mpstthree::role::a_dual::RoleBDual;
use mpstthree::role::all_to_c::RoleAlltoC;
use mpstthree::role::b::RoleY;
use mpstthree::role::b_to_all::RoleZtoXll;
use mpstthree::role::c::RoleZ;
use mpstthree::role::c_dual::RoleZDual;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_to_b;
use mpstthree::functionmpst::recv::recv_mpst_c_to_a;

use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_a;

use mpstthree::functionmpst::offer::offer_mpst_session_to_a_from_b;
use mpstthree::functionmpst::offer::offer_mpst_session_to_c_from_b;

use mpstthree::functionmpst::choose::choose_left_mpst_session_b_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_b_to_all;

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
type QueueOfferB = RoleY<RoleZ<RoleEnd>>;
type QueueOfferBDual = <QueueOfferB as Role>::Dual;
type QueueFullB = RoleAlltoC<RoleEnd, RoleEnd>;

type QueueChoiceC = RoleB<RoleEnd>;
type QueueFullC = RoleZtoXll<QueueChoiceC, QueueChoiceC>;

type QueueOfferA = RoleB<RoleEnd>;
type QueueOfferADual = <QueueOfferA as Role>::Dual;
type QueueFullA = RoleAlltoC<RoleEnd, RoleEnd>;

/// Creating the MP sessions
/// For B
type EndpointBAdd<N> = SessionMpst<BtoCAdd<N>, BtoAAdd<N>, QueueOfferB, RoleB<RoleEnd>>;
type EndpointBNeg<N> = SessionMpst<BtoCNeg<N>, BtoANeg<N>, QueueOfferB, RoleB<RoleEnd>>;

type OfferB<N> = OfferMpst<
    BtoCAdd<N>,
    BtoAAdd<N>,
    BtoCNeg<N>,
    BtoANeg<N>,
    QueueOfferB,
    QueueOfferB,
    RoleB<RoleEnd>,
>;
type EndpointChoiceX<N> = SessionMpst<OfferB<N>, End, QueueFullB, RoleB<RoleEnd>>;

/// For C
type ChooseCtoB<N> = ChooseMpst<
    CtoBAdd<N>,
    AtoBAdd<N>,
    CtoBNeg<N>,
    AtoBNeg<N>,
    QueueOfferBDual,
    QueueOfferBDual,
    RoleBDual<RoleEnd>,
>;
type ChooseCtoZ<N> = ChooseMpst<
    BtoAAdd<N>,
    End,
    BtoANeg<N>,
    End,
    QueueOfferADual,
    QueueOfferADual,
    RoleZDual<RoleEnd>,
>;
type EndpointChoiceC<N> = SessionMpst<ChooseCtoB<N>, ChooseCtoZ<N>, QueueFullC, RoleY<RoleEnd>>;

/// For A
type EndpointAAdd<N> = SessionMpst<AtoBAdd<N>, End, QueueOfferA, RoleZ<RoleEnd>>;
type EndpointANeg<N> = SessionMpst<AtoBNeg<N>, End, QueueOfferA, RoleZ<RoleEnd>>;

type OfferC<N> =
    OfferMpst<AtoBAdd<N>, End, AtoBNeg<N>, End, QueueOfferA, QueueOfferA, RoleZ<RoleEnd>>;
type EndpointChoiceA<N> = SessionMpst<End, OfferC<N>, QueueFullA, RoleZ<RoleEnd>>;

/// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceX<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_a_from_b(
        s,
        |s: EndpointBAdd<i32>| {
            let (x, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_c(x + 1, s);
            close_mpst(s)?;

            assert_eq!(x, 1);
            Ok(())
        },
        |s: EndpointBNeg<i32>| {
            let (x, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_c(x + 1, s);
            close_mpst(s)?;

            assert_eq!(x, 2);
            Ok(())
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    {
        let s = choose_left_mpst_session_b_to_all::<
            AtoBAdd<i32>,
            AtoBNeg<i32>,
            CtoBAdd<i32>,
            End,
            CtoBNeg<i32>,
            End,
            QueueOfferBDual,
            QueueOfferBDual,
            QueueOfferADual,
            QueueOfferADual,
            QueueChoiceC,
            QueueChoiceC,
        >(s);
        let s = send_mpst_b_to_a(1, s);
        close_mpst(s)?;
    }
    Ok(())
}

fn simple_store_client_right(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    {
        let s = choose_right_mpst_session_b_to_all::<
            AtoBAdd<i32>,
            AtoBNeg<i32>,
            CtoBAdd<i32>,
            End,
            CtoBNeg<i32>,
            End,
            QueueOfferBDual,
            QueueOfferBDual,
            QueueOfferADual,
            QueueOfferADual,
            QueueChoiceC,
            QueueChoiceC,
        >(s);
        let s = send_mpst_b_to_a(2, s);
        close_mpst(s)?;
    }
    Ok(())
}

fn simple_store_pawn(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_c_from_b(
        s,
        |s: EndpointAAdd<i32>| {
            let (x, s) = recv_mpst_c_to_a(s)?;
            close_mpst(s)?;

            assert_eq!(x, 2);
            Ok(())
        },
        |s: EndpointANeg<i32>| {
            let (x, s) = recv_mpst_c_to_a(s)?;
            close_mpst(s)?;

            assert_eq!(x, 3);
            Ok(())
        },
    )
}

#[test]
fn double_choice() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test the left branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_server,
                simple_store_client_left,
                simple_store_pawn,
            );

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }

        // Test the right branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_server,
                simple_store_client_right,
                simple_store_pawn,
            );

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

#[test]
fn double_choice_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let s = RandomState::new();
            let hm: HashMap<String, &Vec<String>> = HashMap::with_hasher(s);

            let (s1, _): (EndpointChoiceX<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointChoiceC<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointChoiceA<i32>, _) = SessionMpst::new();

            let (a, b, c) = checker(s1, s2, s3, &hm)?;

            assert_eq!(a, "A: ( A?B.A!C.0 & A?B.A!C.0 )");
            assert_eq!(b, "B: ( B!A.0 + B!A.0 )");
            assert_eq!(c, "C: ( C?A.0 & C?A.0 )");
        }
        Ok(())
    }()
    .is_ok());
}
