use binary::Session;
use role::Role;

/// A `struct` which encapsulates two binary session types and a queue.
///
/// This `struct` is the main one used in this library.
/// Each process is linked to the others with one `Session`,
/// and the order of the operations is given by the queue composed of `Role`.
#[must_use]
#[derive(Debug)]
pub struct SessionMpst<S1: Session, S2: Session, R: Role> {
    pub session1: S1,
    pub session2: S2,
    pub stack: R,
}

#[doc(hidden)]
impl<S1: Session, S2: Session, R: Role> Session for SessionMpst<S1, S2, R> {
    type Dual = SessionMpst<<S1 as Session>::Dual, <S2 as Session>::Dual, <R as Role>::Dual>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = S1::new();
        let (sender_two, receiver_two) = S2::new();

        let (role_one, role_two) = R::new();

        (
            SessionMpst {
                session1: sender_one,
                session2: sender_two,
                stack: role_one,
            },
            SessionMpst {
                session1: receiver_one,
                session2: receiver_two,
                stack: role_two,
            },
        )
    }

    #[doc(hidden)]
    fn head() -> String {
        format!("{} + {}", S1::head(), S2::head())
    }
}
