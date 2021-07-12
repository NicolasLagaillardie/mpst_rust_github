use crate::binary::cancel::cancel;
use crate::binary::send::send;
use crate::binary::struct_trait::{End, Recv, Send, Session};
use crate::functionmpst::close::close_mpst;
use crate::functionmpst::OfferMpst;
use crate::meshedchannels::MeshedChannels;
use crate::role::a::RoleA;
use crate::role::all_to_a::RoleAlltoA;
use crate::role::all_to_c::RoleAlltoC;
use crate::role::b::RoleB;
use crate::role::b_to_all::RoleBtoAll;
use crate::role::c::RoleC;
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::{recv_all_aux_simple, recv_aux_simple, send_aux_simple};

use either::Either;
use std::error::Error;
use std::marker;

type ReturnType<S1, S2, R> = MeshedChannels<S1, S2, R, RoleB<RoleEnd>>;
type ResultType<T, S1, S2, R> = Result<(T, ReturnType<S1, S2, R>), Box<dyn Error>>;

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    MeshedChannels<Send<T, S1>, S2, RoleA<R>, RoleB<RoleEnd>>
{
    pub fn send(self, payload: T) -> ReturnType<S1, S2, R> {
        send_aux_simple!(self, payload, RoleA, 1)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    MeshedChannels<S1, Send<T, S2>, RoleC<R>, RoleB<RoleEnd>>
{
    pub fn send(self, payload: T) -> ReturnType<S1, S2, R> {
        send_aux_simple!(self, payload, RoleC, 2)
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    MeshedChannels<Recv<T, S1>, S2, RoleA<R>, RoleB<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_aux_simple!(self, RoleA, 1)()
    }
}

impl<S1: Session, S2: Session, R: Role, T: marker::Send>
    MeshedChannels<S1, Recv<T, S2>, RoleC<R>, RoleB<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, R> {
        recv_aux_simple!(self, RoleC, 2)()
    }
}

impl<S1: Session, S2: Session, T: marker::Send>
    MeshedChannels<Recv<T, S1>, S2, RoleAlltoA<RoleEnd, RoleEnd>, RoleB<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, RoleEnd> {
        recv_all_aux_simple!(self, RoleAlltoA, 1)()
    }
}

impl<S1: Session, S2: Session, T: marker::Send>
    MeshedChannels<S1, Recv<T, S2>, RoleAlltoC<RoleEnd, RoleEnd>, RoleB<RoleEnd>>
{
    pub fn recv(self) -> ResultType<T, S1, S2, RoleEnd> {
        recv_all_aux_simple!(self, RoleAlltoC, 2)()
    }
}

impl<'a, S1: Session, S2: Session, S3: Session, S4: Session, R1: Role, R2: Role>
    MeshedChannels<
        OfferMpst<S1, S2, S3, S4, R1, R2, RoleB<RoleEnd>>,
        End,
        RoleAlltoA<RoleEnd, RoleEnd>,
        RoleB<RoleEnd>,
    >
{
    pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn Error + 'a>>
    where
        F: FnOnce(MeshedChannels<S1, S2, R1, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
        G: FnOnce(MeshedChannels<S3, S4, R2, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    {
        let (e, s) = self.recv()?;
        cancel(s);
        e.either(f, g)
    }
}

impl<'a, S1: Session, S2: Session, S3: Session, S4: Session, R1: Role, R2: Role>
    MeshedChannels<
        End,
        OfferMpst<S1, S2, S3, S4, R1, R2, RoleB<RoleEnd>>,
        RoleAlltoC<RoleEnd, RoleEnd>,
        RoleB<RoleEnd>,
    >
{
    pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn Error + 'a>>
    where
        F: FnOnce(MeshedChannels<S1, S2, R1, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
        G: FnOnce(MeshedChannels<S3, S4, R2, RoleB<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
    {
        let (e, s) = self.recv()?;
        cancel(s);
        e.either(f, g)
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
        $sender: ident,
        $session: expr,
        $pat: path
    ) => {{
        let (session_1_2, session_2_1) = <$session_1 as Session>::new();
        let (session_3_1, session_1_3) = <$session_2 as Session>::new();
        let (session_3_2, session_2_3) = <$session_3 as Session>::new();

        let (stack_1, _) = <$stack_1 as Role>::new();
        let (stack_2, _) = <$stack_2 as Role>::new();
        let (stack_3, _) = <$stack_3 as Role>::new();
        let (name_1, _) = <$receiver_1<RoleEnd> as Role>::new();
        let (name_2, _) = <$receiver_2<RoleEnd> as Role>::new();
        let (name_3, _) = $sender::<RoleEnd>::new();

        let choice_1 = MeshedChannels {
            session1: session_1_2,
            session2: session_1_3,
            stack: stack_1,
            name: name_1,
        };

        let choice_2 = MeshedChannels {
            session1: session_3_1,
            session2: session_3_2,
            stack: stack_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);

        let s = MeshedChannels {
            session1: new_session_1,
            session2: new_session_2,
            stack: $session.stack,
            name: $session.name,
        };

        cancel(s);

        MeshedChannels {
            session1: session_2_1,
            session2: session_2_3,
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
    MeshedChannels<
        Send<
            Either<
                MeshedChannels<S2, <S0 as Session>::Dual, R0, RoleA<RoleEnd>>,
                MeshedChannels<S4, <S1 as Session>::Dual, R1, RoleA<RoleEnd>>,
            >,
            End,
        >,
        Send<
            Either<
                MeshedChannels<S0, S3, R2, RoleC<RoleEnd>>,
                MeshedChannels<S1, S5, R3, RoleC<RoleEnd>>,
            >,
            End,
        >,
        RoleBtoAll<R4, R5>,
        RoleB<RoleEnd>,
    >
{
    pub fn choose_left(
        self,
    ) -> MeshedChannels<<S2 as Session>::Dual, <S3 as Session>::Dual, R4, RoleB<RoleEnd>> {
        choose_aux!(
            S2,
            S0,
            S3,
            R0,
            R2,
            R4,
            RoleA,
            RoleC,
            RoleB,
            self,
            Either::Left
        )
    }

    pub fn choose_right(
        self,
    ) -> MeshedChannels<<S4 as Session>::Dual, <S5 as Session>::Dual, R5, RoleB<RoleEnd>> {
        choose_aux!(
            S4,
            S1,
            S5,
            R1,
            R3,
            R5,
            RoleA,
            RoleC,
            RoleB,
            self,
            Either::Right
        )
    }
}

impl MeshedChannels<End, End, RoleEnd, RoleB<RoleEnd>> {
    pub fn close(self) -> Result<(), Box<dyn Error>> {
        close_mpst(self)
    }
}
