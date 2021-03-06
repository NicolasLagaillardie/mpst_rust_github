// A → X → B
// B → Y → C
// C → Z → A

use mpstthree::checking::checker;

use std::boxed::Box;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

use mpstthree::functionmpst::close::close_mpst;

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

use mpstthree::functionmpst::recv::recv_mpst_b_from_c;
use mpstthree::functionmpst::recv::recv_mpst_c_from_a;

use mpstthree::functionmpst::send::send_mpst_a_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_b;

use mpstthree::functionmpst::offer::offer_mpst_session_to_b_from_a;
use mpstthree::functionmpst::offer::offer_mpst_session_to_c_from_a;

use mpstthree::functionmpst::choose::choose_left_mpst_session_a_to_all;
use mpstthree::functionmpst::choose::choose_right_mpst_session_a_to_all;

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
    offer_mpst_session_to_c_from_a(
        s,
        |s: EndpointCAdd<i32>| {
            let (x, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b(x + 1, s);

            assert_eq!(x, 1);

            close_mpst(s)
        },
        |s: EndpointCNeg<i32>| {
            let (x, s) = recv_mpst_c_from_a(s)?;
            let s = send_mpst_c_to_b(x + 1, s);

            assert_eq!(x, 2);

            close_mpst(s)
        },
    )
}

fn simple_store_client_left(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    let s = choose_left_mpst_session_a_to_all::<
        CtoBAdd<i32>,
        CtoBNeg<i32>,
        End,
        AtoCAdd<i32>,
        End,
        AtoCNeg<i32>,
        StackOfferBDual,
        StackOfferBDual,
        StackOfferCDual,
        StackOfferCDual,
        StackChoiceA,
        StackChoiceA,
    >(s);
    let s = send_mpst_a_to_c(1, s);
    close_mpst(s)
}

fn simple_store_client_right(s: EndpointChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    let s = choose_right_mpst_session_a_to_all::<
        CtoBAdd<i32>,
        CtoBNeg<i32>,
        End,
        AtoCAdd<i32>,
        End,
        AtoCNeg<i32>,
        StackOfferBDual,
        StackOfferBDual,
        StackOfferCDual,
        StackOfferCDual,
        StackChoiceA,
        StackChoiceA,
    >(s);
    let s = send_mpst_a_to_c(2, s);
    close_mpst(s)
}

fn simple_store_pawn(s: EndpointChoiceB<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_to_b_from_a(
        s,
        |s: EndpointBAdd<i32>| {
            let (x, s) = recv_mpst_b_from_c(s)?;

            assert_eq!(x, 2);

            close_mpst(s)
        },
        |s: EndpointBNeg<i32>| {
            let (x, s) = recv_mpst_b_from_c(s)?;

            assert_eq!(x, 3);

            close_mpst(s)
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
