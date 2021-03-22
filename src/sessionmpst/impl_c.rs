use crate::binary::struct_trait::{End, Recv, Send, Session};
use crate::functionmpst::recv::*;
use crate::functionmpst::send::*;
use crate::role::a::RoleA;
use crate::role::all_to_a::RoleAlltoA;
use crate::role::all_to_b::RoleAlltoB;
use crate::role::b::RoleB;
use crate::role::c::RoleC;
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use std::error::Error;
use std::marker;

type ReturnType<S1, S2, R> = SessionMpst<S1, S2, R, RoleC<RoleEnd>>;
type ResultType<T, S1, S2, R> = Result<(T, ReturnType<S1, S2, R>), Box<dyn Error>>;

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<Send<T, S1>, S2, RoleA<R>, RoleC<RoleEnd>>
{
    pub fn send(self, payload: T) -> ReturnType<S1, S2, R> {
        send_mpst_c_to_a(payload, self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<S1, Send<T, S2>, RoleB<R>, RoleC<RoleEnd>>
{
    pub fn send(self, payload: T) -> ReturnType<S1, S2, R> {
        send_mpst_c_to_b(payload, self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<Recv<T, S1>, S2, RoleA<R>, RoleC<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_c_from_a(self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<S1, Recv<T, S2>, RoleB<R>, RoleC<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_c_from_b(self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<Recv<T, S1>, S2, RoleAlltoA<R, R>, RoleC<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_c_all_to_a(self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<S1, Recv<T, S2>, RoleAlltoB<R, R>, RoleC<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_c_all_to_b(self)
    }
}

impl SessionMpst<End, End, RoleEnd, RoleC<RoleEnd>> {
    pub fn close(self) -> Result<(), Box<dyn Error>> {
        crate::functionmpst::close::close_mpst(self)
    }
}
