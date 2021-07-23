use mpstthree::checking::checker;

use std::boxed::Box;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

/// Test a simple storage server, implemented using binary
/// choice. Simple types
type AtoBNeg<N> = Recv<N, End>;
type AtoBAdd<N> = Recv<N, End>;

type AtoCNeg<N> = Send<N, End>;
type AtoCAdd<N> = Send<N, End>;

type BtoANeg<N> = <AtoBNeg<N> as Session>::Dual;
type BtoAAdd<N> = <AtoBAdd<N> as Session>::Dual;

type CtoANeg<N> = <AtoCNeg<N> as Session>::Dual;
type CtoAAdd<N> = <AtoCAdd<N> as Session>::Dual;

/// Stacks
type StackOfferA = RoleB<RoleC<RoleEnd>>;
type StackOfferADual = <StackOfferA as Role>::Dual;
type StackFullA = RoleAlltoB<RoleEnd, RoleEnd>;

type StackChoiceB = RoleA<RoleEnd>;
type StackFullB = RoleBtoAll<StackChoiceB, StackChoiceB>;

type StackOfferC = RoleA<RoleEnd>;
type StackOfferCDual = <StackOfferC as Role>::Dual;
type StackFullC = RoleAlltoB<RoleEnd, RoleEnd>;

/// Creating the MP sessions
/// For A
type EndpointAAdd<N> = MeshedChannels<AtoBAdd<N>, AtoCAdd<N>, StackOfferA, RoleA<RoleEnd>>;
type EndpointANeg<N> = MeshedChannels<AtoBNeg<N>, AtoCNeg<N>, StackOfferA, RoleA<RoleEnd>>;

type OfferA<N> = OfferMpst<
    AtoBAdd<N>,
    AtoCAdd<N>,
    AtoBNeg<N>,
    AtoCNeg<N>,
    StackOfferA,
    StackOfferA,
    RoleA<RoleEnd>,
>;
type EndpointChoiceA<N> = MeshedChannels<OfferA<N>, End, StackFullA, RoleA<RoleEnd>>;

/// For B
type ChooseBtoA<N> = ChooseMpst<
    BtoAAdd<N>,
    CtoAAdd<N>,
    BtoANeg<N>,
    CtoANeg<N>,
    StackOfferADual,
    StackOfferADual,
    RoleADual<RoleEnd>,
>;
type ChooseBtoC<N> = ChooseMpst<
    AtoCAdd<N>,
    End,
    AtoCNeg<N>,
    End,
    StackOfferCDual,
    StackOfferCDual,
    RoleCDual<RoleEnd>,
>;
type EndpointChoiceB<N> = MeshedChannels<ChooseBtoA<N>, ChooseBtoC<N>, StackFullB, RoleB<RoleEnd>>;

/// For C
type EndpointCAdd<N> = MeshedChannels<CtoAAdd<N>, End, StackOfferC, RoleC<RoleEnd>>;
type EndpointCNeg<N> = MeshedChannels<CtoANeg<N>, End, StackOfferC, RoleC<RoleEnd>>;

type OfferC<N> =
    OfferMpst<CtoAAdd<N>, End, CtoANeg<N>, End, StackOfferC, StackOfferC, RoleC<RoleEnd>>;
type EndpointChoiceC<N> = MeshedChannels<End, OfferC<N>, StackFullC, RoleC<RoleEnd>>;

/// Functions related to endpoints
fn simple_store_server(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointAAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 1);

            s.send(x + 1).close()
        },
        |s: EndpointANeg<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.send(x + 1).close()
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_left().send(1).close()
}

fn simple_store_client_right(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    s.choose_right().send(2).close()
}

fn simple_store_pawn(s: EndpointChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointCAdd<i32>| {
            let (x, s) = s.recv()?;

            assert_eq!(x, 2);

            s.close()
        },
        |s: EndpointCNeg<i32>| {
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
                simple_store_server,
                simple_store_client_left,
                simple_store_pawn,
            );

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        // Test the right branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(
                simple_store_server,
                simple_store_client_right,
                simple_store_pawn,
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

            assert_eq!(a, "A: ( A?B.A!C.0 & A?B.A!C.0 )");
            assert_eq!(b, "B: ( B!A.0 + B!A.0 )");
            assert_eq!(c, "C: ( C?A.0 & C?A.0 )");
        }
        Ok(())
    }()
    .is_ok());
}
