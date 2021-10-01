use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::a_to_all::RoleAtoAll;
use mpstthree::role::all_to_a::RoleAlltoA;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::all_to_c::RoleAlltoC;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_dual::RoleBDual;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::c_to_all::RoleCtoAll;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;

pub fn role_end_fields_1() {
    let (role_end_1, role_end_2) = RoleEnd::new();

    assert_eq!(role_end_1.sender.send(()), Ok(()));
    assert_eq!(role_end_2.sender.send(()), Ok(()));
    assert_eq!(role_end_1.receiver.recv(), Ok(()));
    assert_eq!(role_end_2.receiver.recv(), Ok(()));

    assert_eq!(role_end_1.self_head_str(), "RoleEnd".to_string());
    assert_eq!(role_end_1.self_tail_str(), "".to_string());
    assert_eq!(role_end_2.self_head_str(), "RoleEnd".to_string());
    assert_eq!(role_end_2.self_tail_str(), "".to_string());
}

pub fn role_end_fields_2() {
    let (role_end_1, role_end_2) = RoleEnd::new();

    assert_eq!(role_end_2.sender.send(()), Ok(()));
    assert_eq!(role_end_1.sender.send(()), Ok(()));
    assert_eq!(role_end_2.receiver.recv(), Ok(()));
    assert_eq!(role_end_1.receiver.recv(), Ok(()));

    assert_eq!(role_end_1.self_head_str(), "RoleEnd".to_string());
    assert_eq!(role_end_1.self_tail_str(), "".to_string());
    assert_eq!(role_end_2.self_head_str(), "RoleEnd".to_string());
    assert_eq!(role_end_2.self_tail_str(), "".to_string());
}

pub fn role_a_fields() {
    let (role_sender, role_receiver) = RoleA::<RoleEnd>::new();

    assert_eq!(
        role_sender.continuation().self_head_str(),
        "RoleEnd".to_string()
    );

    // role_sender
    let (here, there) = RoleEnd::new();
    role_sender.sender.send(there).unwrap_or(());

    assert_eq!(here.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender.self_head_str(), "RoleA".to_string());
    assert_eq!(role_sender.self_tail_str(), "RoleEnd<>".to_string());

    // role_sender
    let (here, there) = RoleEnd::new();
    role_receiver.sender.send(there).unwrap_or(());

    assert_eq!(here.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_receiver.self_head_str(), "RoleADual".to_string());
    assert_eq!(role_receiver.self_tail_str(), "RoleEnd<>".to_string());
}

pub fn role_b_fields() {
    let (role_sender, role_receiver) = RoleB::<RoleEnd>::new();

    assert_eq!(
        role_sender.continuation().self_head_str(),
        "RoleEnd".to_string()
    );

    // role_sender
    let (here, there) = RoleEnd::new();
    role_sender.sender.send(there).unwrap_or(());

    assert_eq!(here.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender.self_head_str(), "RoleB".to_string());
    assert_eq!(role_sender.self_tail_str(), "RoleEnd<>".to_string());

    // role_sender
    let (here, there) = RoleEnd::new();
    role_receiver.sender.send(there).unwrap_or(());

    assert_eq!(here.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_receiver.self_head_str(), "RoleBDual".to_string());
    assert_eq!(role_receiver.self_tail_str(), "RoleEnd<>".to_string());
}

pub fn role_c_fields() {
    let (role_sender, role_receiver) = RoleC::<RoleEnd>::new();

    assert_eq!(
        role_sender.continuation().self_head_str(),
        "RoleEnd".to_string()
    );

    // role_sender
    let (here, there) = RoleEnd::new();
    role_sender.sender.send(there).unwrap_or(());

    assert_eq!(here.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender.self_head_str(), "RoleC".to_string());
    assert_eq!(role_sender.self_tail_str(), "RoleEnd<>".to_string());

    // role_sender
    let (here, there) = RoleEnd::new();
    role_receiver.sender.send(there).unwrap_or(());

    assert_eq!(here.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_receiver.self_head_str(), "RoleCDual".to_string());
    assert_eq!(role_receiver.self_tail_str(), "RoleEnd<>".to_string());
}

