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

type ShortChooseMpstOne<S0, S1, S2, S4, R0, R1, N0> = ChooseMpst<S2, S0, S4, S1, R0, R1, N0>;
type ShortChooseMpstTwo<S0, S1, S3, S5, R0, R1, N0> =
    ChooseMpst<S3, <S0 as Session>::Dual, S5, <S1 as Session>::Dual, R0, R1, N0>;

type ShortSessionMpstAtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5> = SessionMpst<
    ShortChooseMpstOne<S0, S1, S2, S4, R0, R1, RoleBDual<RoleEnd>>,
    ShortChooseMpstTwo<S0, S1, S3, S5, R2, R3, RoleCDual<RoleEnd>>,
    RoleAtoAll<R4, R5>,
    RoleA<RoleEnd>,
>;
type ShortSessionMpstBtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5> = SessionMpst<
    ShortChooseMpstOne<S0, S1, S2, S4, R0, R1, RoleADual<RoleEnd>>,
    ShortChooseMpstTwo<
        <S3 as Session>::Dual,
        <S5 as Session>::Dual,
        <S0 as Session>::Dual,
        <S1 as Session>::Dual,
        R2,
        R3,
        RoleCDual<RoleEnd>,
    >,
    RoleBtoAll<R4, R5>,
    RoleB<RoleEnd>,
>;
type ShortSessionMpstCtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5> = SessionMpst<
    ShortChooseMpstOne<S2, S4, S0, S1, R0, R1, RoleADual<RoleEnd>>,
    ShortChooseMpstTwo<
        <S3 as Session>::Dual,
        <S5 as Session>::Dual,
        <S0 as Session>::Dual,
        <S1 as Session>::Dual,
        R2,
        R3,
        RoleBDual<RoleEnd>,
    >,
    RoleCtoAll<R4, R5>,
    RoleC<RoleEnd>,
>;

macro_rules! choose_mpst_a {
    ($session_1:ty, $session_2:ty, $session_3:ty, $stack_1:ty, $stack_2:ty, $stack_3:ty, $receiver_1:ident, $receiver_2:ident, $sender:ident, $session:expr, $pat:path, $next:ident) => {{
        let (session_1_2, session_2_1) = <$session_1>::new();
        let (session_1_3, session_3_1) = <$session_2>::new();
        let (session_3_2, session_2_3) = <$session_3>::new();
        let (_, stack_1) = <$stack_1>::new();
        let (_, stack_2) = <$stack_2>::new();
        let (stack_3, _) = <$stack_3>::new();
        let (name_1, _) = <$receiver_1<RoleEnd> as Role>::Dual::new();
        let (name_2, _) = <$receiver_2<RoleEnd> as Role>::Dual::new();
        let (name_3, _) = $sender::<RoleEnd>::new();

        let choice_1 = SessionMpst {
            session1: session_2_1,
            session2: session_2_3,
            stack: stack_1,
            name: name_1,
        };

        let choice_2 = SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: stack_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);
        let (_, new_queue) = $next($session.stack);

        let s = SessionMpst {
            session1: new_session_1,
            session2: new_session_2,
            stack: new_queue,
            name: $session.name,
        };

        cancel(s);

        SessionMpst {
            session1: session_1_2,
            session2: session_1_3,
            stack: stack_3,
            name: name_3,
        }
    }};
}

macro_rules! choose_mpst_b {
    ($session_1:ty, $session_2:ty, $session_3:ty, $stack_1:ty, $stack_2:ty, $stack_3:ty, $receiver_1:ident, $receiver_2:ident, $sender:ident, $session:expr, $pat:path, $next:ident) => {{
        let (session_2_1, session_1_2) = <$session_1>::new();
        let (session_2_3, session_3_2) = <$session_2>::new();
        let (session_3_1, session_1_3) = <$session_3>::new();
        let (_, stack_1) = <$stack_1>::new();
        let (_, stack_2) = <$stack_2>::new();
        let (stack_3, _) = <$stack_3>::new();
        let (name_1, _) = <$receiver_1<RoleEnd> as Role>::Dual::new();
        let (name_2, _) = <$receiver_2<RoleEnd> as Role>::Dual::new();
        let (name_3, _) = $sender::<RoleEnd>::new();

        let choice_1 = SessionMpst {
            session1: session_1_2,
            session2: session_1_3,
            stack: stack_1,
            name: name_1,
        };

        let choice_2 = SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: stack_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);
        let (_, new_queue) = $next($session.stack);

        let s = SessionMpst {
            session1: new_session_1,
            session2: new_session_2,
            stack: new_queue,
            name: $session.name,
        };

        cancel(s);

        SessionMpst {
            session1: session_2_1,
            session2: session_2_3,
            stack: stack_3,
            name: name_3,
        }
    }};
}

