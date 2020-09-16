extern crate mpstthree;
use mpstthree::checking::checker;

use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::fork_mpst;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
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
type AtoBNeg<N> = Recv<N, End>;
type AtoBAdd<N> = Recv<N, End>;

type AtoCNeg<N> = Send<N, End>;
type AtoCAdd<N> = Send<N, End>;

type BtoANeg<N> = <AtoBNeg<N> as Session>::Dual;
type BtoAAdd<N> = <AtoBAdd<N> as Session>::Dual;

type CtoANeg<N> = <AtoCNeg<N> as Session>::Dual;
type CtoAAdd<N> = <AtoCAdd<N> as Session>::Dual;

/// Queues
type QueueOfferA = RoleB<RoleC<RoleEnd>>;
type QueueOfferADual = <QueueOfferA as Role>::Dual;
type QueueFullA = RoleAlltoB<QueueOfferA, QueueOfferA>;

type QueueChoiceB = RoleA<RoleEnd>;
type QueueFullB = RoleBtoAll<QueueChoiceB, QueueChoiceB>;

type QueueOfferC = RoleA<RoleEnd>;
type QueueOfferCDual = <QueueOfferC as Role>::Dual;
type QueueFullC = RoleAlltoB<QueueOfferC, QueueOfferC>;

/// Creating the MP sessions
/// For A
type EndpointAAdd<N> = SessionMpst<AtoBAdd<N>, AtoCAdd<N>, QueueOfferA, RoleA<RoleEnd>>;
type EndpointANeg<N> = SessionMpst<AtoBNeg<N>, AtoCNeg<N>, QueueOfferA, RoleA<RoleEnd>>;

type OfferA<N> = OfferMpst<
    AtoBAdd<N>,
    AtoCAdd<N>,
    AtoBNeg<N>,
    AtoCNeg<N>,
    QueueOfferA,
    QueueOfferA,
    RoleA<RoleEnd>,
>;
type EndpointChoiceA<N> = SessionMpst<OfferA<N>, End, QueueFullA, RoleA<RoleEnd>>;

/// For B
type ChooseBtoA<N> = ChooseMpst<
    BtoAAdd<N>,
    CtoAAdd<N>,
    BtoANeg<N>,
    CtoANeg<N>,
    QueueOfferADual,
    QueueOfferADual,
    RoleADual<RoleEnd>,
>;
type ChooseBtoC<N> = ChooseMpst<
    AtoCAdd<N>,
    End,
    AtoCNeg<N>,
    End,
    QueueOfferCDual,
    QueueOfferCDual,
    RoleCDual<RoleEnd>,
>;
type EndpointChoiceB<N> = SessionMpst<ChooseBtoA<N>, ChooseBtoC<N>, QueueFullB, RoleB<RoleEnd>>;

/// For C
type EndpointCAdd<N> = SessionMpst<CtoAAdd<N>, End, QueueOfferC, RoleC<RoleEnd>>;
type EndpointCNeg<N> = SessionMpst<CtoANeg<N>, End, QueueOfferC, RoleC<RoleEnd>>;

type OfferC<N> =
    OfferMpst<CtoAAdd<N>, End, CtoANeg<N>, End, QueueOfferC, QueueOfferC, RoleC<RoleEnd>>;
type EndpointChoiceC<N> = SessionMpst<End, OfferC<N>, QueueFullC, RoleC<RoleEnd>>;

/// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_a_from_b(
        s,
        |s: EndpointAAdd<i32>| {
            let (x, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_c(x + 1, s);
            close_mpst(s)?;

            assert_eq!(x, 1);
            Ok(())
        },
        |s: EndpointANeg<i32>| {
            let (x, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_c(x + 1, s);
            close_mpst(s)?;

            assert_eq!(x, 2);
            Ok(())
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    {
        let s = choose_left_mpst_session_b_to_all::<
            CtoAAdd<i32>,
            CtoANeg<i32>,
            BtoAAdd<i32>,
            End,
            BtoANeg<i32>,
            End,
            QueueOfferADual,
            QueueOfferADual,
            QueueOfferCDual,
            QueueOfferCDual,
            QueueChoiceB,
            QueueChoiceB,
        >(s);
        let s = send_mpst_b_to_a(1, s);
        close_mpst(s)?;
    }
    Ok(())
}

fn simple_store_client_right(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    {
        let s = choose_right_mpst_session_b_to_all::<
            CtoAAdd<i32>,
            CtoANeg<i32>,
            BtoAAdd<i32>,
            End,
            BtoANeg<i32>,
            End,
            QueueOfferADual,
            QueueOfferADual,
            QueueOfferCDual,
            QueueOfferCDual,
            QueueChoiceB,
            QueueChoiceB,
        >(s);
        let s = send_mpst_b_to_a(2, s);
        close_mpst(s)?;
    }
    Ok(())
}

fn simple_store_pawn(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_c_from_b(
        s,
        |s: EndpointCAdd<i32>| {
            let (x, s) = recv_mpst_c_to_a(s)?;
            close_mpst(s)?;

            assert_eq!(x, 2);
            Ok(())
        },
        |s: EndpointCNeg<i32>| {
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
            let hm: HashMap<String, &Vec<String>> = HashMap::new();

            let (s1, _): (EndpointChoiceA<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointChoiceB<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointChoiceC<i32>, _) = SessionMpst::new();

            let (a, b, c) = checker(s1, s2, s3, &hm)?;

            assert_eq!(a, "A: ( A?B.A!C.0 & A?B.A!C.0 )");
            assert_eq!(b, "B: ( B!A.0 + B!A.0 )");
            assert_eq!(c, "C: ( C?A.0 & C?A.0 )");
        }
        Ok(())
    }()
    .is_ok());
}
