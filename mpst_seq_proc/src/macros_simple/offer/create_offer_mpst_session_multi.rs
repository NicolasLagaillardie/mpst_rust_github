use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Result, Token};

#[derive(Debug)]
pub struct OfferMPSTSessionMulti {
    func_name: Ident,
    type_name: Ident,
    role: Ident,
    name: Ident,
    meshedchannels_name: Ident,
    n_sessions: u64,
    exclusion: u64,
}

impl Parse for OfferMPSTSessionMulti {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let type_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let role = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();
        <Token![,]>::parse(input)?;

        let exclusion = (LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(OfferMPSTSessionMulti {
            func_name,
            type_name,
            role,
            name,
            meshedchannels_name,
            n_sessions,
            exclusion,
        })
    }
}

impl From<OfferMPSTSessionMulti> for TokenStream {
    fn from(input: OfferMPSTSessionMulti) -> TokenStream {
        input.expand()
    }
}

impl OfferMPSTSessionMulti {
    fn expand(&self) -> TokenStream {
        let func_name = self.func_name.clone();
        let type_name = self.type_name.clone();
        let role = self.role.clone();
        let name = self.name.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();

        let all_sessions: Vec<TokenStream> = (1..(2 * self.n_sessions - 1))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let all_sessions_struct: Vec<TokenStream> = (1..(2 * self.n_sessions - 1))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::session::Session,
                }
            })
            .collect();

        let new_types: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                if i != self.exclusion {
                    quote! {
                        mpstthree::binary::struct_trait::end::End,
                    }
                } else {
                    let temp_all_sessions: Vec<TokenStream> = (1..(2 * self.n_sessions - 1))
                        .map(|i| {
                            let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
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
                            #name
                        >,
                    }
                }
            })
            .collect();

        let sessions_left: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_right: Vec<TokenStream> = (self.n_sessions..(2 * self.n_sessions - 1))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let all_recv: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                if i != self.exclusion {
                    quote! {}
                } else {
                    let temp_ident = Ident::new(&format!("session{}", i), Span::call_site());
                    quote! {
                        let (e, new_session) = mpstthree::binary::recv::recv(s.#temp_ident)?;
                    }
                }
            })
            .collect();

        let new_sessions: Vec<TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident = Ident::new(&format!("session{}", i), Span::call_site());
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
                s: #meshedchannels_name<
                    #(
                        #new_types
                    )*
                    #role<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    #name,
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
                    #meshedchannels_name<
                        #(
                            #sessions_left
                        )*
                        R1,
                        #name,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                G: FnOnce(
                    #meshedchannels_name<
                        #(
                            #sessions_right
                        )*
                        R2,
                        #name,
                    >,
                ) -> Result<U, Box<dyn std::error::Error + 'a>>,
            {
                #(
                    #all_recv
                )*

                let new_stack = s.stack.continuation_left();

                let s = #meshedchannels_name {
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
