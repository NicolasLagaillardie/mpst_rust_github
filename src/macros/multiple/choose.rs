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
    ($type_name: ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            type $type_name<
                #(S#N:0,)2:0 R0, R1, N0
            > = mpstthree::binary::struct_trait::Send<
                either::Either<
                    <
                        $sessionmpst_name<#(S#N:0,)0:0 R0, N0
                    > as mpstthree::binary::struct_trait::Session>::Dual,
                    <
                        $sessionmpst_name<#(S#N:0,)3:0 R1, N0
                    > as mpstthree::binary::struct_trait::Session>::Dual
                    >,
                mpstthree::binary::struct_trait::End
            >;
        });
    }
}

/// Create the *ChooseMpst* function to send a *Choose* left branch to be used with more than 3
/// participants.
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the *ChooseMpst* type that is used
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the *next* function that is related
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
    ($func_name:ident, $type_name: ident, $role_dual:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<'a, #(S#N:0,)10:0 #(R#N:0,)11:0>(
                s: $sessionmpst_name<
                    #( // i in 1..K
                        $type_name<
                            ~( // j in 0..K
                                <S~N:2 as mpstthree::binary::struct_trait::Session>::Dual,
                            )(
                                S~N:2, // S(i + j) (with Dual if needed)
                            )0*
                            ~( // j in 0..K
                                <S~N:3 as mpstthree::binary::struct_trait::Session>::Dual,
                            )(
                                S~N:3, // S(diff * (diff + 1) / 2 + K + i + j) (with Dual if needed)
                            )0*
                            ~( // j in  0..3
                                R~N:4, // R(3 * (i - 1) + 1 + j)
                                // Side note: we lose the checking for the right order for the name on R(3 * (i - 1) + 3) → RoleADual<RoleEnd>
                            )()3*
                        >,
                    )0:0

                    $role_dual<
                        #(
                            R#N:0,
                        )13:0
                    >,

                    $name<mpstthree::role::end::RoleEnd>,
                >,
            )
            -> $sessionmpst_name<
                #( // K-1 + i in (K-1..0)
                    <S#N:8 as mpstthree::binary::struct_trait::Session>::Dual, // S(i) or  S(i + diff * (diff + 1))
                )0:0
                R^N:0, // R(3K-2) or R(3K-1)
                $name<mpstthree::role::end::RoleEnd>
            >
            where
                #( // i in 1..(diff * (diff + 1) + 1)
                    S#N:0: mpstthree::binary::struct_trait::Session + 'a, // S(i)
                )10:0
                #( // i in 1..(3 * K)
                    R#N:0: mpstthree::role::Role + 'a, // R(i)
                )11:0
            {
                #( // i in 1..(diff * (diff + 1))
                    let (channel_#N:3, channel_#N:4) = S#N:0::new(); // channel_(get from matrix), channel_(opposite get from matrix) = S(i)
                )4:0

                #( // i in 1..K
                    let (_, stack_#N:0) = R#N:5::new();
                )0:0
                let (stack_^N:2, _) = R^N:0::new();

                #( // i in 1..K
                    let (name_#N:0, _) = <<R#N:6 as mpstthree::role::Role>::Dual as mpstthree::role::Role>::new();
                )0:0
                let (name_^N:2, _) = <$name::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                #( // i in 1..K
                    let choice_#N:0 = $sessionmpst_name {
                            ~(
                                session#N:1 : channel_~N:5,
                            )(
                                session#N:1 : channel_~N:5,
                            )0*
                            stack: stack_#N:0,
                            name: name_#N:0,
                        };
                )0:0

                #( // i in 1..K
                    let new_session_#N:0 = mpstthree::binary::send::send(either::Either::Left(choice_#N:0), s.session#N:0);
                )0:0

                let s = $sessionmpst_name {
                    #(
                        session#N:0: new_session_#N:0,
                    )0:0
                    stack: s.stack,
                    name: s.name,
                };

                mpstthree::binary::cancel::cancel(s);

                $sessionmpst_name {
                    #(
                        session#N:0: channel_#N:7 ,
                    )0:0
                    stack: stack_^N:2,
                    name: name_^N:2,
                }
            }
        });
    }
}

