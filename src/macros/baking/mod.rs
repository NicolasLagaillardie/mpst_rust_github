#[macro_export]
macro_rules! bundle_impl {
    ( $($role_name:ident),+ $(,)? ) => {
        $(mpstthree::create_normal_role_short!($role_name);)+
     }
}

#[macro_export]
macro_rules! create_impl {
    ( $struct_name:ident, $exclusion:literal => $($all_roles:ident),+ $(,)? ) => {
        mpst_seq::seq!(N in 1..3 > ($($all_roles,)+) > ($($all_roles,)+) { // 3 is useless, replaced by sum($($other_roles)). First ($($all_roles,)+) is useless

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
                    R#N:0,
                )11:0
            >
                $struct_name<
                    Send<
                        Either<
                            $struct_name<S2, <S0 as Session>::Dual, R0, RoleB<RoleEnd>>,
                            $struct_name<S4, <S1 as Session>::Dual, R1, RoleB<RoleEnd>>,
                        >,
                        End,
                    >,
                    Send<
                        Either<
                        $struct_name<S3, S0, R2, RoleC<RoleEnd>>,
                            $struct_name<S5, S1, R3, RoleC<RoleEnd>>,
                        >,
                        End,
                    >,
                    RoleAtoAll<R4, R5>,
                    unused^N:4<RoleEnd>,
                >
            {
                pub fn choose_left(
                    self,
                ) -> $struct_name<
                    <S2 as Session>::Dual,
                    <S3 as Session>::Dual,
                    R^N:1,
                    unused^N:4<RoleEnd>
                > {
                    choose_aux!(
                        S2,
                        S3,
                        S0,
                        R0,
                        R2,
                        R4,
                        RoleB,
                        RoleC,
                        RoleA,
                        self,
                        Either::Left
                    )
                }
            
                pub fn choose_right(
                    self,
                ) -> $struct_name<
                    <S4 as Session>::Dual,
                    <S5 as Session>::Dual,
                    R5,
                    unused^N:4<RoleEnd>
                > {
                    choose_aux!(
                        S4,
                        S5,
                        S1,
                        R1,
                        R3,
                        R5,
                        RoleB,
                        RoleC,
                        RoleA,
                        self,
                        Either::Right
                    )
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

        });
    }
}
