
#[macro_export]
macro_rules! bundle_impl {
    ( $sessionmpst_name:ident => $($all_roles:ident),+ $(,)? ) => {
        mpst_seq::seq!(N in 1..3 > ($($all_roles,)+) { // 3 is useless, replaced by sum($($other_roles)).
            #[must_use]
            #[derive(Debug)]
            pub struct $sessionmpst_name<
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
            impl<#(S#N:0: mpstthree::binary::struct_trait::Session,)20:0 R: mpstthree::role::Role, N: mpstthree::role::Role> mpstthree::binary::struct_trait::Session for $sessionmpst_name<#(S#N:0, )20:0 R, N> {
                type Dual =
                $sessionmpst_name<#(<S#N:0 as mpstthree::binary::struct_trait::Session>::Dual, )20:0 <R as mpstthree::role::Role>::Dual, <N as mpstthree::role::Role>::Dual, >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    #(
                        let (sender#N:0, receiver#N:0) = S#N:0::new();
                    )20:0

                    let (role_one, role_two) = R::new();
                    let (name_one, name_two) = N::new();

                    (
                        $sessionmpst_name {
                            #(
                                session#N:0: sender#N:0,
                            )20:0
                            stack: role_one,
                            name: name_one,
                        },
                        $sessionmpst_name {
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

            #[doc(hidden)]
            impl<
                    #(
                        S#N:0: mpstthree::binary::struct_trait::Session,
                    )20:0
                    R: mpstthree::role::Role,
                    N: mpstthree::role::Role
                > $sessionmpst_name<#(S#N:0, )20:0 R, N> {
                #[doc(hidden)]
                pub fn field_names(self) ->
                    (
                        &'static [&'static str],
                        $sessionmpst_name<#(S#N:0, )20:0 R, N>
                    ) {
                    (
                        &[
                            #(
                                stringify!(session#N:0),
                            )20:0
                        ],
                        self
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
                        $sessionmpst_name<
                            |(
                                mpstthree::binary::struct_trait::Send<T, S|N:0>,
                            )(
                                S|N:0,
                            )0*
                            unused~N:9<R>,
                            unused#N:23<mpstthree::role::end::RoleEnd>
                        >
                    {
                        pub fn send(self, payload: T) -> $sessionmpst_name<
                            |(
                                S|N:0,
                            )(
                                S|N:0,
                            )0*
                            R,
                            unused#N:23<mpstthree::role::end::RoleEnd>
                        > {
                            let new_session = mpstthree::binary::send::send(payload, self.session~N:17);

                            let new_stack = {
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

                            $sessionmpst_name {
                                |(
                                    session|N:0: new_session,
                                )(
                                    session|N:0: self.session|N:0,
                                )0*
                                stack: new_stack,
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
                        $sessionmpst_name<
                            |(
                                mpstthree::binary::struct_trait::Recv<T, S|N:0>,
                            )(
                                S|N:0,
                            )0*
                            unused~N:9<R>,
                            unused#N:23<mpstthree::role::end::RoleEnd>
                        >
                    {
                        pub fn recv(self) -> Result<(
                                T,
                                $sessionmpst_name<
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

                            let new_stack = {
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
                                $sessionmpst_name {
                                    |(
                                        session|N:0: new_session,
                                    )(
                                        session|N:0: self.session|N:0,
                                    )0*
                                    stack: new_stack,
                                    name: self.name,
                                }
                            ))
                        }
                    }

                )7*
            )21:0

            #(
                ~(
                )(
                    impl<#(S#N:0 : mpstthree::binary::struct_trait::Session,)20:0 T: std::marker::Send>
                        $sessionmpst_name<
                            |(
                                mpstthree::binary::struct_trait::Recv<T, S|N:0>,
                            )(
                                S|N:0,
                            )0*
                            unused~N:15<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                            unused#N:23<mpstthree::role::end::RoleEnd>
                        >
                    {
                        pub fn recv(self) -> Result<(
                                T,
                                $sessionmpst_name<
                                    |(
                                        S|N:0,
                                    )(
                                        S|N:0,
                                    )0*
                                    mpstthree::role::end::RoleEnd,
                                    unused#N:23<mpstthree::role::end::RoleEnd>
                            >),
                            Box<dyn std::error::Error>
                        > {
                            let (v, new_session) = mpstthree::binary::recv::recv(self.session~N:17)?;

                            let (here1, there1) = <mpstthree::role::end::RoleEnd as mpstthree::role::Role>::new();
                            let (_here2, there2) = <mpstthree::role::end::RoleEnd as mpstthree::role::Role>::new();
                            self.stack.sender1.send(there1).unwrap_or(());
                            self.stack.sender2.send(there2).unwrap_or(());
                            Ok((
                                v,
                                $sessionmpst_name {
                                    |(
                                        session|N:0: new_session,
                                    )(
                                        session|N:0: self.session|N:0,
                                    )0*
                                    stack: here1,
                                    name: self.name,
                                }
                            ))
                        }
                    }
                )7*
            )21:0

            #(
                ~(
                )(
                    impl<
                        'a,
                        #(S#N:0 : mpstthree::binary::struct_trait::Session,)22:0
                        R1: mpstthree::role::Role,
                        R2: mpstthree::role::Role,
                    >
                        $sessionmpst_name<
                            |(
                                mpstthree::binary::struct_trait::Recv<
                                    either::Either<
                                        $sessionmpst_name<
                                            #(S#N:0,)20:0
                                            R1,
                                            unused#N:23<mpstthree::role::end::RoleEnd>
                                        >,
                                        $sessionmpst_name<
                                            #(S#N:0,)23:0
                                            R2,
                                            unused#N:23<mpstthree::role::end::RoleEnd>
                                        >
                                    >,
                                    mpstthree::binary::struct_trait::End
                                >,
                            )(
                                mpstthree::binary::struct_trait::End,
                            )0*
                            unused~N:15<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                            unused#N:23<mpstthree::role::end::RoleEnd>,
                        >
                    {
                        pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn std::error::Error + 'a>>
                        where
                            F: FnOnce(
                                $sessionmpst_name<
                                    #(S#N:0,)20:0
                                    R1,
                                    unused#N:23<mpstthree::role::end::RoleEnd>,
                                >,
                            ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                            G: FnOnce(
                                $sessionmpst_name<
                                    #(S#N:0,)23:0
                                    R2,
                                    unused#N:23<mpstthree::role::end::RoleEnd>,
                                >,
                            ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                        {
                            let (e, s) = self.recv()?;
                            mpstthree::binary::cancel::cancel(s);
                            e.either(f, g)
                        }
                    }

                )7*
            )21:0

            #(
                impl<
                    #(
                        S#N:0: mpstthree::binary::struct_trait::Session,
                    )25:0
                    #(
                        R#N:0: mpstthree::role::Role,
                    )26:0
                >
                    $sessionmpst_name<
                        ~(
                        )(
                            mpstthree::binary::struct_trait::Send<
                                either::Either<
                                    $sessionmpst_name<
                                        |(
                                            S|N:1,
                                        )(
                                            <S|N:1 as mpstthree::binary::struct_trait::Session>::Dual,
                                        )2*
                                        R~N:23,
                                        unused~N:9<mpstthree::role::end::RoleEnd>
                                    >,
                                    $sessionmpst_name<
                                        |(
                                            S|N:2,
                                        )(
                                            <S|N:2 as mpstthree::binary::struct_trait::Session>::Dual,
                                        )2*
                                        R~N:24,
                                        unused~N:9<mpstthree::role::end::RoleEnd>
                                    >
                                >,
                                mpstthree::binary::struct_trait::End,
                            >,
                        )7*

                        unused#N:27<
                            #(
                                R#N:0,
                            )27:0
                        >,
                        unused#N:23<mpstthree::role::end::RoleEnd>,
                    >
                {
                    pub fn choose_left(self) -> $sessionmpst_name<
                        ~(
                            <S~N:25 as mpstthree::binary::struct_trait::Session>::Dual,
                        )(
                            <S~N:25 as mpstthree::binary::struct_trait::Session>::Dual,
                        )10*
                        R^N:12,
                        unused#N:23<mpstthree::role::end::RoleEnd>
                    >
                    {
                        ~(
                            let (channel_~N:27, channel_~N:28) = S~N:29::new();
                        )(
                            let (channel_~N:28, channel_~N:27) = S~N:29::new();
                        )9*

                        #(
                            let (stack_#N:0, _) = <R#N:35 as mpstthree::role::Role>::new();
                        )20:0
                        let (stack_^N:14, _) = <R^N:12 as mpstthree::role::Role>::new();

                        ~(
                        )(
                            let (name_~N:22, _) = <unused~N:9<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                        )7*
                        let (name_^N:14, _) = <unused#N:23::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                        ~(
                        )(
                            let choice_~N:22 = $sessionmpst_name {
                                    |(
                                        session|N:0 : channel_|N:3,
                                    )(
                                        session|N:0 : channel_|N:3,
                                    )0*
                                    stack: stack_~N:22,
                                    name: name_~N:22,
                                };
                        )7*

                        #(
                            let new_session_#N:0 = mpstthree::binary::send::send(either::Either::Left(choice_#N:0), self.session#N:0);
                        )20:0

                        let s = $sessionmpst_name {
                            #(
                                session#N:0: new_session_#N:0,
                            )20:0
                            stack: self.stack,
                            name: self.name,
                        };

                        mpstthree::binary::cancel::cancel(s);

                        $sessionmpst_name {
                            ~(
                            )(
                                session~N:22: channel_~N:31,
                            )7*
                            stack: stack_^N:14,
                            name: name_^N:14,
                        }
                    }

                    pub fn choose_right(self) -> $sessionmpst_name<
                        ~(
                            <S~N:26 as mpstthree::binary::struct_trait::Session>::Dual,
                        )(
                            <S~N:26 as mpstthree::binary::struct_trait::Session>::Dual,
                        )10*
                        R^N:13,
                        unused#N:23<mpstthree::role::end::RoleEnd>
                    >
                    {
                        ~(
                            let (channel_~N:27, channel_~N:28) = S~N:30::new();
                        )(
                            let (channel_~N:28, channel_~N:27) = S~N:30::new();
                        )9*

                        #(
                            let (stack_#N:0, _) = <R#N:36 as mpstthree::role::Role>::new();
                        )20:0
                        let (stack_^N:14, _) = <R^N:13 as mpstthree::role::Role>::new();

                        ~(
                        )(
                            let (name_~N:22, _) = <unused~N:9<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                        )7*
                        let (name_^N:14, _) = <unused#N:23::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                        ~(
                        )(
                            let choice_~N:22 = $sessionmpst_name {
                                    |(
                                        session|N:0 : channel_|N:3,
                                    )(
                                        session|N:0 : channel_|N:3,
                                    )0*
                                    stack: stack_~N:22,
                                    name: name_~N:22,
                                };
                        )7*

                        #(
                            let new_session_#N:0 = mpstthree::binary::send::send(either::Either::Right(choice_#N:0), self.session#N:0);
                        )20:0

                        let s = $sessionmpst_name {
                            #(
                                session#N:0: new_session_#N:0,
                            )20:0
                            stack: self.stack,
                            name: self.name,
                        };

                        mpstthree::binary::cancel::cancel(s);

                        $sessionmpst_name {
                            ~(
                            )(
                                session~N:22: channel_~N:31,
                            )7*
                            stack: stack_^N:14,
                            name: name_^N:14,
                        }
                    }
                }
            )21:0

            ////////////////////////////////////////////
            // The close methods

            #(
                impl
                    $sessionmpst_name<
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


// #[macro_export]
// macro_rules! bundle_impl {
//     ( $sessionmpst_name:ident => $($all_roles:ident),+ $(,)? ) => {
//         mpst_seq::baking!($sessionmpst_name, ($($all_roles,)+));
//     };
// }