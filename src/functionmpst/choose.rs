use binary::{cancel, send, Session};
use either::Either;
use functionmpst::ChooseMpst;
use role::a_to_all::{next_a_to_all, RoleAtoAll};
use role::b_to_all::{next_b_to_all, RoleBtoAll};
use role::c_to_all::{next_c_to_all, RoleCtoAll};
use role::Role;
use sessionmpst::SessionMpst;

pub fn choose_left_mpst_session_a_to_all<'a, S, S1, S2, S3, S4, R1, R2, R3, R4, R5>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S1, S, R1>, SessionMpst<S3, S, R2>>,
        ChooseMpst<
            SessionMpst<S2, <S as Session>::Dual, R3>,
            SessionMpst<S4, <S as Session>::Dual, R4>,
        >,
        RoleAtoAll<R5>,
    >,
) -> SessionMpst<S1, S2, R5>
where
    S: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
{
    let (session_ab, session_ba) = S1::new();
    let (session_ac, session_ca) = S2::new();
    let (session_bc, session_cb) = Session::new();
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
    let new_queue = next_a_to_all(s.queue);

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

pub fn choose_right_mpst_session_a_to_all<'a, S, S1, S2, S3, S4, R1, R2, R3, R4, R5>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S1, S, R1>, SessionMpst<S3, S, R2>>,
        ChooseMpst<
            SessionMpst<S2, <S as Session>::Dual, R3>,
            SessionMpst<S4, <S as Session>::Dual, R4>,
        >,
        RoleAtoAll<R5>,
    >,
) -> SessionMpst<S3, S4, R5>
where
    S: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
{
    let (session_ab, session_ba) = S3::new();
    let (session_ac, session_ca) = S4::new();
    let (session_bc, session_cb) = Session::new();
    let (_, role_b) = R2::new();
    let (_, role_c) = R4::new();
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

    let new_session_1 = send(Either::Right(choice_b), s.session1);
    let new_session_2 = send(Either::Right(choice_c), s.session2);
    let new_queue = next_a_to_all(s.queue);

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

pub fn choose_left_mpst_session_b_to_all<'a, S, S1, S2, S3, S4, R1, R2, R3, R4, R5>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S1, S, R1>, SessionMpst<S3, S, R2>>,
        ChooseMpst<
            SessionMpst<<S as Session>::Dual, S2, R3>,
            SessionMpst<<S as Session>::Dual, S4, R4>,
        >,
        RoleBtoAll<R5>,
    >,
) -> SessionMpst<S1, S2, R5>
where
    S: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
{
    let (session_ba, session_ab) = S1::new();
    let (session_bc, session_cb) = S2::new();
    let (session_ac, session_ca) = Session::new();
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
    let new_queue = next_b_to_all(s.queue);

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

pub fn choose_right_mpst_session_b_to_all<'a, S, S1, S2, S3, S4, R1, R2, R3, R4, R5>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S1, S, R1>, SessionMpst<S3, S, R2>>,
        ChooseMpst<
            SessionMpst<<S as Session>::Dual, S2, R3>,
            SessionMpst<<S as Session>::Dual, S4, R4>,
        >,
        RoleBtoAll<R5>,
    >,
) -> SessionMpst<S3, S4, R5>
where
    S: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
{
    let (session_ba, session_ab) = S3::new();
    let (session_bc, session_cb) = S4::new();
    let (session_ac, session_ca) = Session::new();
    let (_, role_a) = R2::new();
    let (_, role_c) = R4::new();
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

    let new_session_1 = send(Either::Right(choice_a), s.session1);
    let new_session_2 = send(Either::Right(choice_c), s.session2);
    let new_queue = next_b_to_all(s.queue);

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

pub fn choose_left_mpst_session_c_to_all<'a, S, S1, S2, S3, S4, R1, R2, R3, R4, R5>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S, S1, R1>, SessionMpst<S, S3, R2>>,
        ChooseMpst<
            SessionMpst<<S as Session>::Dual, S2, R3>,
            SessionMpst<<S as Session>::Dual, S4, R4>,
        >,
        RoleCtoAll<R5>,
    >,
) -> SessionMpst<S1, S2, R5>
where
    S: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
{
    let (session_ca, session_ac) = S1::new();
    let (session_cb, session_bc) = S2::new();
    let (session_ab, session_ba) = Session::new();
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
    let new_queue = next_c_to_all(s.queue);

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

pub fn choose_right_mpst_session_c_to_all<'a, S, S1, S2, S3, S4, R1, R2, R3, R4, R5>(
    s: SessionMpst<
        ChooseMpst<SessionMpst<S, S1, R1>, SessionMpst<S, S3, R2>>,
        ChooseMpst<
            SessionMpst<<S as Session>::Dual, S2, R3>,
            SessionMpst<<S as Session>::Dual, S4, R4>,
        >,
        RoleCtoAll<R5>,
    >,
) -> SessionMpst<S3, S4, R5>
where
    S: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
    R4: Role,
    R5: Role,
{
    let (session_ca, session_ac) = S3::new();
    let (session_cb, session_bc) = S4::new();
    let (session_ab, session_ba) = Session::new();
    let (_, role_a) = R2::new();
    let (_, role_b) = R4::new();
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

    let new_session_1 = send(Either::Right(choice_a), s.session1);
    let new_session_2 = send(Either::Right(choice_b), s.session2);
    let new_queue = next_c_to_all(s.queue);

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
