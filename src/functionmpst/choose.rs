use binary::{cancel, send, Session};
use either::Either;
use functionmpst::ChooseMpst;
use role::a_to_all::{next_a_to_all, RoleAtoAll};
use role::b_to_all::{next_b_to_all, RoleBtoAll};
use role::c_to_all::{next_c_to_all, RoleCtoAll};
use role::Role;
use sessionmpst::SessionMpst;

type ShortChooseMpstOne<S0, S1, S2, S4, R1, R2> = ChooseMpst<S2, S0, S4, S1, R1, R2>;
type ShortChooseMpstTwo<S0, S1, S3, S5, R3, R4> =
    ChooseMpst<S3, <S0 as Session>::Dual, S5, <S1 as Session>::Dual, R3, R4>;
type ShortChooseMpstThree<S0, S1, S3, S5, R3, R4> =
    ChooseMpst<<S0 as Session>::Dual, S3, <S1 as Session>::Dual, S5, R3, R4>;
type ShortChooseMpstFour<S0, S1, S2, S4, R1, R2> = ChooseMpst<S0, S2, S1, S4, R1, R2>;

type ShortSessionMpstAtoAll<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6> = SessionMpst<
    ShortChooseMpstOne<S0, S1, S2, S4, R1, R2>,
    ShortChooseMpstTwo<S0, S1, S3, S5, R3, R4>,
    RoleAtoAll<R5, R6>,
>;
type ShortSessionMpstBtoAll<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6> = SessionMpst<
    ShortChooseMpstOne<S0, S1, S2, S4, R1, R2>,
    ShortChooseMpstThree<S0, S1, S3, S5, R3, R4>,
    RoleBtoAll<R5, R6>,
>;
type ShortSessionMpstCtoAll<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6> = SessionMpst<
    ShortChooseMpstFour<S0, S1, S2, S4, R1, R2>,
    ShortChooseMpstThree<S0, S1, S3, S5, R3, R4>,
    RoleCtoAll<R5, R6>,
>;

/// Given a choice from A, to other processes, between two `SessionMpst`, choose the first option for each.
///
/// A has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each first option then the related `SessionMpst`.
/// It then sends those options to the related processes.
pub fn choose_left_mpst_session_a_to_all<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: ShortSessionMpstAtoAll<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>,
) -> SessionMpst<S2, S3, R5>
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    S4: Session + 'static,
    S5: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    R4: Role + 'static,
    R5: Role + 'static,
    R6: Role + 'static,
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
        stack: role_b,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
    };

    let new_session_1 = send(Either::Left(choice_b), s.session1);
    let new_session_2 = send(Either::Left(choice_c), s.session2);
    let (new_queue, _) = next_a_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ab,
        session2: session_ac,
        stack: role_a,
    }
}

/// Given a choice from A, to other processes, between two `SessionMpst`, choose the second option for each.
///
/// A has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each second option then the related `SessionMpst`.
/// It then sends those options to the related processes.
pub fn choose_right_mpst_session_a_to_all<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: ShortSessionMpstAtoAll<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>,
) -> SessionMpst<S4, S5, R6>
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    S4: Session + 'static,
    S5: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    R4: Role + 'static,
    R5: Role + 'static,
    R6: Role + 'static,
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
        stack: role_b,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
    };

    let new_session_1 = send(Either::Right(choice_b), s.session1);
    let new_session_2 = send(Either::Right(choice_c), s.session2);
    let (_, new_queue) = next_a_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ab,
        session2: session_ac,
        stack: role_a,
    }
}

/// Given a choice from B, to other processes, between two `SessionMpst`, choose the first option for each.
///
/// B has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each first option then the related `SessionMpst`.
/// It then sends those options to the related processes.
pub fn choose_left_mpst_session_b_to_all<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: ShortSessionMpstBtoAll<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>,
) -> SessionMpst<S2, S3, R5>
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    S4: Session + 'static,
    S5: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    R4: Role + 'static,
    R5: Role + 'static,
    R6: Role + 'static,
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
        stack: role_a,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
    };

    let new_session_1 = send(Either::Left(choice_a), s.session1);
    let new_session_2 = send(Either::Left(choice_c), s.session2);
    let (new_queue, _) = next_b_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ba,
        session2: session_bc,
        stack: role_b,
    }
}

/// Given a choice from B, to other processes, between two `SessionMpst`, choose the second option for each.
///
/// B has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each second option then the related `SessionMpst`.
/// It then sends those options to the related processes.
pub fn choose_right_mpst_session_b_to_all<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: ShortSessionMpstBtoAll<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>,
) -> SessionMpst<S4, S5, R6>
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    S4: Session + 'static,
    S5: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    R4: Role + 'static,
    R5: Role + 'static,
    R6: Role + 'static,
{
    let (session_ba, session_ab) = S4::new();
    let (session_bc, session_cb) = S5::new();
    let (session_ca, session_ac) = S1::new();
    let (_, role_a) = R2::new();
    let (_, role_c) = R4::new();
    let (role_b, _) = R6::new();

    let choice_a = SessionMpst {
        session1: session_ab,
        session2: session_ac,
        stack: role_a,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
    };

    let new_session_1 = send(Either::Right(choice_a), s.session1);
    let new_session_2 = send(Either::Right(choice_c), s.session2);
    let (_, new_queue) = next_b_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ba,
        session2: session_bc,
        stack: role_b,
    }
}

