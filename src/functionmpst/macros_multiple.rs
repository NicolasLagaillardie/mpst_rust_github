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
                    %(
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
                %(
                )(
                    let new_session = mpstthree::binary::send(x, s.session#N:0);
                )0*
                let new_queue = $next(s.stack);

                $struct_name {
                    %(
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
                    %(
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
                %(
                )(
                    let (v, new_session) = mpstthree::binary::recv(s.session#N:0)?;
                )0*
                let new_queue = $next(s.stack);

                let result = $struct_name {
                    %(
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
                    %(
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
                %(
                )(
                    let (v, new_session) = mpstthree::binary::recv(s.session#N:0)?;
                )0*
                let (new_queue, _) = $next(s.stack);
                let result = $struct_name {
                    %(
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
                    %(
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

/// Get a mpst offer
#[macro_export]
macro_rules! offer_mpst {
    ($session:expr, $recv_mpst:ident, { $($pat:pat => $result:block, )* }) => {
        (move || -> Result<_, _> {
            let (l, s) = $recv_mpst($session)?;
            mpstthree::binary::cancel(s);
            match l {
                $(
                    $pat => { $result },
                )*
            }
        })()
    };
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

/// TODO
#[macro_export]
macro_rules! create_choose_mpst_session_multi {
    ($func_name:ident, $type_name: ident, $role:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion {

            // For left branch

            fn $func_name<'a, #(S#N:0,)4:0 #(R#N:0,)3:0>(
                s: $sessionmpst_name<



                    #( // i in 1..K Done
                        $type_name<
                            ~( // j in 0..K Done
                                S~N:2, // S(i + j) (with Dual if needed) Done
                            )(
                                <S~N:2 as Session>::Dual, Done
                            )1*
                            ~( // j in 0..K
                                S~N:3, // S(diff * (diff + 1) / 2 + K + i + j) (with Dual if needed) Done
                            )(
                                <S~N:3 as Session>::Dual, Done
                            )1*
                            ~( // j in  0..3 Done
                                R~N:4, // R(3 * (i - 1) + 1 + j) Done
                                // Side note: we lose the checking for the right order for the name on R(3 * (i - 1) + 3) → RoleADual<RoleEnd>
                            )()2*
                        >,
                    )0:0


                    $role<
                        R#N:3, // R(3K-2) Done
                        R#N:4, // R(3K-1) Done
                    >,

                    $sender<mpstthree::role::end::RoleEnd>,
                >,
            ) -> $sessionmpst_name<
                #( // i in K-1..0 Done
                    S#N:0, // S(i) or  S(i + diff * (diff + 1)) Done
                )12:0
                R#N:3, // R(3K-2) or R(3K-1) Done
                $sender<mpstthree::role::end::RoleEnd>
            >
            where
                #( // i in 1..(diff * (diff + 1) + 1) Done
                    S#N:0: mpstthree::binary::Session + 'a, // S(i) Done
                )10:0
                #( // i in 1..(3 * K) Done
                    R#N:0: mpstthree::role::Role + 'a, // R(i) Done
                )11:0
            {


                #( // i in 1..(diff * (diff + 1)) Done
                    let (channel_#N:5, channel_#N:6) = S#N:0::new(); // channel_(get from matrix), channel_(opposite get from matrix) = S(i) Done
                )4:0

                #( // i in 1..K
                    let (_, role_#N:0) = R#N:7::new();
                )
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

/// Choose, for C, between many different sessions wrapped in an `enum`
/// TODO
#[macro_export]
macro_rules! choose_mpst_X_to_all {
    ($session:expr, $labelone:path, $labeltwo:path) => {{
        let (session_ac, session_ca) = <_ as Session>::new();
        let (session_bc, session_cb) = <_ as Session>::new();
        let (session_ab, session_ba) = <_ as Session>::new();
        let (queue_a, _) = <_ as Role>::new();
        let (queue_b, _) = <_ as Role>::new();
        let (queue_c, _) = <_ as Role>::new();
        let (name_a, _) = RoleA::<RoleEnd>::new();
        let (name_b, _) = RoleB::<RoleEnd>::new();
        let (name_c, _) = RoleC::<RoleEnd>::new();

        let s = send_mpst_c_to_a(
            $labelone(SessionMpst {
                session1: session_ab,
                session2: session_ac,
                stack: queue_a,
                name: name_a,
            }),
            $session,
        );
        let s = send_mpst_c_to_b(
            $labeltwo(SessionMpst {
                session1: session_ba,
                session2: session_bc,
                stack: queue_b,
                name: name_b,
            }),
            s,
        );

        cancel(s);

        SessionMpst {
            session1: session_ca,
            session2: session_cb,
            stack: queue_c,
            name: name_c,
        }
    }};
}

////////////////////////////////////////////
/// FORK

#[macro_export]
macro_rules! fork_simple_multi {
    ($func_name: ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(K in 1..$nsessions {
            fn fork_simple_multi<#(S#K:0,)0:0 R, N, P>(p: P, s: $struct_name<#(S#K:0,)0:0 R, N>) -> std::thread::JoinHandle<()>
            where
                #(
                    S#K:0: mpstthree::binary::Session + 'static,
                )0:0
                R: mpstthree::role::Role + 'static,
                N: mpstthree::role::Role + 'static,
                P: FnOnce($struct_name<#(S#K:0,)0:0 R, N>) -> Result<(), Box<dyn std::error::Error>> + std::marker::Send + 'static,
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

/// TODO
#[macro_export]
macro_rules! fork_mpst_multi {
    ($func_name: ident, $fork_function: ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(K in 1..=$nsessions {
            fn $func_name<#(S#K:0,)0:0 #(R#K:0,)0:0 #(N#K:0,)0:0 #(F#K:0,)0:0>(
                #(
                    f#K:0: F#K:0,
                )0:0
            ) -> (
                #(
                    Result<(), Box<(dyn std::any::Any + std::marker::Send + 'static)>>,
                )0:0
            )
            where
                #(
                    S#K:0: mpstthree::binary::Session + 'static,
                    R#K:0: mpstthree::role::Role + 'static,
                    N#K:0: mpstthree::role::Role + 'static,
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
                    let (role_#K:0, _) = R#K:0::new();
                    let (name_#K:0, _) = N#K:0::new();
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
                    let thread_#K:0 = $fork_function(f#K:0, sessionmpst_#K:0);
                )0:0

                (
                    #(
                        thread_#K:0.join(),
                    )0:0
                )
            }
        });
    }
}
