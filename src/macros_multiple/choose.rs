//! This module contains the macros
//! for creating choose functions for any number
//! of participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_multiple"` feature.*

/// Create the *ChooseMpst* type to be used with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* type
/// * The *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_choose_type_multi, create_meshedchannels};
///
/// create_meshedchannels!(MeshedChannels, 3);
/// create_choose_type_multi!(ChooseMpstThree, MeshedChannels, 3);
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_choose_type_multi {
    ($type_name: ident, $meshedchannels_name: ident, $n_sessions: literal) => {
        mpst_seq::create_choose_type_multi!($type_name, $meshedchannels_name, $n_sessions);
    };
}

/// Create the *ChooseMpst* function to send a *Choose* left branch to be used with more than 3
/// participants.
/// Only works when active role is the last one.
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the *ChooseMpst* type that is used
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{
///     create_broadcast_role, create_choose_mpst_session_multi_left, create_choose_type_multi,
///     create_meshedchannels, create_normal_name, create_normal_role,
/// };
///
/// create_normal_role!(RoleD, RoleDDual);
/// create_broadcast_role!(RoleAlltoD, RoleDtoAll);
///
/// create_normal_name!(NameD);
///
/// create_meshedchannels!(MeshedChannels, 3);
/// create_choose_type_multi!(ChooseMpstThree, MeshedChannels, 3);
///
/// create_choose_mpst_session_multi_left!(
///     choose_left_mpst_session_d_to_all,
///     ChooseMpstThree,
///     RoleDtoAll,
///     NameD,
///     MeshedChannels,
///     3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_choose_mpst_session_multi_left {
    (
        $func_name: ident,
        $type_name: ident,
        $role_dual: ident,
        $name: ident,
        $meshedchannels_name: ident,
        $n_sessions: literal
    ) => {
        mpst_seq::create_choose_mpst_session_multi_left!(
            $func_name,
            $type_name,
            $role_dual,
            $name,
            $meshedchannels_name,
            $n_sessions
        );
    };
}

/// Create the *ChooseMpst* function to send a *Choose* right branch to be used with more than 3
/// participants.
/// Only works when active role is the last one.
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the *ChooseMpst* type that is used
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{
///     create_broadcast_role, create_choose_mpst_session_multi_right, create_choose_type_multi,
///     create_meshedchannels, create_normal_name, create_normal_role,
/// };
///
/// create_normal_role!(RoleD, RoleDDual);
/// create_broadcast_role!(RoleAlltoD, RoleDtoAll);
///
/// create_normal_name!(NameD);
///
/// create_meshedchannels!(MeshedChannels, 3);
/// create_choose_type_multi!(ChooseMpstThree, MeshedChannels, 3);
///
/// create_choose_mpst_session_multi_right!(
///     choose_right_mpst_session_d_to_all,
///     ChooseMpstThree,
///     RoleDtoAll,
///     NameD,
///     MeshedChannels,
///     3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_choose_mpst_session_multi_right {
    (
        $func_name: ident,
        $type_name: ident,
        $role_dual: ident,
        $name: ident,
        $meshedchannels_name: ident,
        $n_sessions: literal
    ) => {
        mpst_seq::create_choose_mpst_session_multi_right!(
            $func_name,
            $type_name,
            $role_dual,
            $name,
            $meshedchannels_name,
            $n_sessions
        );
    };
}

/// Create the two *ChooseMpst* functions to send a *Choose* on each branch to be used with more
/// than 3 participants.
/// Only works when active role is the last one.
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function for the left branch
/// * The name of the new *ChooseMpst* function for the right branch
/// * The name of the *ChooseMpst* type that is used
/// * The name of the dual of the broadcasting sender. This one should contain *toAll* according to
///   the convention
/// * The name of the sender
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{
///     create_broadcast_role, create_choose_mpst_session_multi_both, create_choose_type_multi,
///     create_meshedchannels, create_normal_name, create_normal_role,
/// };
///
/// create_normal_role!(RoleD, RoleDDual);
/// create_broadcast_role!(RoleAlltoD, RoleDtoAll);
///
/// create_normal_name!(NameD);
///
/// create_meshedchannels!(MeshedChannels, 3);
/// create_choose_type_multi!(ChooseMpstThree, MeshedChannels, 3);
///
/// create_choose_mpst_session_multi_both!(
///     choose_left_mpst_session_d_to_all,
///     choose_right_mpst_session_d_to_all,
///     ChooseMpstThree,
///     RoleDtoAll,
///     NameD,
///     MeshedChannels,
///     3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_choose_mpst_session_multi_both {
    (
        $func_name_left: ident,
        $func_name_right: ident,
        $type_name: ident,
        $role_dual: ident,
        $name: ident,
        $meshedchannels_name: ident,
        $n_sessions: literal
    ) => {
        mpstthree::create_choose_mpst_session_multi_left!(
            $func_name_left,
            $type_name,
            $role_dual,
            $name,
            $meshedchannels_name,
            $n_sessions
        );

        mpstthree::create_choose_mpst_session_multi_right!(
            $func_name_right,
            $type_name,
            $role_dual,
            $name,
            $meshedchannels_name,
            $n_sessions
        );
    };
}

