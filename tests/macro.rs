extern crate crossbeam_channel;
extern crate mpstthree;

use crossbeam_channel::{bounded, Sender};
use mpstthree::role::Role;
use mpstthree::{create_broadcast_role, create_normal_role};



use mpstthree::role::end::RoleEnd;

use mpstthree::{create_send_mpst_session_1, create_send_mpst_session_2};

use std::marker;

use mpstthree::binary::{send, Send, Session};

use mpstthree::sessionmpst::SessionMpst;

#[test]
fn main() {
    create_normal_role!(RoleAtoD, next_a_to_d, RoleDtoA, next_d_to_a);

    type TestAAndD = RoleAtoD<RoleDtoA<RoleEnd>>;

    create_broadcast_role!(RoleDtoAll, next_d_to_all, RoleAlltoD, next_all_to_d);

    type TestDbroadcast = RoleDtoAll<RoleDtoA<RoleEnd>, RoleDtoA<RoleEnd>>;
    type TestbroadcastD = RoleAlltoD<RoleDtoA<RoleEnd>, RoleDtoA<RoleEnd>>;

    create_send_mpst_session_1!(send_mpst_d_to_a, RoleDtoA, next_d_to_a);

    create_send_mpst_session_2!(send_mpst_a_to_d, RoleAtoD, next_a_to_d);
}
