use crate::binary::struct_trait::{End, Recv, Send, Session};
use crate::functionmpst::offer::{offer_mpst_session_to_b_from_a, offer_mpst_session_to_b_from_c};
use crate::functionmpst::recv::*;
use crate::functionmpst::send::*;
use crate::functionmpst::OfferMpst;
use crate::role::a::RoleA;
use crate::role::all_to_a::RoleAlltoA;
use crate::role::all_to_c::RoleAlltoC;
use crate::role::b::RoleB;
use crate::role::c::RoleC;
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use std::error::Error;
use std::marker;

type ReturnType<S1, S2, R> = SessionMpst<S1, S2, R, RoleB<RoleEnd>>;
type ResultType<T, S1, S2, R> = Result<(T, ReturnType<S1, S2, R>), Box<dyn Error>>;

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<Send<T, S1>, S2, RoleA<R>, RoleB<RoleEnd>>
{
    pub fn send(self, payload: T) -> ReturnType<S1, S2, R> {
        send_mpst_b_to_a(payload, self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<S1, Send<T, S2>, RoleC<R>, RoleB<RoleEnd>>
{
    pub fn send(self, payload: T) -> ReturnType<S1, S2, R> {
        send_mpst_b_to_c(payload, self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<Recv<T, S1>, S2, RoleA<R>, RoleB<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_b_from_a(self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<S1, Recv<T, S2>, RoleC<R>, RoleB<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_b_from_c(self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<Recv<T, S1>, S2, RoleAlltoA<R, R>, RoleB<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_b_all_to_a(self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<S1, Recv<T, S2>, RoleAlltoC<R, R>, RoleB<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_b_all_to_c(self)
    }
}

impl SessionMpst<End, End, RoleEnd, RoleB<RoleEnd>> {
    pub fn close(self) -> Result<(), Box<dyn Error>> {
        crate::functionmpst::close::close_mpst(self)
    }
}

impl<'a, S1: Session, S2: Session, S3: Session, S4: Session, R1: Role, R2: Role>
    SessionMpst<
        OfferMpst<S1, S2, S3, S4, R1, R2, RoleB<RoleEnd>>,
        End,
        RoleAlltoA<RoleEnd, RoleEnd>,
        RoleB<RoleEnd>,
    >
{
    pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn Error + 'a>>
    where
        F: FnOnce(SessionMpst<S1, S2, R1, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
        G: FnOnce(SessionMpst<S3, S4, R2, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    {
        offer_mpst_session_to_b_from_a(self, f, g)
    }
}

impl<'a, S1: Session, S2: Session, S3: Session, S4: Session, R1: Role, R2: Role>
    SessionMpst<
        End,
        OfferMpst<S1, S2, S3, S4, R1, R2, RoleB<RoleEnd>>,
        RoleAlltoC<RoleEnd, RoleEnd>,
        RoleB<RoleEnd>,
    >
{
    pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn Error + 'a>>
    where
        F: FnOnce(SessionMpst<S1, S2, R1, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
        G: FnOnce(SessionMpst<S3, S4, R2, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    {
        offer_mpst_session_to_b_from_c(self, f, g)
    }
}
