use binary::{cancel, Session};
use either::Either;
use functionmpst::send::send_mpst_session_one_a_to_b;
use functionmpst::send::send_mpst_session_one_b_to_a;
use functionmpst::send::send_mpst_session_one_c_to_a;
use functionmpst::send::send_mpst_session_two_a_to_c;
use functionmpst::send::send_mpst_session_two_b_to_c;
use functionmpst::send::send_mpst_session_two_c_to_b;
use functionmpst::ChooseMpst;
use role::a_to_b::RoleAtoB;
use role::a_to_c::RoleAtoC;
use role::b_to_a::RoleBtoA;
use role::b_to_c::RoleBtoC;
use role::c_to_a::RoleCtoA;
use role::c_to_b::RoleCtoB;
use role::Role;
use sessionmpst::SessionMpst;

pub fn choose_left_mpst_session_a_to_b<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleAtoB<R3>>,
) -> SessionMpst<S1, S2, R1>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S1::new();
    let (here_2, there_2) = S2::new();
    let (role_1, role_2) = R1::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_one_a_to_b(Either::Left(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_right_mpst_session_a_to_b<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleAtoB<R3>>,
) -> SessionMpst<S3, S4, R2>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S3::new();
    let (here_2, there_2) = S4::new();
    let (role_1, role_2) = R2::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_one_a_to_b(Either::Right(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_left_mpst_session_b_to_a<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleBtoA<R3>>,
) -> SessionMpst<S1, S2, R1>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S1::new();
    let (here_2, there_2) = S2::new();
    let (role_1, role_2) = R1::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_one_b_to_a(Either::Left(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_right_mpst_session_b_to_a<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleBtoA<R3>>,
) -> SessionMpst<S3, S4, R2>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S3::new();
    let (here_2, there_2) = S4::new();
    let (role_1, role_2) = R2::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_one_b_to_a(Either::Right(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_left_mpst_session_a_to_c<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<S5, ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleAtoC<R3>>,
) -> SessionMpst<S1, S2, R1>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S1::new();
    let (here_2, there_2) = S2::new();
    let (role_1, role_2) = R1::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_two_a_to_c(Either::Left(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_right_mpst_session_a_to_c<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<S5, ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleAtoC<R3>>,
) -> SessionMpst<S3, S4, R2>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S3::new();
    let (here_2, there_2) = S4::new();
    let (role_1, role_2) = R2::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_two_a_to_c(Either::Right(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_left_mpst_session_c_to_a<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleCtoA<R3>>,
) -> SessionMpst<S1, S2, R1>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S1::new();
    let (here_2, there_2) = S2::new();
    let (role_1, role_2) = R1::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_one_c_to_a(Either::Left(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_right_mpst_session_c_to_a<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleCtoA<R3>>,
) -> SessionMpst<S3, S4, R2>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S3::new();
    let (here_2, there_2) = S4::new();
    let (role_1, role_2) = R2::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_one_c_to_a(Either::Right(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_left_mpst_session_b_to_c<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<S5, ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleBtoC<R3>>,
) -> SessionMpst<S1, S2, R1>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S1::new();
    let (here_2, there_2) = S2::new();
    let (role_1, role_2) = R1::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_two_b_to_c(Either::Left(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_right_mpst_session_b_to_c<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<S5, ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleBtoC<R3>>,
) -> SessionMpst<S3, S4, R2>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S3::new();
    let (here_2, there_2) = S4::new();
    let (role_1, role_2) = R2::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_two_b_to_c(Either::Right(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_left_mpst_session_c_to_b<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<S5, ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleCtoB<R3>>,
) -> SessionMpst<S1, S2, R1>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S1::new();
    let (here_2, there_2) = S2::new();
    let (role_1, role_2) = R1::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_two_c_to_b(Either::Left(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}

pub fn choose_right_mpst_session_c_to_b<'a, S1, S2, S3, S4, S5, R1, R2, R3>(
    s: SessionMpst<S5, ChooseMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleCtoB<R3>>,
) -> SessionMpst<S3, S4, R2>
where
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R1: Role,
    R2: Role,
    R3: Role,
{
    let (here_1, there_1) = S3::new();
    let (here_2, there_2) = S4::new();
    let (role_1, role_2) = R2::new();

    let choice = SessionMpst {
        session1: there_1,
        session2: there_2,
        queue: role_2,
    };

    let s = send_mpst_session_two_c_to_b(Either::Right(choice), s);

    cancel(s);

    return SessionMpst {
        session1: here_1,
        session2: here_2,
        queue: role_1,
    };
}
