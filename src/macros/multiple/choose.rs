////////////////////////////////////////////
/// CHOICE

///  Create the *ChooseMpst* type to be used with more than 3 participants.
///  
///  # Arguments
///  
///  * The name of the new *ChooseMpst* type
///  * The *SessionMpst* type that will be used
///  * The number of participants (all together)
///  
///  # Example
///  
///  ```
///  use mpstthree::{create_sessionmpst, create_choose_type_multi};
///
///  create_sessionmpst!(SessionMpst, 3);
///  create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3);
///  ```
#[macro_export]
macro_rules! create_choose_type_multi {
    ($type_name: ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            type $type_name<
                #(S#N:0,)2:0 R0, R1, N0
            > = mpstthree::binary::Send<
                either::Either<
                    <
                        $sessionmpst_name<#(S#N:0,)0:0 R0, N0
                    > as mpstthree::binary::Session>::Dual,
                    <
                        $sessionmpst_name<#(S#N:0,)3:0 R1, N0
                    > as mpstthree::binary::Session>::Dual
                    >,
                mpstthree::binary::End
            >;
        });
    }
}

///  Create the *ChooseMpst* function to send a *Choose* left branch to be used with more than 3 participants.
///  
///  # Arguments
///  
///  * The name of the new *ChooseMpst* function
///  * The name of the *ChooseMpst* type that is used
///  * The name of the broadcasting sender. This one should contain *toAll* according to the convention
///  * The name of the *next* function that is related
///  * The name of the sender
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_broadcast_role, create_sessionmpst, create_choose_type_multi, create_choose_mpst_session_multi_left};
///
///  create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
///  create_broadcast_role!(RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///  create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3);
///
/// create_choose_mpst_session_multi_left!(
///     choose_left_mpst_session_d_to_all,
///     ChooseMpstThree,
///     RoleDtoAll,
///     next_d_to_all,
///     RoleD,
///     SessionMpst,
///     3
/// );
///  ```
#[macro_export]
macro_rules! create_choose_mpst_session_multi_left {
    ($func_name:ident, $type_name: ident, $role_dual:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<'a, #(S#N:0,)10:0 #(R#N:0,)11:0>(
                s: $sessionmpst_name<
                    #( // i in 1..K
                        $type_name<
                            ~( // j in 0..K
                                <S~N:2 as mpstthree::binary::Session>::Dual,
                            )(
                                S~N:2, // S(i + j) (with Dual if needed)
                            )0*
                            ~( // j in 0..K
                                <S~N:3 as mpstthree::binary::Session>::Dual,
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
                    <S#N:8 as mpstthree::binary::Session>::Dual, // S(i) or  S(i + diff * (diff + 1))
                )0:0
                R^N:0, // R(3K-2) or R(3K-1)
                $name<mpstthree::role::end::RoleEnd>
            >
            where
                #( // i in 1..(diff * (diff + 1) + 1)
                    S#N:0: mpstthree::binary::Session + 'a, // S(i)
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
                    let (name_#N:0, _) = <R#N:6 as mpstthree::role::Role>::Dual::new();
                )0:0
                let (name_^N:2, _) = $name::<mpstthree::role::end::RoleEnd>::new();

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
                    let new_session_#N:0 = mpstthree::binary::send(either::Either::Left(choice_#N:0), s.session#N:0);
                )0:0
                let (_, new_queue) = $recv_func(s.stack);

                let s = $sessionmpst_name {
                    #(
                        session#N:0: new_session_#N:0,
                    )0:0
                    stack: new_queue,
                    name: s.name,
                };

                mpstthree::binary::cancel(s);

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

///  Create the *ChooseMpst* function to send a *Choose* right branch to be used with more than 3 participants.
///  
///  # Arguments
///  
///  * The name of the new *ChooseMpst* function
///  * The name of the *ChooseMpst* type that is used
///  * The name of the broadcasting sender. This one should contain *toAll* according to the convention
///  * The name of the *next* function that is related
///  * The name of the sender
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_broadcast_role, create_sessionmpst, create_choose_type_multi, create_choose_mpst_session_multi_right};
///
///  create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
///  create_broadcast_role!(RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///  create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3);
///
/// create_choose_mpst_session_multi_right!(
///     choose_right_mpst_session_d_to_all,
///     ChooseMpstThree,
///     RoleDtoAll,
///     next_d_to_all,
///     RoleD,
///     SessionMpst,
///     3
/// );
///  ```
#[macro_export]
macro_rules! create_choose_mpst_session_multi_right {
    ($func_name:ident, $type_name: ident, $role_dual:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<'a, #(S#N:0,)10:0 #(R#N:0,)11:0>(
                s: $sessionmpst_name<
                    #( // i in 1..K
                        $type_name<
                            ~( // j in 0..K
                                <S~N:2 as mpstthree::binary::Session>::Dual,
                            )(
                                S~N:2, // S(i + j) (with Dual if needed)
                            )0*
                            ~( // j in 0..K
                                <S~N:3 as mpstthree::binary::Session>::Dual,
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
                    <S#N:9 as mpstthree::binary::Session>::Dual, // S(i) or  S(i + diff * (diff + 1))
                )0:0
                R^N:1, // R(3K-2) or R(3K-1)
                $name<mpstthree::role::end::RoleEnd>
            >
            where
                #( // i in 1..(diff * (diff + 1) + 1)
                    S#N:0: mpstthree::binary::Session + 'a, // S(i)
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
                    let (name_#N:0, _) = <R#N:6 as mpstthree::role::Role>::Dual::new();
                )0:0
                let (name_^N:2, _) = $name::<mpstthree::role::end::RoleEnd>::new();

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
                    let new_session_#N:0 = mpstthree::binary::send(either::Either::Right(choice_#N:0), s.session#N:0);
                )0:0
                let (_, new_queue) = $recv_func(s.stack);

                let s = $sessionmpst_name {
                    #(
                        session#N:0: new_session_#N:0,
                    )0:0
                    stack: new_queue,
                    name: s.name,
                };

                mpstthree::binary::cancel(s);

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

