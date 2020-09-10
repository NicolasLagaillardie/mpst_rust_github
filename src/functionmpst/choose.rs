use crate::binary::{cancel, send, Session};
use crate::functionmpst::ChooseMpst;
use crate::role::a::RoleA;
use crate::role::a_dual::RoleADual;
use crate::role::a_to_all::{next_a_to_all, RoleAtoAll};
use crate::role::b::RoleB;
use crate::role::b_dual::RoleBDual;
use crate::role::b_to_all::{next_b_to_all, RoleBtoAll};
use crate::role::c::RoleC;
use crate::role::c_dual::RoleCDual;
use crate::role::c_to_all::{next_c_to_all, RoleCtoAll};
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use either::Either;

type ShortChooseMpstOne<S0, S1, S2, S4, R0, R1, N0, N1> =
    ChooseMpst<S2, S0, S4, S1, R0, R1, N0, N1>;
type ShortChooseMpstTwo<S0, S1, S3, S5, R0, R1, N0, N1> =
    ChooseMpst<S3, <S0 as Session>::Dual, S5, <S1 as Session>::Dual, R0, R1, N0, N1>;

type ShortSessionMpstAtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5> = SessionMpst<
    ShortChooseMpstOne<S0, S1, S2, S4, R0, R1, RoleBDual<RoleEnd>, RoleBDual<RoleEnd>>,
    ShortChooseMpstTwo<S0, S1, S3, S5, R2, R3, RoleCDual<RoleEnd>, RoleCDual<RoleEnd>>,
    RoleAtoAll<R4, R5>,
    RoleA<RoleEnd>,
>;
type ShortSessionMpstBtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5> = SessionMpst<
    ShortChooseMpstOne<S0, S1, S2, S4, R0, R1, RoleADual<RoleEnd>, RoleADual<RoleEnd>>,
    ShortChooseMpstTwo<
        <S3 as Session>::Dual,
        <S5 as Session>::Dual,
        <S0 as Session>::Dual,
        <S1 as Session>::Dual,
        R2,
        R3,
        RoleCDual<RoleEnd>,
        RoleCDual<RoleEnd>,
    >,
    RoleBtoAll<R4, R5>,
    RoleB<RoleEnd>,
>;
type ShortSessionMpstCtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5> = SessionMpst<
    ShortChooseMpstOne<S2, S4, S0, S1, R0, R1, RoleADual<RoleEnd>, RoleADual<RoleEnd>>,
    ShortChooseMpstTwo<
        <S3 as Session>::Dual,
        <S5 as Session>::Dual,
        <S0 as Session>::Dual,
        <S1 as Session>::Dual,
        R2,
        R3,
        RoleBDual<RoleEnd>,
        RoleBDual<RoleEnd>,
    >,
    RoleCtoAll<R4, R5>,
    RoleC<RoleEnd>,
>;

/// Given a choice from A, to other processes, between two `SessionMpst`, choose the first option for each.
///
/// A has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each first option then the related `SessionMpst`.
/// It then sends those options to the related processes.
///
/// * S0: dual session from B to C on left branch
/// * S1: dual session from B to C on right branch
/// * S2: session from A to B on left branch
/// * S3: session from A to C on left branch
/// * S4: session from A to B on right branch
/// * S5: session from A to C on right branch
///
/// * R0: dual stack of B on left branch
/// * R1: dual stack of B on right branch
/// * R2: dual stack of C on left branch
/// * R3: dual stack of C on right branch
/// * R4: stack of A on left branch
/// * R5: stack of A on right branch
pub fn choose_left_mpst_session_a_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortSessionMpstAtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> SessionMpst<S2, S3, R4, RoleA<RoleEnd>>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    let (session_ab, session_ba) = S2::new();
    let (session_ac, session_ca) = S3::new();
    let (session_cb, session_bc) = S0::new();
    let (_, role_b) = R0::new();
    let (_, role_c) = R2::new();
    let (role_a, _) = R4::new();
    let (name_a, _) = RoleA::<RoleEnd>::new();
    let (name_b, _) = RoleB::<RoleEnd>::new();
    let (name_c, _) = RoleC::<RoleEnd>::new();

    let choice_b = SessionMpst {
        session1: session_ba,
        session2: session_bc,
        stack: role_b,
        name: name_b,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
        name: name_c,
    };

    let new_session_1 = send(Either::Left(choice_b), s.session1);
    let new_session_2 = send(Either::Left(choice_c), s.session2);
    let (new_queue, _) = next_a_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
        name: s.name,
    };

    cancel(s);

    SessionMpst {
        session1: session_ab,
        session2: session_ac,
        stack: role_a,
        name: name_a,
    }
}

