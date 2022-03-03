// Test for Macro, exact same as usecase
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::close::close_mpst;
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use std::error::Error;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

use rand::{thread_rng, Rng};

use mpstthree::{
    create_broadcast_role, create_choose_both_from_3_to_1_and_2, create_multiple_normal_name,
    create_multiple_normal_role, create_offer_mpst_session_2, create_recv_mpst_session_1,
    create_recv_mpst_session_2, create_send_mpst_session_1, create_send_mpst_session_2,
};

// Create new roles
// normal
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleC, RoleCDual |
);
// broadcast
create_broadcast_role!(RoleAlltoC, RoleCtoAll);

// Create new names
create_multiple_normal_name!(NameA, NameB, NameC,);

// Create new send functions
create_send_mpst_session_1!(send_mpst_c_to_a, RoleA, NameC);
create_send_mpst_session_2!(send_mpst_a_to_c, RoleC, NameA);
create_send_mpst_session_2!(send_mpst_c_to_b, RoleB, NameC);
create_send_mpst_session_1!(send_mpst_b_to_a, RoleA, NameB);
create_send_mpst_session_1!(send_mpst_a_to_b, RoleB, NameA);

// Create new recv functions and related types
// normal
create_recv_mpst_session_1!(recv_mpst_c_from_a, RoleA, NameC);
create_recv_mpst_session_2!(recv_mpst_a_from_c, RoleC, NameA);
create_recv_mpst_session_2!(recv_mpst_b_from_c, RoleC, NameB);
create_recv_mpst_session_1!(recv_mpst_b_from_a, RoleA, NameB);
create_recv_mpst_session_1!(recv_mpst_a_from_b, RoleB, NameA);

// Create the offer functions
create_offer_mpst_session_2!(offer_mpst_session_b_to_c, RoleAlltoC, NameB);
create_offer_mpst_session_2!(offer_mpst_session_a_to_c, RoleAlltoC, NameA);

// Create the choose functions
create_choose_both_from_3_to_1_and_2!(
    choose_right_mpst_session_c_to_all,
    choose_left_mpst_session_c_to_all,
    NameA,
    NameB,
    RoleCtoAll,
    NameC
);

// Types
type AtoCClose = End;
type AtoBClose = End;
type AtoCVideo<N> = Recv<N, Send<N, End>>;
type AtoBVideo<N> = Send<N, Recv<N, End>>;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = End;
type BtoAVideo<N> = <AtoBVideo<N> as Session>::Dual;

type CtoBClose = <BtoCClose as Session>::Dual;
type CtoAClose = <AtoCClose as Session>::Dual;
type CtoAVideo<N> = <AtoCVideo<N> as Session>::Dual;

// Stacks
type StackAEnd = RoleEnd;
type StackAEndDual = <StackAEnd as Role>::Dual;
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleEnd>>>>;
type StackAVideoDual = <StackAVideo as Role>::Dual;
type StackAFull = RoleC<RoleC<RoleAlltoC<RoleEnd, RoleEnd>>>;

type StackBEnd = RoleEnd;
type StackBEndDual = <StackBEnd as Role>::Dual;
type StackBVideo = RoleA<RoleA<RoleEnd>>;
type StackBVideoDual = <StackBVideo as Role>::Dual;
type StackBFull = RoleAlltoC<RoleEnd, RoleEnd>;

type StackCEnd = RoleEnd;
type StackCVideo = RoleA<RoleA<RoleEnd>>;
type StackCChoice = RoleCtoAll<StackCVideo, StackCEnd>;
type StackCFull = RoleA<RoleA<StackCChoice>>;

// Creating the MP sessions
// For C
type ChooseCtoA<N> =
    ChooseMpst<BtoAVideo<N>, CtoAVideo<N>, BtoAClose, CtoAClose, StackAVideoDual, StackAEnd, NameA>;
type ChooseCtoB<N> =
    ChooseMpst<AtoBVideo<N>, CtoBClose, AtoBClose, CtoBClose, StackBVideoDual, StackBEnd, NameB>;
type InitC<N> = Send<N, Recv<N, ChooseCtoA<N>>>;
type EndpointCFull<N> = MeshedChannels<InitC<N>, ChooseCtoB<N>, StackCFull, NameC>;

// For A
type EndpointAVideo<N> = MeshedChannels<AtoBVideo<N>, AtoCVideo<N>, StackAVideo, NameA>;

type OfferA<N> =
    OfferMpst<AtoBVideo<N>, AtoCVideo<N>, AtoBClose, AtoCClose, StackAVideo, StackAEnd, NameA>;
type InitA<N> = Recv<N, Send<N, OfferA<N>>>;
type EndpointAFull<N> = MeshedChannels<End, InitA<N>, StackAFull, NameA>;

// For B
type EndpointBVideo<N> = MeshedChannels<BtoAVideo<N>, BtoCClose, StackBVideo, NameB>;

type OfferB<N> =
    OfferMpst<BtoAVideo<N>, BtoCClose, BtoAClose, BtoCClose, StackBVideo, StackBEnd, NameB>;
type EndpointBFull<N> = MeshedChannels<End, OfferB<N>, StackBFull, NameB>;

// Functions related to endpoints
fn server(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst_session_b_to_c(
        s,
        |s: EndpointBVideo<i32>| {
            let (request, s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);

            close_mpst(s)
        },
        close_mpst,
    )
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_from_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    offer_mpst_session_a_to_c(
        s,
        |s: EndpointAVideo<i32>| {
            let (request, s) = recv_mpst_a_from_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_from_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            close_mpst(s)
        },
        close_mpst,
    )
}

fn client_video(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_c_to_a(id, s);
    let (accept, s) = recv_mpst_c_from_a(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_left_mpst_session_c_to_all::<
        BtoAVideo<i32>,
        BtoAClose,
        CtoAVideo<i32>,
        CtoAClose,
        BtoCClose,
        AtoCClose,
        StackAVideoDual,
        StackAEndDual,
        StackBVideoDual,
        StackBEndDual,
        StackCVideo,
        StackCEnd,
    >(s);

    let s = send_mpst_c_to_a(accept, s);
    let (result, s) = recv_mpst_c_from_a(s)?;

    assert_eq!(result, accept + 3);

    close_mpst(s)
}

fn client_close(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let id: i32 = rng.gen();

    let s = send_mpst_c_to_a(id, s);
    let (accept, s) = recv_mpst_c_from_a(s)?;

    assert_eq!(accept, id + 1);

    let s = choose_right_mpst_session_c_to_all::<
        BtoAVideo<i32>,
        BtoAClose,
        CtoAVideo<i32>,
        CtoAClose,
        BtoCClose,
        AtoCClose,
        StackAVideoDual,
        StackAEndDual,
        StackBVideoDual,
        StackBEndDual,
        StackCVideo,
        StackCEnd,
    >(s);

    close_mpst(s)
}

/////////////////////////////////////////

pub fn run_usecase_right() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test end branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_close);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn run_usecase_left() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(authenticator, server, client_video);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}
