use crate::binary::cancel::cancel;
use crate::binary::send::send;
use crate::binary::struct_trait::{End, Recv, Send, Session};
use crate::functionmpst::offer::{offer_mpst_session_to_a_from_b, offer_mpst_session_to_a_from_c};
use crate::functionmpst::recv::*;
use crate::functionmpst::send::*;
use crate::functionmpst::ChooseMpst;
use crate::functionmpst::OfferMpst;
use crate::role::a::RoleA;
use crate::role::a_to_all::{next_a_to_all, RoleAtoAll};
use crate::role::all_to_b::RoleAlltoB;
use crate::role::all_to_c::RoleAlltoC;
use crate::role::b::RoleB;
use crate::role::b_dual::RoleBDual;
use crate::role::c::RoleC;
use crate::role::c_dual::RoleCDual;
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use either::Either;
use std::error::Error;
use std::marker;

type ReturnType<S1, S2, R> = SessionMpst<S1, S2, R, RoleA<RoleEnd>>;
type ResultType<T, S1, S2, R> = Result<(T, ReturnType<S1, S2, R>), Box<dyn Error>>;

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<Send<T, S1>, S2, RoleB<R>, RoleA<RoleEnd>>
{
    pub fn send(self, payload: T) -> ReturnType<S1, S2, R> {
        send_mpst_a_to_b(payload, self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<S1, Send<T, S2>, RoleC<R>, RoleA<RoleEnd>>
{
    pub fn send(self, payload: T) -> ReturnType<S1, S2, R> {
        send_mpst_a_to_c(payload, self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<Recv<T, S1>, S2, RoleB<R>, RoleA<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_a_from_b(self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<S1, Recv<T, S2>, RoleC<R>, RoleA<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_a_from_c(self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<Recv<T, S1>, S2, RoleAlltoB<R, R>, RoleA<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_a_all_to_b(self)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    SessionMpst<S1, Recv<T, S2>, RoleAlltoC<R, R>, RoleA<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_mpst_a_all_to_c(self)
    }
}

impl<'a, S1: Session, S2: Session, S3: Session, S4: Session, R1: Role, R2: Role>
    SessionMpst<
        OfferMpst<S1, S2, S3, S4, R1, R2, RoleA<RoleEnd>>,
        End,
        RoleAlltoB<RoleEnd, RoleEnd>,
        RoleA<RoleEnd>,
    >
{
    pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn Error + 'a>>
    where
        F: FnOnce(SessionMpst<S1, S2, R1, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
        G: FnOnce(SessionMpst<S3, S4, R2, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    {
        offer_mpst_session_to_a_from_b(self, f, g)
    }
}

impl<'a, S1: Session, S2: Session, S3: Session, S4: Session, R1: Role, R2: Role>
    SessionMpst<
        End,
        OfferMpst<S1, S2, S3, S4, R1, R2, RoleA<RoleEnd>>,
        RoleAlltoC<RoleEnd, RoleEnd>,
        RoleA<RoleEnd>,
    >
{
    pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn Error + 'a>>
    where
        F: FnOnce(SessionMpst<S1, S2, R1, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
        G: FnOnce(SessionMpst<S3, S4, R2, RoleA<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    {
        offer_mpst_session_to_a_from_c(self, f, g)
    }
}

// impl<
//         S0: Session,
//         S1: Session,
//         S2: Session,
//         S3: Session,
//         S4: Session,
//         S5: Session,
//         R0: Role,
//         R1: Role,
//         R2: Role,
//         R3: Role,
//         R4: Role,
//         R5: Role,
//     >
//     SessionMpst<
//         ChooseMpst<S2, S0, S4, S1, R0, R1, RoleBDual<RoleEnd>>,
//         ChooseMpst<
//             S3,
//             <S0 as Session>::Dual,
//             S5,
//             <S1 as Session>::Dual,
//             R2,
//             R3,
//             RoleCDual<RoleEnd>,
//         >,
//         RoleAtoAll<R4, R5>,
//         RoleA<RoleEnd>,
//     >
// {
//     pub fn choose_left(self) -> SessionMpst<S2, S3, R4, RoleA<RoleEnd>> {
//         let (session_1_2, session_2_1) = <S2 as Session>::new();
//         let (session_1_3, session_3_1) = <S3 as Session>::new();
//         let (session_3_2, session_2_3) = <S0 as Session>::new();

//         let (_, stack_1) = <R0 as Role>::new();
//         let (_, stack_2) = <R2 as Role>::new();
//         let (stack_3, _) = <R4 as Role>::new();
//         let (name_1, _) = <RoleBDual<RoleEnd> as Role>::Dual::new();
//         let (name_2, _) = <RoleCDual<RoleEnd> as Role>::Dual::new();
//         let (name_3, _) = RoleA::<RoleEnd>::new();

//         let choice_1 = SessionMpst {
//             session1: session_2_1,
//             session2: session_2_3,
//             stack: stack_1,
//             name: name_1,
//         };

//         let choice_2 = SessionMpst {
//             session1: session_3_1,
//             session2: session_3_2,
//             stack: stack_2,
//             name: name_2,
//         };

//         let new_session_1 = send(Either::Left(choice_1), self.session1);
//         let new_session_2 = send(Either::Left(choice_2), self.session2);
//         let (_, new_queue) = next_a_to_all(self.stack);

//         let s = SessionMpst {
//             session1: new_session_1,
//             session2: new_session_2,
//             stack: new_queue,
//             name: self.name,
//         };

//         cancel(s);

//         SessionMpst {
//             session1: session_1_2,
//             session2: session_1_3,
//             stack: stack_3,
//             name: name_3,
//         }
//     }

//     pub fn choose_right(self) -> SessionMpst<S4, S5, R5, RoleA<RoleEnd>> {
//         let (session_1_2, session_2_1) = <S4 as Session>::new();
//         let (session_1_3, session_3_1) = <S5 as Session>::new();
//         let (session_3_2, session_2_3) = <S1 as Session>::new();

//         let (_, stack_1) = <R1 as Role>::new();
//         let (_, stack_2) = <R3 as Role>::new();
//         let (stack_3, _) = <R5 as Role>::new();
//         let (name_1, _) = <RoleBDual<RoleEnd> as Role>::Dual::new();
//         let (name_2, _) = <RoleCDual<RoleEnd> as Role>::Dual::new();
//         let (name_3, _) = RoleA::<RoleEnd>::new();

//         let choice_1 = SessionMpst {
//             session1: session_2_1,
//             session2: session_2_3,
//             stack: stack_1,
//             name: name_1,
//         };

//         let choice_2 = SessionMpst {
//             session1: session_3_1,
//             session2: session_3_2,
//             stack: stack_2,
//             name: name_2,
//         };

//         let new_session_1 = send(Either::Right(choice_1), self.session1);
//         let new_session_2 = send(Either::Right(choice_2), self.session2);
//         let (_, new_queue) = next_a_to_all(self.stack);

//         let s = SessionMpst {
//             session1: new_session_1,
//             session2: new_session_2,
//             stack: new_queue,
//             name: self.name,
//         };

//         cancel(s);

//         SessionMpst {
//             session1: session_1_2,
//             session2: session_1_3,
//             stack: stack_3,
//             name: name_3,
//         }
//     }
// }

impl SessionMpst<End, End, RoleEnd, RoleA<RoleEnd>> {
    pub fn close(self) -> Result<(), Box<dyn Error>> {
        crate::functionmpst::close::close_mpst(self)
    }
}
