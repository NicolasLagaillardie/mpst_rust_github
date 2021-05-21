use quote::{format_ident, quote};
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct BakingMacroInput {
    sessionmpst_name: syn::Ident,
    roles: proc_macro2::TokenStream,
}

impl Parse for BakingMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let sessionmpst_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let content_roles;
        let _parentheses = syn::parenthesized!(content_roles in input);
        let roles = proc_macro2::TokenStream::parse(&content_roles)?;

        Ok(BakingMacroInput {
            sessionmpst_name,
            roles,
        })
    }
}

impl From<BakingMacroInput> for proc_macro2::TokenStream {
    fn from(input: BakingMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl BakingMacroInput {
    fn expand_options(
        &self,
        tt: proc_macro2::TokenTree,
        _rest: &mut proc_macro2::token_stream::IntoIter,
    ) -> std::option::Option<proc_macro2::TokenStream> {
        match tt {
            proc_macro2::TokenTree::Group(g) => Some(g.stream()),
            _ => None,
        }
    }

    fn expand_stream(&self, stream: &proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
        let mut out: Vec<proc_macro2::TokenStream> = Vec::new();
        let mut tts = stream.clone().into_iter();
        while let Some(tt) = tts.next() {
            let elt = self.expand_options(tt, &mut tts);
            if let Some(elt_tt) = elt {
                out.push(elt_tt)
            }
        }
        out
    }

    fn expand_send(
        &self,
        all_roles: Vec<proc_macro2::TokenStream>,
        sender: u64,
        receiver: u64,
        number_roles: u64,
        sessionmpst_name: syn::Ident,
        session_types: Vec<syn::Ident>,
        session_types_struct: Vec<syn::Ident>,
    ) -> proc_macro2::TokenStream {
        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender).unwrap()) {
            let concatenated_elt = format!("Role{}", elt);
            syn::Ident::new(&concatenated_elt, proc_macro2::Span::call_site())
        } else {
            panic!("Not enough arguments")
        };

        let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver).unwrap()) {
            let concatenated_elt = format!("Role{}", elt);
            syn::Ident::new(&concatenated_elt, proc_macro2::Span::call_site())
        } else {
            panic!("Not enough arguments")
        };

        let send_sessions: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|k| {
                let cond = if k >= sender { receiver - 1 } else { receiver };
                if k == cond {
                    quote! { mpstthree::binary::struct_trait::Send<T, S#k> ,}
                } else {
                    quote! { S#k ,}
                }
            })
            .collect();

        let new_sessions: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|k| {
                let cond = if k >= sender { receiver - 1 } else { receiver };
                if k == cond {
                    quote! { session#k: new_session , }
                } else {
                    quote! { session#k: self.session#k , }
                }
            })
            .collect();

        let index = if receiver >= sender {
            receiver - 1
        } else {
            receiver
        };

        let new_session = format_ident!("session{}", index);

        quote! {
            impl<#( #session_types_struct )* R: mpstthree::role::Role, T: std::marker::Send>
                #sessionmpst_name<
                    #( #send_sessions )*
                    #receiver_ident<R>,
                    #sender_ident<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn send(self, payload: T) -> #sessionmpst_name<
                    #( #session_types )*
                    R,
                    #sender_ident<mpstthree::role::end::RoleEnd>
                > {
                    let new_session = mpstthree::binary::send::send(payload, self.#new_session);
                    let new_stack = {
                        fn temp<R>(r: #receiver_ident<R>) -> R
                        where
                            R: mpstthree::role::Role,
                        {
                            let (here, there) = <R as mpstthree::role::Role>::new();
                            r.sender.send(there).unwrap_or(());
                            here
                        }
                        temp(self.stack)
                    };
                    #sessionmpst_name {
                        #( #new_sessions )*
                        stack: new_stack,
                        name: self.name,
                    }
                }
            }
        }
    }

    fn expand_recv(
        &self,
        all_roles: Vec<proc_macro2::TokenStream>,
        receiver: u64,
        sender: u64,
        number_roles: u64,
        sessionmpst_name: syn::Ident,
        session_types: Vec<syn::Ident>,
        session_types_struct: Vec<syn::Ident>,
    ) -> proc_macro2::TokenStream {
        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender).unwrap()) {
            let concatenated_elt = format!("Role{}", elt);
            syn::Ident::new(&concatenated_elt, proc_macro2::Span::call_site())
        } else {
            panic!("Not enough arguments")
        };

        let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver).unwrap()) {
            let concatenated_elt = format!("Role{}", elt);
            syn::Ident::new(&concatenated_elt, proc_macro2::Span::call_site())
        } else {
            panic!("Not enough arguments")
        };

        let send_sessions: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };
                if k == cond {
                    quote! { mpstthree::binary::struct_trait::Recv<T, S#k> ,}
                } else {
                    quote! { S#k ,}
                }
            })
            .collect();

        let new_sessions: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };
                if k == cond {
                    quote! { session#k: new_session , }
                } else {
                    quote! { session#k: self.session#k , }
                }
            })
            .collect();

        let index = if sender >= receiver {
            sender - 1
        } else {
            sender
        };

        let new_session = format_ident!("session{}", index);

        quote! {
            impl<#( #session_types_struct )* R: mpstthree::role::Role, T: std::marker::Send>
                #sessionmpst_name<
                    #( #send_sessions )*
                    #sender_ident<R>,
                    #receiver_ident<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn recv(self) -> Result<(
                    T,
                    #sessionmpst_name<
                        #( #session_types )*
                        R,
                        #receiver_ident<mpstthree::role::end::RoleEnd>
                    >),
                    Box<dyn std::error::Error>
                > {
                    let new_session = mpstthree::binary::send::recv(self.#new_session)?;
                    let new_stack = {
                        fn temp<R>(r: #sender_ident<R>) -> R
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
                        #sessionmpst_name {
                            #( #new_sessions )*
                            stack: new_stack,
                            name: self.name,
                        }
                    ))
                }
            }
        }
    }

    fn expand_recv_from_all(
        &self,
        all_roles: Vec<proc_macro2::TokenStream>,
        receiver: u64,
        sender: u64,
        number_roles: u64,
        sessionmpst_name: syn::Ident,
        session_types: Vec<syn::Ident>,
        session_types_struct: Vec<syn::Ident>,
    ) -> proc_macro2::TokenStream {
        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender).unwrap()) {
            let concatenated_elt = format!("RoleAllto{}", elt);
            syn::Ident::new(&concatenated_elt, proc_macro2::Span::call_site())
        } else {
            panic!("Not enough arguments")
        };

        let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver).unwrap()) {
            let concatenated_elt = format!("Role{}", elt);
            syn::Ident::new(&concatenated_elt, proc_macro2::Span::call_site())
        } else {
            panic!("Not enough arguments")
        };

        let send_sessions: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };
                if k == cond {
                    quote! { mpstthree::binary::struct_trait::Recv<T, S#k> ,}
                } else {
                    quote! { S#k ,}
                }
            })
            .collect();

        let new_sessions: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };
                if k == cond {
                    quote! { session#k: new_session , }
                } else {
                    quote! { session#k: self.session#k , }
                }
            })
            .collect();

        let index = if sender >= receiver {
            sender - 1
        } else {
            sender
        };

        let new_session = format_ident!("session{}", index);

        quote! {
            impl<#( #session_types_struct )* R: mpstthree::role::Role, T: std::marker::Send>
                #sessionmpst_name<
                    #( #send_sessions )*
                    #sender_ident<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    #receiver_ident<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn recv(self) -> Result<(
                    T,
                    #sessionmpst_name<
                        #( #session_types )*
                        mpstthree::role::end::RoleEnd,
                        #receiver_ident<mpstthree::role::end::RoleEnd>
                    >),
                    Box<dyn std::error::Error>
                > {
                    let new_session = mpstthree::binary::send::recv(self.#new_session)?;
                    let (here1, there1) = <mpstthree::role::end::RoleEnd as mpstthree::role::Role>::new();
                    let (_here2, there2) = <mpstthree::role::end::RoleEnd as mpstthree::role::Role>::new();
                    self.stack.sender1.send(there1).unwrap_or(());
                    self.stack.sender2.send(there2).unwrap_or(());

                    Ok((
                        v,
                        #sessionmpst_name {
                            #( #new_sessions )*
                            stack: new_stack,
                            name: self.name,
                        }
                    ))
                }
            }
        }
    }

    fn expand_offer(
        &self,
        all_roles: Vec<proc_macro2::TokenStream>,
        sender: u64,
        receiver: u64,
        number_roles: u64,
        sessionmpst_name: syn::Ident,
        session_types: Vec<syn::Ident>,
        session_types_struct: Vec<syn::Ident>,
    ) -> proc_macro2::TokenStream {
        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender).unwrap()) {
            let concatenated_elt = format!("RoleAllto{}", elt);
            syn::Ident::new(&concatenated_elt, proc_macro2::Span::call_site())
        } else {
            panic!("Not enough arguments")
        };

        let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver).unwrap()) {
            let concatenated_elt = format!("Role{}", elt);
            syn::Ident::new(&concatenated_elt, proc_macro2::Span::call_site())
        } else {
            panic!("Not enough arguments")
        };

        let offer_session_types_struct: Vec<syn::Ident> = (1..number_roles)
            .map(|i| {
                syn::Ident::new(
                    &format!("S{} : mpstthree::binary::struct_trait::Session,", i),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect();

        let left_sessions: Vec<syn::Ident> = (1..number_roles)
            .map(|i| syn::Ident::new(&format!("S{} ,", i), proc_macro2::Span::call_site()))
            .collect();

        let right_sessions: Vec<syn::Ident> = (number_roles..(2 * number_roles - 1))
            .map(|i| syn::Ident::new(&format!("S{} ,", i), proc_macro2::Span::call_site()))
            .collect();

        let offer_sessions: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };
                if k == cond {
                    quote! {
                        mpstthree::binary::struct_trait::Recv<
                            either::Either<
                                #sessionmpst_name<
                                    #( #left_sessions )*
                                    R1,
                                    #receiver_ident<mpstthree::role::end::RoleEnd>
                                >,
                                #sessionmpst_name<
                                    #( #right_sessions )*
                                    R2,
                                    #receiver_ident<mpstthree::role::end::RoleEnd>
                                >
                            >,
                            mpstthree::binary::struct_trait::End
                        >,
                    }
                } else {
                    quote! { mpstthree::binary::struct_trait::End, }
                }
            })
            .collect();

        let new_sessions: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };
                if k == cond {
                    quote! { session#k: new_session , }
                } else {
                    quote! { session#k: self.session#k , }
                }
            })
            .collect();

        let index = if sender >= receiver {
            sender - 1
        } else {
            sender
        };

        let new_session = format_ident!("session{}", index);

        quote! {
            impl<
                'a,
                #( #offer_session_types_struct )*
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
            >
                #sessionmpst_name<
                    #( #offer_sessions )*
                    #sender_ident<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    #receiver_ident<mpstthree::role::end::RoleEnd>,
                >
            {
                pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn std::error::Error + 'a>>
                where
                    F: FnOnce(
                        #sessionmpst_name<
                            #( #left_sessions )*
                            R1,
                            #receiver_ident<mpstthree::role::end::RoleEnd>,
                        >,
                    ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                    G: FnOnce(
                        #sessionmpst_name<
                            #( #right_sessions )*
                            R2,
                            #receiver_ident<mpstthree::role::end::RoleEnd>,
                        >,
                    ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                {
                    let (e, s) = self.recv()?;
                    mpstthree::binary::cancel::cancel(s);
                    e.either(f, g)
                }
            }
        }
    }

    fn expand_choose(
        &self,
        all_roles: Vec<proc_macro2::TokenStream>,
        sender: u64,
        receiver: u64,
        number_roles: u64,
        sessionmpst_name: syn::Ident,
        session_types: Vec<syn::Ident>,
        session_types_struct: Vec<syn::Ident>,
    ) -> proc_macro2::TokenStream {
        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender).unwrap()) {
            let concatenated_elt = format!("RoleAllto{}", elt);
            syn::Ident::new(&concatenated_elt, proc_macro2::Span::call_site())
        } else {
            panic!("Not enough arguments")
        };

        let receiver_ident = if let Some(elt) = all_roles.get(usize::try_from(receiver).unwrap()) {
            let concatenated_elt = format!("Role{}", elt);
            syn::Ident::new(&concatenated_elt, proc_macro2::Span::call_site())
        } else {
            panic!("Not enough arguments")
        };

        let offer_session_types_struct: Vec<syn::Ident> = (1..number_roles)
            .map(|i| {
                syn::Ident::new(
                    &format!("S{} : mpstthree::binary::struct_trait::Session,", i),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect();

        let left_sessions: Vec<syn::Ident> = (1..number_roles)
            .map(|i| syn::Ident::new(&format!("S{} ,", i), proc_macro2::Span::call_site()))
            .collect();

        let right_sessions: Vec<syn::Ident> = (number_roles..(2 * number_roles - 1))
            .map(|i| syn::Ident::new(&format!("S{} ,", i), proc_macro2::Span::call_site()))
            .collect();

        let offer_sessions: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };
                if k == cond {
                    quote! {
                        mpstthree::binary::struct_trait::Recv<
                            either::Either<
                                #sessionmpst_name<
                                    #( #left_sessions )*
                                    R1,
                                    #receiver_ident<mpstthree::role::end::RoleEnd>
                                >,
                                #sessionmpst_name<
                                    #( #right_sessions )*
                                    R2,
                                    #receiver_ident<mpstthree::role::end::RoleEnd>
                                >
                            >,
                            mpstthree::binary::struct_trait::End
                        >,
                    }
                } else {
                    quote! { mpstthree::binary::struct_trait::End, }
                }
            })
            .collect();

        let new_sessions: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };
                if k == cond {
                    quote! { session#k: new_session , }
                } else {
                    quote! { session#k: self.session#k , }
                }
            })
            .collect();

        let index = if sender >= receiver {
            sender - 1
        } else {
            sender
        };

        let new_session = format_ident!("session{}", index);

        quote! {
            impl<
                #(
                    S#N:0: mpstthree::binary::struct_trait::Session,
                )25:0
                #(
                    R#N:0: mpstthree::role::Role,
                )26:0
            >
                #sessionmpst_name<
                    ~(
                    )(
                        mpstthree::binary::struct_trait::Send<
                            either::Either<
                                #sessionmpst_name<
                                    |(
                                        S|N:1,
                                    )(
                                        <S|N:1 as mpstthree::binary::struct_trait::Session>::Dual,
                                    )2*
                                    R~N:23,
                                    unused~N:9<mpstthree::role::end::RoleEnd>
                                >,
                                #sessionmpst_name<
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
                pub fn choose_left(self) -> #sessionmpst_name<
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
                        let choice_~N:22 = #sessionmpst_name {
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
                    let s = #sessionmpst_name {
                        #(
                            session#N:0: new_session_#N:0,
                        )20:0
                        stack: self.stack,
                        name: self.name,
                    };
                    mpstthree::binary::cancel::cancel(s);
                    #sessionmpst_name {
                        ~(
                        )(
                            session~N:22: channel_~N:31,
                        )7*
                        stack: stack_^N:14,
                        name: name_^N:14,
                    }
                }
                pub fn choose_right(self) -> #sessionmpst_name<
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
                        let choice_~N:22 = #sessionmpst_name {
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
                    let s = #sessionmpst_name {
                        #(
                            session#N:0: new_session_#N:0,
                        )20:0
                        stack: self.stack,
                        name: self.name,
                    };
                    mpstthree::binary::cancel::cancel(s);
                    #sessionmpst_name {
                        ~(
                        )(
                            session~N:22: channel_~N:31,
                        )7*
                        stack: stack_^N:14,
                        name: name_^N:14,
                    }
                }
            }
        }
    }

    fn expand_role(&self, role: String) -> proc_macro2::TokenStream {
        // role
        let concatenated_role = format!("Role{}", role);
        let role_name = syn::Ident::new(&concatenated_role, proc_macro2::Span::call_site());
        // dual
        let concatenated_dual = format!("Role{}Dual", role);
        let dual_name = syn::Ident::new(&concatenated_dual, proc_macro2::Span::call_site());
        // role to all
        let concatenated_role_to_all_name = format!("Role{}toAll", role);
        let role_to_all_name = syn::Ident::new(
            &concatenated_role_to_all_name,
            proc_macro2::Span::call_site(),
        );
        // dual to all
        let concatenated_dual_to_all = format!("RoleAllto{}", role);
        let dual_to_all_name =
            syn::Ident::new(&concatenated_dual_to_all, proc_macro2::Span::call_site());

        quote! {
            ////////////////////////////////////////////
            /// The normal Role
            #[derive(Debug)]
            struct #role_name<R>
            where
                R: mpstthree::role::Role,
                R::Dual: mpstthree::role::Role,
            {
                sender: crossbeam_channel::Sender<R::Dual>,
            }
            ////////////////////////////////////////////
            /// The normal Dual
            #[derive(Debug)]
            struct #dual_name<R>
            where
                R: mpstthree::role::Role,
                R::Dual: mpstthree::role::Role,
            {
                sender: crossbeam_channel::Sender<R::Dual>,
            }
            ////////////////////////////////////////////
            /// The normal Role implementation of Role
            impl<R: mpstthree::role::Role> mpstthree::role::Role for #role_name<R> {
                type Dual = #dual_name<<R as mpstthree::role::Role>::Dual>;
                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                    let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
                    (
                        #role_name {
                            sender: sender_dual,
                        },
                        #dual_name {
                            sender: sender_normal,
                        },
                    )
                }
                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#role_name))
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
            impl<R: mpstthree::role::Role> mpstthree::role::Role for #dual_name<R> {
                type Dual = #role_name<<R as mpstthree::role::Role>::Dual>;
                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                    let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
                    (
                        #dual_name {
                            sender: sender_dual,
                        },
                        #role_name {
                            sender: sender_normal,
                        },
                    )
                }
                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#dual_name))
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
            struct #role_to_all_name<R1, R2>
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
            struct #dual_to_all_name<R1, R2>
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
                for #role_to_all_name<R1, R2>
            {
                type Dual = #dual_to_all_name<
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
                        #role_to_all_name {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        #dual_to_all_name {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
                        },
                    )
                }
                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#role_to_all_name))
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
                for #dual_to_all_name<R1, R2>
            {
                type Dual = #role_to_all_name<
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
                        #dual_to_all_name {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        #role_to_all_name {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
                        },
                    )
                }
                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#dual_to_all_name))
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
        }
    }

    fn expand(&self) -> proc_macro2::TokenStream {
        let sessionmpst_name = self.sessionmpst_name.clone();

        // Get all the roles provided into a Vec
        let all_roles = self.expand_stream(&self.roles.clone());

        // Get the first role, panic if it does not exist
        let first_role = if let Some(elt) = all_roles.get(usize::try_from(0).unwrap()) {
            format!("{}", elt)
        } else {
            panic!("Not enough arguments")
        };

        let number_roles = all_roles.len().to_string().parse::<u64>().unwrap();

        let session_types: Vec<syn::Ident> =
            (1..number_roles).map(|i| format_ident!("S{}", i)).collect();

        let session_types_struct: Vec<syn::Ident> = (1..number_roles)
            .map(|i| {
                syn::Ident::new(
                    &format!("S{} : mpstthree::binary::struct_trait::Session,", i),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect();

        let session_types_pub: Vec<syn::Ident> = (1..number_roles)
            .map(|i| {
                syn::Ident::new(
                    &format!("pub session{}: S{},", i, i),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect();

        let sender_receiver: Vec<syn::Ident> = (1..number_roles)
            .map(|i| {
                syn::Ident::new(
                    &format!("let (sender{}, receiver{}) = S{}::new();", i, i, i),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect();

        let sender_struct: Vec<syn::Ident> = (1..number_roles)
            .map(|i| {
                syn::Ident::new(
                    &format!("session{}: sender{},", i, i),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect();

        let receiver_struct: Vec<syn::Ident> = (1..number_roles)
            .map(|i| {
                syn::Ident::new(
                    &format!("session{}: receiver{},", i, i),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect();

        let head_str: Vec<syn::Ident> = (1..number_roles)
            .map(|i| {
                syn::Ident::new(
                    &format!(
                        "result = format!(\"{{}} + {{}}\", result, S{}::head_str());",
                        i
                    ),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect();

        let stringify: Vec<syn::Ident> = (1..number_roles)
            .map(|i| {
                syn::Ident::new(
                    &format!("stringify!(session{}),", i),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect();

        let roles_struct: Vec<proc_macro2::TokenStream> = all_roles
            .iter()
            .map(|i| self.expand_role(format!("{}", i)))
            .collect();

        let send_methods: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|sender| {
                (1..number_roles)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            std::option::Option::Some(self.expand_send(
                                all_roles,
                                sender - 1,
                                receiver - 1,
                                number_roles,
                                sessionmpst_name,
                                session_types,
                                session_types_struct,
                            ))
                        } else {
                            std::option::Option::None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_methods: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|receiver| {
                (1..number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            std::option::Option::Some(self.expand_recv(
                                all_roles,
                                receiver - 1,
                                sender - 1,
                                number_roles,
                                sessionmpst_name,
                                session_types,
                                session_types_struct,
                            ))
                        } else {
                            std::option::Option::None
                        }
                    })
                    .collect()
            })
            .collect();

        let offer_methods: Vec<proc_macro2::TokenStream> = (1..number_roles)
            .map(|receiver| {
                (1..number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            std::option::Option::Some(self.expand_offer(
                                all_roles,
                                sender - 1,
                                receiver - 1,
                                number_roles,
                                sessionmpst_name,
                                session_types,
                                session_types_struct,
                            ))
                        } else {
                            std::option::Option::None
                        }
                    })
                    .collect()
            })
            .collect();

        quote! {
            #[must_use]
            #[derive(Debug)]
            pub struct #sessionmpst_name<
                #( #session_types , )*
                R,
                N
            >
            where
                #( #session_types_struct )*
                R: mpstthree::role::Role,
                N: mpstthree::role::Role
            {
                #( #session_types_pub )*
                pub stack: R,
                pub name: N,
            }
            #[doc(hidden)]
            impl<
                #( #session_types_struct )*
                R: mpstthree::role::Role,
                N: mpstthree::role::Role
            > mpstthree::binary::struct_trait::Session for #sessionmpst_name< #( #session_types , )* R, N> {
                type Dual =
                #sessionmpst_name<#( #session_types_struct )* <R as mpstthree::role::Role>::Dual, <N as mpstthree::role::Role>::Dual, >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    #( #sender_receiver )*

                    let (role_one, role_two) = R::new();
                    let (name_one, name_two) = N::new();
                    (
                        #sessionmpst_name {
                            #( #sender_struct )*
                            stack: role_one,
                            name: name_one,
                        },
                        #sessionmpst_name {
                            #( #receiver_struct )*
                            stack: role_two,
                            name: name_two,
                        }
                    )
                }
                #[doc(hidden)]
                fn head_str() -> String {
                    let mut result = String::from("");
                    #( #head_str )*
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
                    #( #head_str )*
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
                    #( #session_types_struct )*
                    R: mpstthree::role::Role,
                    N: mpstthree::role::Role
                > #sessionmpst_name<#( #session_types , )* R, N> {
                #[doc(hidden)]
                pub fn field_names(self) ->
                    (
                        &'static [&'static str],
                        #sessionmpst_name<#( #session_types , )* R, N>
                    ) {
                    (
                        &[
                            #( #stringify )*
                        ],
                        self
                    )
                }
            }


            #( #roles_struct )*

            #( #send_methods )*

            #( #recv_methods )*

            #( #offer_methods )*







            #(
                impl<
                    #(
                        S#N:0: mpstthree::binary::struct_trait::Session,
                    )25:0
                    #(
                        R#N:0: mpstthree::role::Role,
                    )26:0
                >
                    #sessionmpst_name<
                        ~(
                        )(
                            mpstthree::binary::struct_trait::Send<
                                either::Either<
                                    #sessionmpst_name<
                                        |(
                                            S|N:1,
                                        )(
                                            <S|N:1 as mpstthree::binary::struct_trait::Session>::Dual,
                                        )2*
                                        R~N:23,
                                        unused~N:9<mpstthree::role::end::RoleEnd>
                                    >,
                                    #sessionmpst_name<
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
                    pub fn choose_left(self) -> #sessionmpst_name<
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
                            let choice_~N:22 = #sessionmpst_name {
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
                        let s = #sessionmpst_name {
                            #(
                                session#N:0: new_session_#N:0,
                            )20:0
                            stack: self.stack,
                            name: self.name,
                        };
                        mpstthree::binary::cancel::cancel(s);
                        #sessionmpst_name {
                            ~(
                            )(
                                session~N:22: channel_~N:31,
                            )7*
                            stack: stack_^N:14,
                            name: name_^N:14,
                        }
                    }
                    pub fn choose_right(self) -> #sessionmpst_name<
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
                            let choice_~N:22 = #sessionmpst_name {
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
                        let s = #sessionmpst_name {
                            #(
                                session#N:0: new_session_#N:0,
                            )20:0
                            stack: self.stack,
                            name: self.name,
                        };
                        mpstthree::binary::cancel::cancel(s);
                        #sessionmpst_name {
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
                    #sessionmpst_name<
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
        }
    }
}