/// Choose among different sessions that are provided.
///
/// # Arguments
///
///  * The session to be used
///  * The different `enum` variants which represent the different branches to be sent to each
///    passive role
///  * The different passive roles
///  * The name of the sender
///  * The name of the *MeshedChannels* type that will be used
///  * The index of the active role
///
/// # Example
///
/// Available on the *cases/13_macro_multi_recursion* test.
///
/// ```ignore
/// match xs.pop() {
///    Option::Some(_) => {
///        let s = choose_mpst_multi_to_all!(
///            s,
///            CBranchesAtoC::Video,
///            CBranchesBtoC::Video, =>
///            NameD,
///            MeshedChannels,
///            3
///        );
///        let s = send_mpst_d_to_a(1, s);
///        let (_, s) = recv_mpst_d_from_a(s)?;
///        client_recurs(s, xs, index + 1)
///    }

///    Option::None => {
///        let s = choose_mpst_multi_to_all!(
///            s,
///            CBranchesAtoC::End,
///            CBranchesBtoC::End, =>
///            NameD,
///            MeshedChannels,
///            3
///        );
///        close_mpst_multi(s)
///    }
/// }
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! choose_mpst_multi_to_all {
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

/// Create a macro that simplifies the usage of [`choose_mpst_multi_to_all`].
///
/// # Arguments
///
///  * The name of the new macro
///  * The different passive roles
///  * The name of the sender
///  * The name of the *MeshedChannels* type that will be used
///  * The index of the active role
///
/// # Example
///
/// Available on the *cases_short/macro_multi_recursion_macro_of_macro* test.
///
/// ```ignore
/// choose_mpst_create_multi_to_all!(
///     choose_mpst_client_to_all,
///     NameA, NameB, =>
///     NameD, MeshedChannels,
///     3
/// );
///
/// match xs.pop() {
///     Option::Some(_) => {
///         let s: EndpointDVideo<i32> = choose_mpst_client_to_all!(
///             s,
///             Branches0AtoD::Video,
///             Branches0BtoD::Video,
///         );
///
///         let (_, s) = s.send(1).recv()?;
///
///         client_recurs(s, xs, index + 1)
///     }

///     Option::None => {
///         let s = choose_mpst_client_to_all!(
///             s,
///             Branches0AtoD::End,
///             Branches0BtoD::End,
///         );
///
///         assert_eq!(index, 100);
///
///         s.close()
///     }
/// }
/// ```
///
/// [`choose_mpst_multi_to_all`]: crate::choose_mpst_multi_to_all
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! choose_mpst_create_multi_to_all {
    ($name:expr, $sender: ident, $meshedchannels_name: ident, $exclusion: literal) => {
        mpst_seq::choose_mpst_create_multi_to_all!(
            $name,
            $sender,
            $meshedchannels_name,
            $exclusion
        );
    };
}

/// Choose among different sessions that are provided,
/// may fail because of a canceled session. Need to exclude the first participant
///
/// # Arguments
///
///  * The session to be used
///  * The different `enum` variants which represent the different branches to be sent to each
///    passive role
///  * The different passive roles
///  * The name of the sender
///  * The name of the *MeshedChannels* type that will be used
///  * The index of the active role
///
/// # Example
///
/// Available on the *cancel/cancel_10* test.
///
/// ```ignore
/// match xs.pop() {
///    Option::Some(_) => {
///        let s = choose_mpst_multi_cancel_to_all!(
///            s,
///            CBranchesBtoC::Video, =>
///            NameA,
///            NameB, =>
///            NameD,
///            MeshedChannels,
///            3
///        );
///        let s = send_mpst_d_to_a(1, s);
///        let (_, s) = recv_mpst_d_from_a(s)?;
///        client_recurs(s, xs, index + 1)
///    }

///    Option::None => {
///        let s = choose_mpst_multi_cancel_to_all!(
///            s,
///            CBranchesBtoC::End, =>
///            NameA,
///            NameB, =>
///            NameD,
///            MeshedChannels,
///            3
///        );
///        close_mpst_multi(s)
///    }
/// }
/// ```
///
/// # Compile fail
///
/// Available on the *cancel/cancel_8* test.
///
/// ```compile_fail
/// use mpstthree::{create_multiple_normal_role, choose_mpst_multi_cancel_to_all, create_multiple_normal_name};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_multiple_normal_name!(
///     NameA,
///     NameB,
///     NameD
/// );
///
/// bundle_struct_fork_close_multi!(close_mpst, fork_mpst, MeshedChannels, 3);
///
/// match xs.pop() {
///    Option::Some(_) => {
///        let s = choose_mpst_multi_cancel_to_all!(
///            s,
///            CBranchesAtoC::Video,
///            CBranchesBtoC::Video, =>
///            NameA,
///            NameB, =>
///            NameD,
///            MeshedChannels,
///            3
///        );
///        client_recurs(s, xs, index + 1)
///    }

///    Option::None => {
///        let s = choose_mpst_multi_cancel_to_all!(
///            s,
///            CBranchesAtoC::End,
///            CBranchesBtoC::End, =>
///            NameA,
///            NameB, =>
///            NameD,
///            MeshedChannels,
///            3
///        );
///        close_mpst_multi(s)
///    }
/// }
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! choose_mpst_multi_cancel_to_all {
    (
        $session: expr,
        $( $label: path , )+ =>
        $( $receiver: ident , )+ =>
        $broadcaster: ident,
        $sender: ident,
        $meshedchannels_name: ident,
        $exclusion: literal
    ) => {
        mpst_seq::choose_mpst_multi_cancel_to_all!(
            $session ,
            ( $( ( $label ) )+ ) ,
            ( $( ( $receiver ) )+ ) ,
            $broadcaster ,
            $sender ,
            $meshedchannels_name ,
            $exclusion
        );
    }
}

/// Create *choose* fuunctions, to choose among different sessions that are provided.
///
/// # Arguments
///
///  * The name of the new functions
///  * The name of the branches, need to be the same for every participants
///  * The new type adopted by the sender
///  * The name of the Enum containing the branches
///  * The different passive roles
///  * The name of the sender
///  * The name of the *MeshedChannels* type that will be used
///  * The index of the sender among all participants
///
/// # Example
///
/// Available on the *long_simple_three_mpst_short* examples.
///
/// ```ignore
/// type EndpointDoneC = MeshedChannels<End, End, RoleEnd, NameC>;
/// type EndpointMoreC = MeshedChannels<
///     Send<(), Recv<(), Choose0fromCtoA>>,
///     Send<(), Recv<(), Choose0fromCtoB>>,
///     R2A<R2B<RoleA<RoleB<RoleEnd>>>>,
///     NameC,
/// >;
/// create_fn_choose_mpst_multi_to_all_bundle!(
///     done_from_c_to_all, more_from_c_to_all, =>
///     Done, More, =>
///     EndpointDoneC, EndpointMoreC, =>
///     Branching0fromCtoA, Branching0fromCtoB, =>
///     NameA, NameB, =>
///     NameC, MeshedChannels, 3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_fn_choose_mpst_multi_to_all_bundle {
    (
        $( $fn_name: ident , )+ =>
        $( $branch: expr , )+ =>
        $( $new_type: ty , )+ =>
        $( $label: path , )+ =>
        $( $receiver: ident , )+ =>
        $sender: ident,
        $meshedchannels_name: ident,
        $exclusion: literal
    ) => {
        mpst_seq::create_fn_choose_mpst_multi_to_all_bundle!(
            ( $( ( $fn_name ) )+ ) ,
            ( $( ( $branch ) )+ ) ,
            ( $( ( $label ) )+ ) ,
            ( $( ( $receiver ) )+ ) ,
            ( $( ( $new_type ) )+ ) ,
            $sender ,
            $meshedchannels_name ,
            $exclusion
        );
    };
}