macro_rules! choose_mpst_c {
    ($session_1:ty, $session_2:ty, $session_3:ty, $stack_1:ty, $stack_2:ty, $stack_3:ty, $receiver_1:ident, $receiver_2:ident, $sender:ident, $session:expr, $pat:path, $next:ident) => {{
        let (session_3_1, session_1_3) = <$session_1>::new();
        let (session_3_2, session_2_3) = <$session_2>::new();
        let (session_2_1, session_1_2) = <$session_3>::new();
        let (_, stack_1) = <$stack_1>::new();
        let (_, stack_2) = <$stack_2>::new();
        let (stack_3, _) = <$stack_3>::new();
        let (name_1, _) = <$receiver_1<RoleEnd> as Role>::Dual::new();
        let (name_2, _) = <$receiver_2<RoleEnd> as Role>::Dual::new();
        let (name_3, _) = $sender::<RoleEnd>::new();

        let choice_1 = SessionMpst {
            session1: session_1_2,
            session2: session_1_3,
            stack: stack_1,
            name: name_1,
        };

        let choice_2 = SessionMpst {
            session1: session_2_1,
            session2: session_2_3,
            stack: stack_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);
        let (_, new_queue) = $next($session.stack);

        let s = SessionMpst {
            session1: new_session_1,
            session2: new_session_2,
            stack: new_queue,
            name: $session.name,
        };

        cancel(s);

        SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: stack_3,
            name: name_3,
        }
    }};
}

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
    choose_mpst_a!(
        S2,
        S3,
        S0,
        R0,
        R2,
        R4,
        RoleBDual,
        RoleCDual,
        RoleA,
        s,
        Either::Left,
        next_a_to_all
    )
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
    choose_mpst_a!(
        S4,
        S5,
        S1,
        R1,
        R3,
        R5,
        RoleBDual,
        RoleCDual,
        RoleA,
        s,
        Either::Right,
        next_a_to_all
    )
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
    choose_mpst_b!(
        S2,
        S3,
        S0,
        R0,
        R2,
        R4,
        RoleADual,
        RoleCDual,
        RoleB,
        s,
        Either::Left,
        next_b_to_all
    )
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
    choose_mpst_b!(
        S4,
        S5,
        S1,
        R1,
        R3,
        R5,
        RoleADual,
        RoleCDual,
        RoleB,
        s,
        Either::Right,
        next_b_to_all
    )
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
    choose_mpst_c!(
        S2,
        S3,
        S0,
        R0,
        R2,
        R4,
        RoleADual,
        RoleBDual,
        RoleC,
        s,
        Either::Left,
        next_c_to_all
    )
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
    choose_mpst_c!(
        S4,
        S5,
        S1,
        R1,
        R3,
        R5,
        RoleADual,
        RoleBDual,
        RoleC,
        s,
        Either::Right,
        next_c_to_all
    )
}

