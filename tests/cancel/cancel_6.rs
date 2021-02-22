use mpstthree::binary::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_fork_multi, close_mpst, create_normal_role, create_recv_mpst_session,
    create_recv_mpst_session_bundle, create_send_mpst_session, create_send_mpst_session_bundle,
    create_sessionmpst,
};

use rand::random;
use std::error::Error;

// B --> C canceled
// C-->B.C-->D

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstFour, 4);

// Create new roles
// normal
create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);

// Create new send functions
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b,
    RoleB,
    next_b,
    2, |
    send_mpst_c_to_d,
    RoleD,
    next_d,
    3, | =>
    RoleC,
    SessionMpstFour,
    4
);

// Create new recv functions and related types
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_to_c,
    RoleC,
    next_c,
    2, | =>
    RoleB,
    SessionMpstFour,
    4
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_to_c,
    RoleC,
    next_c,
    3, | =>
    RoleD,
    SessionMpstFour,
    4
);

broadcast_cancel!(cancel_mpst, RoleB, SessionMpstFour, 4);

// Create close function
close_mpst!(close_mpst_multi, SessionMpstFour, 4);

// Create fork function
bundle_fork_multi!(fork_mpst, fork_simple, SessionMpstFour, 4);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;

// Types
type EndpointA = SessionMpstFour<End, End, End, RoleEnd, NameA>;
type EndpointB = SessionMpstFour<End, Recv<i32, End>, End, RoleC<RoleEnd>, NameB>;
type EndpointC = SessionMpstFour<End, Send<i32, End>, Send<i32, End>, RoleB<RoleD<RoleEnd>>, NameC>;
type EndpointD = SessionMpstFour<End, End, Recv<i32, End>, RoleC<RoleEnd>, NameD>;

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    loop {
        match s.session1.receiver.try_recv() {
            Ok(_) => {
                s.session2.sender.send(()).unwrap();
                s.session3.sender.send(()).unwrap();
                panic!("Error");
            }
            _ => {}
        };
        match s.session2.receiver.try_recv() {
            Ok(_) => {
                s.session1.sender.send(()).unwrap();
                s.session3.sender.send(()).unwrap();
                panic!("Error");
            }
            _ => {}
        };
        match s.session3.receiver.try_recv() {
            Ok(_) => {
                s.session1.sender.send(()).unwrap();
                s.session2.sender.send(()).unwrap();
                panic!("Error");
            }
            _ => {}
        };
    }
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    cancel_mpst(s);

    // let (_, s) = recv_mpst_b_to_a(s)?;
    // close_mpst_multi(s)

    Ok(())
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    println!("Canceled C 0");
    match s.session1.receiver.try_recv() {
        Ok(_) => panic!("Error"),
        _ => {}
    };
    println!("Canceled C 1");
    let s = send_mpst_c_to_b(random(), s);
    match s.session1.receiver.try_recv() {
        Ok(_) => panic!("Error"),
        _ => {}
    };
    let s = send_mpst_c_to_d(random(), s);
    match s.session1.receiver.try_recv() {
        Ok(_) => panic!("Error"),
        _ => {}
    };
    close_mpst_multi(s)
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    println!("Canceled D 0");
    match s.session1.receiver.try_recv() {
        Ok(_) => panic!("Error"),
        _ => {}
    };
    println!("Canceled D 1");
    let (_, s) = recv_mpst_d_to_c(s)?;
    match s.session1.receiver.try_recv() {
        Ok(_) => panic!("Error"),
        _ => {}
    };
    close_mpst_multi(s)
}

pub fn main() {
    let (thread_a, thread_b, thread_c, thread_d) =
        fork_mpst(endpoint_a, endpoint_b, endpoint_c, endpoint_d);

    assert!(thread_a.join().is_err());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_err());
    assert!(thread_d.join().is_err());
}
