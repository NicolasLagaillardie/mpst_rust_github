////////////////////////////////////////////
/// CHOICE

/// Create the *ChooseMpst* type to be used with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* type
/// * The *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_choose_type_multi, create_sessionmpst};
///
/// create_sessionmpst!(SessionMpst, 3);
/// create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3);
/// ```
#[macro_export]
macro_rules! create_choose_type_multi {
    ($type_name: ident, $sessionmpst_name: ident, $nsessions: literal) => {
        mpst_seq::create_choose_type_multi!($type_name, $sessionmpst_name, $nsessions);
    };
}

/// Create the *ChooseMpst* function to send a *Choose* left branch to be used with more than 3
/// participants.
/// Only works when active role is the last one (TODO: adapt to any index role).
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the *ChooseMpst* type that is used
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{
///     create_broadcast_role, create_choose_mpst_session_multi_left, create_choose_type_multi,
///     create_normal_role, create_sessionmpst,
/// };
///
/// create_normal_role!(RoleD, RoleDDual);
/// create_broadcast_role!(RoleAlltoD, RoleDtoAll);
///
/// create_sessionmpst!(SessionMpst, 3);
/// create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3);
///
/// create_choose_mpst_session_multi_left!(
///     choose_left_mpst_session_d_to_all,
///     ChooseMpstThree,
///     RoleDtoAll,
///     RoleD,
///     SessionMpst,
///     3
/// );
/// ```
#[macro_export]
macro_rules! create_choose_mpst_session_multi_left {
    ($func_name: ident, $type_name: ident, $role_dual: ident, $name: ident, $sessionmpst_name: ident, $nsessions: literal) => {
        mpst_seq::create_choose_mpst_session_multi_left!(
            $func_name,
            $type_name,
            $role_dual,
            $name,
            $sessionmpst_name,
            $nsessions
        );
    };
}

/// Create the *ChooseMpst* function to send a *Choose* right branch to be used with more than 3
/// participants.
/// Only works when active role is the last one (TODO: adapt to any index role).
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the *ChooseMpst* type that is used
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{
///     create_broadcast_role, create_choose_mpst_session_multi_right, create_choose_type_multi,
///     create_normal_role, create_sessionmpst,
/// };
///
/// create_normal_role!(RoleD, RoleDDual);
/// create_broadcast_role!(RoleAlltoD, RoleDtoAll);
///
/// create_sessionmpst!(SessionMpst, 3);
/// create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3);
///
/// create_choose_mpst_session_multi_right!(
///     choose_right_mpst_session_d_to_all,
///     ChooseMpstThree,
///     RoleDtoAll,
///     RoleD,
///     SessionMpst,
///     3
/// );
/// ```
#[macro_export]
macro_rules! create_choose_mpst_session_multi_right {
    ($func_name: ident, $type_name: ident, $role_dual: ident, $name: ident, $sessionmpst_name: ident, $nsessions: literal) => {
        mpst_seq::create_choose_mpst_session_multi_right!(
            $func_name,
            $type_name,
            $role_dual,
            $name,
            $sessionmpst_name,
            $nsessions
        );
    };
}

/// Create the two *ChooseMpst* functions to send a *Choose* on each branch to be used with more
/// than 3 participants.
/// Only works when active role is the last one (TODO: adapt to any index role).
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function for the left branch
/// * The name of the new *ChooseMpst* function for the right branch
/// * The name of the *ChooseMpst* type that is used
/// * The name of the dual of the broadcasting sender. This one should contain *toAll* according to
///   the convention
/// * The name of the sender
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{
///     create_broadcast_role, create_choose_mpst_session_multi_both, create_choose_type_multi,
///     create_normal_role, create_sessionmpst,
/// };
///
/// create_normal_role!(RoleD, RoleDDual);
/// create_broadcast_role!(RoleAlltoD, RoleDtoAll);
///
/// create_sessionmpst!(SessionMpst, 3);
/// create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3);
///
/// create_choose_mpst_session_multi_both!(
///     choose_left_mpst_session_d_to_all,
///     choose_right_mpst_session_d_to_all,
///     ChooseMpstThree,
///     RoleDtoAll,
///     RoleD,
///     SessionMpst,
///     3
/// );
/// ```
#[macro_export]
macro_rules! create_choose_mpst_session_multi_both {
    (
        $func_name_left: ident,
        $func_name_right: ident,
        $type_name: ident,
        $role_dual: ident,
        $name: ident,
        $sessionmpst_name: ident,
        $nsessions: literal
    ) => {
        mpstthree::create_choose_mpst_session_multi_left!(
            $func_name_left,
            $type_name,
            $role_dual,
            $name,
            $sessionmpst_name,
            $nsessions
        );

        mpstthree::create_choose_mpst_session_multi_right!(
            $func_name_right,
            $type_name,
            $role_dual,
            $name,
            $sessionmpst_name,
            $nsessions
        );
    };
}

/// Choose among different sessions that are provided, for protocols with more than 3 participants
///
/// # Arguments
///
///  * The session to be used
///  * The different `enum` variants which represent the different branches to be sent to each
///    passive role
///  * The different passive roles
///  * The name of the sender
///  * The name of the *SessionMpst* type that will be used
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
///            RoleA,
///            RoleB, =>
///            RoleD,
///            SessionMpst,
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
///            RoleA,
///            RoleB, =>
///            RoleD,
///            SessionMpst,
///            3
///        );
///        close_mpst_multi(s)
///    }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_multi_to_all {
    ($session: expr, $($label: path,)+ => $($receiver: ident,)+ => $sender: ident, $sessionmpst_name: ident, $exclusion: literal) => {
        mpst_seq::choose_mpst_multi_to_all!(
            ( $session ) ,
            ( $( $label , )* ) ,
            ( $( $receiver , )* ) ,
            $sender ,
            $sessionmpst_name ,
            $exclusion
        );
    }
}

/// Choose among different sessions that are provided, for protocols with more than 3 participants,
/// may fail because of a canceled session. Need to exclude the first participant
///
/// # Arguments
///
///  * The session to be used
///  * The different `enum` variants which represent the different branches to be sent to each
///    passive role
///  * The different passive roles
///  * The name of the sender
///  * The name of the *SessionMpst* type that will be used
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
///            RoleA,
///            RoleB, =>
///            RoleD,
///            SessionMpst,
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
///            RoleA,
///            RoleB, =>
///            RoleD,
///            SessionMpst,
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
/// use mpstthree::role::Role;
/// use mpstthree::{create_multiple_normal_role, create_sessionmpst, choose_mpst_multi_cancel_to_all};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// bundle_struct_fork_close_multi!(close_mpst, fork_mpst, SessionMpst, 3);
///
/// match xs.pop() {
///    Option::Some(_) => {
///        let s = choose_mpst_multi_cancel_to_all!(
///            s,
///            CBranchesAtoC::Video,
///            CBranchesBtoC::Video, =>
///            RoleA,
///            RoleB, =>
///            RoleD,
///            SessionMpst,
///            3
///        );
///        client_recurs(s, xs, index + 1)
///    }
///    Option::None => {
///        let s = choose_mpst_multi_cancel_to_all!(
///            s,
///            CBranchesAtoC::End,
///            CBranchesBtoC::End, =>
///            RoleA,
///            RoleB, =>
///            RoleD,
///            SessionMpst,
///            3
///        );
///        close_mpst_multi(s)
///    }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_multi_cancel_to_all {
    ($session: expr, $($label: path,)+ => $($receiver: ident,)+ => $pawn: ident, $sender: ident, $sessionmpst_name: ident, $exclusion: literal) => {
        mpst_seq::choose_mpst_multi_cancel_to_all!(
            ( $session ) ,
            ( $( $label , )* ) ,
            ( $( $receiver , )* ) ,
            $pawn ,
            $sender ,
            $sessionmpst_name ,
            $exclusion);
    }
}

/// Choose among different sessions that are provided, for protocols with more than 3 participants
///
/// # Arguments
///
///  * The session to be used
///  * The different `enum` variants which represent the different branches to be sent to each
///    passive role
///  * The different passive roles
///  * The name of the sender
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///
/// # Example
///
/// ```ignore
/// match xs.pop() {
///    Option::Some(_) => {
///        let s = choose_mpst_multi_http_to_all!(
///            s,
///            CBranchesAtoC::Video,
///            CBranchesBtoC::Video, =>
///            RoleA,
///            RoleB, =>
///            RoleD,
///            SessionMpst,
///            3
///        );
///        let s = send_http_d_to_a(1, s);
///        let (_, s) = recv_http_d_to_a(s)?;
///        client_recurs(s, xs, index + 1)
///    }
///    Option::None => {
///        let s = choose_mpst_multi_http_to_all!(
///            s,
///            CBranchesAtoC::End,
///            CBranchesBtoC::End, =>
///            RoleA,
///            RoleB, =>
///            RoleD,
///            SessionMpst,
///            3
///        );
///        close_mpst_multi(s)
///    }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_multi_http_to_all {
    ($session: expr, $($label: path,)+ => $($receiver: ident,)+ => $sender: ident, $sessionmpst_name: ident, $exclusion: literal) => {
        mpst_seq::choose_mpst_multi_http_to_all!(
            ( $session ) ,
            ( $( $label , )* ) ,
            ( $( $receiver , )* ) ,
            $sender ,
            $sessionmpst_name ,
            $exclusion
        );
    }
}

