use binary::Session;
use role::Role;

pub struct SessionMpst<S1: Session, S2: Session, R: Role> {
    pub session1: S1,
    pub session2: S2,
    pub queue: R,
}

impl<S1: Session, S2: Session, R: Role> Session for SessionMpst<S1, S2, R> {
    type Dual = SessionMpst<<S1 as Session>::Dual, <S2 as Session>::Dual, <R as Role>::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = S1::new();
        let (sender_two, receiver_two) = S2::new();

        let (role_one, role_two) = R::new();

        return (
            SessionMpst {
                session1: sender_one,
                session2: sender_two,
                queue: role_one,
            },
            SessionMpst {
                session1: receiver_one,
                session2: receiver_two,
                queue: role_two,
            },
        );
    }
}
