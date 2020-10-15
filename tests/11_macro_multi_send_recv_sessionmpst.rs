// Test for parametrisation on the number of roles
extern crate crossbeam_channel;
extern crate either;
extern crate mpstthree;
use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    close_mpst, create_broadcast_role, create_normal_role, create_offer_mpst_session_multi,
    create_offer_type_multi, create_recv_mpst_all_session, create_recv_mpst_session,
    create_send_mpst_session, create_sessionmpst,
};
use std::error::Error;

type ResultAnySend = Result<(), Box<(dyn std::any::Any + std::marker::Send + 'static)>>;

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpst, 3);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
// broadcast
create_broadcast_role!(RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);

// Create new send functions
create_send_mpst_session!(send_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
create_send_mpst_session!(send_mpst_a_to_d, RoleD, next_d, RoleD, SessionMpst, 3, 2);
create_send_mpst_session!(send_mpst_d_to_b, RoleB, next_b, RoleD, SessionMpst, 3, 2);
create_send_mpst_session!(send_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpst, 3, 1);
create_send_mpst_session!(send_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpst, 3, 1);

// Create new recv functions and related types
// normal
create_recv_mpst_session!(recv_mpst_d_to_a, RoleA, next_a, RoleD, SessionMpst, 3, 1);
create_recv_mpst_session!(recv_mpst_a_to_d, RoleD, next_d, RoleA, SessionMpst, 3, 2);
create_recv_mpst_session!(recv_mpst_b_to_d, RoleD, next_d, RoleB, SessionMpst, 3, 2);
create_recv_mpst_session!(recv_mpst_b_to_a, RoleA, next_a, RoleB, SessionMpst, 3, 1);
create_recv_mpst_session!(recv_mpst_a_to_b, RoleB, next_b, RoleA, SessionMpst, 3, 1);
// broadcast
create_recv_mpst_all_session!(
    recv_mpst_b_all_to_d,
    RoleAlltoD,
    next_all_to_d,
    RoleB,
    SessionMpst,
    3,
    2
);
create_recv_mpst_all_session!(
    recv_mpst_a_all_to_d,
    RoleAlltoD,
    next_all_to_d,
    RoleA,
    SessionMpst,
    3,
    2
);

close_mpst!(close_mpst_multi, SessionMpst, 3);

create_offer_type_multi!(OfferMpstMultiThree, SessionMpst, 3, 2);

create_offer_mpst_session_multi!(
    offer_mpst_session_a_to_c,
    OfferMpstMultiThree,
    RoleAlltoD,
    recv_mpst_a_all_to_d,
    RoleA,
    SessionMpst,
    3,
    2
);

create_offer_mpst_session_multi!(
    offer_mpst_session_b_to_c,
    OfferMpstMultiThree,
    RoleAlltoD,
    recv_mpst_b_all_to_d,
    RoleB,
    SessionMpst,
    3,
    2
);

type TestA = RoleA<RoleEnd>;
type TestB = RoleB<RoleEnd>;
type TestD = RoleD<RoleEnd>;

type SendSessionMPSTD<N> = SessionMpst<Send<N, End>, End, TestA, TestD>;

type RecvSessionMPSTA<N> = SessionMpst<End, Recv<N, End>, TestD, TestA>;

type Pawn = SessionMpst<End, End, RoleEnd, TestB>;

fn send_d_to_a(s: SendSessionMPSTD<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_d_to_a(0, s);
    close_mpst_multi(s)?;
    Ok(())
}

fn recv_a_to_d(s: RecvSessionMPSTA<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_a_to_d(s)?;
    close_mpst_multi(s)?;
    Ok(())
}

fn pawn(s: Pawn) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)?;
    Ok(())
}

////////////////////////////////////////
/// To be replaced
/// TODO

#[doc(hidden)]
pub fn fork_simple<S1, S2, R, N, P>(
    p: P,
    s: SessionMpst<S1, S2, R, N>,
) -> std::thread::JoinHandle<()>
where
    S1: mpstthree::binary::Session + 'static,
    S2: mpstthree::binary::Session + 'static,
    R: mpstthree::role::Role + 'static,
    N: mpstthree::role::Role + 'static,
    P: FnOnce(SessionMpst<S1, S2, R, N>) -> Result<(), Box<dyn Error>>
        + std::marker::Send
        + 'static,
{
    std::thread::spawn(move || {
        std::panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        match p(s) {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e),
        }
    })
}

// fork_simple_multi!(SessionMpst, 3);

/// Creates and returns three child processes for three `SessionMpst` linked together.
///
/// Creates 3 pairs of endpoints, each pair of type `S` and `S::Dual`.
/// Creates 3 `Role` for each queue.
/// Creates 3 `SessionMpst`, linked together with the pairs of endpoints, and get the related child processes from `fork_simple`.
/// Returns the tuple of the 3 child processes.
pub fn fork_mpst<S1, S2, S3, R1, R2, R3, N1, N2, N3, F1, F2, F3>(
    f1: F1,
    f2: F2,
    f3: F3,
) -> (ResultAnySend, ResultAnySend, ResultAnySend)
where
    S1: mpstthree::binary::Session + 'static,
    S2: mpstthree::binary::Session + 'static,
    S3: mpstthree::binary::Session + 'static,
    R1: mpstthree::role::Role + 'static,
    R2: mpstthree::role::Role + 'static,
    R3: mpstthree::role::Role + 'static,
    N1: mpstthree::role::Role + 'static,
    N2: mpstthree::role::Role + 'static,
    N3: mpstthree::role::Role + 'static,
    F1: FnOnce(
            SessionMpst<S1, <S3 as mpstthree::binary::Session>::Dual, R1, N1>,
        ) -> Result<(), Box<dyn Error>>
        + std::marker::Send
        + 'static,
    F2: FnOnce(
            SessionMpst<<S1 as mpstthree::binary::Session>::Dual, S2, R2, N2>,
        ) -> Result<(), Box<dyn Error>>
        + std::marker::Send
        + 'static,
    F3: FnOnce(
            SessionMpst<S3, <S2 as mpstthree::binary::Session>::Dual, R3, N3>,
        ) -> Result<(), Box<dyn Error>>
        + std::marker::Send
        + 'static,
{
    let (channel_ab, channel_ba) = S1::new();
    let (channel_ca, channel_ac) = S3::new();
    let (channel_bc, channel_cb) = S2::new();

    let (role_a, _) = R1::new();
    let (role_b, _) = R2::new();
    let (role_c, _) = R3::new();

    let (name_a, _) = N1::new();
    let (name_b, _) = N2::new();
    let (name_c, _) = N3::new();

    let a = SessionMpst {
        session1: channel_ab,
        session2: channel_ac,
        stack: role_a,
        name: name_a,
    };
    let b = SessionMpst {
        session1: channel_ba,
        session2: channel_bc,
        stack: role_b,
        name: name_b,
    };
    let c = SessionMpst {
        session1: channel_ca,
        session2: channel_cb,
        stack: role_c,
        name: name_c,
    };

    let thread_a = fork_simple(f1, a);
    let thread_b = fork_simple(f2, b);
    let thread_c = fork_simple(f3, c);

    (thread_a.join(), thread_b.join(), thread_c.join())
}

////////////////////////////////////////

#[test]
fn test_new_send() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_pawn, thread_d) = fork_mpst(recv_a_to_d, pawn, send_d_to_a);

            assert!(thread_a.is_ok());
            assert!(thread_pawn.is_ok());
            assert!(thread_d.is_ok());
        }
        Ok(())
    }()
    .is_ok());
}
