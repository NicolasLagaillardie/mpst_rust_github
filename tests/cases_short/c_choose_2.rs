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
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

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

/// Stacks
type StackOfferC = RoleA<RoleB<RoleEnd>>;
type StackOfferCDual = <StackOfferC as Role>::Dual;
type StackFullC = RoleAlltoA<RoleEnd, RoleEnd>;

type StackChoiceA = RoleC<RoleEnd>;
type StackFullA = RoleAtoAll<StackChoiceA, StackChoiceA>;

type StackOfferB = RoleC<RoleEnd>;
type StackOfferBDual = <StackOfferB as Role>::Dual;
type StackFullB = RoleAlltoA<RoleEnd, RoleEnd>;

/// Creating the MP sessions
/// For C
type EndpointCAdd<N> = MeshedChannels<CtoAAdd<N>, CtoBAdd<N>, StackOfferC, RoleC<RoleEnd>>;
type EndpointCNeg<N> = MeshedChannels<CtoANeg<N>, CtoBNeg<N>, StackOfferC, RoleC<RoleEnd>>;

type OfferC<N> = OfferMpst<
    CtoAAdd<N>,
    CtoBAdd<N>,
    CtoANeg<N>,
    CtoBNeg<N>,
    StackOfferC,
    StackOfferC,
    RoleC<RoleEnd>,
>;
type EndpointChoiceC<N> = MeshedChannels<OfferC<N>, End, StackFullC, RoleC<RoleEnd>>;

/// For A
type ChooseAtoC<N> = ChooseMpst<
    AtoCAdd<N>,
    BtoCAdd<N>,
    AtoCNeg<N>,
    BtoCNeg<N>,
    StackOfferCDual,
    StackOfferCDual,
    RoleCDual<RoleEnd>,
>;
type ChooseCtoA<N> = ChooseMpst<
    End,
    CtoBAdd<N>,
    End,
    CtoBNeg<N>,
    StackOfferBDual,
    StackOfferBDual,
    RoleBDual<RoleEnd>,
>;
type EndpointChoiceA<N> = MeshedChannels<ChooseCtoA<N>, ChooseAtoC<N>, StackFullA, RoleA<RoleEnd>>;

/// For B
type EndpointBAdd<N> = MeshedChannels<End, BtoCAdd<N>, StackOfferB, RoleB<RoleEnd>>;
type EndpointBNeg<N> = MeshedChannels<End, BtoCNeg<N>, StackOfferB, RoleB<RoleEnd>>;

type OfferA<N> =
    OfferMpst<End, BtoCAdd<N>, End, BtoCNeg<N>, StackOfferB, StackOfferB, RoleB<RoleEnd>>;
type EndpointChoiceB<N> = MeshedChannels<OfferA<N>, End, StackFullB, RoleB<RoleEnd>>;

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

            let (s1, _): (EndpointChoiceA<i32>, _) = MeshedChannels::new();
            let (s2, _): (EndpointChoiceB<i32>, _) = MeshedChannels::new();
            let (s3, _): (EndpointChoiceC<i32>, _) = MeshedChannels::new();

            let (a, b, c) = checker(s1, s2, s3, &hm, &HashMap::new())?;

            assert_eq!(a, "A: ( A!C.0 + A!C.0 )");
            assert_eq!(b, "B: ( B?C.0 & B?C.0 )");
            assert_eq!(c, "C: ( C?A.C!B.0 & C?A.C!B.0 )");
        }
        Ok(())
    }()
    .is_ok());
}
