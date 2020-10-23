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
                    > as mpstthree::binary::Session>::Dual,
                    <
                        $sessionmpst_name<#(S#N:0,)3:0 R1, N0
                    > as mpstthree::binary::Session>::Dual
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

            fn $func_name<'a, #(S#N:0,)10:0 #(R#N:0,)11:0>(
                s: $sessionmpst_name<



                    #( // i in 1..K
                        $type_name<
                            ~( // j in 0..K
                                S~N:2, // S(i + j) (with Dual if needed)
                            )(
                                <S~N:2 as Session>::Dual,
                            )0*
                            ~( // j in 0..K
                                S~N:3, // S(diff * (diff + 1) / 2 + K + i + j) (with Dual if needed)
                            )(
                                <S~N:3 as mpstthree::binary::Session>::Dual,
                            )0*
                            ~( // j in  0..3
                                R~N:4, // R(3 * (i - 1) + 1 + j)
                                // Side note: we lose the checking for the right order for the name on R(3 * (i - 1) + 3) → RoleADual<RoleEnd>
                            )()2*
                        >,
                    )0:0


                    $role<
                        R^N:0, // R(3K-2)
                        R^N:1, // R(3K-1)
                    >,

                    $name<mpstthree::role::end::RoleEnd>,
                >,
            )
            // -> $sessionmpst_name<
            //     #( // i in K-1..0
            //         S#N:0, // S(i) or  S(i + diff * (diff + 1))
            //     )12:0
            //     R#N:3, // R(3K-2) or R(3K-1)
            //     $sender<mpstthree::role::end::RoleEnd>
            // >
            where
                #( // i in 1..(diff * (diff + 1) + 1)
                    S#N:0: mpstthree::binary::Session + 'a, // S(i)
                )10:0
                #( // i in 1..(3 * K)
                    R#N:0: mpstthree::role::Role + 'a, // R(i)
                )11:0
            {


                // #( // i in 1..(diff * (diff + 1))
                //     let (channel_#N:3, channel_#N:4) = S#N:0::new(); // channel_(get from matrix), channel_(opposite get from matrix) = S(i)
                // )4:0

                // #( // i in 1..K
                //     let (_, role_#N:0) = R#N:5::new();
                // )
                // let (_, role_1) = R0::new();
                // let (_, role_2) = R2::new();
                // let (role_3, _) = R4::new();

                // let (name_1, _) =
                //     <$dual_1<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::Dual::new();
                // let (name_2, _) =
                //     <$dual_2<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::Dual::new();
                // let (name_3, _) = $sender::<mpstthree::role::end::RoleEnd>::new();

                // let choice_1 = $type_name {
                //     session1: session_1_2,
                //     session2: session_1_3,
                //     stack: role_1,
                //     name: name_1,
                // };

                // let choice_2 = $type_name {
                //     session1: session_2_1,
                //     session2: session_2_3,
                //     stack: role_2,
                //     name: name_2,
                // };

                // let new_session_1 = mpstthree::binary::send(either::Either::Left(choice_1), s.session1);
                // let new_session_2 = mpstthree::binary::send(either::Either::Left(choice_2), s.session2);
                // let (_, new_queue) = $next(s.stack);

                // let s = $type_name {
                //     session1: new_session_1,
                //     session2: new_session_2,
                //     stack: new_queue,
                //     name: s.name,
                // };

                // mpstthree::binary::cancel(s);

                // $type_name {
                //     session1: session_3_1,
                //     session2: session_3_2,
                //     stack: role_3,
                //     name: name_3,
                // }


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