/// Create the *ChooseMpst* function to send a *Choose* right branch to be used with more than 3
/// participants.
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the *ChooseMpst* type that is used
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the *next* function that is related
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
    ($func_name:ident, $type_name: ident, $role_dual:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<'a, #(S#N:0,)10:0 #(R#N:0,)11:0>(
                s: $sessionmpst_name<
                    #( // i in 1..K
                        $type_name<
                            ~( // j in 0..K
                                <S~N:2 as mpstthree::binary::struct_trait::Session>::Dual,
                            )(
                                S~N:2, // S(i + j) (with Dual if needed)
                            )0*
                            ~( // j in 0..K
                                <S~N:3 as mpstthree::binary::struct_trait::Session>::Dual,
                            )(
                                S~N:3, // S(diff * (diff + 1) / 2 + K + i + j) (with Dual if needed)
                            )0*
                            ~( // j in  0..3
                                R~N:4, // R(3 * (i - 1) + 1 + j)
                                // Side note: we lose the checking for the right order for the name on R(3 * (i - 1) + 3) → RoleADual<RoleEnd>
                            )()3*
                        >,
                    )0:0

                    $role_dual<
                        #(
                            R#N:0,
                        )13:0
                    >,

                    $name<mpstthree::role::end::RoleEnd>,
                >,
            )
            -> $sessionmpst_name<
                #( // K-1 + i in (K-1..0)
                    <S#N:9 as mpstthree::binary::struct_trait::Session>::Dual, // S(i) or  S(i + diff * (diff + 1))
                )0:0
                R^N:1, // R(3K-2) or R(3K-1)
                $name<mpstthree::role::end::RoleEnd>
            >
            where
                #( // i in 1..(diff * (diff + 1) + 1)
                    S#N:0: mpstthree::binary::struct_trait::Session + 'a, // S(i)
                )10:0
                #( // i in 1..(3 * K)
                    R#N:0: mpstthree::role::Role + 'a, // R(i)
                )11:0
            {
                #( // i in 1..(diff * (diff + 1))
                    let (channel_#N:3, channel_#N:4) = S#N:10::new(); // channel_(get from matrix), channel_(opposite get from matrix) = S(i)
                )4:0

                #( // i in 1..K
                    let (_, stack_#N:0) = R#N:11::new();
                )0:0
                let (stack_^N:2, _) = R^N:1::new();

                #( // i in 1..K
                    let (name_#N:0, _) = <<R#N:6 as mpstthree::role::Role>::Dual as mpstthree::role::Role>::new();
                )0:0
                let (name_^N:2, _) = <$name::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                #( // i in 1..K
                    let choice_#N:0 = $sessionmpst_name {
                            ~(
                                session#N:1 : channel_~N:5,
                            )(
                                session#N:1 : channel_~N:5,
                            )0*
                            stack: stack_#N:0,
                            name: name_#N:0,
                        };
                )0:0

                #( // i in 1..K
                    let new_session_#N:0 = mpstthree::binary::send::send(either::Either::Right(choice_#N:0), s.session#N:0);
                )0:0

                let s = $sessionmpst_name {
                    #(
                        session#N:0: new_session_#N:0,
                    )0:0
                    stack: s.stack,
                    name: s.name,
                };

                mpstthree::binary::cancel::cancel(s);

                $sessionmpst_name {
                    #(
                        session#N:0: channel_#N:7 ,
                    )0:0
                    stack: stack_^N:2,
                    name: name_^N:2,
                }
            }
        });
    }
}