/// Given a choice from C, to other processes, between two `SessionMpst`, choose the first option for each.
///
/// C has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each first option then the related `SessionMpst`.
/// It then sends those options to the related processes.
pub fn choose_left_mpst_session_c_to_all<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: ShortSessionMpstCtoAll<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>,
) -> SessionMpst<S2, S3, R5>
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    S4: Session + 'static,
    S5: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    R4: Role + 'static,
    R5: Role + 'static,
    R6: Role + 'static,
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
        stack: role_a,
    };

    let choice_b = SessionMpst {
        session1: session_ba,
        session2: session_bc,
        stack: role_b,
    };

    let new_session_1 = send(Either::Left(choice_a), s.session1);
    let new_session_2 = send(Either::Left(choice_b), s.session2);
    let (new_queue, _) = next_c_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
    }
}

/// Given a choice from C, to other processes, between two `SessionMpst`, choose the second option for each.
///
/// C has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each second option then the related `SessionMpst`.
/// It then sends those options to the related processes.
pub fn choose_right_mpst_session_c_to_all<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>(
    s: ShortSessionMpstCtoAll<S0, S1, S2, S3, S4, S5, R1, R2, R3, R4, R5, R6>,
) -> SessionMpst<S4, S5, R6>
where
    S0: Session + 'static,
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    S4: Session + 'static,
    S5: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    R4: Role + 'static,
    R5: Role + 'static,
    R6: Role + 'static,
{
    let (session_ca, session_ac) = S4::new();
    let (session_cb, session_bc) = S5::new();
    let (session_ba, session_ab) = S1::new();
    let (_, role_a) = R2::new();
    let (_, role_b) = R4::new();
    let (role_c, _) = R6::new();

    let choice_a = SessionMpst {
        session1: session_ab,
        session2: session_ac,
        stack: role_a,
    };

    let choice_b = SessionMpst {
        session1: session_ba,
        session2: session_bc,
        stack: role_b,
    };

    let new_session_1 = send(Either::Right(choice_a), s.session1);
    let new_session_2 = send(Either::Right(choice_b), s.session2);
    let (_, new_queue) = next_c_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
    };

    cancel(s);

    SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
    }
}

/// Choose, for A, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_a_to_all {
    ($labelone:path, $labeltwo:path, $session:expr) => {{
        let (session_ac, session_ca) = <_ as Session>::new();
        let (session_bc, session_cb) = <_ as Session>::new();
        let (session_ab, session_ba) = <_ as Session>::new();
        let (queue_a, _) = <_ as Role>::new();
        let (queue_b, _) = <_ as Role>::new();
        let (queue_c, _) = <_ as Role>::new();

        let s = send_mpst_a_to_b(
            $labelone(SessionMpst {
                session1: session_ba,
                session2: session_bc,
                stack: queue_b,
            }),
            $session,
        );
        let s = send_mpst_a_to_c(
            $labeltwo(SessionMpst {
                session1: session_ca,
                session2: session_cb,
                stack: queue_c,
            }),
            s,
        );

        cancel(s);

        SessionMpst {
            session1: session_ab,
            session2: session_ac,
            stack: queue_a,
        }
    }};
}

/// Choose, for B, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_b_to_all {
    ($labelone:path, $labeltwo:path, $session:expr) => {{
        let (session_ac, session_ca) = <_ as Session>::new();
        let (session_bc, session_cb) = <_ as Session>::new();
        let (session_ab, session_ba) = <_ as Session>::new();
        let (queue_a, _) = <_ as Role>::new();
        let (queue_b, _) = <_ as Role>::new();
        let (queue_c, _) = <_ as Role>::new();

        let s = send_mpst_b_to_a(
            $labelone(SessionMpst {
                session1: session_ab,
                session2: session_ac,
                stack: queue_a,
            }),
            $session,
        );
        let s = send_mpst_b_to_c(
            $labeltwo(SessionMpst {
                session1: session_ca,
                session2: session_cb,
                stack: queue_c,
            }),
            s,
        );

        cancel(s);

        SessionMpst {
            session1: session_ba,
            session2: session_bc,
            stack: queue_b,
        }
    }};
}

/// Choose, for C, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_c_to_all {
    ($labelone:path, $labeltwo:path, $session:expr) => {{
        let (session_ac, session_ca) = <_ as Session>::new();
        let (session_bc, session_cb) = <_ as Session>::new();
        let (session_ab, session_ba) = <_ as Session>::new();
        let (queue_a, _) = <_ as Role>::new();
        let (queue_b, _) = <_ as Role>::new();
        let (queue_c, _) = <_ as Role>::new();

        let s = send_mpst_c_to_a(
            $labelone(SessionMpst {
                session1: session_ab,
                session2: session_ac,
                stack: queue_a,
            }),
            $session,
        );
        let s = send_mpst_c_to_b(
            $labeltwo(SessionMpst {
                session1: session_ba,
                session2: session_bc,
                stack: queue_b,
            }),
            s,
        );

        cancel(s);

        SessionMpst {
            session1: session_ca,
            session2: session_cb,
            stack: queue_c,
        }
    }};
}