/// Given a choice from A, to other processes, between two `SessionMpst`, choose the second option for each.
///
/// A has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each second option then the related `SessionMpst`.
/// It then sends those options to the related processes.
///
/// * S0: dual session from B to C on left branch
/// * S1: dual session from B to C on right branch
/// * S2: session from A to B on left branch
/// * S3: session from A to C on left branch
/// * S4: session from A to B on right branch
/// * S5: session from A to C on right branch
///
/// * R0: dual stack of B on left branch
/// * R1: dual stack of B on right branch
/// * R2: dual stack of C on left branch
/// * R3: dual stack of C on right branch
/// * R4: stack of A on left branch
/// * R5: stack of A on right branch
pub fn choose_right_mpst_session_a_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortSessionMpstAtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> SessionMpst<S4, S5, R5, RoleA<RoleEnd>>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    let (session_ab, session_ba) = S4::new();
    let (session_ac, session_ca) = S5::new();
    let (session_cb, session_bc) = S1::new();
    let (_, role_b) = R1::new();
    let (_, role_c) = R3::new();
    let (role_a, _) = R5::new();
    let (name_a, _) = RoleA::<RoleEnd>::new();
    let (name_b, _) = RoleB::<RoleEnd>::new();
    let (name_c, _) = RoleC::<RoleEnd>::new();

    let choice_b = SessionMpst {
        session1: session_ba,
        session2: session_bc,
        stack: role_b,
        name: name_b,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
        name: name_c,
    };

    let new_session_1 = send(Either::Right(choice_b), s.session1);
    let new_session_2 = send(Either::Right(choice_c), s.session2);
    let (_, new_queue) = next_a_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
        name: s.name,
    };

    cancel(s);

    SessionMpst {
        session1: session_ab,
        session2: session_ac,
        stack: role_a,
        name: name_a,
    }
}

/// Given a choice from B, to other processes, between two `SessionMpst`, choose the first option for each.
///
/// B has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each first option then the related `SessionMpst`.
/// It then sends those options to the related processes.
///
/// * S0: dual session from A to C on left branch
/// * S1: dual session from A to C on right branch
/// * S2: session from B to A on left branch
/// * S3: session from B to C on left branch
/// * S4: session from B to A on right branch
/// * S5: session from B to C on right branch
///
/// * R0: dual stack of A on left branch
/// * R1: dual stack of A on right branch
/// * R2: dual stack of C on left branch
/// * R3: dual stack of C on right branch
/// * R4: stack of B on left branch
/// * R5: stack of B on right branch
pub fn choose_left_mpst_session_b_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortSessionMpstBtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> SessionMpst<S2, S3, R4, RoleB<RoleEnd>>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    let (session_ba, session_ab) = S2::new();
    let (session_bc, session_cb) = S3::new();
    let (session_ca, session_ac) = S0::new();
    let (_, role_a) = R0::new();
    let (_, role_c) = R2::new();
    let (role_b, _) = R4::new();
    let (name_a, _) = RoleA::<RoleEnd>::new();
    let (name_b, _) = RoleB::<RoleEnd>::new();
    let (name_c, _) = RoleC::<RoleEnd>::new();

    let choice_a = SessionMpst {
        session1: session_ab,
        session2: session_ac,
        stack: role_a,
        name: name_a,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
        name: name_c,
    };

    let new_session_1 = send(Either::Left(choice_a), s.session1);
    let new_session_2 = send(Either::Left(choice_c), s.session2);
    let (new_queue, _) = next_b_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
        name: s.name,
    };

    cancel(s);

    SessionMpst {
        session1: session_ba,
        session2: session_bc,
        stack: role_b,
        name: name_b,
    }
}

/// Given a choice from B, to other processes, between two `SessionMpst`, choose the second option for each.
///
/// B has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each second option then the related `SessionMpst`.
/// It then sends those options to the related processes.
///
/// * S0: dual session from A to C on left branch
/// * S1: dual session from A to C on right branch
/// * S2: session from B to A on left branch
/// * S3: session from B to C on left branch
/// * S4: session from B to A on right branch
/// * S5: session from B to C on right branch
///
/// * R0: dual stack of A on left branch
/// * R1: dual stack of A on right branch
/// * R2: dual stack of C on left branch
/// * R3: dual stack of C on right branch
/// * R4: stack of B on left branch
/// * R5: stack of B on right branch
pub fn choose_right_mpst_session_b_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortSessionMpstBtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> SessionMpst<S4, S5, R5, RoleB<RoleEnd>>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    let (session_ba, session_ab) = S4::new();
    let (session_bc, session_cb) = S5::new();
    let (session_ca, session_ac) = S1::new();
    let (_, role_a) = R1::new();
    let (_, role_c) = R3::new();
    let (role_b, _) = R5::new();
    let (name_a, _) = RoleA::<RoleEnd>::new();
    let (name_b, _) = RoleB::<RoleEnd>::new();
    let (name_c, _) = RoleC::<RoleEnd>::new();

    let choice_a = SessionMpst {
        session1: session_ab,
        session2: session_ac,
        stack: role_a,
        name: name_a,
    };

    let choice_c = SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
        name: name_c,
    };

    let new_session_1 = send(Either::Right(choice_a), s.session1);
    let new_session_2 = send(Either::Right(choice_c), s.session2);
    let (_, new_queue) = next_b_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
        name: s.name,
    };

    cancel(s);

    SessionMpst {
        session1: session_ba,
        session2: session_bc,
        stack: role_b,
        name: name_b,
    }
}