///  Create the two *ChooseMpst* functions to send a *Choose* on each branch to be used with more than 3 participants.
///  
///  # Arguments
///  
///  * The name of the new *ChooseMpst* function for the left branch
///  * The name of the new *ChooseMpst* function for the right branch
///  * The name of the *ChooseMpst* type that is used
///  * The name of the dual of the broadcasting sender. This one should contain *toAll* according to the convention
///  * The name of the *next* function that is related
///  * The name of the sender
///  * The name of the *SessionMpst* type that will be used
///  * The number of participants (all together)
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_broadcast_role, create_sessionmpst, create_choose_type_multi, create_choose_mpst_session_multi_both};
///
///  create_normal_role!(RoleD, next_d, RoleDDual, next_d_dual);
///  create_broadcast_role!(RoleAlltoD, next_all_to_d, RoleDtoAll, next_d_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///  create_choose_type_multi!(ChooseMpstThree, SessionMpst, 3);
///
/// create_choose_mpst_session_multi_both!(
///     choose_left_mpst_session_d_to_all,
///     choose_right_mpst_session_d_to_all,
///     ChooseMpstThree,
///     RoleDtoAll,
///     next_d_to_all,
///     RoleD,
///     SessionMpst,
///     3
/// );
///  ```
#[macro_export]
macro_rules! create_choose_mpst_session_multi_both {
    ($func_name_left:ident, $func_name_right:ident, $type_name: ident, $role_dual:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpstthree::create_choose_mpst_session_multi_left!(
            $func_name_left,
            $type_name,
            $role_dual,
            $recv_func,
            $name,
            $sessionmpst_name,
            $nsessions
        );

        mpstthree::create_choose_mpst_session_multi_right!(
            $func_name_right,
            $type_name,
            $role_dual,
            $recv_func,
            $name,
            $sessionmpst_name,
            $nsessions
        );
    };
}

///  Choose among different sessions that are provided, for protocols with more than 3 participants
///  
///  # Arguments
///  
///   * The session to be used
///   * The different send functions to broadcast the choice
///   * The different `enum` variants which represent the different branches to be sent to each passive role
///   * The different passive roles
///   * The name of the sender
///   * The name of the *SessionMpst* type that will be used
///   * The number of participants (all together)
///  
///  # Example
///
/// Available on the *13_macro_multi_recursion* test.
///  
///  ```ignore
/// match xs.pop() {
///     Option::Some(_) => {
///         let s = choose_mpst_X_to_all!(
///             s,
///             send_mpst_d_to_a,
///             send_mpst_d_to_b, =>
///             CBranchesAtoC::Video,
///             CBranchesBtoC::Video, =>
///             RoleA,
///             RoleB, =>
///             RoleD,
///             SessionMpst,
///             3
///         );
///         let s = send_mpst_d_to_a(1, s);
///         let (_, s) = recv_mpst_d_to_a(s)?;
///         client_recurs(s, xs, index + 1)
///     }
///     Option::None => {
///         let s = choose_mpst_X_to_all!(
///             s,
///             send_mpst_d_to_a,
///             send_mpst_d_to_b, =>
///             CBranchesAtoC::End,
///             CBranchesBtoC::End, =>
///             RoleA,
///             RoleB, =>
///             RoleD,
///             SessionMpst,
///             3
///         );
///         close_mpst_multi(s)?;
///         Ok(())
///     }
/// }
/// ```
#[macro_export]
macro_rules! choose_mpst_X_to_all {
    ($session:expr, $($fn_send:ident,)+ => $($label:path,)+ => $($receiver:ident,)+ => $sender:ident, $sessionmpst_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! 1 : ($($fn_send$args,)+) : ($($label,)+) : ($($receiver,)+) {{

            #(
                let (channel_#N:3, channel_#N:4) = <_ as mpstthree::binary::Session>::new();
            )4:0

            #(
                let (stack_#N:0, _) = <_ as mpstthree::role::Role>::new();
            )15:0

            #(
                let (name_#N:0, _) = useless#N:16::<mpstthree::role::end::RoleEnd>::new();
            )0:0

            let (name_^N:2, _) = <$sender<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

            %(
                let s = useless#N:14(
                    useless#N:15($sessionmpst_name {
                        ~(
                            session#N:1 : channel_~N:5,
                        )(
                            session#N:1 : channel_~N:5,
                        )0*
                        stack: stack_#N:0,
                        name: name_#N:0,
                    }),
                    s,
                );
            )(
                let s = useless#N:14(
                    useless#N:15($sessionmpst_name {
                        ~(
                            session#N:1 : channel_~N:5,
                        )(
                            session#N:1 : channel_~N:5,
                        )0*
                        stack: stack_#N:0,
                        name: name_#N:0,
                    }),
                    $session,
                );
            )0*

            mpstthree::binary::cancel(s);

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
