// A → X → B
// B → Y → C
// C → Z → A

use mpstthree::checking::checker;

use std::boxed::Box;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::fork::fork_mpst;
use mpstthree::role::Role;
use mpstthree::sessionmpst::SessionMpst;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_to_all::RoleAtoAll;
use mpstthree::role::all_to_a::RoleAlltoA;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_dual::RoleBDual;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// Test a simple storage server, implemented using binary
/// choice. Simple types
type CtoANeg<N> = Recv<N, End>;
type CtoAAdd<N> = Recv<N, End>;

type CtoBNeg<N> = Send<N, End>;
type CtoBAdd<N> = Send<N, End>;

type AtoCNeg<N> = <CtoANeg<N> as Session>::Dual;
type AtoCAdd<N> = <CtoAAdd<N> as Session>::Dual;

type BtoCNeg<N> = <CtoBNeg<N> as Session>::Dual;
type BtoCAdd<N> = <CtoBAdd<N> as Session>::Dual;

/// Queues
type QueueOfferC = RoleA<RoleB<RoleEnd>>;
type QueueOfferCDual = <QueueOfferC as Role>::Dual;
type QueueFullC = RoleAlltoA<RoleEnd, RoleEnd>;

type QueueChoiceA = RoleC<RoleEnd>;
type QueueFullA = RoleAtoAll<QueueChoiceA, QueueChoiceA>;

type QueueOfferB = RoleC<RoleEnd>;
type QueueOfferBDual = <QueueOfferB as Role>::Dual;
type QueueFullB = RoleAlltoA<RoleEnd, RoleEnd>;

/// Creating the MP sessions
/// For C
type EndpointCAdd<N> = SessionMpst<CtoAAdd<N>, CtoBAdd<N>, QueueOfferC, RoleC<RoleEnd>>;
type EndpointCNeg<N> = SessionMpst<CtoANeg<N>, CtoBNeg<N>, QueueOfferC, RoleC<RoleEnd>>;

type OfferC<N> = OfferMpst<
    CtoAAdd<N>,
    CtoBAdd<N>,
    CtoANeg<N>,
    CtoBNeg<N>,
    QueueOfferC,
    QueueOfferC,
    RoleC<RoleEnd>,
>;
type EndpointChoiceC<N> = SessionMpst<OfferC<N>, End, QueueFullC, RoleC<RoleEnd>>;

/// For A
type ChooseAtoC<N> = ChooseMpst<
    AtoCAdd<N>,
    BtoCAdd<N>,
    AtoCNeg<N>,
    BtoCNeg<N>,
    QueueOfferCDual,
    QueueOfferCDual,
    RoleCDual<RoleEnd>,
>;
type ChooseCtoA<N> = ChooseMpst<
    End,
    CtoBAdd<N>,
    End,
    CtoBNeg<N>,
    QueueOfferBDual,
    QueueOfferBDual,
    RoleBDual<RoleEnd>,
>;
type EndpointChoiceA<N> = SessionMpst<ChooseCtoA<N>, ChooseAtoC<N>, QueueFullA, RoleA<RoleEnd>>;

/// For B
type EndpointBAdd<N> = SessionMpst<End, BtoCAdd<N>, QueueOfferB, RoleB<RoleEnd>>;
type EndpointBNeg<N> = SessionMpst<End, BtoCNeg<N>, QueueOfferB, RoleB<RoleEnd>>;

type OfferA<N> =
    OfferMpst<End, BtoCAdd<N>, End, BtoCNeg<N>, QueueOfferB, QueueOfferB, RoleB<RoleEnd>>;
type EndpointChoiceB<N> = SessionMpst<OfferA<N>, End, QueueFullB, RoleB<RoleEnd>>;

/// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointCAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 1);

            s.send(x + 1).close()
        },
        |s: EndpointCNeg<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.send(x + 1).close()
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_left().send(1).close()
}

fn simple_store_client_right(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_right().send(2).close()
}

fn simple_store_pawn(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointBAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.close()
        },
        |s: EndpointBNeg<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 3);

            s.close()
        },
    )
}

/////////////////////////////////////////

pub fn double_choice() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test the left branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_client_left,
                simple_store_pawn,
                simple_store_server,
            );

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        // Test the right branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_client_right,
                simple_store_pawn,
                simple_store_server,
            );

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn double_choice_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let s = RandomState::new();
            let hm: HashMap<String, &Vec<String>> = HashMap::with_hasher(s);

            let (s1, _): (EndpointChoiceA<i32>, _) = SessionMpst::new();
            let (s2, _): (EndpointChoiceB<i32>, _) = SessionMpst::new();
            let (s3, _): (EndpointChoiceC<i32>, _) = SessionMpst::new();

            let (a, b, c) = checker(s1, s2, s3, &hm, &HashMap::new())?;

            assert_eq!(a, "A: ( A!C.0 + A!C.0 )");
            assert_eq!(b, "B: ( B?C.0 & B?C.0 )");
            assert_eq!(c, "C: ( C?A.C!B.0 & C?A.C!B.0 )");
        }
        Ok(())
    }()
    .is_ok());
}
