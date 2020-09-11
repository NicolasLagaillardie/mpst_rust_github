// Test for parametrisation on the number of roles
extern crate crossbeam_channel;
extern crate either;
extern crate mpstthree;
use mpstthree::binary::{End, Session};
use mpstthree::create_sessionmpst;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use std::error::Error;

// Create new SessionMpst for three participants
create_sessionmpst!(SessionMpstThree, session1, S1, session2, S2,);

// Create new SessionMpst for four participants
create_sessionmpst!(SessionMpstFour, session1, S1, session2, S2, session3, S3,);

#[test]
fn basic_macros() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (sender1, _) = End::new();
            let (sender2, _) = End::new();
            let (role_one, _) = RoleEnd::new();

            let _test = SessionMpstThree {
                session1: sender1,
                session2: sender2,
                stack: role_one,
            };

            let (_test2, _) = SessionMpstThree::<End, End, RoleEnd>::new();
        }
        Ok(())
    }()
    .is_ok());

    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (sender1, _) = End::new();
            let (sender2, _) = End::new();
            let (sender3, _) = End::new();
            let (role_one, _) = RoleEnd::new();

            let _test = SessionMpstFour {
                session1: sender1,
                session2: sender2,
                session3: sender3,
                stack: role_one,
            };

            let (_test2, _) = SessionMpstFour::<End, End, End, RoleEnd>::new();
        }
        Ok(())
    }()
    .is_ok());
}
