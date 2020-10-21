////////////////////////////////////////////
/// CLOSE

#[macro_export]
macro_rules! close_mpst {
    ($func_name:ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<R>(s: $struct_name<#(mpstthree::binary::End,)0:0 mpstthree::role::end::RoleEnd, R>) -> Result<(), Box<dyn Error>>
            where
                R: mpstthree::role::Role,
            {
                #(
                    s.session#N:0.sender.send(()).unwrap_or(());
                )0:0

                #(
                    s.session#N:0.receiver.recv()?;
                )0:0

                Ok(())
            }
        });
    }
}

////////////////////////////////////////////
/// SEND

// create a function send_mpst for the excluded session
#[macro_export]
macro_rules! create_send_mpst_session {
    ($func_name:ident, $role:ident, $next:ident, $name:ident, $struct_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<T, #(S#N:0,)0:0 R>(
                x: T,
                s: $struct_name<
                    ~(
                        S#N:0,
                    )(
                        mpstthree::binary::Send<T, S#N:0>,
                    )0*
                    $role<R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> $struct_name<#(S#N:0,)0:0 R, $name<mpstthree::role::end::RoleEnd>>
            where
                T: std::marker::Send,
                #(
                    S#N:0: mpstthree::binary::Session,
                )0:0
                R: mpstthree::role::Role,
            {
                ~(
                )(
                    let new_session = mpstthree::binary::send(x, s.session#N:0);
                )0*
                let new_queue = $next(s.stack);

                $struct_name {
                    ~(
                        session#N:0: s.session#N:0,
                    )(
                        session#N:0: new_session,
                    )0*
                    stack: new_queue,
                    name: s.name,
                }
            }
        });
    }
}

////////////////////////////////////////////
/// RECV

// create a function recv_mpst for the excluded session
#[macro_export]
macro_rules! create_recv_mpst_session {
    ($func_name:ident, $role:ident, $next:ident, $name:ident, $struct_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<T, #(S#N:0,)0:0 R>(
                s: $struct_name<
                    ~(
                        S#N:0,
                    )(
                        mpstthree::binary::Recv<T, S#N:0>,
                    )0*
                    $role<R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                (
                    T,
                    $struct_name<#(S#N:0,)0:0 R, $name<mpstthree::role::end::RoleEnd>,
                    >,
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    S#N:0: mpstthree::binary::Session,
                )0:0
                R: mpstthree::role::Role,
            {
                ~(
                )(
                    let (v, new_session) = mpstthree::binary::recv(s.session#N:0)?;
                )0*
                let new_queue = $next(s.stack);

                let result = $struct_name {
                    ~(
                        session#N:0: s.session#N:0,
                    )(
                        session#N:0: new_session,
                    )0*
                    stack: new_queue,
                    name: s.name,
                };

                Ok((v, result))
            }
        });
    }
}

// create a function recv_mpst_all for the excluded session
#[macro_export]
macro_rules! create_recv_mpst_all_session {
    ($func_name:ident, $role:ident, $next:ident, $name:ident, $struct_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<T, #(S#N:0,)0:0 R>(
                s: $struct_name<
                    ~(
                        S#N:0,
                    )(
                        mpstthree::binary::Recv<T, S#N:0>,
                    )0*
                    $role<R, R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                (
                    T,
                    $struct_name<
                        #(S#N:0,)0:0
                        R,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    S#N:0: mpstthree::binary::Session,
                )0:0
                R: mpstthree::role::Role,
            {
                ~(
                )(
                    let (v, new_session) = mpstthree::binary::recv(s.session#N:0)?;
                )0*
                let (new_queue, _) = $next(s.stack);
                let result = $struct_name {
                    ~(
                        session#N:0: s.session#N:0,
                    )(
                        session#N:0: new_session,
                    )0*
                    stack: new_queue,
                    name: s.name,
                };

                Ok((v, result))
            }
        });
    }
}

////////////////////////////////////////////
/// OFFER

