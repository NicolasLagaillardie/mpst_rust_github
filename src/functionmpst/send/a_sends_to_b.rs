use binary::{send, Send, Session};
use role::a_to_b::{next_a_to_b, RoleAtoB};
use role::a_to_c::{next_a_to_c, RoleAtoC};
use role::b_to_a::{next_b_to_a, RoleBtoA};
use role::b_to_c::{next_b_to_c, RoleBtoC};
use role::c_to_a::{next_c_to_a, RoleCtoA};
use role::c_to_b::{next_c_to_b, RoleCtoB};
use role::Role;
use sessionmpst::SessionMpst;
use std::marker;

pub fn send_mpst_session_one_a_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleAtoB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_a_to_b(s.queue);
    let result = SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    };

    result
}