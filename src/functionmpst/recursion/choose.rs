/// Choose, for A, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_a_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        let (session_ac, session_ca) = <_ as mpstthree::binary::Session>::new();
        let (session_bc, session_cb) = <_ as mpstthree::binary::Session>::new();
        let (session_ab, session_ba) = <_ as mpstthree::binary::Session>::new();
        let (stack_a, _) = <_ as mpstthree::role::Role>::new();
        let (stack_b, _) = <_ as mpstthree::role::Role>::new();
        let (stack_c, _) = <_ as mpstthree::role::Role>::new();
        let (name_a, _) = mpstthree::role::a::RoleA::<mpstthree::role::end::RoleEnd>::new();
        let (name_b, _) = mpstthree::role::b::RoleB::<mpstthree::role::end::RoleEnd>::new();
        let (name_c, _) = mpstthree::role::c::RoleC::<mpstthree::role::end::RoleEnd>::new();

        let s = mpstthree::functionmpst::send::send_mpst_a_to_b(
            $labelone(mpstthree::sessionmpst::SessionMpst {
                session1: session_ba,
                session2: session_bc,
                stack: stack_b,
                name: name_b,
            }),
            $session,
        );
        let s = mpstthree::functionmpst::send::send_mpst_a_to_c(
            $labeltwo(mpstthree::sessionmpst::SessionMpst {
                session1: session_ca,
                session2: session_cb,
                stack: stack_c,
                name: name_c,
            }),
            s,
        );

        mpstthree::binary::cancel(s);

        mpstthree::sessionmpst::SessionMpst {
            session1: session_ab,
            session2: session_ac,
            stack: stack_a,
            name: name_a,
        }
    }};
}

/// Choose, for B, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_b_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        let (session_ac, session_ca) = <_ as mpstthree::binary::Session>::new();
        let (session_bc, session_cb) = <_ as mpstthree::binary::Session>::new();
        let (session_ab, session_ba) = <_ as mpstthree::binary::Session>::new();
        let (stack_a, _) = <_ as mpstthree::role::Role>::new();
        let (stack_b, _) = <_ as mpstthree::role::Role>::new();
        let (stack_c, _) = <_ as mpstthree::role::Role>::new();
        let (name_a, _) = mpstthree::role::a::RoleA::<mpstthree::role::end::RoleEnd>::new();
        let (name_b, _) = mpstthree::role::b::RoleB::<mpstthree::role::end::RoleEnd>::new();
        let (name_c, _) = mpstthree::role::c::RoleC::<mpstthree::role::end::RoleEnd>::new();

        let s = mpstthree::functionmpst::send::send_mpst_b_to_a(
            $labelone(mpstthree::sessionmpst::SessionMpst {
                session1: session_ab,
                session2: session_ac,
                stack: stack_a,
                name: name_a,
            }),
            $session,
        );
        let s = mpstthree::functionmpst::send::send_mpst_b_to_c(
            $labeltwo(mpstthree::sessionmpst::SessionMpst {
                session1: session_ca,
                session2: session_cb,
                stack: stack_c,
                name: name_c,
            }),
            s,
        );

        mpstthree::binary::cancel(s);

        mpstthree::sessionmpst::SessionMpst {
            session1: session_ba,
            session2: session_bc,
            stack: stack_b,
            name: name_b,
        }
    }};
}

/// Choose, for C, between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_c_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        let (session_ac, session_ca) = <_ as mpstthree::binary::Session>::new();
        let (session_bc, session_cb) = <_ as mpstthree::binary::Session>::new();
        let (session_ab, session_ba) = <_ as mpstthree::binary::Session>::new();
        let (stack_a, _) = <_ as mpstthree::role::Role>::new();
        let (stack_b, _) = <_ as mpstthree::role::Role>::new();
        let (stack_c, _) = <_ as mpstthree::role::Role>::new();
        let (name_a, _) = mpstthree::role::a::RoleA::<mpstthree::role::end::RoleEnd>::new();
        let (name_b, _) = mpstthree::role::b::RoleB::<mpstthree::role::end::RoleEnd>::new();
        let (name_c, _) = mpstthree::role::c::RoleC::<mpstthree::role::end::RoleEnd>::new();

        let s = mpstthree::functionmpst::send::send_mpst_c_to_a(
            $labelone(mpstthree::sessionmpst::SessionMpst {
                session1: session_ab,
                session2: session_ac,
                stack: stack_a,
                name: name_a,
            }),
            $session,
        );
        let s = mpstthree::functionmpst::send::send_mpst_c_to_b(
            $labeltwo(mpstthree::sessionmpst::SessionMpst {
                session1: session_ba,
                session2: session_bc,
                stack: stack_b,
                name: name_b,
            }),
            s,
        );

        mpstthree::binary::cancel(s);

        mpstthree::sessionmpst::SessionMpst {
            session1: session_ca,
            session2: session_cb,
            stack: stack_c,
            name: name_c,
        }
    }};
}
