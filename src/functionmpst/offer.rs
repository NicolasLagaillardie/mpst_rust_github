use binary::{cancel, Session};

use functionmpst::recv::recv_mpst_session_one_a_to_b;
use functionmpst::recv::recv_mpst_session_one_b_to_a;
use functionmpst::recv::recv_mpst_session_one_c_to_a;
use functionmpst::recv::recv_mpst_session_two_a_to_c;
use functionmpst::recv::recv_mpst_session_two_b_to_c;
use functionmpst::recv::recv_mpst_session_two_c_to_b;

use role::a_receives_from_b::RoleAReceiveFromB;
use role::b_receives_from_a::RoleBReceiveFromA;
use role::c_receives_from_b::RoleCReceiveFromB;
use role::b_receives_from_c::RoleBReceiveFromC;
use role::a_receives_from_c::RoleAReceiveFromC;
use role::c_receives_from_a::RoleCReceiveFromA;

use role::Role;
use sessionmpst::SessionMpst;
use std::error::Error;
use functionmpst::OfferMpst;

pub fn offer_mpst_session_b_to_a<'a, S1, S2, R1, S3, S4, R2, S5, F, G, R3, U>(
    s: SessionMpst<OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleBReceiveFromA<R3>>,
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

pub fn offer_mpst_session_a_to_b<'a, S1, S2, R1, S3, S4, R2, S5, F, G, R3, U>(
    s: SessionMpst<OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleAReceiveFromB<R3>>,
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

pub fn offer_mpst_session_a_to_c<'a, S1, S2, R1, S3, S4, R2, S5, F, G, R3, U>(
    s: SessionMpst<S5, OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleAReceiveFromC<R3>>,
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

pub fn offer_mpst_session_c_to_a<'a, S1, S2, R1, S3, S4, R2, S5, F, G, R3, U>(
    s: SessionMpst<OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, S5, RoleCReceiveFromA<R3>>,
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

pub fn offer_mpst_session_b_to_c<'a, S1, S2, R1, S3, S4, R2, S5, F, G, R3, U>(
    s: SessionMpst<S5, OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleBReceiveFromC<R3>>,
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

pub fn offer_mpst_session_c_to_b<'a, S1, S2, R1, S3, S4, R2, S5, F, G, R3, U>(
    s: SessionMpst<S5, OfferMpst<SessionMpst<S1, S2, R1>, SessionMpst<S3, S4, R2>>, RoleCReceiveFromB<R3>>,
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