/// Create *choose* fuunctions, to choose among different sessions that are provided.
///
/// # Arguments
///
///  * The name of the new functions
///  * The name of the branches, need to be the same for every participants
///  * The new type adopted by the sender
///  * The name of the Enum containing the branches
///  * The different passive roles
///  * The name of the broadcaster
///  * The name of the sender
///  * The name of the *MeshedChannels* type that will be used
///  * The index of the sender among all participants
///
/// # Example
///
/// Available on the *long_simple_three_mpst_short* examples.
///
/// ```ignore
/// type EndpointDoneC = MeshedChannels<End, End, RoleEnd, NameC>;
/// type EndpointMoreC = MeshedChannels<
///     End,
///     Send<(), Recv<(), Choose0fromCtoB>>,
///     R2A<R2B<RoleB<RoleEnd>>>,
///     NameC,
/// >;
/// create_fn_choose_mpst_cancel_multi_to_all_bundle!(
///     done_cancel_from_c_to_all, more_cancel_from_c_to_all, =>
///     Done, More, =>
///     EndpointDoneC, EndpointMoreC, =>
///     Branching0fromCtoA, Branching0fromCtoB, =>
///     NameB, =>
///     NameA, NameC, MeshedChannels, 3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
macro_rules! create_fn_choose_mpst_cancel_multi_to_all_bundle {
    (
        $( $fn_name: ident , )+ =>
        $( $branch: expr , )+ =>
        $( $new_type: ty , )+ =>
        $( $label: path , )+ =>
        $( $receiver: ident , )+ =>
        $broadcaster: ident,
        $sender: ident,
        $meshedchannels_name: ident,
        $exclusion: literal
    ) => {
        mpst_seq::create_fn_choose_mpst_cancel_multi_to_all_bundle!(
            ( $( ( $fn_name ) )+ ) ,
            ( $( ( $branch ) )+ ) ,
            ( $( ( $label ) )+ ) ,
            ( $( ( $receiver ) )+ ) ,
            ( $( ( $new_type ) )+ ) ,
            $sender ,
            $broadcaster ,
            $meshedchannels_name ,
            $exclusion
        );
    }
}
