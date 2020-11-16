///  Choose, for A, among two different sessions
///  
///  # Arguments
///  
///   * The session to be used
///   * The first path to be used
///   * The second path to be used
///  
///  # Examples
///  
///  ```ignore
///  match xs.pop() {
///      Option::Some(_) => {
///          let s = choose_mpst_a_to_all!(s, CBranchesBtoA::Video, CBranchesCtoA::Video);
///          let s = send_mpst_a_to_b(1, s);
///          let (_, s) = recv_mpst_a_to_b(s)?;
///          client_recurs(s, xs, index + 1)
///      }
///      Option::None => {
///          let s = choose_mpst_a_to_all!(s, CBranchesBtoA::End, CBranchesCtoA::End);
///          close_mpst(s)?;
///          Ok(())
///      }
///  }
///  ```
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

///  Choose, for B, among two different sessions
///  
///  # Arguments
///  
///   * The session to be used
///   * The first path to be used
///   * The second path to be used
///  
///  # Examples
///  
///  ```ignore
///  match xs.pop() {
///      Option::Some(_) => {
///          let s = choose_mpst_b_to_all!(s, CBranchesAtoB::Video, CBranchesCtoB::Video);
///          let s = send_mpst_b_to_a(1, s);
///          let (_, s) = recv_mpst_b_to_a(s)?;
///          client_recurs(s, xs, index + 1)
///      }
///      Option::None => {
///          let s = choose_mpst_b_to_all!(s, CBranchesAtoB::End, CBranchesCtoB::End);
///          close_mpst(s)?;
///          Ok(())
///      }
///  }
///  ```
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

///  Choose, for C, among two different sessions
///  
///  # Arguments
///  
///   * The session to be used
///   * The first path to be used
///   * The second path to be used
///  
///  # Examples
///  
///  ```ignore
///  match xs.pop() {
///      Option::Some(_) => {
///          let s = choose_mpst_c_to_all!(s, CBranchesAtoC::Video, CBranchesBtoC::Video);
///          let s = send_mpst_c_to_a(1, s);
///          let (_, s) = recv_mpst_c_to_a(s)?;
///          client_recurs(s, xs, index + 1)
///      }
///      Option::None => {
///          let s = choose_mpst_c_to_all!(s, CBranchesAtoC::End, CBranchesBtoC::End);
///          close_mpst(s)?;
///          Ok(())
///      }
///  }
///  ```
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
