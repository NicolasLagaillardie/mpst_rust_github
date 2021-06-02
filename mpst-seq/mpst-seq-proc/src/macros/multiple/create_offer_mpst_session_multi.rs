use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct OfferMPSTSessionMultiMacroInput {
    func_name: syn::Ident,
    type_name: syn::Ident,
    role: syn::Ident,
    name: syn::Ident,
    sessionmpst_name: syn::Ident,
    nsessions: u64,
    exclusion: u64,
}

impl Parse for OfferMPSTSessionMultiMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let type_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let role = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sessionmpst_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let exclusion = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(OfferMPSTSessionMultiMacroInput {
            func_name,
            type_name,
            role,
            name,
            sessionmpst_name,
            nsessions,
            exclusion,
        })
    }
}

impl From<OfferMPSTSessionMultiMacroInput> for proc_macro2::TokenStream {
    fn from(input: OfferMPSTSessionMultiMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl OfferMPSTSessionMultiMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let func_name = self.func_name.clone();
        let type_name = self.type_name.clone();
        let role = self.role.clone();
        let name = self.name.clone();
        let sessionmpst_name = self.sessionmpst_name.clone();

        let all_sessions: Vec<proc_macro2::TokenStream> = (1..(2 * self.nsessions - 1))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let all_sessions_struct: Vec<proc_macro2::TokenStream> = (1..(2 * self.nsessions - 1))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::Session,
                }
            })
            .collect();

        let new_types: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                if i != self.exclusion {
                    quote! {
                        mpstthree::binary::struct_trait::End,
                    }
                } else {
                    let temp_all_sessions: Vec<proc_macro2::TokenStream> = (1..(2 * self
                        .nsessions
                        - 1))
                        .map(|i| {
                            let temp_ident =
                                syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                            quote! {
                                #temp_ident ,
                            }
                        })
                        .collect();

                    quote! {
                        #type_name<
                            #(
                                #temp_all_sessions
                            )*
                            R1,
                            R2,
                            #name<
                                mpstthree::role::end::RoleEnd
                            >
                        >,
                    }
                }
            })
            .collect();

        let sessions_left: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_right: Vec<proc_macro2::TokenStream> = (self.nsessions
            ..(2 * self.nsessions - 1))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let all_recv: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                if i != self.exclusion {
                    quote! {}
                } else {
                    let temp_ident =
                        syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                    quote! {
                        let (e, new_session) = mpstthree::binary::recv::recv(s.#temp_ident)?;
                    }
                }
            })
            .collect();

        let new_sessions: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                if i == self.exclusion {
                    quote! {
                        #temp_ident : new_session ,
                    }
                } else {
                    quote! {
                        #temp_ident : s.#temp_ident ,
                    }
                }
            })
            .collect();

        quote! {
            fn #func_name<
                'a,
                #(
                    #all_sessions
                )*
                F,
                G,
                R1,
                R2,
                U
            >(
                s: #sessionmpst_name<
                    #(
                        #new_types
                    )*
                    #role<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    #name<mpstthree::role::end::RoleEnd>,
                >,
                f: F,
                g: G,
            ) -> Result<U, Box<dyn std::error::Error + 'a>>
            where
                #(
                   #all_sessions_struct
                )*
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
                F: FnOnce(
                    #sessionmpst_name<
                        #(
                            #sessions_left
                        )*
                        R1,
                        #name<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                G: FnOnce(
                    #sessionmpst_name<
                        #(
                            #sessions_right
                        )*
                        R2,
                        #name<mpstthree::role::end::RoleEnd>,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
            {
                #(
                    #all_recv
                )*
                let (new_stack, _) = {
                    fn temp<R1, R2>(r: #role<R1, R2>) -> (R1, R2)
                    where
                        R1: mpstthree::role::Role,
                        R2: mpstthree::role::Role,
                    {
                        let (here1, there1) = <R1 as mpstthree::role::Role>::new();
                        let (here2, there2) = <R2 as mpstthree::role::Role>::new();
                        r.sender1.send(there1).unwrap_or(());
                        r.sender2.send(there2).unwrap_or(());
                        (here1, here2)
                    }
                    temp(s.stack)
                };

                let s = #sessionmpst_name {
                    #(
                        #new_sessions
                    )*
                    stack: new_stack,
                    name: s.name,
                };

                mpstthree::binary::cancel::cancel(s);
                e.either(f, g)
            }
        }
    }
}