pub fn role_a_to_all_fields() {
    let (role_sender_1, role_sender_2) = RoleAtoAll::<RoleEnd, RoleEnd>::new();

    assert_eq!(
        role_sender_1.continuation_left().self_head_str(),
        "RoleEnd".to_string()
    );

    assert_eq!(
        role_sender_1.continuation_right().self_head_str(),
        "RoleEnd".to_string()
    );

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_1.self_head_str(), "RoleAtoAll".to_string());
    assert_eq!(
        role_sender_1.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_2.self_head_str(), "RoleAlltoA".to_string());
    assert_eq!(
        role_sender_2.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
}

pub fn role_all_to_a_fields() {
    let (role_sender_1, role_sender_2) = RoleAlltoA::<RoleEnd, RoleEnd>::new();

    assert_eq!(
        role_sender_1.continuation_left().self_head_str(),
        "RoleEnd".to_string()
    );

    assert_eq!(
        role_sender_1.continuation_right().self_head_str(),
        "RoleEnd".to_string()
    );

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_1.self_head_str(), "RoleAlltoA".to_string());
    assert_eq!(
        role_sender_1.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_2.self_head_str(), "RoleAtoAll".to_string());
    assert_eq!(
        role_sender_2.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
}

pub fn role_b_to_all_fields() {
    let (role_sender_1, role_sender_2) = RoleBtoAll::<RoleEnd, RoleEnd>::new();

    assert_eq!(
        role_sender_1.continuation_left().self_head_str(),
        "RoleEnd".to_string()
    );

    assert_eq!(
        role_sender_1.continuation_right().self_head_str(),
        "RoleEnd".to_string()
    );

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_1.self_head_str(), "RoleBtoAll".to_string());
    assert_eq!(
        role_sender_1.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_2.self_head_str(), "RoleAlltoB".to_string());
    assert_eq!(
        role_sender_2.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
}

pub fn role_all_to_b_fields() {
    let (role_sender_1, role_sender_2) = RoleAlltoB::<RoleEnd, RoleEnd>::new();

    assert_eq!(
        role_sender_1.continuation_left().self_head_str(),
        "RoleEnd".to_string()
    );

    assert_eq!(
        role_sender_1.continuation_right().self_head_str(),
        "RoleEnd".to_string()
    );

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_1.self_head_str(), "RoleAlltoB".to_string());
    assert_eq!(
        role_sender_1.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_2.self_head_str(), "RoleBtoAll".to_string());
    assert_eq!(
        role_sender_2.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
}

pub fn role_c_to_all_fields() {
    let (role_sender_1, role_sender_2) = RoleCtoAll::<RoleEnd, RoleEnd>::new();

    assert_eq!(
        role_sender_1.continuation_left().self_head_str(),
        "RoleEnd".to_string()
    );

    assert_eq!(
        role_sender_1.continuation_right().self_head_str(),
        "RoleEnd".to_string()
    );

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_1.self_head_str(), "RoleCtoAll".to_string());
    assert_eq!(
        role_sender_1.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_2.self_head_str(), "RoleAlltoC".to_string());
    assert_eq!(
        role_sender_2.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
}

pub fn role_all_to_c_fields() {
    let (role_sender_1, role_sender_2) = RoleAlltoC::<RoleEnd, RoleEnd>::new();

    assert_eq!(
        role_sender_1.continuation_left().self_head_str(),
        "RoleEnd".to_string()
    );

    assert_eq!(
        role_sender_1.continuation_right().self_head_str(),
        "RoleEnd".to_string()
    );

    // role_sender_1
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_1.sender1.send(there1).unwrap_or(());
    role_sender_1.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_1.self_head_str(), "RoleAlltoC".to_string());
    assert_eq!(
        role_sender_1.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );

    // role_sender_2
    let (here1, there1) = RoleEnd::new();
    let (here2, there2) = RoleEnd::new();
    role_sender_2.sender1.send(there1).unwrap_or(());
    role_sender_2.sender2.send(there2).unwrap_or(());

    assert_eq!(here1.sender.send(()).unwrap_or(()), ());
    assert_eq!(here2.sender.send(()).unwrap_or(()), ());
    assert_eq!(role_sender_2.self_head_str(), "RoleCtoAll".to_string());
    assert_eq!(
        role_sender_2.self_tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
}

