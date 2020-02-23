use binary::{cancel, send, Session};
use either::Either;
use functionmpst::ChooseMpst;
use role::a_to_all::{next_a_to_all, RoleAtoAll};
use role::b_to_all::{next_b_to_all, RoleBtoAll};
use role::c_to_all::{next_c_to_all, RoleCtoAll};
use role::Role;
use sessionmpst::SessionMpst;

pub fn choose_left_mpst_session_a_to_all<'a, S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S2, S0, R1>, SessionMpst<S5, S1, R2>>,
        ChooseMpst<
            SessionMpst<S3, <S0 as Session>::Dual, R3>,
            SessionMpst<S4, <S1 as Session>::Dual, R4>,
        >,
        RoleAtoAll<R5, R6>,
    >,
) -> SessionMpst<S2, S3, R5>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
    R6: Role,
{
    let (session_ab, session_ba) = S2::new();
    let (session_ac, session_ca) = S3::new();
    let (session_cb, session_bc) = S0::new();
    let (_, role_b) = R1::new();
    let (_, role_c) = R3::new();
    let (role_a, _) = R5::new();

    let choice_b = SessionMpst {
        session1: session_ba,
        session2: session_bc,
        queue: role_b,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        queue: role_c,
    };

    let new_session_1 = send(Either::Left(choice_b), s.session1);
    let new_session_2 = send(Either::Left(choice_c), s.session2);
    let (new_queue, _) = next_a_to_all(s.queue);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        queue: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ab,
        session2: session_ac,
        queue: role_a,
    }
}

pub fn choose_right_mpst_session_a_to_all<'a, S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S2, S0, R1>, SessionMpst<S4, S1, R2>>,
        ChooseMpst<
            SessionMpst<S3, <S0 as Session>::Dual, R3>,
            SessionMpst<S5, <S1 as Session>::Dual, R4>,
        >,
        RoleAtoAll<R5, R6>,
    >,
) -> SessionMpst<S4, S5, R6>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
    R6: Role,
{
    let (session_ab, session_ba) = S4::new();
    let (session_ac, session_ca) = S5::new();
    let (session_cb, session_bc) = S1::new();
    let (_, role_b) = R2::new();
    let (_, role_c) = R4::new();
    let (role_a, _) = R6::new();

    let choice_b = SessionMpst {
        session1: session_ba,
        session2: session_bc,
        queue: role_b,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        queue: role_c,
    };

    let new_session_1 = send(Either::Right(choice_b), s.session1);
    let new_session_2 = send(Either::Right(choice_c), s.session2);
    let (_, new_queue) = next_a_to_all(s.queue);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        queue: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ab,
        session2: session_ac,
        queue: role_a,
    }
}

pub fn choose_left_mpst_session_b_to_all<'a, S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S2, S0, R1>, SessionMpst<S5, S1, R2>>,
        ChooseMpst<
            SessionMpst<<S0 as Session>::Dual, S3, R3>,
            SessionMpst<<S1 as Session>::Dual, S4, R4>,
        >,
        RoleBtoAll<R5, R6>,
    >,
) -> SessionMpst<S2, S3, R5>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
    R6: Role,
{
    let (session_ba, session_ab) = S2::new();
    let (session_bc, session_cb) = S3::new();
    let (session_ca, session_ac) = S0::new();
    let (_, role_a) = R1::new();
    let (_, role_c) = R3::new();
    let (role_b, _) = R5::new();

    let choice_a = SessionMpst {
        session1: session_ab,
        session2: session_ac,
        queue: role_a,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        queue: role_c,
    };

    let new_session_1 = send(Either::Left(choice_a), s.session1);
    let new_session_2 = send(Either::Left(choice_c), s.session2);
    let (new_queue, _) = next_b_to_all(s.queue);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        queue: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ba,
        session2: session_bc,
        queue: role_b,
    }
}

pub fn choose_right_mpst_session_b_to_all<'a, S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S2, S0, R1>, SessionMpst<S5, S1, R2>>,
        ChooseMpst<
            SessionMpst<<S0 as Session>::Dual, S3, R3>,
            SessionMpst<<S1 as Session>::Dual, S4, R4>,
        >,
        RoleBtoAll<R5, R6>,
    >,
) -> SessionMpst<S5, S4, R6>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
    R6: Role,
{
    let (session_ba, session_ab) = S5::new();
    let (session_bc, session_cb) = S4::new();
    let (session_ca, session_ac) = S1::new();
    let (_, role_a) = R2::new();
    let (_, role_c) = R4::new();
    let (role_b, _) = R6::new();

    let choice_a = SessionMpst {
        session1: session_ab,
        session2: session_ac,
        queue: role_a,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        queue: role_c,
    };

    let new_session_1 = send(Either::Right(choice_a), s.session1);
    let new_session_2 = send(Either::Right(choice_c), s.session2);
    let (_, new_queue) = next_b_to_all(s.queue);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        queue: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ba,
        session2: session_bc,
        queue: role_b,
    }
}

pub fn choose_left_mpst_session_c_to_all<'a, S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S0, S2, R1>, SessionMpst<S1, S5, R2>>,
        ChooseMpst<
            SessionMpst<<S0 as Session>::Dual, S3, R3>,
            SessionMpst<<S1 as Session>::Dual, S4, R4>,
        >,
        RoleCtoAll<R5, R6>,
    >,
) -> SessionMpst<S2, S3, R5>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
    R6: Role,
{
    let (session_ca, session_ac) = S2::new();
    let (session_cb, session_bc) = S3::new();
    let (session_ba, session_ab) = S0::new();
    let (_, role_a) = R1::new();
    let (_, role_b) = R3::new();
    let (role_c, _) = R5::new();

    let choice_a = SessionMpst {
        session1: session_ab,
        session2: session_ac,
        queue: role_a,
    };

    let choice_b = SessionMpst {
        session1: session_ba,
        session2: session_bc,
        queue: role_b,
    };

    let new_session_1 = send(Either::Left(choice_a), s.session1);
    let new_session_2 = send(Either::Left(choice_b), s.session2);
    let (new_queue, _) = next_c_to_all(s.queue);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        queue: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ca,
        session2: session_cb,
        queue: role_c,
    }
}

pub fn choose_right_mpst_session_c_to_all<'a, S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S0, S2, R1>, SessionMpst<S1, S5, R2>>,
        ChooseMpst<
            SessionMpst<<S0 as Session>::Dual, S3, R3>,
            SessionMpst<<S1 as Session>::Dual, S4, R4>,
        >,
        RoleCtoAll<R5, R6>,
    >,
) -> SessionMpst<S5, S4, R6>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
    R6: Role,
{
    let (session_ca, session_ac) = S5::new();
    let (session_cb, session_bc) = S4::new();
    let (session_ba, session_ab) = S1::new();
    let (_, role_a) = R2::new();
    let (_, role_b) = R4::new();
    let (role_c, _) = R6::new();

    let choice_a = SessionMpst {
        session1: session_ab,
        session2: session_ac,
        queue: role_a,
    };

    let choice_b = SessionMpst {
        session1: session_ba,
        session2: session_bc,
        queue: role_b,
    };

    let new_session_1 = send(Either::Right(choice_a), s.session1);
    let new_session_2 = send(Either::Right(choice_b), s.session2);
    let (_, new_queue) = next_c_to_all(s.queue);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        queue: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ca,
        session2: session_cb,
        queue: role_c,
    }
}

#[macro_export]
macro_rules! choose_mpst {
    ($label:path, $session:expr) => {{
        let (here, there) = <_ as Session>::new();
        let s = send($label(there), $session);
        cancel(s);
        here
    }};
}
