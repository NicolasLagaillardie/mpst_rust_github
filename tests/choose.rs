extern crate mpstthree;
use mpstthree::checking::checker;

use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::binary::{End, Recv, Session};
use mpstthree::fork_mpst;
use mpstthree::sessionmpst::SessionMpst;

use mpstthree::role::a_to_b::RoleAtoB;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::b_to_a::RoleBtoA;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_to_b;

use mpstthree::functionmpst::send::send_mpst_b_to_a;

use mpstthree::functionmpst::offer::offer_mpst_session_a_to_b;
use mpstthree::functionmpst::offer::offer_mpst_session_c_to_b;

use mpstthree::functionmpst::choose::choose_left_mpst_session_b_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_b_to_all;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// A: ( A?B.0 + A?B.0 )
/// B: ( B!A.0 + B!A.0 )
/// C: ( 0 + 0 )

/// Test a simple storage server, implemented using binary choice.
/// Simple types
type AtoBNeg<N> = Recv<N, End>;
type AtoBAdd<N> = Recv<N, End>;

// type BtoANeg<N> = Send<N, End>;
// type BtoAAdd<N> = Send<N, End>;
type BtoANeg<N> = <AtoBNeg<N> as Session>::Dual;
type BtoAAdd<N> = <AtoBAdd<N> as Session>::Dual;

/// Queues
type QueueOfferA = RoleAtoB<RoleEnd>;
type QueueFullA = RoleAlltoB<QueueOfferA, QueueOfferA>;

type QueueChoiceB = RoleBtoA<RoleEnd>;
type QueueFullB = RoleBtoAll<QueueChoiceB, QueueChoiceB>;

type QueueOfferC = RoleEnd;
type QueueFullC = RoleAlltoB<QueueOfferC, QueueOfferC>;

/// Creating the MP sessions
/// For A
type EndpointAAdd<N> = SessionMpst<AtoBAdd<N>, End, QueueOfferA>;
type EndpointANeg<N> = SessionMpst<AtoBNeg<N>, End, QueueOfferA>;

type OfferA<N> = OfferMpst<AtoBAdd<N>, End, AtoBNeg<N>, End, QueueOfferA, QueueOfferA>;
type EndpointChoiceA<N> = SessionMpst<OfferA<N>, End, QueueFullA>;

/// For B
type ChooseBtoA<N> = ChooseMpst<BtoAAdd<N>, End, BtoANeg<N>, End, QueueChoiceB, QueueChoiceB>;
type ChooseBtoC = ChooseMpst<End, End, End, End, RoleEnd, RoleEnd>;
type EndpointChoiceB<N> = SessionMpst<ChooseBtoA<N>, ChooseBtoC, QueueFullB>;

/// For C
type EndpointCAdd = SessionMpst<End, End, QueueOfferC>;
type EndpointCNeg = SessionMpst<End, End, QueueOfferC>;

type OfferC = OfferMpst<End, End, End, End, QueueOfferC, QueueOfferC>;
type EndpointChoiceC = SessionMpst<End, OfferC, QueueFullC>;

/// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_a_to_b(
        s,
        |s: EndpointAAdd<i32>| {
            let (x, s) = recv_mpst_a_to_b(s)?;
            close_mpst(s)?;

            assert_eq!(x, 1);
            Ok(())
        },
        |s: EndpointANeg<i32>| {
            let (x, s) = recv_mpst_a_to_b(s)?;
            close_mpst(s)?;

            assert_eq!(x, 2);
            Ok(())
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    {
        let s = choose_left_mpst_session_b_to_all::<
            End,
            End,
            BtoAAdd<i32>,
            End,
            BtoANeg<i32>,
            End,
            QueueChoiceB,
            QueueChoiceB,
            RoleEnd,
            RoleEnd,
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
            End,
            End,
            BtoAAdd<i32>,
            End,
            BtoANeg<i32>,
            End,
            QueueChoiceB,
            QueueChoiceB,
            RoleEnd,
            RoleEnd,
            QueueChoiceB,
            QueueChoiceB,
        >(s);
        let s = send_mpst_b_to_a(2, s);
        close_mpst(s)?;
    }
    Ok(())
}

fn simple_store_pawn(s: EndpointChoiceC) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_c_to_b(
        s,
        |s: EndpointCAdd| {
            close_mpst(s)?;
            Ok(())
        },
        |s: EndpointCNeg| {
            close_mpst(s)?;
            Ok(())
        },
    )
}

#[test]
fn simple_choice() {
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

    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let hm: HashMap<String, &Vec<String>> = HashMap::new();

            let (s1, _): (EndpointChoiceA<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointChoiceB<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointChoiceC, _) = SessionMpst::new();

            let (a, b, c) = checker(s1, s2, s3, &hm)?;

            assert_eq!(a, "A: ( A?B.0 & A?B.0 )");
            assert_eq!(b, "B: ( B!A.0 + B!A.0 )");
            assert_eq!(c, "C: ( 0 & 0 )");
        }
        Ok(())
    }()
    .is_ok());
}