/// Given a choice from C, to other processes, between two `SessionMpst`, choose the first option for each.
///
/// C has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each first option then the related `SessionMpst`.
/// It then sends those options to the related processes.
///
/// * S0: dual session from A to B on left branch
/// * S1: dual session from A to B on right branch
/// * S2: session from C to A on left branch
/// * S3: session from C to B on left branch
/// * S4: session from C to A on right branch
/// * S5: session from C to B on right branch
///
/// * R0: dual stack of A on left branch
/// * R1: dual stack of A on right branch
/// * R2: dual stack of B on left branch
/// * R3: dual stack of B on right branch
/// * R4: stack of C on left branch
/// * R5: stack of C on right branch
pub fn choose_left_mpst_session_c_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortSessionMpstCtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> SessionMpst<S2, S3, R4, RoleC<RoleEnd>>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    let (session_ca, session_ac) = S2::new();
    let (session_cb, session_bc) = S3::new();
    let (session_ba, session_ab) = S0::new();
    let (_, role_a) = R0::new();
    let (_, role_b) = R2::new();
    let (role_c, _) = R4::new();
    let (name_a, _) = RoleA::<RoleEnd>::new();
    let (name_b, _) = RoleB::<RoleEnd>::new();
    let (name_c, _) = RoleC::<RoleEnd>::new();

    let choice_a = SessionMpst {
        session1: session_ab,
        session2: session_ac,
        stack: role_a,
        name: name_a,
    };

    let choice_b = SessionMpst {
        session1: session_ba,
        session2: session_bc,
        stack: role_b,
        name: name_b,
    };

    let new_session_1 = send(Either::Left(choice_a), s.session1);
    let new_session_2 = send(Either::Left(choice_b), s.session2);
    let (new_queue, _) = next_c_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
        name: s.name,
    };

    cancel(s);

    SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
        name: name_c,
    }
}

/// Given a choice from C, to other processes, between two `SessionMpst`, choose the second option for each.
///
/// C has to encapsulate all possible `SessionMpst` for each other role.
/// This function creates the 6 new binary `Session`, the 3 new `Role` related to each second option then the related `SessionMpst`.
/// It then sends those options to the related processes.
///
/// * S0: dual session from A to B on left branch
/// * S1: dual session from A to B on right branch
/// * S2: session from C to A on left branch
/// * S3: session from C to B on left branch
/// * S4: session from C to A on right branch
/// * S5: session from C to B on right branch
///
/// * R0: dual stack of A on left branch
/// * R1: dual stack of A on right branch
/// * R2: dual stack of B on left branch
/// * R3: dual stack of B on right branch
/// * R4: stack of C on left branch
/// * R5: stack of C on right branch
pub fn choose_right_mpst_session_c_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortSessionMpstCtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> SessionMpst<S4, S5, R5, RoleC<RoleEnd>>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    let (session_ca, session_ac) = S4::new();
    let (session_cb, session_bc) = S5::new();
    let (session_ba, session_ab) = S1::new();
    let (_, role_a) = R1::new();
    let (_, role_b) = R3::new();
    let (role_c, _) = R5::new();
    let (name_a, _) = RoleA::<RoleEnd>::new();
    let (name_b, _) = RoleB::<RoleEnd>::new();
    let (name_c, _) = RoleC::<RoleEnd>::new();

    let choice_a = SessionMpst {
        session1: session_ab,
        session2: session_ac,
        stack: role_a,
        name: name_a,
    };

    let choice_b = SessionMpst {
        session1: session_ba,
        session2: session_bc,
        stack: role_b,
        name: name_b,
    };

    let new_session_1 = send(Either::Right(choice_a), s.session1);
    let new_session_2 = send(Either::Right(choice_b), s.session2);
    let (_, new_queue) = next_c_to_all(s.stack);

    let s = SessionMpst {
        session1: new_session_1,
        session2: new_session_2,
        stack: new_queue,
        name: s.name,
    };

    cancel(s);

    SessionMpst {
        session1: session_ca,
        session2: session_cb,
        stack: role_c,
        name: name_c,
    }
}

