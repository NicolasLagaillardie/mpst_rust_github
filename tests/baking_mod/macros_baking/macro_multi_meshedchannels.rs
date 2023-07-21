// Test for parametrisation on the number of roles
use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::name::Name;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{create_meshedchannels, create_normal_name, create_normal_role};
use std::error::Error;

// Create new MeshedChannels for three participants
create_meshedchannels!(MeshedChannels, 3);

// Create new MeshedChannels for four participants
create_meshedchannels!(MeshedChannels, 4);

// Create an A dummy
create_normal_role!(RoleA, RoleADual);

// Create an A dummy
create_normal_name!(NameA);

/////////////////////////////////////////

pub fn basic_macros() {
    assert!({
        {
            let (sender1, _) = End::new();
            let (sender2, _) = End::new();
            let (role_one, _) = RoleEnd::new();
            let (name_one, _) = NameA::new();

            let _test = MeshedChannels {
                session1: sender1,
                session2: sender2,
                stack: role_one,
                name: name_one,
            };

            let (_test2, _) = MeshedChannels::<End, End, RoleEnd, NameA>::new();
        }
        Ok::<(), Box<dyn Error>>(())
    }
    .is_ok());

    assert!({
        {
            let (sender1, _) = End::new();
            let (sender2, _) = End::new();
            let (sender3, _) = End::new();
            let (role_one, _) = RoleEnd::new();
            let (name_one, _) = NameA::new();

            let _test = MeshedChannels {
                session1: sender1,
                session2: sender2,
                session3: sender3,
                stack: role_one,
                name: name_one,
            };

            let (_test2, _) = MeshedChannels::<End, End, End, RoleEnd, NameA>::new();
        }
        Ok::<(), Box<dyn Error>>(())
    }
    .is_ok());
}
