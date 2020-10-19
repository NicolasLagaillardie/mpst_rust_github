////////////////////////////////////////////
/// CLOSE

#[macro_export]
macro_rules! close_mpst {
    ($func_name:ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<R>(s: $struct_name<#(mpstthree::binary::End,)0* mpstthree::role::end::RoleEnd, R>) -> Result<(), Box<dyn Error>>
            where
                R: mpstthree::role::Role,
            {
                #(
                    s.session#N.sender.send(()).unwrap_or(());
                )0*

                #(
                    s.session#N.receiver.recv()?;
                )0*

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
            fn $func_name<T, #(S#N,)0* R>(
                x: T,
                s: $struct_name<
                    ~(
                        S#N,
                    )(
                        mpstthree::binary::Send<T, S#N>,
                    )*
                    $role<R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> $struct_name<#(S#N,)0* R, $name<mpstthree::role::end::RoleEnd>>
            where
                T: std::marker::Send,
                #(
                    S#N: mpstthree::binary::Session,
                )0*
                R: mpstthree::role::Role,
            {
                ~(
                )(
                    let new_session = mpstthree::binary::send(x, s.session#N);
                )*
                let new_queue = $next(s.stack);

                $struct_name {
                    ~(
                        session#N: s.session#N,
                    )(
                        session#N: new_session,
                    )*
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
            fn $func_name<T, #(S#N,)0* R>(
                s: $struct_name<
                    ~(
                        S#N,
                    )(
                        mpstthree::binary::Recv<T, S#N>,
                    )*
                    $role<R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                (
                    T,
                    $struct_name<#(S#N,)0* R, $name<mpstthree::role::end::RoleEnd>,
                    >,
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    S#N: mpstthree::binary::Session,
                )0*
                R: mpstthree::role::Role,
            {
                ~(
                )(
                    let (v, new_session) = mpstthree::binary::recv(s.session#N)?;
                )*
                let new_queue = $next(s.stack);

                let result = $struct_name {
                    ~(
                        session#N: s.session#N,
                    )(
                        session#N: new_session,
                    )*
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
            fn $func_name<T, #(S#N,)0* R>(
                s: $struct_name<
                    ~(
                        S#N,
                    )(
                        mpstthree::binary::Recv<T, S#N>,
                    )*
                    $role<R, R>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            ) -> Result<
                (
                    T,
                    $struct_name<
                        #(S#N,)0*
                        R,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    S#N: mpstthree::binary::Session,
                )0*
                R: mpstthree::role::Role,
            {
                ~(
                )(
                    let (v, new_session) = mpstthree::binary::recv(s.session#N)?;
                )*
                let (new_queue, _) = $next(s.stack);
                let result = $struct_name {
                    ~(
                        session#N: s.session#N,
                    )(
                        session#N: new_session,
                    )*
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
            type $type_name<#(S#N,)0^ R0, R1, N0> = mpstthree::binary::Recv<either::Either<$sessionmpst_name<#(S#N,)0* R0, N0>, $sessionmpst_name<#(S#N,)0+ R1, N0>>, End>;
        });
    }
}

/// Get an offer on $eclusion
#[macro_export]
macro_rules! create_offer_mpst_session_multi {
    ($func_name:ident, $type_name: ident, $role:ident, $recv_func:ident, $name:ident, $sessionmpst_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! $exclusion { fn $func_name<'a, #(S#N,)0^ F, G, R1, R2, U>(
                s: $sessionmpst_name<
                    ~(
                        mpstthree::binary::End,
                    )(
                        $type_name<#(S#N,)0^ R1, R2, $name<mpstthree::role::end::RoleEnd>>,
                    )*
                    $role<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
                f: F,
                g: G,
            ) -> Result<U, Box<dyn std::error::Error + 'a>>
            where
                #(S#N: mpstthree::binary::Session,)0^
                #(
                    R#N: mpstthree::role::Role,
                )0*
                F: FnOnce(
                    $sessionmpst_name<
                        #(S#N,)0*
                        R1,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                G: FnOnce(
                    $sessionmpst_name<
                        #(S#N,)2*
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
                #(S#N,)0^ R0, R1, N0
            > = mpstthree::binary::Send<
                either::Either<
                    <
                        $sessionmpst_name<#(S#N,)0* R0, N0
                    > as Session>::Dual,
                    <
                        $sessionmpst_name<#(S#N,)0+ R1, N0
                    > as Session>::Dual
                    >,
                End
            >;
        });
    }
}

////////////////////////////////////////////
/// FORK

#[macro_export]
macro_rules! fork_simple_multi {
    ($func_name: ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(K in 1..$nsessions {
            fn fork_simple_multi<#(S#K,)0* R, N, P>(p: P, s: $struct_name<#(S#K,)0* R, N>) -> std::thread::JoinHandle<()>
            where
                #(
                    S#K: mpstthree::binary::Session + 'static,
                )0*
                R: mpstthree::role::Role + 'static,
                N: mpstthree::role::Role + 'static,
                P: FnOnce($struct_name<#(S#K,)0* R, N>) -> Result<(), Box<dyn std::error::Error>> + std::marker::Send + 'static,
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
            fn $func_name<#(S#K,)0* #(R#K,)0* #(N#K,)0* #(F#K,)0*>(
                #(
                    f#K: F#K,
                )0*
            ) -> (
                #(
                    Result<(), Box<(dyn std::any::Any + std::marker::Send + 'static)>>,
                )0*
            )
            where
                #(
                    S#K: mpstthree::binary::Session + 'static,
                    R#K: mpstthree::role::Role + 'static,
                    N#K: mpstthree::role::Role + 'static,
                )0*

                // ^(
                //     F#N: FnOnce($struct_name<^(S+N)(<S+N as mpstthree::binary::Session>::Dual)* R1, N1>) -> Result<(), Box<dyn std::error::Error>>
                //     + std::marker::Send
                //     + 'static,
                // )(
                //     F#N: FnOnce($struct_name<^(S+N)(<S+N as mpstthree::binary::Session>::Dual)* R1, N1>) -> Result<(), Box<dyn std::error::Error>>
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
                //     let ( channel|N, channel%N ) = S#N::new();
                // )0*

                let (channel_1_2, channel_2_1) = S1::new();
                let (channel_1_3, channel_3_1) = S2::new();
                let (channel_2_3, channel_3_2) = S3::new();

                #(
                    let (role_#K, _) = R#K::new();
                    let (name_#K, _) = N#K::new();
                )0*

                let sessionmpst_1 = $struct_name {
                    session1: channel_1_2,
                    session2: channel_1_3,
                    stack: role_1,
                    name: name_1,
                };
                let sessionmpst_2 = $struct_name {
                    session1: channel_2_1,
                    session2: channel_2_3,
                    stack: role_2,
                    name: name_2,
                };
                let sessionmpst_3 = $struct_name {
                    session1: channel_3_1,
                    session2: channel_3_2,
                    stack: role_3,
                    name: name_3,
                };

                #(
                    let thread_#K = $fork_function(f#K, sessionmpst_#K);
                )0*

                (
                    #(
                        thread_#K.join(),
                    )0*
                )
            }
        });
    }
}