/// Choose, for A, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_a_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        let (session_ac, session_ca) = <_ as Session>::new();
        let (session_bc, session_cb) = <_ as Session>::new();
        let (session_ab, session_ba) = <_ as Session>::new();
        let (queue_a, _) = <_ as Role>::new();
        let (queue_b, _) = <_ as Role>::new();
        let (queue_c, _) = <_ as Role>::new();
        let (name_a, _) = RoleA::<RoleEnd>::new();
        let (name_b, _) = RoleB::<RoleEnd>::new();
        let (name_c, _) = RoleC::<RoleEnd>::new();

        let s = send_mpst_a_to_b(
            $labelone(SessionMpst {
                session1: session_ba,
                session2: session_bc,
                stack: queue_b,
                name: name_b,
            }),
            $session,
        );
        let s = send_mpst_a_to_c(
            $labeltwo(SessionMpst {
                session1: session_ca,
                session2: session_cb,
                stack: queue_c,
                name: name_c,
            }),
            s,
        );

        cancel(s);

        SessionMpst {
            session1: session_ab,
            session2: session_ac,
            stack: queue_a,
            name: name_a,
        }
    }};
}

/// Choose, for B, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_b_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        let (session_ac, session_ca) = <_ as Session>::new();
        let (session_bc, session_cb) = <_ as Session>::new();
        let (session_ab, session_ba) = <_ as Session>::new();
        let (queue_a, _) = <_ as Role>::new();
        let (queue_b, _) = <_ as Role>::new();
        let (queue_c, _) = <_ as Role>::new();
        let (name_a, _) = RoleA::<RoleEnd>::new();
        let (name_b, _) = RoleB::<RoleEnd>::new();
        let (name_c, _) = RoleC::<RoleEnd>::new();

        let s = send_mpst_b_to_a(
            $labelone(SessionMpst {
                session1: session_ab,
                session2: session_ac,
                stack: queue_a,
                name: name_a,
            }),
            $session,
        );
        let s = send_mpst_b_to_c(
            $labeltwo(SessionMpst {
                session1: session_ca,
                session2: session_cb,
                stack: queue_c,
                name: name_c,
            }),
            s,
        );

        cancel(s);

        SessionMpst {
            session1: session_ba,
            session2: session_bc,
            stack: queue_b,
            name: name_b,
        }
    }};
}

/// Choose, for C, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_c_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        let (session_ac, session_ca) = <_ as Session>::new();
        let (session_bc, session_cb) = <_ as Session>::new();
        let (session_ab, session_ba) = <_ as Session>::new();
        let (queue_a, _) = <_ as Role>::new();
        let (queue_b, _) = <_ as Role>::new();
        let (queue_c, _) = <_ as Role>::new();
        let (name_a, _) = RoleA::<RoleEnd>::new();
        let (name_b, _) = RoleB::<RoleEnd>::new();
        let (name_c, _) = RoleC::<RoleEnd>::new();

        let s = send_mpst_c_to_a(
            $labelone(SessionMpst {
                session1: session_ab,
                session2: session_ac,
                stack: queue_a,
                name: name_a,
            }),
            $session,
        );
        let s = send_mpst_c_to_b(
            $labeltwo(SessionMpst {
                session1: session_ba,
                session2: session_bc,
                stack: queue_b,
                name: name_b,
            }),
            s,
        );

        cancel(s);

        SessionMpst {
            session1: session_ca,
            session2: session_cb,
            stack: queue_c,
            name: name_c,
        }
    }};
}

// create the core for the choose_mpst macros
#[macro_export]
macro_rules! create_choose {
    ($session_1:ty, $session_2:ty, $session_3:ty, $role_1:ty, $role_2:ty, $role_3:ty, $name_1:ty, $name_2:ty, $name_3:ty, $session:expr, $pat:path, $next:ident) => {{
        let (session_3_1, session_1_3) = <$session_1>::new();
        let (session_3_2, session_2_3) = <$session_2>::new();
        let (session_2_1, session_1_2) = <$session_3>::new();
        let (_, role_1) = <$role_1>::new();
        let (_, role_2) = <$role_2>::new();
        let (role_3, _) = <$role_3>::new();
        let (name_1, _) = <$name_1>::new();
        let (name_2, _) = <$name_2>::new();
        let (name_3, _) = <$name_3>::new();

        let choice_1 = SessionMpst {
            session1: session_1_2,
            session2: session_1_3,
            stack: role_1,
            name: name_1,
        };

        let choice_2 = SessionMpst {
            session1: session_2_1,
            session2: session_2_3,
            stack: role_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);
        let (_, new_queue) = $next($session.stack);

        let s = SessionMpst {
            session1: new_session_1,
            session2: new_session_2,
            stack: new_queue,
            name: name_3,
        };

        cancel(s);

        SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_3,
            name: name_3,
        }
    }};
}
