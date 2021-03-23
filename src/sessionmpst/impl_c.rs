use crate::binary::cancel::cancel;
use crate::binary::send::send;
use crate::binary::struct_trait::{End, Recv, Send, Session};
use crate::functionmpst::offer::{offer_mpst_session_to_c_from_a, offer_mpst_session_to_c_from_b};
use crate::functionmpst::recv::*;
use crate::functionmpst::send::*;
use crate::functionmpst::OfferMpst;
use crate::role::a::RoleA;
use crate::role::all_to_a::RoleAlltoA;
use crate::role::all_to_b::RoleAlltoB;
use crate::role::b::RoleB;
use crate::role::c::RoleC;
use crate::role::c_to_all::{next_c_to_all, RoleCtoAll};
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use either::Either;
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

impl<'a, S1: Session, S2: Session, S3: Session, S4: Session, R1: Role, R2: Role>
    SessionMpst<
        OfferMpst<S1, S2, S3, S4, R1, R2, RoleC<RoleEnd>>,
        End,
        RoleAlltoA<RoleEnd, RoleEnd>,
        RoleC<RoleEnd>,
    >
{
    pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn Error + 'a>>
    where
        F: FnOnce(SessionMpst<S1, S2, R1, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
        G: FnOnce(SessionMpst<S3, S4, R2, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    {
        offer_mpst_session_to_c_from_a(self, f, g)
    }
}

impl<'a, S1: Session, S2: Session, S3: Session, S4: Session, R1: Role, R2: Role>
    SessionMpst<
        End,
        OfferMpst<S1, S2, S3, S4, R1, R2, RoleC<RoleEnd>>,
        RoleAlltoB<RoleEnd, RoleEnd>,
        RoleC<RoleEnd>,
    >
{
    pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn Error + 'a>>
    where
        F: FnOnce(SessionMpst<S1, S2, R1, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
        G: FnOnce(SessionMpst<S3, S4, R2, RoleC<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    {
        offer_mpst_session_to_c_from_b(self, f, g)
    }
}

#[doc(hidden)]
macro_rules! choose_aux {
    (
        $session_1:ty,
        $session_2:ty,
        $session_3:ty,
        $stack_1:ty,
        $stack_2:ty,
        $stack_3:ty,
        $receiver_1:ident,
        $receiver_2:ident,
        $sender:ident,
        $session:expr,
        $pat:path,
        $next:ident
    ) => {{
        let (session_2_1, session_1_2) = <$session_1 as Session>::new();
        let (session_1_3, session_3_1) = <$session_2 as Session>::new();
        let (session_2_3, session_3_2) = <$session_3 as Session>::new();

        let (stack_1, _) = <$stack_1 as Role>::new();
        let (stack_2, _) = <$stack_2 as Role>::new();
        let (stack_3, _) = <$stack_3 as Role>::new();
        let (name_1, _) = <$receiver_1<RoleEnd> as Role>::new();
        let (name_2, _) = <$receiver_2<RoleEnd> as Role>::new();
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

impl<
        S0: Session,
        S1: Session,
        S2: Session,
        S3: Session,
        S4: Session,
        S5: Session,
        R0: Role,
        R1: Role,
        R2: Role,
        R3: Role,
        R4: Role,
        R5: Role,
    >
    SessionMpst<
        Send<
            Either<
                SessionMpst<<S0 as Session>::Dual, S2, R0, RoleA<RoleEnd>>,
                SessionMpst<<S1 as Session>::Dual, S4, R1, RoleA<RoleEnd>>,
            >,
            End,
        >,
        Send<
            Either<
                SessionMpst<S0, S3, R2, RoleB<RoleEnd>>,
                SessionMpst<S1, S5, R3, RoleB<RoleEnd>>,
            >,
            End,
        >,
        RoleCtoAll<R4, R5>,
        RoleC<RoleEnd>,
    >
{
    pub fn choose_left(
        self,
    ) -> SessionMpst<<S2 as Session>::Dual, <S3 as Session>::Dual, R4, RoleC<RoleEnd>> {
        choose_aux!(
            S0,
            S2,
            S3,
            R0,
            R2,
            R4,
            RoleA,
            RoleB,
            RoleC,
            self,
            Either::Left,
            next_c_to_all
        )
    }

    pub fn choose_right(
        self,
    ) -> SessionMpst<<S4 as Session>::Dual, <S5 as Session>::Dual, R5, RoleC<RoleEnd>> {
        choose_aux!(
            S1,
            S4,
            S5,
            R1,
            R3,
            R5,
            RoleA,
            RoleB,
            RoleC,
            self,
            Either::Right,
            next_c_to_all
        )
    }
}

impl SessionMpst<End, End, RoleEnd, RoleC<RoleEnd>> {
    pub fn close(self) -> Result<(), Box<dyn Error>> {
        crate::functionmpst::close::close_mpst(self)
    }
}
