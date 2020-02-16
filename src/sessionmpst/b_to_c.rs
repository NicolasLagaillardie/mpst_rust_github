use binary::{End, Recv, Send, Session};
use crossbeam_channel::bounded;
use role::b_to_c::RoleBtoC;
use role::c_to_b::RoleCtoB;
use role::Role;
use sessionmpst::SessionMpst;
use std::marker;

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Recv<T2, S2>, RoleBtoC<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Send<T2, S2::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<R::Dual>(1);
        let (sender_role_two, receiver_role_two) = bounded::<R>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Send<T2, S2>, RoleBtoC<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<R::Dual>(1);
        let (sender_role_two, receiver_role_two) = bounded::<R>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Send<T1, S1>, Recv<T2, S2>, RoleBtoC<R>>
{
    type Dual = SessionMpst<Recv<T1, S1::Dual>, Send<T2, S2::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1::Dual)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<R::Dual>(1);
        let (sender_role_two, receiver_role_two) = bounded::<R>(1);

        return (
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T1: marker::Send, T2: marker::Send, S1: Session, S2: Session, R: Role> Session
    for SessionMpst<Recv<T1, S1>, Send<T2, S2>, RoleBtoC<R>>
{
    type Dual = SessionMpst<Send<T1, S1::Dual>, Recv<T2, S2::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = bounded::<(T1, S1)>(1);
        let (sender_two, receiver_two) = bounded::<(T2, S2::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<R::Dual>(1);
        let (sender_role_two, receiver_role_two) = bounded::<R>(1);

        return (
            SessionMpst {
                session1: Recv {
                    channel: receiver_one,
                },
                session2: Send {
                    channel: sender_two,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send {
                    channel: sender_one,
                },
                session2: Recv {
                    channel: receiver_two,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Recv<T, S>, End, RoleBtoC<R>> {
    type Dual = SessionMpst<Send<T, S::Dual>, End, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<R::Dual>(1);
        let (sender_role_two, receiver_role_two) = bounded::<R>(1);

        return (
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Recv<T, S>, RoleBtoC<R>> {
    type Dual = SessionMpst<End, Send<T, S::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<R::Dual>(1);
        let (sender_role_two, receiver_role_two) = bounded::<R>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Recv { channel: receiver },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Send { channel: sender },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<End, Send<T, S>, RoleBtoC<R>> {
    type Dual = SessionMpst<End, Recv<T, S::Dual>, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<R::Dual>(1);
        let (sender_role_two, receiver_role_two) = bounded::<R>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                session2: Send { channel: sender },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                session2: Recv { channel: receiver },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}

impl<T: marker::Send, S: Session, R: Role> Session for SessionMpst<Send<T, S>, End, RoleBtoC<R>> {
    type Dual = SessionMpst<Recv<T, S::Dual>, End, RoleCtoB<R::Dual>>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1, receiver1) = bounded::<()>(1);
        let (sender2, receiver2) = bounded::<()>(1);

        let (sender, receiver) = bounded::<(T, S::Dual)>(1);

        let (sender_role_one, receiver_role_one) = bounded::<R::Dual>(1);
        let (sender_role_two, receiver_role_two) = bounded::<R>(1);

        return (
            SessionMpst {
                session1: Send { channel: sender },
                session2: End {
                    sender: sender1,
                    receiver: receiver2,
                },
                queue: RoleBtoC {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: Recv { channel: receiver },
                session2: End {
                    sender: sender2,
                    receiver: receiver1,
                },
                queue: RoleCtoB {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}