#[macro_export]
macro_rules! create_offer_type_multi {
    ($type_name: ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            type $type_name<#(S#N:0,)2:0 R0, R1, N0> = mpstthree::binary::Recv<either::Either<$sessionmpst_name<#(S#N:0,)0:0 R0, N0>, $sessionmpst_name<#(S#N:0,)3:0 R1, N0>>, End>;
        });
    }
}

/// Get an offer on $exclusion
#[macro_export]
macro_rules! create_offer_mpst_session_multi {
    ($func_name:ident, $type_name: ident, $role:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {
            fn $func_name<'a, #(S#N:0,)2:0 F, G, R1, R2, U>(
                s: $sessionmpst_name<
                    ~(
                        mpstthree::binary::End,
                    )(
                        $type_name<#(S#N:0,)2:0 R1, R2, $name<mpstthree::role::end::RoleEnd>>,
                    )0*
                    $role<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
                f: F,
                g: G,
            ) -> Result<U, Box<dyn std::error::Error + 'a>>
            where
                #(S#N:0: mpstthree::binary::Session,)2:0
                #(
                    R#N:0: mpstthree::role::Role,
                )0:0
                F: FnOnce(
                    $sessionmpst_name<
                        #(S#N:0,)0:0
                        R1,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                G: FnOnce(
                    $sessionmpst_name<
                        #(S#N:0,)0:2
                        R2,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
            {
                let (e, s) = $recv_func(s)?;
                mpstthree::binary::cancel(s);
                e.either(f, g)
            }
        });
    }
}

////////////////////////////////////////////
/// CHOICE

#[macro_export]
macro_rules! create_choose_type_multi {
    ($type_name: ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            type $type_name<
                #(S#N:0,)2:0 R0, R1, N0
            > = mpstthree::binary::Send<
                either::Either<
                    <
                        $sessionmpst_name<#(S#N:0,)0:0 R0, N0
                    > as Session>::Dual,
                    <
                        $sessionmpst_name<#(S#N:0,)3:0 R1, N0
                    > as Session>::Dual
                    >,
                End
            >;
        });
    }
}

#[macro_export]
macro_rules! create_choose_mpst_session_multi {
    ($func_name:ident, $type_name: ident, $role:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {



            fn $func_name<'a, #(S#N:0,)4:0 #(R#N:0,)3:0>(
                s: $sessionmpst_name<



                    #(
                        $type_name<
                            #(
                                S#N:0,
                            )0:0
                            #(
                                S#N:0,
                            )0:0
                            #(
                                R#N:0,
                            )6:0
                        >,
                    )0:0


                    $role<#(
                        R#N:0, // Done
                    )8:0>,

                    $sender<mpstthree::role::end::RoleEnd>,
                >,
            ) -> $sessionmpst_name<S1, S2, R6, $sender<mpstthree::role::end::RoleEnd>>
            where
                #(
                    S#N:0: mpstthree::binary::Session + 'a, // Done
                )4:0
                #(
                    R#N:0: mpstthree::role::Role + 'a, // Done
                )9:0
            {




                let (session_3_1, session_1_3) = S2::new();
                let (session_3_2, session_2_3) = S3::new();
                let (session_2_1, session_1_2) = S0::new();

                let (_, role_1) = R0::new();
                let (_, role_2) = R2::new();
                let (role_3, _) = R4::new();

                let (name_1, _) =
                    <$dual_1<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::Dual::new();
                let (name_2, _) =
                    <$dual_2<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::Dual::new();
                let (name_3, _) = $sender::<mpstthree::role::end::RoleEnd>::new();

                let choice_1 = $type_name {
                    session1: session_1_2,
                    session2: session_1_3,
                    stack: role_1,
                    name: name_1,
                };

                let choice_2 = $type_name {
                    session1: session_2_1,
                    session2: session_2_3,
                    stack: role_2,
                    name: name_2,
                };

                let new_session_1 = mpstthree::binary::send(either::Either::Left(choice_1), s.session1);
                let new_session_2 = mpstthree::binary::send(either::Either::Left(choice_2), s.session2);
                let (_, new_queue) = $next(s.stack);

                let s = $type_name {
                    session1: new_session_1,
                    session2: new_session_2,
                    stack: new_queue,
                    name: s.name,
                };

                mpstthree::binary::cancel(s);

                $type_name {
                    session1: session_3_1,
                    session2: session_3_2,
                    stack: role_3,
                    name: name_3,
                }
            }



        });
    }
}

