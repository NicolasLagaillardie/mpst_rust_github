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
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::all_to_c::RoleAlltoC;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_dual::RoleBDual;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_to_all::RoleCtoAll;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// Test a simple storage server, implemented using binary
/// choice. Simple types
type BtoCNeg<N> = Recv<N, End>;
type BtoCAdd<N> = Recv<N, End>;

type BtoANeg<N> = Send<N, End>;
type BtoAAdd<N> = Send<N, End>;

type CtoBNeg<N> = <BtoCNeg<N> as Session>::Dual;
type CtoBAdd<N> = <BtoCAdd<N> as Session>::Dual;

type AtoBNeg<N> = <BtoANeg<N> as Session>::Dual;
type AtoBAdd<N> = <BtoAAdd<N> as Session>::Dual;

/// Stacks
type StackOfferB = RoleC<RoleA<RoleEnd>>;
type StackOfferBDual = <StackOfferB as Role>::Dual;
type StackFullB = RoleAlltoC<RoleEnd, RoleEnd>;

type StackChoiceC = RoleB<RoleEnd>;
type StackFullC = RoleCtoAll<StackChoiceC, StackChoiceC>;

type StackOfferA = RoleB<RoleEnd>;
type StackOfferADual = <StackOfferA as Role>::Dual;
type StackFullA = RoleAlltoC<RoleEnd, RoleEnd>;

/// Creating the MP sessions
/// For B
type EndpointBAdd<N> = MeshedChannels<BtoAAdd<N>, BtoCAdd<N>, StackOfferB, RoleB<RoleEnd>>;
type EndpointBNeg<N> = MeshedChannels<BtoANeg<N>, BtoCNeg<N>, StackOfferB, RoleB<RoleEnd>>;

type OfferB<N> = OfferMpst<
    BtoAAdd<N>,
    BtoCAdd<N>,
    BtoANeg<N>,
    BtoCNeg<N>,
    StackOfferB,
    StackOfferB,
    RoleB<RoleEnd>,
>;
type EndpointChoiceB<N> = MeshedChannels<End, OfferB<N>, StackFullB, RoleB<RoleEnd>>;

/// For C
type ChooseCtoB<N> = ChooseMpst<
    AtoBAdd<N>,
    CtoBAdd<N>,
    AtoBNeg<N>,
    CtoBNeg<N>,
    StackOfferBDual,
    StackOfferBDual,
    RoleBDual<RoleEnd>,
>;
type ChooseCtoA<N> = ChooseMpst<
    BtoAAdd<N>,
    End,
    BtoANeg<N>,
    End,
    StackOfferADual,
    StackOfferADual,
    RoleADual<RoleEnd>,
>;
type EndpointChoiceC<N> = MeshedChannels<ChooseCtoA<N>, ChooseCtoB<N>, StackFullC, RoleC<RoleEnd>>;

/// For A
type EndpointAAdd<N> = MeshedChannels<AtoBAdd<N>, End, StackOfferA, RoleA<RoleEnd>>;
type EndpointANeg<N> = MeshedChannels<AtoBNeg<N>, End, StackOfferA, RoleA<RoleEnd>>;

type OfferA<N> =
    OfferMpst<AtoBAdd<N>, End, AtoBNeg<N>, End, StackOfferA, StackOfferA, RoleA<RoleEnd>>;
type EndpointChoiceA<N> = MeshedChannels<End, OfferA<N>, StackFullA, RoleA<RoleEnd>>;

/// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointBAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 1);

            s.send(x + 1).close()
        },
        |s: EndpointBNeg<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.send(x + 1).close()
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_left().send(1).close()
}

fn simple_store_client_right(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_right().send(2).close()
}

fn simple_store_pawn(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointAAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.close()
        },
        |s: EndpointANeg<i32>| {
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

pub fn double_choice_checker() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let s = RandomState::new();
            let hm: HashMap<String, &Vec<String>> = HashMap::with_hasher(s);

            let (s1, _): (EndpointChoiceA<i32>, _) = MeshedChannels::new();
            let (s2, _): (EndpointChoiceB<i32>, _) = MeshedChannels::new();
            let (s3, _): (EndpointChoiceC<i32>, _) = MeshedChannels::new();

            let (a, b, c) = checker(s1, s2, s3, &hm, &HashMap::new())?;

            assert_eq!(a, "A: ( A?B.0 & A?B.0 )");
            assert_eq!(b, "B: ( B?C.B!A.0 & B?C.B!A.0 )");
            assert_eq!(c, "C: ( C!B.0 + C!B.0 )");
        }
        Ok(())
    }()
    .is_ok());
}
