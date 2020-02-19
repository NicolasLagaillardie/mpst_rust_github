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

// pub fn send_mpst_session_a_to_all<S, S1,S2,S3,S4,S5, R,R1,R2>(
//     x: SessionMpst<S1, S, R1>,
//     y: SessionMpst<S, S2, R1>,
//     s: SessionMpst<ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleAtoAll<R>>,
// ) -> SessionMpst<End, End, R>
// where
// S: Session,
// S1: Session,
// S2: Session,
// S3: Session,
// S4: Session,
// R: Role,
// R1: Role,
// R2: Role,
// {

//     // pub fn choose_left<'a, S1, S2>(s: Send<Either<<S1 as Session>::Dual, <S2 as Session>::Dual>, End>) -> S1
//     // where
//     //     S1: Session + 'a,
//     //     S2: Session + 'a,
//     // {
//     //     let (here, there) = S1::new();
//     //     let s = send(Either::Left(there), s);
//     //     cancel(s);
//     //     here
//     // }

//     let new_session_1 = send(x, s.session1);
//     let new_session_2 = send(y, s.session2);
//     let new_queue = next_a_to_all(s.queue);

//     SessionMpst {
//         session1: new_session_1,
//         session2: new_session_2,
//         queue: new_queue,
//     }
// }

// pub fn send_mpst_session_b_to_all<T1, T2, R>(
//     x: T1,
//     y: T2,
//     s: SessionMpst<Send<T1, End>, Send<T2, End>, RoleBtoAll<R>>,
// ) -> SessionMpst<End, End, R>
// where
//     T1: marker::Send,
//     T2: marker::Send,
//     R: Role,
// {
//     let new_session_1 = send(x, s.session1);
//     let new_session_2 = send(y, s.session2);
//     let new_queue = next_b_to_all(s.queue);
//     SessionMpst {
//         session1: new_session_1,
//         session2: new_session_2,
//         queue: new_queue,
//     }
// }

// pub fn send_mpst_session_c_to_all<T1, T2, R>(
//     x: T1,
//     y: T2,
//     s: SessionMpst<Send<T1, End>, Send<T2, End>, RoleCtoAll<R>>,
// ) -> SessionMpst<End, End, R>
// where
//     T1: marker::Send,
//     T2: marker::Send,
//     R: Role,
// {
//     let new_session_1 = send(x, s.session1);
//     let new_session_2 = send(y, s.session2);
//     let new_queue = next_c_to_all(s.queue);
//     SessionMpst {
//         session1: new_session_1,
//         session2: new_session_2,
//         queue: new_queue,
//     }
// }

pub fn send_mpst_session_one_b_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleBtoA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_b_to_a(s.queue);
    SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    }
}

pub fn send_mpst_session_one_a_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleAtoC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_a_to_c(s.queue);
    SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    }
}

pub fn send_mpst_session_one_c_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleCtoA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_c_to_a(s.queue);
    SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    }
}

pub fn send_mpst_session_one_c_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleCtoB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_c_to_b(s.queue);
    SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    }
}

pub fn send_mpst_session_one_b_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<Send<T, S1>, S2, RoleBtoC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session1);
    let new_queue = next_b_to_c(s.queue);
    SessionMpst {
        session1: new_session,
        session2: s.session2,
        queue: new_queue,
    }
}

/// Sending on session 2
pub fn send_mpst_session_two_a_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleAtoB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_a_to_b(s.queue);
    SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    }
}

pub fn send_mpst_session_two_b_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleBtoA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_b_to_a(s.queue);
    SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    }
}

pub fn send_mpst_session_two_a_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleAtoC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_a_to_c(s.queue);
    SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    }
}

pub fn send_mpst_session_two_c_to_a<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleCtoA<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_c_to_a(s.queue);
    SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    }
}

pub fn send_mpst_session_two_b_to_c<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleBtoC<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_b_to_c(s.queue);
    SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    }
}

pub fn send_mpst_session_two_c_to_b<T, S1, S2, R>(
    x: T,
    s: SessionMpst<S1, Send<T, S2>, RoleCtoB<R>>,
) -> SessionMpst<S1, S2, R>
where
    T: marker::Send,
    S1: Session,
    S2: Session,
    R: Role,
{
    let new_session = send(x, s.session2);
    let new_queue = next_c_to_b(s.queue);
    SessionMpst {
        session1: s.session1,
        session2: new_session,
        queue: new_queue,
    }
}