/// Create the two *ChooseMpst* functions to send a *Choose* on each branch to be used with more
/// than 3 participants.  # Arguments
///
/// * The name of the new *ChooseMpst* function for the left branch
/// * The name of the new *ChooseMpst* function for the right branch
/// * The name of the *ChooseMpst* type that is used
/// * The name of the dual of the broadcasting sender. This one should contain *toAll* according to
///   the convention
/// * The name of the *next* function that is related
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
        $func_name_left:ident,
        $func_name_right:ident,
        $type_name:ident,
        $role_dual:ident,
        $name:ident,
        $sessionmpst_name:ident,
        $nsessions:literal
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
///  * The number of participants (all together)
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
///            3,
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
///            3,
///            3
///        );
///        close_mpst_multi(s)
///    }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_multi_to_all {
    ($session:expr, $($label:path,)+ => $($receiver:ident,)+ => $sender:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion : ($($label,)+) : ($($receiver,)+) {{
            #(
                let (channel_#N:3, channel_#N:4) = <_ as mpstthree::binary::struct_trait::Session>::new();
            )4:0

            #(
                let (stack_#N:0, _) = <_ as mpstthree::role::Role>::new();
            )15:0

            #(
                let (name_#N:0, _) = <unused#N:16::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
            )0:0

            let (name_^N:2, _) = <$sender<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

            let s = $session;

            let _ = {
                fn temp(r: &mpstthree::role::broadcast::RoleBroadcast) -> Result<(), Box<dyn std::error::Error>>
                {
                    Ok(())
                }
                temp(&s.stack)
            };

            %(
                let _ = mpstthree::binary::send::send(
                    unused#N:15($sessionmpst_name {
                        ~(
                            session#N:1 : channel_~N:7,
                        )(
                            session#N:1 : channel_~N:7,
                        )0*
                        stack: stack_#N:0,
                        name: name_#N:0,
                    }),
                    s.session#N:0,
                );
            )(
                let _ = mpstthree::binary::send::send(
                    unused#N:15($sessionmpst_name {
                        ~(
                            session#N:1 : channel_~N:7,
                        )(
                            session#N:1 : channel_~N:7,
                        )0*
                        stack: stack_#N:0,
                        name: name_#N:0,
                    }),
                    s.session#N:0,
                );
            )2*

            // mpstthree::binary::cancel::cancel(s.stack);

            // mpstthree::binary::cancel::cancel(s.name);

            $sessionmpst_name {
                #(
                    session#N:0: channel_#N:17 ,
                )0:0
                stack: stack_^N:2,
                name: name_^N:2,
            }
        }});
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
///  * The number of participants (all together)
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
///            3,
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
///            3,
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
///            3,
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
///            3,
///            3
///        );
///        close_mpst_multi(s)
///    }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_multi_cancel_to_all {
    ($session:expr, $($label:path,)+ => $($receiver:ident,)+ => $pawn: ident, $sender:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion : ($($label,)+) : ($($receiver,)+) {{

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

            let mut s = $session;

            let _ = {
                fn temp(r: &mpstthree::role::broadcast::RoleBroadcast) -> Result<(), Box<dyn std::error::Error>>
                {
                    Ok(())
                }
                temp(&s.stack)
            };

            %(
                let elt = match temp.pop() {
                    Some(e) => e,
                    _ => panic!("Error type"),
                };

                let _  = mpstthree::binary::send::send_canceled(
                    (
                        elt,
                        unused#N:21($sessionmpst_name {
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
                        unused#N:21($sessionmpst_name {
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

            // mpstthree::binary::cancel::cancel(s.stack);

            // mpstthree::binary::cancel::cancel(s.name);

            $sessionmpst_name {
                #(
                    session#N:0: channel_#N:17 ,
                )0:0
                stack: stack_^N:2,
                name: name_^N:2,
            }
        }});
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
///            3,
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
///            3,
///            3
///        );
///        close_mpst_multi(s)
///    }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_multi_http_to_all {
    ($session:expr, $($label:path,)+ => $($receiver:ident,)+ => $sender:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion : ($($label,)+) : ($($receiver,)+) {{
            #(
                let (channel_#N:3, channel_#N:4) = <_ as mpstthree::binary::struct_trait::Session>::new();
            )4:0

            #(
                let (stack_#N:0, _) = <_ as mpstthree::role::Role>::new();
            )15:0

            #(
                let (name_#N:0, _) = <unused#N:16::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
            )0:0

            let (name_^N:2, _) = <$sender<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

            let mut s = $session;

            let _ = {
                fn temp(r: &mpstthree::role::broadcast::RoleBroadcast) -> Result<(), Box<dyn std::error::Error>>
                {
                    Ok(())
                }
                temp(&s.stack)
            };

            %(
                let _ = mpstthree::binary::send::send_http(
                    unused#N:15($sessionmpst_name {
                        ~(
                            session#N:1 : channel_~N:7,
                        )(
                            session#N:1 : channel_~N:7,
                        )0*
                        stack: stack_#N:0,
                        name: name_#N:0,
                    }),
                    s.session#N:0,
                    false,
                    Request::default()
                )?;
            )(
                let _ = mpstthree::binary::send::send_http(
                    unused#N:15($sessionmpst_name {
                        ~(
                            session#N:1 : channel_~N:7,
                        )(
                            session#N:1 : channel_~N:7,
                        )0*
                        stack: stack_#N:0,
                        name: name_#N:0,
                    }),
                    s.session#N:0,
                    false,
                    Request::default()
                )?;
            )2*

            // mpstthree::binary::cancel::cancel(s);

            $sessionmpst_name {
                #(
                    session#N:0: channel_#N:17 ,
                )0:0
                stack: stack_^N:2,
                name: name_^N:2,
            }
        }});
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
/// create_fn_choose_mpst_multi_to_all_bundle!(
///     done_from_c_to_all, more_from_c_to_all, =>
///     Done, More, =>
///     EndpointDoneC, EndpointMoreC, =>
///     Branching0fromCtoA, Branching0fromCtoB, =>
///     RoleA, RoleB, =>
///     RoleC, SessionMpstThree, 3, 3
/// );
/// ```
#[macro_export]
macro_rules! create_fn_choose_mpst_multi_to_all_bundle {
    ($($fn_name:ident,)+ => $($branch:expr,)+ => $($new_type:ty,)+ => $($label:path,)+ => $($receiver:ident,)+ => $sender:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        create_fn_choose_mpst_multi_to_all_bundle!(@call_tuple $($fn_name,)+ => $($branch,)+ => $($new_type,)+ => ($($label,)+) => ($($receiver,)+) => $sender, $sessionmpst_name, $nsessions, $exclusion);
    };
    (@call_tuple $($fn_name:ident,)+ => $($branch:expr,)+ => $($new_type:ty,)+ => $label:tt => $receiver:tt => $sender:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        $(create_fn_choose_mpst_multi_to_all_bundle!(@call $fn_name, $branch, $new_type => $label => $receiver => $sender, $sessionmpst_name, $nsessions, $exclusion);)+
    };
    (@call $fn_name:ident, $branch:expr, $new_type:ty => ($($label:path,)+) => ($($receiver:ident,)+) => $sender:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion : ($($label,)+) : ($($receiver,)+) {
            fn $fn_name(
                s: $sessionmpst_name<
                    #(
                        Send<unused#N:15, mpstthree::binary::struct_trait::End>,
                    )0:0
                    mpstthree::role::broadcast::RoleBroadcast,
                    $sender<mpstthree::role::end::RoleEnd>,
                >
            ) -> $new_type
            {
                #(
                    let (channel_#N:3, channel_#N:4) = <_ as mpstthree::binary::struct_trait::Session>::new();
                )4:0

                #(
                    let (stack_#N:0, _) = <_ as mpstthree::role::Role>::new();
                )15:0

                #(
                    let (name_#N:0, _) = <unused#N:16::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                )0:0

                let (name_^N:2, _) = <$sender<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                #(
                    let s = {
                        let new_session = mpstthree::binary::send::send(
                            unused#N:15::$branch($sessionmpst_name {
                                ~(
                                    session#N:1 : channel_~N:7,
                                )(
                                    session#N:1 : channel_~N:7,
                                )0*
                                stack: stack_#N:0,
                                name: name_#N:0,
                            }),
                            s.session#N:0
                        );

                        $sessionmpst_name {
                            ~(
                                session~N:8: new_session,
                            )(
                                session~N:8: s.session~N:8,
                            )5*
                            stack: s.stack,
                            name: s.name,
                        }
                    };
                )0:0

                mpstthree::binary::cancel::cancel(s);

                $sessionmpst_name {
                    #(
                        session#N:0: channel_#N:17 ,
                    )0:0
                    stack: stack_^N:2,
                    name: name_^N:2,
                }
            }
        });
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
    ($($fn_name:ident,)+ => $($branch:expr,)+ => $($new_type:ty,)+ => $($label:path,)+ => $($receiver:ident,)+ => $pawn:ident, $sender:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        create_fn_choose_mpst_cancel_multi_to_all_bundle!(@call_tuple $($fn_name,)+ => $($branch,)+ => $($new_type,)+ => ($($label,)+) => ($($receiver,)+) => $pawn, $sender, $sessionmpst_name, $nsessions, $exclusion);
    };
    (@call_tuple $($fn_name:ident,)+ => $($branch:expr,)+ => $($new_type:ty,)+ => $label:tt => $receiver:tt => $pawn:ident, $sender:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        $(create_fn_choose_mpst_cancel_multi_to_all_bundle!(@call $fn_name, $branch, $new_type => $label => $receiver => $pawn, $sender, $sessionmpst_name, $nsessions, $exclusion);)+
    };
    (@call $fn_name:ident, $branch:expr, $new_type:ty => ($($label:path,)+) => ($($receiver:ident,)+) => $pawn:ident, $sender:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
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

                // mpstthree::binary::cancel::cancel(s.stack);

                // mpstthree::binary::cancel::cancel(s.name);

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
