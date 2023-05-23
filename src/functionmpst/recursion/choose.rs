//! This module contains the *choose* macros
//! for recursion for roles A, B and C.
//! They all accept the current session
//! and the different branches.

#[doc(hidden)]
#[macro_export]
macro_rules! choose_aux {
    (
        $session: expr,
        $( $label: path , )+ =>
        $sender: ident,
        $meshedchannels_name: ident,
        $exclusion: literal
    ) => {
        mpst_seq::choose_mpst_multi_to_all!(
            $session ,
            ( $( ( $label ) )+ ) ,
            $sender ,
            $meshedchannels_name ,
            $exclusion
        );
    }
}

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
///         let s = choose_mpst_a_to_all!(
///             s,
///             CBranchesBtoA::Video,
///             CBranchesCtoA::Video
///         );
///         let s = send_mpst_a_to_b(1, s);
///         let (_, s) = recv_mpst_a_from_b(s)?;
///         client_recurs(s, xs, index + 1)
///     }

///     Option::None => {
///         let s = choose_mpst_a_to_all!(
///             s,
///             CBranchesBtoA::End,
///             CBranchesCtoA::End
///         );
///         close_mpst(s)
///     }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_a_to_all {
    ($session: expr, $( $label: path),+ $(,)? ) => {{
        use mpstthree::name::a::NameA;
        use mpstthree::meshedchannels::MeshedChannels;

        mpstthree::choose_aux!(
            $session,
            $( $label , )+ =>
            NameA,
            MeshedChannels,
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
///         let s = choose_mpst_b_to_all!(
///             s,
///             CBranchesAtoB::Video,
///             CBranchesCtoB::Video
///         );
///         let s = send_mpst_b_to_a(1, s);
///         let (_, s) = recv_mpst_b_from_a(s)?;
///         client_recurs(s, xs, index + 1)
///     }

///     Option::None => {
///         let s = choose_mpst_b_to_all!(
///             s,
///             CBranchesAtoB::End,
///             CBranchesCtoB::End
///         );
///         close_mpst(s)
///     }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_b_to_all {
    ($session: expr,  $( $label: path),+ $(,)? ) => {{
        use mpstthree::name::b::NameB;
        use mpstthree::meshedchannels::MeshedChannels;

        mpstthree::choose_aux!(
            $session,
            $( $label , )+ =>
            NameB,
            MeshedChannels,
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
///         let s = choose_mpst_c_to_all!(
///             s,
///             CBranchesAtoC::Video,
///             CBranchesBtoC::Video
///         );
///         let s = send_mpst_c_to_a(1, s);
///         let (_, s) = recv_mpst_c_from_a(s)?;
///         client_recurs(s, xs, index + 1)
///     }

///     Option::None => {
///         let s = choose_mpst_c_to_all!(
///             s,
///             CBranchesAtoC::End,
///             CBranchesBtoC::End
///         );
///         close_mpst(s)
///     }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_c_to_all {
    ($session: expr,  $( $label: path),+ $(,)? ) => {{
        use mpstthree::name::c::NameC;
        use mpstthree::meshedchannels::MeshedChannels;

        mpstthree::choose_aux!(
            $session,
            $( $label , )+ =>
            NameC,
            MeshedChannels,
            3
        )
    }};
}