/// Choose, for A, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_a_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        let (session_ac, session_ca) = <_ as mpstthree::binary::Session>::new();
        let (session_bc, session_cb) = <_ as mpstthree::binary::Session>::new();
        let (session_ab, session_ba) = <_ as mpstthree::binary::Session>::new();
        let (stack_a, _) = <_ as mpstthree::role::Role>::new();
        let (stack_b, _) = <_ as mpstthree::role::Role>::new();
        let (stack_c, _) = <_ as mpstthree::role::Role>::new();
        let (name_a, _) = mpstthree::role::a::RoleA::<mpstthree::role::end::RoleEnd>::new();
        let (name_b, _) = mpstthree::role::b::RoleB::<mpstthree::role::end::RoleEnd>::new();
        let (name_c, _) = mpstthree::role::c::RoleC::<mpstthree::role::end::RoleEnd>::new();

        let s = mpstthree::functionmpst::send::send_mpst_a_to_b(
            $labelone(mpstthree::sessionmpst::SessionMpst {
                session1: session_ba,
                session2: session_bc,
                stack: stack_b,
                name: name_b,
            }),
            $session,
        );
        let s = mpstthree::functionmpst::send::send_mpst_a_to_c(
            $labeltwo(mpstthree::sessionmpst::SessionMpst {
                session1: session_ca,
                session2: session_cb,
                stack: stack_c,
                name: name_c,
            }),
            s,
        );

        mpstthree::binary::cancel(s);

        mpstthree::sessionmpst::SessionMpst {
            session1: session_ab,
            session2: session_ac,
            stack: stack_a,
            name: name_a,
        }
    }};
}

/// Choose, for B, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_b_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        let (session_ac, session_ca) = <_ as mpstthree::binary::Session>::new();
        let (session_bc, session_cb) = <_ as mpstthree::binary::Session>::new();
        let (session_ab, session_ba) = <_ as mpstthree::binary::Session>::new();
        let (stack_a, _) = <_ as mpstthree::role::Role>::new();
        let (stack_b, _) = <_ as mpstthree::role::Role>::new();
        let (stack_c, _) = <_ as mpstthree::role::Role>::new();
        let (name_a, _) = mpstthree::role::a::RoleA::<mpstthree::role::end::RoleEnd>::new();
        let (name_b, _) = mpstthree::role::b::RoleB::<mpstthree::role::end::RoleEnd>::new();
        let (name_c, _) = mpstthree::role::c::RoleC::<mpstthree::role::end::RoleEnd>::new();

        let s = mpstthree::functionmpst::send::send_mpst_b_to_a(
            $labelone(mpstthree::sessionmpst::SessionMpst {
                session1: session_ab,
                session2: session_ac,
                stack: stack_a,
                name: name_a,
            }),
            $session,
        );
        let s = mpstthree::functionmpst::send::send_mpst_b_to_c(
            $labeltwo(mpstthree::sessionmpst::SessionMpst {
                session1: session_ca,
                session2: session_cb,
                stack: stack_c,
                name: name_c,
            }),
            s,
        );

        mpstthree::binary::cancel(s);

        mpstthree::sessionmpst::SessionMpst {
            session1: session_ba,
            session2: session_bc,
            stack: stack_b,
            name: name_b,
        }
    }};
}

/// Choose, for C, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_c_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        let (session_ac, session_ca) = <_ as mpstthree::binary::Session>::new();
        let (session_bc, session_cb) = <_ as mpstthree::binary::Session>::new();
        let (session_ab, session_ba) = <_ as mpstthree::binary::Session>::new();
        let (stack_a, _) = <_ as mpstthree::role::Role>::new();
        let (stack_b, _) = <_ as mpstthree::role::Role>::new();
        let (stack_c, _) = <_ as mpstthree::role::Role>::new();
        let (name_a, _) = mpstthree::role::a::RoleA::<mpstthree::role::end::RoleEnd>::new();
        let (name_b, _) = mpstthree::role::b::RoleB::<mpstthree::role::end::RoleEnd>::new();
        let (name_c, _) = mpstthree::role::c::RoleC::<mpstthree::role::end::RoleEnd>::new();

        let s = mpstthree::functionmpst::send::send_mpst_c_to_a(
            $labelone(mpstthree::sessionmpst::SessionMpst {
                session1: session_ab,
                session2: session_ac,
                stack: stack_a,
                name: name_a,
            }),
            $session,
        );
        let s = mpstthree::functionmpst::send::send_mpst_c_to_b(
            $labeltwo(mpstthree::sessionmpst::SessionMpst {
                session1: session_ba,
                session2: session_bc,
                stack: stack_b,
                name: name_b,
            }),
            s,
        );

        mpstthree::binary::cancel(s);

        mpstthree::sessionmpst::SessionMpst {
            session1: session_ca,
            session2: session_cb,
            stack: stack_c,
            name: name_c,
        }
    }};
}