/// Create *choose* fuunctions, to choose among different sessions that are provided, for protocols
/// with more than 3 participants.
///
/// # Arguments
///
///  * The name of the new functions
///  * The name of the branches, need to be the same for every participants
///  * The new type adopted by the sender
///  * The name of the Enum containing the branches
///  * The different passive roles
///  * The name of the sender
///  * The name of the *SessionMpst* type that will be used
///  * The index of the sender among all participants
///
/// # Example
///
/// Available on the *long_simple_three_mpst_short* examples.
///
/// ```ignore
/// type EndpointDoneC = SessionMpstThree<End, End, RoleEnd, NameC>;
/// type EndpointMoreC = SessionMpstThree<
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
///     RoleA, RoleB, =>
///     RoleC, SessionMpstThree, 3
/// );
/// ```
#[macro_export]
macro_rules! create_fn_choose_mpst_multi_to_all_bundle {
    ($($fn_name: ident,)+ => $($branch: expr,)+ => $($new_type: ty,)+ => $($label: path,)+ => $($receiver: ident,)+ => $sender: ident, $sessionmpst_name: ident, $exclusion: literal) => {
        mpst_seq::create_fn_choose_mpst_multi_to_all_bundle!(
            ( $( $fn_name , )* ) ,
            ( $( $branch , )* ) ,
            ( $( $label , )* ) ,
            ( $( $receiver , )* ) ,
            ( $( $new_type , )* ) ,
            $sender ,
            $sessionmpst_name ,
            $exclusion
        );
    };
}

