use binary::{cancel, Session};
use functionmpst::recv::recv_mpst_session_one_a_to_b;
use functionmpst::recv::recv_mpst_session_one_b_to_a;
use functionmpst::recv::recv_mpst_session_one_c_to_a;
use functionmpst::recv::recv_mpst_session_two_a_to_c;
use functionmpst::recv::recv_mpst_session_two_b_to_c;
use functionmpst::recv::recv_mpst_session_two_c_to_b;
use functionmpst::OfferMpst;
use role::a_to_b::RoleAtoB;
use role::a_to_c::RoleAtoC;
use role::b_to_a::RoleBtoA;
use role::b_to_c::RoleBtoC;
use role::c_to_a::RoleCtoA;
use role::c_to_b::RoleCtoB;
use role::Role;
use sessionmpst::SessionMpst;
use std::error::Error;

pub fn offer_mpst_session_b_to_a<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleBtoA<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_session_one_b_to_a(s)?;
    cancel(s);
    e.either(f, g)
}

pub fn offer_mpst_session_a_to_b<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleAtoB<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_session_one_a_to_b(s)?;
    cancel(s);
    e.either(f, g)
}

pub fn offer_mpst_session_a_to_c<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<S5, OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleAtoC<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_session_two_a_to_c(s)?;
    cancel(s);
    e.either(f, g)
}

pub fn offer_mpst_session_c_to_a<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleCtoA<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_session_one_c_to_a(s)?;
    cancel(s);
    e.either(f, g)
}

pub fn offer_mpst_session_b_to_c<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<S5, OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleBtoC<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_session_two_b_to_c(s)?;
    cancel(s);
    e.either(f, g)
}

pub fn offer_mpst_session_c_to_b<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
    s: SessionMpst<S5, OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleCtoB<R3>>,
    f: F,
    g: G,
) -> Result<U, Box<dyn Error + 'a>>
where
    S1: Session,
    S2: Session,
    S3: Session,
    S4: Session,
    S5: Session,
    R1: Role,
    R2: Role,
    R3: Role,
    F: FnOnce(SessionMpst<S1, S2, R1>) -> Result<U, Box<dyn Error + 'a>>,
    G: FnOnce(SessionMpst<S3, S4, R2>) -> Result<U, Box<dyn Error + 'a>>,
{
    let (e, s) = recv_mpst_session_two_c_to_b(s)?;
    cancel(s);
    e.either(f, g)
}
