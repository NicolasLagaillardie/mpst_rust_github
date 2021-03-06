use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct CloseMpstCheckCancelMacroInput {
    func_name: syn::Ident,
    meshedchannels_name: syn::Ident,
    nsessions: u64,
}

impl Parse for CloseMpstCheckCancelMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let meshedchannels_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CloseMpstCheckCancelMacroInput {
            func_name,
            meshedchannels_name,
            nsessions,
        })
    }
}

impl From<CloseMpstCheckCancelMacroInput> for proc_macro2::TokenStream {
    fn from(input: CloseMpstCheckCancelMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CloseMpstCheckCancelMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let func_name = self.func_name.clone();
        let meshedchannels_name = self.meshedchannels_name.clone();

        let session_types: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|_| {
                quote! { mpstthree::binary::struct_trait::End , }
            })
            .collect();

        let session_send: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
                .map(|i| {
                    let temp_ident =
                        syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                    quote! {
                        s.#temp_ident.sender.send(mpstthree::binary::struct_trait::Signal::Stop).unwrap_or(());
                    }
                })
                .collect();

        let session_recv: Vec<proc_macro2::TokenStream> = (1..self.nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("session{}", i), proc_macro2::Span::call_site());
                quote! {
                    match s.#temp_ident.receiver.recv() {
                        Ok(mpstthree::binary::struct_trait::Signal::Stop) => {},
                        Ok(mpstthree::binary::struct_trait::Signal::Cancel) => panic!("Received a cancel signal"),
                        Ok(mpstthree::binary::struct_trait::Signal::Offer(_)) => {},
                        Err(e) => panic!("{}", e.to_string()),
                    };
                }
            })
            .collect();

        quote! {
            fn #func_name<R>(
                s: #meshedchannels_name<
                    #(
                        #session_types
                    )*
                    mpstthree::role::end::RoleEnd,
                    R
                >
            ) -> Result<(), Box<dyn std::error::Error>>
            where
                R: mpstthree::role::Role,
            {
                #(
                    #session_send
                )*

                #(
                    #session_recv
                )*

                Ok(())
            }
        }
    }
}
