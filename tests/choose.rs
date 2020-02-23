extern crate mpst;

use std::boxed::Box;
use std::error::Error;

use mpst::functionmpst::close::close_mpst;

use mpst::binary::{End, Recv, Session};
use mpst::run_processes;
use mpst::sessionmpst::SessionMpst;

use mpst::role::a_to_b::RoleAtoB;
use mpst::role::b_to_a::RoleBtoA;
use mpst::role::b_to_all::RoleBtoAll;
use mpst::role::c_to_b::RoleCtoB;
use mpst::role::end::RoleEnd;

use mpst::functionmpst::recv::recv_mpst_a_to_b;

use mpst::functionmpst::send::send_mpst_b_to_a;

use mpst::functionmpst::offer::offer_mpst_session_a_to_b;
use mpst::functionmpst::offer::offer_mpst_session_c_to_b;

use mpst::functionmpst::choose::choose_left_mpst_session_b_to_all;
use mpst::functionmpst::choose::choose_right_mpst_session_b_to_all;

use mpst::functionmpst::ChooseMpst;
use mpst::functionmpst::OfferMpst;

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
type QueueFullA = RoleAtoB<QueueOfferA>;

type QueueChoiceB = RoleBtoA<RoleEnd>;
type QueueFullB = RoleBtoAll<QueueChoiceB, QueueChoiceB>;

type QueueOfferC = RoleEnd;
type QueueFullC = RoleCtoB<QueueOfferC>;

/// Creating the MP sessions
/// For A
type EndpointAAdd<N> = SessionMpst<AtoBAdd<N>, End, QueueOfferA>;
type EndpointANeg<N> = SessionMpst<AtoBNeg<N>, End, QueueOfferA>;

type OfferA<N> = OfferMpst<EndpointAAdd<N>, EndpointANeg<N>>;
type EndpointChoiceA<N> = SessionMpst<OfferA<N>, End, QueueFullA>;

/// For B
type EndpointBAdd<N> = SessionMpst<BtoAAdd<N>, End, QueueChoiceB>;
type EndpointBNeg<N> = SessionMpst<BtoANeg<N>, End, QueueChoiceB>;

type EndpointBEnd = SessionMpst<End, End, RoleEnd>;

type ChooseBtoA<N> = ChooseMpst<EndpointBAdd<N>, EndpointBNeg<N>>;
type ChooseBtoC = ChooseMpst<EndpointBEnd, EndpointBEnd>;
type EndpointChoiceB<N> = SessionMpst<ChooseBtoA<N>, ChooseBtoC, QueueFullB>;

/// For C
type EndpointCAdd = SessionMpst<End, End, QueueOfferC>;
type EndpointCNeg = SessionMpst<End, End, QueueOfferC>;

type OfferC = OfferMpst<EndpointCAdd, EndpointCNeg>;
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
            End,
            BtoANeg<i32>,
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
            End,
            BtoANeg<i32>,
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
fn simple_store_works() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test the left branch.
        {
            let (thread_a, thread_b, thread_c) = run_processes(
                simple_store_server,
                simple_store_client_left,
                simple_store_pawn,
            );

            thread_a.unwrap();
            thread_b.unwrap();
            thread_c.unwrap();
        }

        // Test the right branch.
        {
            let (thread_a, thread_b, thread_c) = run_processes(
                simple_store_server,
                simple_store_client_right,
                simple_store_pawn,
            );

            thread_a.unwrap();
            thread_b.unwrap();
            thread_c.unwrap();
        }

        Ok(())
    }()
    .is_ok());
}
