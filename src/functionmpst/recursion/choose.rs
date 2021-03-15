/// Choose, for A, among two different sessions
///
/// # Arguments
///
///  * The session to be used
///  * The first path to be used
///  * The second path to be used
///
/// # Example
///
/// ```ignore
/// match xs.pop() {
///     Option::Some(_) => {
///         let s = choose_mpst_a_to_all!(s,
/// CBranchesBtoA::Video, CBranchesCtoA::Video);
/// let s = send_mpst_a_to_b(1, s);          let (_, s) =
/// recv_mpst_a_from_b(s)?;          client_recurs(s, xs,
/// index + 1)      }
///     Option::None => {
///         let s = choose_mpst_a_to_all!(s,
/// CBranchesBtoA::End, CBranchesCtoA::End);
/// close_mpst(s)?;          Ok(())
///     }
/// }
/// ```
///
/// TO TEST
#[macro_export]
macro_rules! choose_mpst_a_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        use mpstthree::functionmpst::send::send_mpst_a_to_b;
        use mpstthree::functionmpst::send::send_mpst_a_to_c;
        use mpstthree::role::a::RoleA;
        use mpstthree::role::b::RoleB;
        use mpstthree::role::c::RoleC;
        use mpstthree::sessionmpst::SessionMpst;

        mpstthree::choose_mpst_multi_to_all!(
            $session,
            send_mpst_a_to_b,
            send_mpst_a_to_c, =>
            $labelone,
            $labeltwo, =>
            RoleB,
            RoleC, =>
            RoleA,
            SessionMpst,
            3,
            1
        )
    }};
}

/// Choose, for B, among two different sessions
///
/// # Arguments
///
///  * The session to be used
///  * The first path to be used
///  * The second path to be used
///
/// # Example
///
/// ```ignore
/// match xs.pop() {
///     Option::Some(_) => {
///         let s = choose_mpst_b_to_all!(s,
/// CBranchesAtoB::Video, CBranchesCtoB::Video);
/// let s = send_mpst_b_to_a(1, s);          let (_, s) =
/// recv_mpst_b_from_a(s)?;          client_recurs(s, xs,
/// index + 1)      }
///     Option::None => {
///         let s = choose_mpst_b_to_all!(s,
/// CBranchesAtoB::End, CBranchesCtoB::End);
/// close_mpst(s)?;          Ok(())
///     }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_b_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        use mpstthree::functionmpst::send::send_mpst_b_to_a;
        use mpstthree::functionmpst::send::send_mpst_b_to_c;
        use mpstthree::role::a::RoleA;
        use mpstthree::role::b::RoleB;
        use mpstthree::role::c::RoleC;
        use mpstthree::sessionmpst::SessionMpst;

        mpstthree::choose_mpst_multi_to_all!(
            $session,
            send_mpst_b_to_a,
            send_mpst_b_to_c, =>
            $labelone,
            $labeltwo, =>
            RoleA,
            RoleC, =>
            RoleB,
            SessionMpst,
            3,
            2
        )
    }};
}

/// Choose, for C, among two different sessions
///
/// # Arguments
///
///  * The session to be used
///  * The first path to be used
///  * The second path to be used
///
/// # Example
///
/// ```ignore
/// match xs.pop() {
///     Option::Some(_) => {
///         let s = choose_mpst_c_to_all!(s,
/// CBranchesAtoC::Video, CBranchesBtoC::Video);
/// let s = send_mpst_c_to_a(1, s);          let (_, s) =
/// recv_mpst_c_from_a(s)?;          client_recurs(s, xs,
/// index + 1)      }
///     Option::None => {
///         let s = choose_mpst_c_to_all!(s,
/// CBranchesAtoC::End, CBranchesBtoC::End);
/// close_mpst(s)?;          Ok(())
///     }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_c_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        use mpstthree::functionmpst::send::send_mpst_c_to_a;
        use mpstthree::functionmpst::send::send_mpst_c_to_b;
        use mpstthree::role::a::RoleA;
        use mpstthree::role::b::RoleB;
        use mpstthree::role::c::RoleC;
        use mpstthree::sessionmpst::SessionMpst;

        mpstthree::choose_mpst_multi_to_all!(
            $session,
            send_mpst_c_to_a,
            send_mpst_c_to_b, =>
            $labelone,
            $labeltwo, =>
            RoleA,
            RoleB, =>
            RoleC,
            SessionMpst,
            3,
            3
        )
    }};
}
