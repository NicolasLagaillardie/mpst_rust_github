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