/// Create *choose* fuunctions, to choose among different sessions that are provided, for protocols
/// with more than 3 participants.
///
/// # Arguments
///
///  * The name of the new functions
///  * The name of the branches, need to be the same for every participants
///  * The new type adopted by the sender
///  * The name of the Enum containing the branches
///  * The different passive roles
///  * The name of the sender
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  * The index of the sender among all participants
///
/// # Example
///
/// Available on the *long_simple_three_mpst_short* examples.
///
/// ```ignore
/// type EndpointDoneC = SessionMpstThree<End, End, RoleEnd, NameC>;
/// type EndpointMoreC = SessionMpstThree<
///     Send<(), Recv<(), Choose0fromCtoA>>,
///     Send<(), Recv<(), Choose0fromCtoB>>,
///     R2A<R2B<RoleA<RoleB<RoleEnd>>>>,
///     NameC,
/// >;
/// create_fn_choose_mpst_cancel_multi_to_all_bundle!(
///     done_cancel_from_c_to_all, more_cancel_from_c_to_all, =>
///     Done, More, =>
///     EndpointDoneC, EndpointMoreC, =>
///     Branching0fromCtoA, Branching0fromCtoB, =>
///     RoleA, RoleB, =>
///     RoleC, SessionMpstThree, 3, 3
/// );
/// ```
#[macro_export]
macro_rules! create_fn_choose_mpst_cancel_multi_to_all_bundle {
    ($($fn_name: ident,)+ => $($branch: expr,)+ => $($new_type: ty,)+ => $($label: path,)+ => $($receiver: ident,)+ => $pawn: ident, $sender: ident, $sessionmpst_name: ident, $nsessions: literal, $exclusion: literal) => {
        create_fn_choose_mpst_cancel_multi_to_all_bundle!(@call_tuple $($fn_name,)+ => $($branch,)+ => $($new_type,)+ => ($($label,)+) => ($($receiver,)+) => $pawn, $sender, $sessionmpst_name, $nsessions, $exclusion);
    };
    (@call_tuple $($fn_name: ident,)+ => $($branch: expr,)+ => $($new_type: ty,)+ => $label: tt => $receiver: tt => $pawn: ident, $sender: ident, $sessionmpst_name: ident, $nsessions: literal, $exclusion: literal) => {
        $(create_fn_choose_mpst_cancel_multi_to_all_bundle!(@call $fn_name, $branch, $new_type => $label => $receiver => $pawn, $sender, $sessionmpst_name, $nsessions, $exclusion);)+
    };
    (@call $fn_name: ident, $branch: expr, $new_type: ty => ($($label: path,)+) => ($($receiver: ident,)+) => $pawn: ident, $sender: ident, $sessionmpst_name: ident, $nsessions: literal, $exclusion: literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion : ($($label,)+) : ($($receiver,)+) {
            fn $fn_name(
                s: $sessionmpst_name<
                    mpstthree::binary::struct_trait::End,
                    #(
                        Send<(mpstthree::binary::struct_trait::End, unused#N:21), mpstthree::binary::struct_trait::End>,
                    )19:0
                    mpstthree::role::broadcast::RoleBroadcast,
                    $sender<mpstthree::role::end::RoleEnd>,
                >
            ) -> Result<$new_type, Box<dyn std::error::Error>>
            {
                let mut temp = Vec::<End>::new();

                %(
                    let (channel_#N:3, channel_#N:4) = <_ as mpstthree::binary::struct_trait::Session>::new();
                )(
                    let (channel_#N:3, channel_#N:4) = <mpstthree::binary::struct_trait::End as mpstthree::binary::struct_trait::Session>::new();

                    temp.push(channel_#N:3);
                )5*

                let (stack_1, _) = <mpstthree::binary::struct_trait::End as mpstthree::binary::struct_trait::Session>::new();

                #(
                    let (stack_#N:0, _) = <_ as mpstthree::role::Role>::new();
                )18:0

                let (name_1, _) = <$pawn<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                #(
                    let (name_#N:0, _) = <unused#N:22::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                )19:0

                let (name_^N:2, _) = <$sender<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                %(
                    let elt = match temp.pop() {
                        Some(e) => e,
                        _ => panic!("Error type"),
                    };

                    let _  = mpstthree::binary::send::send_canceled(
                        (
                            elt,
                            unused#N:21::$branch($sessionmpst_name {
                                ~(
                                    session#N:1 : channel_~N:7,
                                )(
                                    session#N:1 : channel_~N:7,
                                )0*
                                stack: stack_#N:0,
                                name: name_#N:0,
                            }
                        )),
                        s.session#N:0,
                    )?;
                )(
                    let elt = match temp.pop() {
                        Some(e) => e,
                        _ => panic!("Error type"),
                    };

                    let _  = mpstthree::binary::send::send_canceled(
                        (
                            elt,
                            unused#N:21::$branch($sessionmpst_name {
                                ~(
                                    session#N:1 : channel_~N:7,
                                )(
                                    session#N:1 : channel_~N:7,
                                )0*
                                stack: stack_#N:0,
                                name: name_#N:0,
                            }
                        )),
                        s.session#N:0,
                    )?;
                )4*

                let elt = match temp.pop() {
                    Some(e) => e,
                    _ => panic!("Error type"),
                };
                let s = s.session1.sender.send(mpstthree::binary::struct_trait::Signal::Offer(elt)).unwrap();

                Ok(
                    $sessionmpst_name {
                        #(
                            session#N:0: channel_#N:17 ,
                        )0:0
                        stack: stack_^N:2,
                        name: name_^N:2,
                    }
                )
            }
        });
    };
}
