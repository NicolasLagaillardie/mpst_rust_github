extern crate mpstthree;

use std::boxed::Box;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::binary::{End, Recv, Send, Session};
use mpstthree::role::Role;
use mpstthree::run_processes;
use mpstthree::sessionmpst::SessionMpst;

use mpstthree::role::a_to_b::RoleAtoB;
use mpstthree::role::a_to_c::RoleAtoC;
use mpstthree::role::b_to_a::RoleBtoA;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c_to_a::RoleCtoA;
use mpstthree::role::c_to_b::RoleCtoB;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_to_b;
use mpstthree::functionmpst::recv::recv_mpst_c_to_a;

use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_b_to_a;

use mpstthree::functionmpst::offer::offer_mpst_session_a_to_b;
use mpstthree::functionmpst::offer::offer_mpst_session_c_to_b;

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
type EndpointAAdd<N> = SessionMpst<AtoBAdd<N>, AtoCAdd<N>, QueueOfferA>;
type EndpointANeg<N> = SessionMpst<AtoBNeg<N>, AtoCNeg<N>, QueueOfferA>;

type OfferA<N> =
    OfferMpst<AtoBAdd<N>, AtoCAdd<N>, AtoBNeg<N>, AtoCNeg<N>, QueueOfferA, QueueOfferA>;
type EndpointChoiceA<N> = SessionMpst<OfferA<N>, End, QueueFullA>;

/// For B
type ChooseBtoA<N> =
    ChooseMpst<BtoAAdd<N>, CtoAAdd<N>, BtoANeg<N>, CtoANeg<N>, QueueOfferADual, QueueOfferADual>;
type ChooseBtoC<N> = ChooseMpst<AtoCAdd<N>, End, AtoCNeg<N>, End, QueueOfferCDual, QueueOfferCDual>;
type EndpointChoiceB<N> = SessionMpst<ChooseBtoA<N>, ChooseBtoC<N>, QueueFullB>;

/// For C
type EndpointCAdd<N> = SessionMpst<CtoAAdd<N>, End, QueueOfferC>;
type EndpointCNeg<N> = SessionMpst<CtoANeg<N>, End, QueueOfferC>;

type OfferC<N> = OfferMpst<CtoAAdd<N>, End, CtoANeg<N>, End, QueueOfferC, QueueOfferC>;
type EndpointChoiceC<N> = SessionMpst<End, OfferC<N>, QueueFullC>;

/// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_a_to_b(
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
    offer_mpst_session_c_to_b(
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
fn simple_store_2_works() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test the left branch.
        {
            let (thread_a, thread_b, thread_c) = run_processes(
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
            let (thread_a, thread_b, thread_c) = run_processes(
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
