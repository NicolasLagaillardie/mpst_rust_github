////////////////////////////////////////////
/// CLOSE

#[macro_export]
macro_rules! close_mpst {
    ($func_name:ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions {
            fn $func_name<R>(s: $struct_name<#(mpstthree::binary::End,)* mpstthree::role::end::RoleEnd, R>) -> Result<(), Box<dyn Error>>
            where
                R: mpstthree::role::Role,
            {
                #(
                    s.session#N.sender.send(()).unwrap_or(());
                )*

                #(
                    s.session#N.receiver.recv()?;
                )*

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
            fn $func_name<T, #(S#N,)* R>(
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
            ) -> $struct_name<#(S#N,)* R, $name<mpstthree::role::end::RoleEnd>>
            where
                T: std::marker::Send,
                #(
                    S#N: mpstthree::binary::Session,
                )*
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
            fn $func_name<T, #(S#N,)* R>(
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
                    $struct_name<#(S#N,)* R, $name<mpstthree::role::end::RoleEnd>,
                    >,
                ),
                Box<dyn std::error::Error>,
            >
            where
                T: std::marker::Send,
                #(
                    S#N: mpstthree::binary::Session,
                )*
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
            fn $func_name<T, #(S#N,)* R>(
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
                        #(S#N,)*
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
                )*
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

/// Get an offer on session 1
#[macro_export]
macro_rules! create_offer_mpst_session {
    ($func_name:ident, $role:ident, $recv_func:ident, $name:ident, $struct_name:ident, $nsessions:literal, $exclusion:literal) => {
        mpst_seq::seq!(N in 1..=$nsessions ! $exclusion {
            fn $func_name<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
                s: $struct_name<
                    OfferMpst<S1, S2, S3, S4, R1, R2, $name<mpstthree::role::end::RoleEnd>>,
                    S5,
                    $role<R3, R3>,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
                f: F,
                g: G,
            ) -> Result<U, Box<dyn std::error::Error + 'a>>
            where
                S1: mpstthree::binary::Session,
                S2: mpstthree::binary::Session,
                S3: mpstthree::binary::Session,
                S4: mpstthree::binary::Session,
                S5: mpstthree::binary::Session,
                #(
                    R#N: mpstthree::role::Role,
                )*
                F: FnOnce(
                    $struct_name<
                        S1,
                        S2,
                        R1,
                        $name<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                G: FnOnce(
                    $struct_name<
                        S3,
                        S4,
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

////////////////////////////////////////////
/// FORK

#[macro_export]
macro_rules! fork_mpst_multi {
    ($struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(K in 1..=$nsessions {
            fn fork_simple_multi<#(S#K,)* R, N, P>(p: P, s: $struct_name<#(S#K,)* R, N>) -> std::thread::JoinHandle<()>
            where
                #(
                    S#K: mpstthree::binary::Session + 'static,
                )*
                R: mpstthree::role::Role + 'static,
                N: mpstthree::role::Role + 'static,
                P: FnOnce($struct_name<#(S#K,)* R, N>) -> Result<(), Box<dyn std::error::Error>> + std::marker::Send + 'static,
            {
                std::thread::spawn(move || {
                    panic::set_hook(Box::new(|_info| {
                        // do nothing
                    }));
                    match p(s) {
                        Ok(()) => (),
                        Err(e) => panic!("{:?}", e),
                    }
                })
            }

            fn fork_mpst_multi<#(S#K,)* #(R#K,)* #(N#K,)* #(F#K,)*>(
                #(
                    f#K: F#K,
                )*
            ) -> (
                #(
                    Result<(), Box<(dyn std::any::Any + std::marker::Send + 'static)>>
                )*
            )
            where
                #(
                    S#K: mpstthree::binary::Session + 'static,
                    R#K: mpstthree::binary::Session + 'static,
                    N#K: mpstthree::binary::Session + 'static,
                )*

                F1: FnOnce($struct_name<S1, <S3 as Session>::Dual, R1, N1>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                F2: FnOnce($struct_name<<S1 as Session>::Dual, S2, R2, N2>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                F3: FnOnce($struct_name<S3, <S2 as Session>::Dual, R3, N3>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
            {
                let (channel_1_2, channel_2_1) = S1::new();
                let (channel_2_3, channel_3_2) = S2::new();
                let (channel_3_1, channel_1_3) = S3::new();

                #(
                    let (role_#K, _) = R#K::new();
                    let (name_#K, _) = N#K::new();
                )*

                let sessionmpst_1 = $struct_name {
                    session1: channel_1_2,
                    session2: channel_1_3,
                    stack: role_a,
                    name: name_a,
                };
                let sessionmpst_2 = $struct_name {
                    session1: channel_2_1,
                    session2: channel_2_3,
                    stack: role_b,
                    name: name_b,
                };
                let sessionmpst_3 = $struct_name {
                    session1: channel_3_1,
                    session2: channel_3_2,
                    stack: role_c,
                    name: name_c,
                };

                #(
                    let thread_#K = fork_simple_multi(f#K, sessionmpst_#K);
                )*

                #(
                    thread_#K.join(),
                )*
            }
        });
    }
}
