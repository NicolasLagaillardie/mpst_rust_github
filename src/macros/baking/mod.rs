#[macro_export]
macro_rules! bundle_impl {
    ( $struct_name:ident, => $($all_roles:ident),+ $(,)? ) => {
        mpst_seq::seq!(N in 1..3 > ($($all_roles,)+) { // 3 is useless, will be replaced by sum($($other_roles)).


            
            //////////////////////////////////////////// from here




            ////////////////////////////////////////////
            /// The normal Role

            #[derive(Debug)]
            struct unused^N:4<R>
            where
                R: mpstthree::role::Role,
                R::Dual: mpstthree::role::Role,
            {
                sender: crossbeam_channel::Sender<R::Dual>,
            }

            ////////////////////////////////////////////
            /// The normal Dual

            #[derive(Debug)]
            struct unused^N:6<R>
            where
                R: mpstthree::role::Role,
                R::Dual: mpstthree::role::Role,
            {
                sender: crossbeam_channel::Sender<R::Dual>,
            }

            ////////////////////////////////////////////
            /// The normal Role implementation of Role

            impl<R: mpstthree::role::Role> mpstthree::role::Role for unused^N:4<R> {
                type Dual = unused^N:6<<R as mpstthree::role::Role>::Dual>;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                    let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);

                    (
                        unused^N:4 {
                            sender: sender_dual,
                        },
                        unused^N:6 {
                            sender: sender_normal,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(unused^N:4))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}>",
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str()
                    )
                }
            }

            ////////////////////////////////////////////
            /// The normal Dual implementation of Role

            impl<R: mpstthree::role::Role> mpstthree::role::Role for unused^N:6<R> {
                type Dual = unused^N:4<<R as mpstthree::role::Role>::Dual>;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                    let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);

                    (
                        unused^N:6 {
                            sender: sender_dual,
                        },
                        unused^N:4 {
                            sender: sender_normal,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(unused^N:6))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}>",
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str()
                    )
                }
            }







            
            ////////////////////////////////////////////
            /// The all Role

            #[derive(Debug)]
            struct unused^N:8<R1, R2>
            where
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
                R1::Dual: mpstthree::role::Role,
                R2::Dual: mpstthree::role::Role,
            {
                sender1: crossbeam_channel::Sender<R1::Dual>,
                sender2: crossbeam_channel::Sender<R2::Dual>,
            }

            ////////////////////////////////////////////
            /// The all Dual

            #[derive(Debug)]
            struct unused^N:10<R1, R2>
            where
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
                R1::Dual: mpstthree::role::Role,
                R2::Dual: mpstthree::role::Role,
            {
                sender1: crossbeam_channel::Sender<R1::Dual>,
                sender2: crossbeam_channel::Sender<R2::Dual>,
            }

            ////////////////////////////////////////////
            /// The all Role implementation of Role

            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role
                for unused^N:8<R1, R2>
            {
                type Dual = unused^N:10<
                    <R1 as mpstthree::role::Role>::Dual,
                    <R2 as mpstthree::role::Role>::Dual,
                >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal_1, _) = crossbeam_channel::bounded::<R1>(1);
                    let (sender_normal_2, _) = crossbeam_channel::bounded::<R2>(1);
                    let (sender_dual_1, _) = crossbeam_channel::bounded::<R1::Dual>(1);
                    let (sender_dual_2, _) = crossbeam_channel::bounded::<R2::Dual>(1);

                    (
                        unused^N:8 {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        unused^N:10 {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(unused^N:8))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}> + {}<{}>",
                        <R1 as mpstthree::role::Role>::head_str(),
                        <R1 as mpstthree::role::Role>::tail_str(),
                        <R2 as mpstthree::role::Role>::head_str(),
                        <R2 as mpstthree::role::Role>::tail_str()
                    )
                }
            }

            ////////////////////////////////////////////
            /// The all Dual implementation of Role

            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role
                for unused^N:10<R1, R2>
            {
                type Dual = unused^N:8<
                    <R1 as mpstthree::role::Role>::Dual,
                    <R2 as mpstthree::role::Role>::Dual,
                >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal_1, _) = crossbeam_channel::bounded::<R1>(1);
                    let (sender_normal_2, _) = crossbeam_channel::bounded::<R2>(1);
                    let (sender_dual_1, _) = crossbeam_channel::bounded::<R1::Dual>(1);
                    let (sender_dual_2, _) = crossbeam_channel::bounded::<R2::Dual>(1);

                    (
                        unused^N:10 {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        unused^N:8 {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(unused^N:10))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}> + {}<{}>",
                        <R1 as mpstthree::role::Role>::head_str(),
                        <R1 as mpstthree::role::Role>::tail_str(),
                        <R2 as mpstthree::role::Role>::head_str(),
                        <R2 as mpstthree::role::Role>::tail_str()
                    )
                }
            }






            //////////////////////////////////////////// to there, must be duplicated for each participant

            


            //////////////////////////////////////////// from here

            





            ////////////////////////////////////////////
            /// The normal Role methods

            impl<#(S#N:0 : mpstthree::binary::struct_trait::Session,)0:0 R: mpstthree::role::Role, T: std::marker::Send>
                $struct_name<
                    mpstthree::binary::struct_trait::Send<T, S1>,
                    S2,
                    RoleB<R>,
                    unused^N:4<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn send(self, payload: T) -> $struct_name<
                    #(S#N:0,)0:0
                    R,
                    unused^N:4<mpstthree::role::end::RoleEnd>
                > {
                    mpstthree::send_aux!(self, payload, RoleB, unused^N:4, $struct_name, 3, 1)
                }
            }

            impl<#(S#N:0 : mpstthree::binary::struct_trait::Session,)0:0 R: mpstthree::role::Role, T: std::marker::Send>
                $struct_name<
                    mpstthree::binary::struct_trait::Recv<T, S1>,
                    S2,
                    RoleB<R>,
                    unused^N:4<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn recv(self) -> Result<(
                    T,
                    $struct_name<
                        #(S#N:0,)0:0
                        R,
                        unused^N:4<mpstthree::role::end::RoleEnd>
                    >),
                    Box<dyn std::error::Error>
                > {
                    mpstthree::recv_aux!(self, RoleB, unused^N:4, $struct_name, 3, 1)()
                }
            }

            impl<#(S#N:0 : mpstthree::binary::struct_trait::Session,)0:0 T: std::marker::Send>
                $struct_name<
                    mpstthree::binary::struct_trait::Recv<T, S1>,
                    S2,
                    RoleAlltoB<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    unused^N:4<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn recv(self) -> Result<(
                    T,
                    $struct_name<
                        #(S#N:0,)0:0
                        mpstthree::role::end::RoleEnd,
                        unused^N:4<mpstthree::role::end::RoleEnd>
                    >),
                    Box<dyn std::error::Error>
                > {
                    mpstthree::recv_all_aux!(self, RoleB, unused^N:4, $struct_name, 3, 1)()
                }
            }

            impl<
                'a,
                #(
                    S#N:0 : mpstthree::binary::struct_trait::Session,
                )2:0
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role
            >
                $struct_name<
                    mpstthree::binary::struct_trait::End,
                    mpstthree::binary::struct_trait::Recv<
                        either::Either<
                        $struct_name<#(S#N:0,)0:0 R1, unused^N:4<mpstthree::role::end::RoleEnd>>,
                        $struct_name<#(S#N:0,)3:0 R2, unused^N:4<mpstthree::role::end::RoleEnd>>>,
                        mpstthree::binary::struct_trait::End
                    >,
                    RoleAlltoC<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    unused^N:4<mpstthree::role::end::RoleEnd>,
                >
            {
                pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn std::error::Error + 'a>>
                where
                    F: FnOnce($struct_name<#(S#N:0,)0:0 R1, unused^N:4<mpstthree::role::end::RoleEnd>>) -> Result<U, Box<dyn std::error::Error + 'a>>,
                    G: FnOnce($struct_name<#(S#N:0,)3:0 R2, unused^N:4<mpstthree::role::end::RoleEnd>>) -> Result<U, Box<dyn std::error::Error + 'a>>,
                {
                    let (e, s) = self.recv()?;
                    mpstthree::binary::cancel::cancel(s);
                    e.either(f, g)
                }
            }









            impl<
                #(
                    S#N:0 : mpstthree::binary::struct_trait::Session,
                )10:0
                #(
                    R#N:0 : mpstthree::role::Role,
                )11:0
            >
                $struct_name<
                    #(
                        mpstthree::binary::struct_trait::Send<
                            either::Either<
                                $sessionmpst_name<
                                    ~(
                                        S~N:2,
                                    )(
                                        <S~N:2 as mpstthree::binary::struct_trait::Session>::Dual,
                                    )0*
                                    R, // Wrong role, should be 2 * i
                                    unused^N:4<RoleEnd>
                                >,
                                $sessionmpst_name<
                                    ~(
                                        S~N:3,
                                    )(
                                        <S~N:3 as mpstthree::binary::struct_trait::Session>::Dual,
                                    )0*
                                    R, // Wrong role, should be 2 * i + 1
                                    unused^N:4<RoleEnd>
                                >,
                            >,
                            mpstthree::binary::struct_trait::End
                        >,
                    )0:0

                    $role_dual<
                        #(
                            R#N:0,
                        )13:0
                    >,
                    unused^N:4<RoleEnd>,
                >
            {
                pub fn choose_left(
                    self,
                ) -> $struct_name<
                    #(
                        <S#N:8 as mpstthree::binary::struct_trait::Session>::Dual,
                    )0:0
                    R^N:0,
                    unused^N:4<RoleEnd>
                > {
                    #(
                        let (channel_#N:3, channel_#N:4) = <S#N:0 as mpstthree::binary::struct_trait::Session>::new();
                    )4:0

                    #(
                        let (stack_#N:0, _) = <R#N:5 as mpstthree::role::Role>::new();
                    )0:0
                    let (stack_^N:2, _) = <R^N:0 as mpstthree::role::Role>::new();

                    #(
                        let (name_#N:0, _) = <R#N:6 as mpstthree::role::Role>::new();
                    )0:0
                    let (name_^N:2, _) = <$name::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                    #(
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

                    #(
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
            
                pub fn choose_right(
                    self,
                ) -> $struct_name<
                    #(
                        <S#N:9 as mpstthree::binary::struct_trait::Session>::Dual,
                    )0:0
                    R^N:1,
                    unused^N:4<RoleEnd>
                > {
                    #(
                        let (channel_#N:3, channel_#N:4) = <S#N:10 as Session>::new(); 
                    )4:0

                    #(
                        let (stack_#N:0, _) = <R#N:11 as Role>::new();
                    )0:0
                    let (stack_^N:2, _) = <R^N:1 as Role>::new();

                    #(
                        let (name_#N:0, _) = <R#N:6 as mpstthree::role::Role>::new();
                    )0:0
                    let (name_^N:2, _) = <$name::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                    #(
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

                    #(
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
            }










            impl
                $struct_name<
                    #(
                        mpstthree::binary::struct_trait::End,
                    )0:0
                    mpstthree::role::end::RoleEnd,
                    unused^N:4<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn close(self) -> Result<(), Box<dyn Error>> {
                    #(
                        self.session#N:0.sender.send(mpstthree::binary::struct_trait::Signal::Stop).unwrap_or(());
                    )0:0
    
                    #(
                        self.session#N:0.receiver.recv()?;
                    )0:0
    
                    Ok(())
                }
            }



            //////////////////////////////////////////// to here, must be duplicated for each participant, for each binary channel, for each possible interaction


        });
    }
}
