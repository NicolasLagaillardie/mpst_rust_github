extern crate mpst;

use mpst::binary::{End, Recv, Send, Session, cancel};
use mpst::role::Role;
use mpst::run_processes;
use mpst::sessionmpst::SessionMpst;

use std::boxed::Box;
use std::error::Error;
use std::marker;

use mpst::functionmpst::close::close_mpst;

use mpst::role::a_to_b::RoleAtoB;
use mpst::role::a_to_c::RoleAtoC;
use mpst::role::b_to_a::RoleBtoA;
use mpst::role::b_to_c::RoleBtoC;
use mpst::role::c_to_a::RoleCtoA;
use mpst::role::c_to_all::RoleCtoAll;
use mpst::role::end::RoleEnd;

use mpst::functionmpst::recv::recv_mpst_a_to_b;
use mpst::functionmpst::recv::recv_mpst_a_to_c;
use mpst::functionmpst::recv::recv_mpst_b_to_a;
use mpst::functionmpst::recv::recv_mpst_b_to_c;
use mpst::functionmpst::recv::recv_mpst_c_to_a;

use mpst::functionmpst::send::send_mpst_a_to_b;
use mpst::functionmpst::send::send_mpst_a_to_c;
use mpst::functionmpst::send::send_mpst_b_to_a;
use mpst::functionmpst::send::send_mpst_c_to_a;

use mpst::functionmpst::offer::offer_mpst_session_a_to_c;
use mpst::functionmpst::offer::offer_mpst_session_b_to_c;

use mpst::functionmpst::choose::choose_left_mpst_session_c_to_all;
use mpst::functionmpst::choose::choose_right_mpst_session_c_to_all;

use mpst::functionmpst::ChooseMpst;
use mpst::functionmpst::OfferMpst;

/// Test our usecase from Places 2020
/// Simple types
/// Client = C
/// Authenticator = A
/// Server = B

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

// /// Queues
type QueueAEnd = RoleEnd;
type QueueAVideo = RoleAtoC<RoleAtoB<RoleAtoB<RoleAtoC<RoleEnd>>>>;
type QueueAVideoDual = <QueueAVideo as Role>::Dual;
type QueueAFull = RoleAtoC<RoleAtoC<RoleAtoC<RoleEnd>>>;

type QueueBEnd = RoleEnd;
type QueueBVideo = RoleBtoA<RoleBtoA<RoleEnd>>;
type QueueBVideoDual = <QueueBVideo as Role>::Dual;
type QueueBFull = RoleBtoC<RoleEnd>;

type QueueCEnd = RoleEnd;
type QueueCVideo = RoleCtoA<RoleCtoA<RoleEnd>>;
type QueueCChoice = RoleCtoAll<QueueCVideo, QueueCEnd>;
type QueueCFull = RoleCtoA<RoleCtoA<QueueCChoice>>;

/// Creating the MP sessions
/// For C
enum CBranchesAtoC<N: marker::Send> {
    End(AtoCClose),
    Video(Recv<N, Send<N, ChooseCforAtoC<N>>>),
}
enum CBranchesBtoC {
    End(BtoCClose),
    Video(BtoCClose),
}
type ChooseCforAtoC<N> = Send<CBranchesAtoC<N>, End>;
type ChooseCforBtoC = Send<CBranchesBtoC, End>;

type EndpointCRecurs<N> = SessionMpst<ChooseCforAtoC<N>, ChooseCforBtoC, QueueCFull>;
type InitC<N> = Send<N, Recv<N, ChooseCforAtoC<N>>>;
type EndpointCFull<N> = SessionMpst<InitC<N>, ChooseCforBtoC, QueueCFull>;

/// For A
type EndpointAEnd = SessionMpst<AtoBClose, AtoCClose, QueueAEnd>;
type EndpointAVideo<N> = SessionMpst<AtoBVideo<N>, AtoCVideo<N>, QueueAVideo>;

