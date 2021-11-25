use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{create_multiple_broadcast_role, create_multiple_normal_role};

// Create new normal roles
create_multiple_normal_role!(
    RoleD, RoleDDual |
    RoleE, RoleEDual |
    RoleF, RoleFDual |
);

// Create new broadcasting roles
create_multiple_broadcast_role!(
    RoleDtoAll, RoleAlltoD |
    RoleEtoAll, RoleAlltoE |
    RoleFtoAll, RoleAlltoF |
);

pub fn role_a_fields() {
    let (role_sender, role_receiver) = RoleD::<RoleEnd>::new();

    // role_sender
    let (here, there) = RoleEnd::new();
    role_sender.sender.send(there).unwrap_or(());

    assert!(here.sender.send(()).is_err());
    assert_eq!(role_sender.self_head_str(), "RoleD".to_string());
    assert_eq!(role_sender.self_tail_str(), "RoleEnd<>".to_string());

    // role_sender
    let (here, there) = RoleEnd::new();
    role_receiver.sender.send(there).unwrap_or(());

    assert!(here.sender.send(()).is_err());
    assert_eq!(role_receiver.self_head_str(), "RoleDDual".to_string());
    assert_eq!(role_receiver.self_tail_str(), "RoleEnd<>".to_string());
}

pub fn role_b_fields() {
    let (role_sender, role_receiver) = RoleE::<RoleEnd>::new();

    // role_sender
    let (here, there) = RoleEnd::new();
    role_sender.sender.send(there).unwrap_or(());

    assert!(here.sender.send(()).is_err());
    assert_eq!(role_sender.self_head_str(), "RoleE".to_string());
    assert_eq!(role_sender.self_tail_str(), "RoleEnd<>".to_string());

    // role_sender
    let (here, there) = RoleEnd::new();
    role_receiver.sender.send(there).unwrap_or(());

    assert!(here.sender.send(()).is_err());
    assert_eq!(role_receiver.self_head_str(), "RoleEDual".to_string());
    assert_eq!(role_receiver.self_tail_str(), "RoleEnd<>".to_string());
}

pub fn role_c_fields() {
    let (role_sender, role_receiver) = RoleF::<RoleEnd>::new();

    // role_sender
    let (here, there) = RoleEnd::new();
    role_sender.sender.send(there).unwrap_or(());

    assert!(here.sender.send(()).is_err());
    assert_eq!(role_sender.self_head_str(), "RoleF".to_string());
    assert_eq!(role_sender.self_tail_str(), "RoleEnd<>".to_string());

    // role_sender
    let (here, there) = RoleEnd::new();
    role_receiver.sender.send(there).unwrap_or(());

    assert!(here.sender.send(()).is_err());
    assert_eq!(role_receiver.self_head_str(), "RoleFDual".to_string());
    assert_eq!(role_receiver.self_tail_str(), "RoleEnd<>".to_string());
}

pub fn role_a_to_all_fields() {
    let (role_sender_1, role_sender_2) = RoleDtoAll::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());
}

pub fn role_all_to_a_fields() {
    let (role_sender_1, role_sender_2) = RoleAlltoD::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());
}

pub fn role_b_to_all_fields() {
    let (role_sender_1, role_sender_2) = RoleEtoAll::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());
}

pub fn role_all_to_b_fields() {
    let (role_sender_1, role_sender_2) = RoleAlltoE::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());
}

pub fn role_c_to_all_fields() {
    let (role_sender_1, role_sender_2) = RoleFtoAll::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());
}

pub fn role_all_to_c_fields() {
    let (role_sender_1, role_sender_2) = RoleAlltoF::<RoleEnd, RoleEnd>::new();

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert!(here1.sender.send(()).is_err());
    assert!(here2.sender.send(()).is_err());
}

pub fn role_head_str() {
    assert_eq!(RoleEnd::head_str(), "RoleEnd".to_string());
    assert_eq!(RoleD::<RoleEnd>::head_str(), "RoleD".to_string());
    assert_eq!(RoleE::<RoleEnd>::head_str(), "RoleE".to_string());
    assert_eq!(RoleF::<RoleEnd>::head_str(), "RoleF".to_string());
    assert_eq!(RoleDDual::<RoleEnd>::head_str(), "RoleDDual".to_string());
    assert_eq!(RoleEDual::<RoleEnd>::head_str(), "RoleEDual".to_string());
    assert_eq!(RoleFDual::<RoleEnd>::head_str(), "RoleFDual".to_string());
    assert_eq!(
        RoleDtoAll::<RoleEnd, RoleEnd>::head_str(),
        "RoleDtoAll".to_string()
    );
    assert_eq!(
        RoleEtoAll::<RoleEnd, RoleEnd>::head_str(),
        "RoleEtoAll".to_string()
    );
    assert_eq!(
        RoleFtoAll::<RoleEnd, RoleEnd>::head_str(),
        "RoleFtoAll".to_string()
    );
    assert_eq!(
        RoleAlltoD::<RoleEnd, RoleEnd>::head_str(),
        "RoleAlltoD".to_string()
    );
    assert_eq!(
        RoleAlltoE::<RoleEnd, RoleEnd>::head_str(),
        "RoleAlltoE".to_string()
    );
    assert_eq!(
        RoleAlltoF::<RoleEnd, RoleEnd>::head_str(),
        "RoleAlltoF".to_string()
    );
}

pub fn role_tail_str() {
    assert!(RoleEnd::tail_str().is_empty());
    assert_eq!(RoleD::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(RoleE::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(RoleF::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(RoleDDual::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(RoleEDual::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(RoleFDual::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(
        RoleDtoAll::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
    assert_eq!(
        RoleEtoAll::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
    assert_eq!(
        RoleFtoAll::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
    assert_eq!(
        RoleAlltoD::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
    assert_eq!(
        RoleAlltoE::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
    assert_eq!(
        RoleAlltoF::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
}
