use binary::End;
use binary::Session;
use crossbeam_channel::bounded;
use role::end::RoleEnd;
use sessionmpst::SessionMpst;

impl Session for SessionMpst<End, End, RoleEnd> {
    type Dual = SessionMpst<End, End, RoleEnd>;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender1_one, receiver1_one) = bounded::<()>(1);
        let (sender2_one, receiver2_one) = bounded::<()>(1);
        let (sender1_two, receiver1_two) = bounded::<()>(1);
        let (sender2_two, receiver2_two) = bounded::<()>(1);

        let (sender_role_one, receiver_role_one) = bounded::<()>(1);
        let (sender_role_two, receiver_role_two) = bounded::<()>(1);

        return (
            SessionMpst {
                session1: End {
                    sender: sender1_one,
                    receiver: receiver2_one,
                },
                session2: End {
                    sender: sender1_two,
                    receiver: receiver2_two,
                },
                queue: RoleEnd {
                    sender: sender_role_one,
                    receiver: receiver_role_two,
                },
            },
            SessionMpst {
                session1: End {
                    sender: sender2_one,
                    receiver: receiver1_one,
                },
                session2: End {
                    sender: sender2_two,
                    receiver: receiver1_two,
                },
                queue: RoleEnd {
                    sender: sender_role_two,
                    receiver: receiver_role_one,
                },
            },
        );
    }
}