pub fn role_head_str() {
    assert_eq!(RoleEnd::head_str(), "RoleEnd".to_string());
    assert_eq!(RoleA::<RoleEnd>::head_str(), "RoleA".to_string());
    assert_eq!(RoleB::<RoleEnd>::head_str(), "RoleB".to_string());
    assert_eq!(RoleC::<RoleEnd>::head_str(), "RoleC".to_string());
    assert_eq!(RoleADual::<RoleEnd>::head_str(), "RoleADual".to_string());
    assert_eq!(RoleBDual::<RoleEnd>::head_str(), "RoleBDual".to_string());
    assert_eq!(RoleCDual::<RoleEnd>::head_str(), "RoleCDual".to_string());
    assert_eq!(
        RoleAtoAll::<RoleEnd, RoleEnd>::head_str(),
        "RoleAtoAll".to_string()
    );
    assert_eq!(
        RoleBtoAll::<RoleEnd, RoleEnd>::head_str(),
        "RoleBtoAll".to_string()
    );
    assert_eq!(
        RoleCtoAll::<RoleEnd, RoleEnd>::head_str(),
        "RoleCtoAll".to_string()
    );
    assert_eq!(
        RoleAlltoA::<RoleEnd, RoleEnd>::head_str(),
        "RoleAlltoA".to_string()
    );
    assert_eq!(
        RoleAlltoB::<RoleEnd, RoleEnd>::head_str(),
        "RoleAlltoB".to_string()
    );
    assert_eq!(
        RoleAlltoC::<RoleEnd, RoleEnd>::head_str(),
        "RoleAlltoC".to_string()
    );
}

pub fn role_tail_str() {
    assert_eq!(RoleEnd::tail_str(), "".to_string());
    assert_eq!(RoleA::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(RoleB::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(RoleC::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(RoleADual::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(RoleBDual::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(RoleCDual::<RoleEnd>::tail_str(), "RoleEnd<>".to_string());
    assert_eq!(
        RoleAtoAll::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
    assert_eq!(
        RoleBtoAll::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
    assert_eq!(
        RoleCtoAll::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
    assert_eq!(
        RoleAlltoA::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
    assert_eq!(
        RoleAlltoB::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
    assert_eq!(
        RoleAlltoC::<RoleEnd, RoleEnd>::tail_str(),
        "RoleEnd<> + RoleEnd<>".to_string()
    );
}

pub fn role_broadcast_fields_1() {
    let (role_end_1, role_end_2) = RoleBroadcast::new();

    assert_eq!(role_end_1.sender.send(()), Ok(()));
    assert_eq!(role_end_2.sender.send(()), Ok(()));
    assert_eq!(role_end_1.receiver.recv(), Ok(()));
    assert_eq!(role_end_2.receiver.recv(), Ok(()));

    assert_eq!(RoleBroadcast::head_str(), "RoleBroadcast".to_string());
    assert_eq!(RoleBroadcast::tail_str(), "".to_string());
}

pub fn role_broadcast_fields_2() {
    let (role_end_1, role_end_2) = RoleBroadcast::new();

    assert_eq!(role_end_2.sender.send(()), Ok(()));
    assert_eq!(role_end_1.sender.send(()), Ok(()));
    assert_eq!(role_end_2.receiver.recv(), Ok(()));
    assert_eq!(role_end_1.receiver.recv(), Ok(()));

    assert_eq!(role_end_1.self_head_str(), "RoleBroadcast".to_string());
    assert_eq!(role_end_1.self_tail_str(), "".to_string());
    assert_eq!(role_end_2.self_head_str(), "RoleBroadcast".to_string());
    assert_eq!(role_end_2.self_tail_str(), "".to_string());
}
