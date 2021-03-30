#[macro_export]
macro_rules! bundle_impl {
    ( $struct_name:ident => $($all_roles:ident),+ $(,)? ) => {
        mpst_seq::seq!(N in 1..3 > ($($all_roles,)+) { // 3 is useless, replaced by sum($($other_roles)).
            #[must_use]
            #[derive(Debug)]
            pub struct $struct_name<
                #(
                    S#N:0,
                )20:0
                R,
                N
            >
            where
                #(
                    S#N:0: mpstthree::binary::struct_trait::Session,
                )20:0
                R: mpstthree::role::Role,
                N: mpstthree::role::Role
            {
                #(
                    pub session#N:0: S#N:0,
                )20:0
                pub stack: R,
                pub name: N,
            }

            #[doc(hidden)]
            impl<#(S#N:0: mpstthree::binary::struct_trait::Session,)20:0 R: mpstthree::role::Role, N: mpstthree::role::Role> mpstthree::binary::struct_trait::Session for $struct_name<#(S#N:0, )20:0 R, N> {
                type Dual =
                $struct_name<#(<S#N:0 as mpstthree::binary::struct_trait::Session>::Dual, )20:0 <R as mpstthree::role::Role>::Dual, <N as mpstthree::role::Role>::Dual, >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    #(
                        let (sender#N:0, receiver#N:0) = S#N:0::new();
                    )20:0

                    let (role_one, role_two) = R::new();
                    let (name_one, name_two) = N::new();

                    (
                        $struct_name {
                            #(
                                session#N:0: sender#N:0,
                            )20:0
                            stack: role_one,
                            name: name_one,
                        },
                        $struct_name {
                            #(
                                session#N:0: receiver#N:0,
                            )20:0
                            stack: role_two,
                            name: name_two,
                        }
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    let mut result = String::from("");
                    #(
                        result = format!("{} + {}", result, S#N:0::head_str());
                    )20:0
                    format!(
                        "{} + {} + {}",
                        result,
                        R::head_str(),
                        N::head_str()
                    )
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    let mut result = String::from("");
                    #(
                        result = format!("{} + {}", result, S#N:0::head_str());
                    )20:0
                    format!(
                        " {} + {} + {}",
                        result,
                        R::tail_str(),
                        N::head_str()
                    )
                }
            }

            #(

                ////////////////////////////////////////////
                /// The normal Role

                #[derive(Debug)]
                struct unused#N:23<R>
                where
                    R: mpstthree::role::Role,
                    R::Dual: mpstthree::role::Role,
                {
                    sender: crossbeam_channel::Sender<R::Dual>,
                }

                ////////////////////////////////////////////
                /// The normal Dual

                #[derive(Debug)]
                struct unused#N:25<R>
                where
                    R: mpstthree::role::Role,
                    R::Dual: mpstthree::role::Role,
                {
                    sender: crossbeam_channel::Sender<R::Dual>,
                }

                ////////////////////////////////////////////
                /// The normal Role implementation of Role

                impl<R: mpstthree::role::Role> mpstthree::role::Role for unused#N:23<R> {
                    type Dual = unused#N:25<<R as mpstthree::role::Role>::Dual>;

                    #[doc(hidden)]
                    fn new() -> (Self, Self::Dual) {
                        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);

                        (
                            unused#N:23 {
                                sender: sender_dual,
                            },
                            unused#N:25 {
                                sender: sender_normal,
                            },
                        )
                    }

                    #[doc(hidden)]
                    fn head_str() -> String {
                        String::from(stringify!(unused#N:23))
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

                impl<R: mpstthree::role::Role> mpstthree::role::Role for unused#N:25<R> {
                    type Dual = unused#N:23<<R as mpstthree::role::Role>::Dual>;

                    #[doc(hidden)]
                    fn new() -> (Self, Self::Dual) {
                        let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                        let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);

                        (
                            unused#N:25 {
                                sender: sender_dual,
                            },
                            unused#N:23 {
                                sender: sender_normal,
                            },
                        )
                    }

                    #[doc(hidden)]
                    fn head_str() -> String {
                        String::from(stringify!(unused#N:25))
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
                struct unused#N:27<R1, R2>
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
                struct unused#N:29<R1, R2>
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
                    for unused#N:27<R1, R2>
                {
                    type Dual = unused#N:29<
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
                            unused#N:27 {
                                sender1: sender_dual_1,
                                sender2: sender_dual_2,
                            },
                            unused#N:29 {
                                sender1: sender_normal_1,
                                sender2: sender_normal_2,
                            },
                        )
                    }

                    #[doc(hidden)]
                    fn head_str() -> String {
                        String::from(stringify!(unused#N:27))
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
                    for unused#N:29<R1, R2>
                {
                    type Dual = unused#N:27<
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
                            unused#N:29 {
                                sender1: sender_dual_1,
                                sender2: sender_dual_2,
                            },
                            unused#N:27 {
                                sender1: sender_normal_1,
                                sender2: sender_normal_2,
                            },
                        )
                    }

                    #[doc(hidden)]
                    fn head_str() -> String {
                        String::from(stringify!(unused#N:29))
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

            )21:0




            //////////////////////////////////////////// from here







            ////////////////////////////////////////////
            // The normal Role methods

            #(
                ~(
                )(

                    impl<#(S#N:0 : mpstthree::binary::struct_trait::Session,)20:0 R: mpstthree::role::Role, T: std::marker::Send>
                        $struct_name<
                            |(
                                mpstthree::binary::struct_trait::Send<T, S|N:0>,
                            )(
                                S|N:0,
                            )0*
                            unused~N:9<R>,
                            unused#N:23<mpstthree::role::end::RoleEnd>
                        >
                    {
                        pub fn send(self, payload: T) -> $struct_name<
                            |(
                                S|N:0,
                            )(
                                S|N:0,
                            )0*
                            R,
                            unused#N:23<mpstthree::role::end::RoleEnd>
                        > {
                            let new_session = mpstthree::binary::send::send(payload, self.session~N:17);

                            let new_queue = {
                                fn temp<R>(r: unused~N:9<R>) -> R
                                where
                                    R: mpstthree::role::Role,
                                {
                                    let (here, there) = <R as mpstthree::role::Role>::new();
                                    r.sender.send(there).unwrap_or(());
                                    here
                                }
                                temp(self.stack)
                            };

                            $struct_name {
                                |(
                                    session|N:0: new_session,
                                )(
                                    session|N:0: self.session|N:0,
                                )0*
                                stack: new_queue,
                                name: self.name,
                            }


                        }
                    }

                )7*

            )21:0




            #(
                ~(
                )(

                    impl<#(S#N:0 : mpstthree::binary::struct_trait::Session,)20:0 R: mpstthree::role::Role, T: std::marker::Send>
                        $struct_name<
                            |(
                                mpstthree::binary::struct_trait::Recv<T, S|N:0>,
                            )(
                                S|N:0,
                            )0*
                            unused~N:9<R>,
                            unused#N:23<mpstthree::role::end::RoleEnd>
                        >
                    {
                        pub fn recv(self, payload: T) -> Result<(
                                T,
                                $struct_name<
                                    |(
                                        S|N:0,
                                    )(
                                        S|N:0,
                                    )0*
                                    R,
                                    unused#N:23<mpstthree::role::end::RoleEnd>
                            >),
                            Box<dyn std::error::Error>
                        > {
                            let (v, new_session) = mpstthree::binary::recv::recv(self.session~N:17)?;

                            let new_queue = {
                                fn temp<R>(r: unused~N:9<R>) -> R
                                where
                                    R: mpstthree::role::Role,
                                {
                                    let (here, there) = <R as mpstthree::role::Role>::new();
                                    r.sender.send(there).unwrap_or(());
                                    here
                                }
                                temp(self.stack)
                            };

                            Ok((
                                v,
                                $struct_name {
                                    |(
                                        session|N:0: new_session,
                                    )(
                                        session|N:0: self.session|N:0,
                                    )0*
                                    stack: new_queue,
                                    name: self.name,
                                }
                            ))
                        }
                    }

                )7*

            )21:0

            // impl<#(S#N:0 : mpstthree::binary::struct_trait::Session,)0:0 T: std::marker::Send>
            //     $struct_name<
            //         mpstthree::binary::struct_trait::Recv<T, S1>,
            //         S2,
            //         RoleAlltoB<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
            //         unused#N:23<mpstthree::role::end::RoleEnd>
            //     >
            // {
            //     pub fn recv(self) -> Result<(
            //         T,
            //         $struct_name<
            //             #(S#N:0,)0:0
            //             mpstthree::role::end::RoleEnd,
            //             unused#N:23<mpstthree::role::end::RoleEnd>
            //         >),
            //         Box<dyn std::error::Error>
            //     > {
            //         mpstthree::recv_all_aux!(self, RoleB, unused#N:23, $struct_name, 3, 1)()
            //     }
            // }

            // impl<
            //     'a,
            //     #(
            //         S#N:0 : mpstthree::binary::struct_trait::Session,
            //     )2:0
            //     R1: mpstthree::role::Role,
            //     R2: mpstthree::role::Role
            // >
            //     $struct_name<
            //         mpstthree::binary::struct_trait::End,
            //         mpstthree::binary::struct_trait::Recv<
            //             either::Either<
            //             $struct_name<#(S#N:0,)0:0 R1, unused#N:23<mpstthree::role::end::RoleEnd>>,
            //             $struct_name<#(S#N:0,)3:0 R2, unused#N:23<mpstthree::role::end::RoleEnd>>>,
            //             mpstthree::binary::struct_trait::End
            //         >,
            //         RoleAlltoC<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
            //         unused#N:23<mpstthree::role::end::RoleEnd>,
            //     >
            // {
            //     pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn std::error::Error + 'a>>
            //     where
            //         F: FnOnce($struct_name<#(S#N:0,)0:0 R1, unused#N:23<mpstthree::role::end::RoleEnd>>) -> Result<U, Box<dyn std::error::Error + 'a>>,
            //         G: FnOnce($struct_name<#(S#N:0,)3:0 R2, unused#N:23<mpstthree::role::end::RoleEnd>>) -> Result<U, Box<dyn std::error::Error + 'a>>,
            //     {
            //         let (e, s) = self.recv()?;
            //         mpstthree::binary::cancel::cancel(s);
            //         e.either(f, g)
            //     }
            // }









            // impl<
            //     #(
            //         S#N:0 : mpstthree::binary::struct_trait::Session,
            //     )10:0
            //     #(
            //         R#N:0 : mpstthree::role::Role,
            //     )11:0
            // >
            //     $struct_name<
            //         #(
            //             mpstthree::binary::struct_trait::Send<
            //                 either::Either<
            //                     $sessionmpst_name<
            //                         ~(
            //                             S~N:2,
            //                         )(
            //                             <S~N:2 as mpstthree::binary::struct_trait::Session>::Dual,
            //                         )0*
            //                         R, // Wrong role, should be 2 * i
            //                         unused#N:23<RoleEnd>
            //                     >,
            //                     $sessionmpst_name<
            //                         ~(
            //                             S~N:3,
            //                         )(
            //                             <S~N:3 as mpstthree::binary::struct_trait::Session>::Dual,
            //                         )0*
            //                         R, // Wrong role, should be 2 * i + 1
            //                         unused#N:23<RoleEnd>
            //                     >,
            //                 >,
            //                 mpstthree::binary::struct_trait::End
            //             >,
            //         )0:0

            //         $role_dual<
            //             #(
            //                 R#N:0,
            //             )13:0
            //         >,
            //         unused#N:23<RoleEnd>,
            //     >
            // {
            //     pub fn choose_left(
            //         self,
            //     ) -> $struct_name<
            //         #(
            //             <S#N:8 as mpstthree::binary::struct_trait::Session>::Dual,
            //         )0:0
            //         R^N:0,
            //         unused#N:23<RoleEnd>
            //     > {
            //         #(
            //             let (channel_#N:3, channel_#N:4) = <S#N:0 as mpstthree::binary::struct_trait::Session>::new();
            //         )4:0

            //         #(
            //             let (stack_#N:0, _) = <R#N:5 as mpstthree::role::Role>::new();
            //         )0:0
            //         let (stack_^N:2, _) = <R^N:0 as mpstthree::role::Role>::new();

            //         #(
            //             let (name_#N:0, _) = <R#N:6 as mpstthree::role::Role>::new();
            //         )0:0
            //         let (name_^N:2, _) = <$name::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

            //         #(
            //             let choice_#N:0 = $sessionmpst_name {
            //                     ~(
            //                         session#N:1 : channel_~N:5,
            //                     )(
            //                         session#N:1 : channel_~N:5,
            //                     )0*
            //                     stack: stack_#N:0,
            //                     name: name_#N:0,
            //                 };
            //         )0:0

            //         #(
            //             let new_session_#N:0 = mpstthree::binary::send::send(either::Either::Left(choice_#N:0), s.session#N:0);
            //         )0:0

            //         let s = $sessionmpst_name {
            //             #(
            //                 session#N:0: new_session_#N:0,
            //             )0:0
            //             stack: s.stack,
            //             name: s.name,
            //         };

            //         mpstthree::binary::cancel::cancel(s);

            //         $sessionmpst_name {
            //             #(
            //                 session#N:0: channel_#N:7 ,
            //             )0:0
            //             stack: stack_^N:2,
            //             name: name_^N:2,
            //         }
            //     }

            //     pub fn choose_right(
            //         self,
            //     ) -> $struct_name<
            //         #(
            //             <S#N:9 as mpstthree::binary::struct_trait::Session>::Dual,
            //         )0:0
            //         R^N:1,
            //         unused#N:23<RoleEnd>
            //     > {
            //         #(
            //             let (channel_#N:3, channel_#N:4) = <S#N:10 as Session>::new();
            //         )4:0

            //         #(
            //             let (stack_#N:0, _) = <R#N:11 as Role>::new();
            //         )0:0
            //         let (stack_^N:2, _) = <R^N:1 as Role>::new();

            //         #(
            //             let (name_#N:0, _) = <R#N:6 as mpstthree::role::Role>::new();
            //         )0:0
            //         let (name_^N:2, _) = <$name::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

            //         #(
            //             let choice_#N:0 = $sessionmpst_name {
            //                     ~(
            //                         session#N:1 : channel_~N:5,
            //                     )(
            //                         session#N:1 : channel_~N:5,
            //                     )0*
            //                     stack: stack_#N:0,
            //                     name: name_#N:0,
            //                 };
            //         )0:0

            //         #(
            //             let new_session_#N:0 = mpstthree::binary::send::send(either::Either::Right(choice_#N:0), s.session#N:0);
            //         )0:0

            //         let s = $sessionmpst_name {
            //             #(
            //                 session#N:0: new_session_#N:0,
            //             )0:0
            //             stack: s.stack,
            //             name: s.name,
            //         };

            //         mpstthree::binary::cancel::cancel(s);

            //         $sessionmpst_name {
            //             #(
            //                 session#N:0: channel_#N:7 ,
            //             )0:0
            //             stack: stack_^N:2,
            //             name: name_^N:2,
            //         }
            //     }
            // }








            ////////////////////////////////////////////
            // The close methods

            #(

                impl
                    $struct_name<
                        #(
                            mpstthree::binary::struct_trait::End,
                        )20:0
                        mpstthree::role::end::RoleEnd,
                        unused#N:23<mpstthree::role::end::RoleEnd>
                    >
                {
                    pub fn close(self) -> Result<(), Box<dyn std::error::Error>> {
                        #(
                            self.session#N:0.sender.send(mpstthree::binary::struct_trait::Signal::Stop).unwrap_or(());
                        )20:0

                        #(
                            self.session#N:0.receiver.recv()?;
                        )20:0

                        Ok(())
                    }
                }

            )21:0
        });
    }
}