////////////////////////////////////////////
/// FORK

#[macro_export]
macro_rules! fork_simple_multi {
    ($func_name: ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(K in 1..$nsessions {
            fn fork_simple_multi<#(S#K,)0:0 R, N, P>(p: P, s: $struct_name<#(S#K,)0:0 R, N>) -> std::thread::JoinHandle<()>
            where
                #(
                    S#K: mpstthree::binary::Session + 'static,
                )0:0
                R: mpstthree::role::Role + 'static,
                N: mpstthree::role::Role + 'static,
                P: FnOnce($struct_name<#(S#K,)0:0 R, N>) -> Result<(), Box<dyn std::error::Error>> + std::marker::Send + 'static,
            {
                std::thread::spawn(move || {
                    std::panic::set_hook(Box::new(|_info| {
                        // do nothing
                    }));
                    match p(s) {
                        Ok(()) => (),
                        Err(e) => panic!("{:?}", e),
                    }
                })
            }
        });
    }
}

#[macro_export]
macro_rules! fork_mpst_multi {
    ($func_name: ident, $fork_function: ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(K in 1..=$nsessions {
            fn $func_name<#(S#K,)0:0 #(R#K,)0:0 #(N#K,)0:0 #(F#K,)0:0>(
                #(
                    f#K: F#K,
                )0:0
            ) -> (
                #(
                    Result<(), Box<(dyn std::any::Any + std::marker::Send + 'static)>>,
                )0:0
            )
            where
                #(
                    S#K: mpstthree::binary::Session + 'static,
                    R#K: mpstthree::role::Role + 'static,
                    N#K: mpstthree::role::Role + 'static,
                )0:0

                // ^(
                //     F#N:0: FnOnce($struct_name<^(S+N)(<S+N as mpstthree::binary::Session>::Dual)* R1, N1>) -> Result<(), Box<dyn std::error::Error>>
                //     + std::marker::Send
                //     + 'static,
                // )(
                //     F#N:0: FnOnce($struct_name<^(S+N)(<S+N as mpstthree::binary::Session>::Dual)* R1, N1>) -> Result<(), Box<dyn std::error::Error>>
                //     + std::marker::Send
                //     + 'static,
                // )~

                F1: FnOnce($struct_name<S1, S2, R1, N1>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                F2: FnOnce($struct_name<<S1 as mpstthree::binary::Session>::Dual, S3, R2, N2>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                F3: FnOnce($struct_name<<S2 as mpstthree::binary::Session>::Dual, <S3 as mpstthree::binary::Session>::Dual, R3, N3>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
            {

                // #(
                //     let ( channel|N, channel%N ) = S#N:0::new();
                // )0:0

                let (channel1_2, channel2_1) = S1::new();
                let (channel1_3, channel3_1) = S2::new();
                let (channel2_3, channel3_2) = S3::new();

                #(
                    let (role_#K, _) = R#K::new();
                    let (name_#K, _) = N#K::new();
                )0:0

                let sessionmpst_1 = $struct_name {
                    session1: channel1_2,
                    session2: channel1_3,
                    stack: role_1,
                    name: name_1,
                };
                let sessionmpst_2 = $struct_name {
                    session1: channel2_1,
                    session2: channel2_3,
                    stack: role_2,
                    name: name_2,
                };
                let sessionmpst_3 = $struct_name {
                    session1: channel3_1,
                    session2: channel3_2,
                    stack: role_3,
                    name: name_3,
                };

                #(
                    let thread_#K = $fork_function(f#K, sessionmpst_#K);
                )0:0

                (
                    #(
                        thread_#K.join(),
                    )0:0
                )
            }
        });
    }
}