type EndpointARecurs<N> = SessionMpst<ChooseCforAtoC<N>, ChooseCforBtoC, QueueCFull>;
type InitA<N> = Recv<N, Send<N, Recv<CBranchesAtoC<N>, End>>>;
type EndpointAFull<N> = SessionMpst<End, InitA<N>, RoleAtoC<RoleAtoC<RoleEnd>>>;

/// For B
type EndpointBEnd = SessionMpst<BtoAClose, BtoCClose, QueueBEnd>;
type EndpointBVideo<N> = SessionMpst<BtoAVideo<N>, BtoCClose, QueueBVideo>;

type InitB = Recv<CBranchesBtoC, End>;
type EndpointBRecurs = SessionMpst<End, InitB, RoleBtoC<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointBRecurs) -> Result<(), Box<dyn Error>> {

    let (l, s) = recv_mpst_b_to_c(s)?;
    cancel(s);

    match l {
        CBranchesBtoC::End(s) => {
            close_mpst(s);
            Ok(())
        },
        CBranchesBtoC::Video(s) => {
            let (request, s) = recv_mpst_b_to_a(s)?;
            let s = send_mpst_b_to_a(request + 1, s);
            server(s)
        },
    }
}

fn authenticator(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = recv_mpst_a_to_c(s)?;
    let s = send_mpst_a_to_c(id + 1, s);

    let result = authenticator_recurs(s)?;

    Ok(result)
}

fn authenticator_recurs(s: EndpointARecurs<i32>) -> Result<(), Box<dyn Error>> {

    let (l, s) = recv_mpst_a_to_c(s)?;
    cancel(s);

    match l {
        CBranchesAtoC::End(s) => {
            close_mpst(s);
            Ok(())
        },
        CBranchesAtoC::Video(s) => {
            let (request, s) = recv_mpst_a_to_c(s)?;
            let s = send_mpst_a_to_b(request + 1, s);
            let (video, s) = recv_mpst_a_to_b(s)?;
            let s = send_mpst_a_to_c(video + 1, s);
            authenticator_recurs(s)
        },
    }
}

fn client(s : EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let xs: Vec<i32> = (1..100).map(|_| 1).collect();

    let s = send_mpst_c_to_a(0, s);
    let (accept, s) = recv_mpst_c_to_a(s)?;

    let result = client_recurs(s, xs)?;

    Ok(result)
}

fn client_recurs(s : EndpointCRecurs<i32>, mut xs: Vec<i32>) -> Result<(), Box<dyn Error>> {
    match xs.pop()
    {
        Option::Some(x) => {
    
            let s = choose_left_mpst_session_c_to_all::<
                BtoAVideo<i32>,
                AtoBClose,
                CtoAVideo<i32>,
                BtoCClose,
                BtoCClose,
                AtoCClose,
                QueueAVideoDual,
                QueueAEnd,
                QueueBVideoDual,
                QueueBEnd,
                QueueCVideo,
                QueueCEnd,
            >(s);
    
            let s = send_mpst_c_to_a(1, s);
            let (result, s) = recv_mpst_c_to_a(s)?;

            client_recurs(s, xs)
        }
        Option::None => {

            let s = choose_right_mpst_session_c_to_all::<
                BtoAVideo<i32>,
                AtoBClose,
                CtoAVideo<i32>,
                BtoCClose,
                BtoCClose,
                AtoCClose,
                QueueAVideoDual,
                QueueAEnd,
                QueueBVideoDual,
                QueueBEnd,
                QueueCVideo,
                QueueCEnd,
            >(s);
    
            close_mpst(s)?;

            Ok(())
        }
    }
}

#[test]
fn run_usecase() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test whole usecase.
        {
            let (thread_a, thread_b, thread_c) = run_processes(authenticator, server, client);

            assert!(thread_a.is_ok());
            assert!(thread_b.is_ok());
            assert!(thread_c.is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
