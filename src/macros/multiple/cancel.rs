////////////////////////////////////////////
/// CHOICE

/// Cancels a session
#[macro_export]
macro_rules! broadcast_cancel {
    ($func_name:ident, $name:ident, $struct_name:ident, $nsessions:literal) => {
        mpst_seq::seq!(N in 1..$nsessions ! 1 {
            fn $func_name<%(S#N:0,)()0* R>(
                s: $struct_name<
                    %(
                        S#N:0,
                    )(
                        End,
                    )0*
                    R,
                    $name<mpstthree::role::end::RoleEnd>,
                >,
            )
            where
                %(
                    S#N:0: mpstthree::binary::Session,
                )(
                )0*
                R: mpstthree::role::Role,
            {
                s.session1.sender.send(()).unwrap();;
                std::mem::drop(s);
            }
        });
    }
}
