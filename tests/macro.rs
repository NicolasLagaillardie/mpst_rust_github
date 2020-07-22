extern crate crossbeam_channel;
extern crate mpstthree;

use crossbeam_channel::{bounded, Sender};
use mpstthree::create_role;
use mpstthree::role::Role;

use mpstthree::role::end::RoleEnd;

#[test]
fn main() {
    create_role!(RoleAtoD, next_a_to_d, RoleDtoA, next_d_to_a);

    type Test = RoleAtoD<RoleDtoA<RoleEnd>>;
}
