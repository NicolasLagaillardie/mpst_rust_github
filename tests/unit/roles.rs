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
pub fn role_end_fields_1() {
    let (role_end_1, role_end_2) = RoleEnd::new();

    assert_eq!(role_end_1.sender.send(()), Ok(()));
    assert_eq!(role_end_2.sender.send(()), Ok(()));
    assert_eq!(role_end_1.receiver.recv(), Ok(()));
    assert_eq!(role_end_2.receiver.recv(), Ok(()));
}

#[test]
pub fn role_end_fields_2() {
    let (role_end_1, role_end_2) = RoleEnd::new();

    assert_eq!(role_end_2.sender.send(()), Ok(()));
    assert_eq!(role_end_1.sender.send(()), Ok(()));
    assert_eq!(role_end_2.receiver.recv(), Ok(()));
    assert_eq!(role_end_1.receiver.recv(), Ok(()));
}

#[test]
pub fn role_a_to_all_fields() {
    let (role_sender_1, role_sender_2) = RoleAtoAll::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
}

#[test]
pub fn role_all_to_a_fields() {
    let (role_sender_1, role_sender_2) = RoleAlltoA::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
}

#[test]
pub fn role_b_to_all_fields() {
    let (role_sender_1, role_sender_2) = RoleBtoAll::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
}

#[test]
pub fn role_all_to_b_fields() {
    let (role_sender_1, role_sender_2) = RoleAlltoB::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
}

#[test]
pub fn role_c_to_all_fields() {
    let (role_sender_1, role_sender_2) = RoleCtoAll::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
}

#[test]
pub fn role_all_to_c_fields() {
    let (role_sender_1, role_sender_2) = RoleAlltoC::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
}

#[test]
pub fn role_head_str() {
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
pub fn role_tail_str() {
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
