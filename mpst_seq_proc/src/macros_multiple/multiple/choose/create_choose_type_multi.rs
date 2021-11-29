use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct ChooseTypeMulti {
    type_name: syn::Ident,
    meshedchannels_name: syn::Ident,
    n_sessions: u64,
}

impl Parse for ChooseTypeMulti {
    fn parse(input: ParseStream) -> Result<Self> {
        let type_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(ChooseTypeMulti {
            type_name,
            meshedchannels_name,
            n_sessions,
        })
    }
}

impl From<ChooseTypeMulti> for proc_macro2::TokenStream {
    fn from(input: ChooseTypeMulti) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl ChooseTypeMulti {
    fn expand(&self) -> proc_macro2::TokenStream {
        let type_name = self.type_name.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();

        let all_sessions: Vec<proc_macro2::TokenStream> = (1..(2 * self.n_sessions - 1))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_left: Vec<proc_macro2::TokenStream> = (1..self.n_sessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_right: Vec<proc_macro2::TokenStream> = (self.n_sessions
            ..(2 * self.n_sessions - 1))
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        quote! {
            type #type_name<
                #(
                    #all_sessions
                )*
                R0,
                R1,
                N0
            > = mpstthree::binary::struct_trait::send::Send<
                either::Either<
                    <
                        #meshedchannels_name<
                            #(
                                #sessions_left
                            )*
                            R0,
                            N0
                        > as mpstthree::binary::struct_trait::session::Session>::Dual,
                    <
                        #meshedchannels_name<
                            #(
                                #sessions_right
                            )*
                            R1,
                            N0
                        > as mpstthree::binary::struct_trait::session::Session>::Dual
                    >,
                mpstthree::binary::struct_trait::end::End
            >;
        }
    }
}
