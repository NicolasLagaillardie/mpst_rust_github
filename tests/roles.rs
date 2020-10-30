extern crate mpstthree;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::a_to_all::RoleAtoAll;
use mpstthree::role::all_to_a::RoleAlltoA;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::all_to_c::RoleAlltoC;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_dual::RoleBDual;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::c_to_all::RoleCtoAll;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;

#[test]
fn role_fields() {
    let (role_end_1, role_end_2) = RoleEnd::new();

    assert_eq!(role_end_1.sender.send(()), Ok(()));
    assert_eq!(role_end_2.sender.send(()), Ok(()));
    assert_eq!(role_end_1.receiver.recv(), Ok(()));
    assert_eq!(role_end_2.receiver.recv(), Ok(()));
}

#[test]
fn role_head_str() {
    assert_eq!(RoleEnd::head_str(), String::from("RoleEnd"));
    assert_eq!(RoleA::<RoleEnd>::head_str(), String::from("RoleA"));
    assert_eq!(RoleB::<RoleEnd>::head_str(), String::from("RoleB"));
    assert_eq!(RoleC::<RoleEnd>::head_str(), String::from("RoleC"));
    assert_eq!(RoleADual::<RoleEnd>::head_str(), String::from("RoleADual"));
    assert_eq!(RoleBDual::<RoleEnd>::head_str(), String::from("RoleBDual"));
    assert_eq!(RoleCDual::<RoleEnd>::head_str(), String::from("RoleCDual"));
    assert_eq!(
        RoleAtoAll::<RoleEnd, RoleEnd>::head_str(),
        String::from("RoleAtoAll")
    );
    assert_eq!(
        RoleBtoAll::<RoleEnd, RoleEnd>::head_str(),
        String::from("RoleBtoAll")
    );
    assert_eq!(
        RoleCtoAll::<RoleEnd, RoleEnd>::head_str(),
        String::from("RoleCtoAll")
    );
    assert_eq!(
        RoleAlltoA::<RoleEnd, RoleEnd>::head_str(),
        String::from("RoleAlltoA")
    );
    assert_eq!(
        RoleAlltoB::<RoleEnd, RoleEnd>::head_str(),
        String::from("RoleAlltoB")
    );
    assert_eq!(
        RoleAlltoC::<RoleEnd, RoleEnd>::head_str(),
        String::from("RoleAlltoC")
    );
}

#[test]
fn role_tail_str() {
    assert_eq!(RoleEnd::tail_str(), String::from(""));
    assert_eq!(RoleA::<RoleEnd>::tail_str(), String::from("RoleEnd<>"));
    assert_eq!(RoleB::<RoleEnd>::tail_str(), String::from("RoleEnd<>"));
    assert_eq!(RoleC::<RoleEnd>::tail_str(), String::from("RoleEnd<>"));
    assert_eq!(RoleADual::<RoleEnd>::tail_str(), String::from("RoleEnd<>"));
    assert_eq!(RoleBDual::<RoleEnd>::tail_str(), String::from("RoleEnd<>"));
    assert_eq!(RoleCDual::<RoleEnd>::tail_str(), String::from("RoleEnd<>"));
    assert_eq!(
        RoleAtoAll::<RoleEnd, RoleEnd>::tail_str(),
        String::from("RoleEnd<> + RoleEnd<>")
    );
    assert_eq!(
        RoleBtoAll::<RoleEnd, RoleEnd>::tail_str(),
        String::from("RoleEnd<> + RoleEnd<>")
    );
    assert_eq!(
        RoleCtoAll::<RoleEnd, RoleEnd>::tail_str(),
        String::from("RoleEnd<> + RoleEnd<>")
    );
    assert_eq!(
        RoleAlltoA::<RoleEnd, RoleEnd>::tail_str(),
        String::from("RoleEnd<> + RoleEnd<>")
    );
    assert_eq!(
        RoleAlltoB::<RoleEnd, RoleEnd>::tail_str(),
        String::from("RoleEnd<> + RoleEnd<>")
    );
    assert_eq!(
        RoleAlltoC::<RoleEnd, RoleEnd>::tail_str(),
        String::from("RoleEnd<> + RoleEnd<>")
    );
}
