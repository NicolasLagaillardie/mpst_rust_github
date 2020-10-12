// ////////////////////////////////////////////
// /// SEND

// // create a function send_mpst for the excluded session
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

// ////////////////////////////////////////////
// /// RECV

// // create a function recv_mpst for the excluded session
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
