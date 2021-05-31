use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct CloseMpstMacroInput {
    func_name: syn::Ident,
    sessionmpst_name: syn::Ident,
    nsessions: u64,
}

impl Parse for CloseMpstMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sessionmpst_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = (syn::LitInt::parse(input)?).base10_parse::<u64>().unwrap();

        Ok(CloseMpstMacroInput {
            func_name,
            sessionmpst_name,
            nsessions,
        })
    }
}

impl From<CloseMpstMacroInput> for proc_macro2::TokenStream {
    fn from(input: CloseMpstMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CloseMpstMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let func_name = self.func_name.clone();
        let sessionmpst_name = self.sessionmpst_name.clone();

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
                    s.#temp_ident.receiver.recv()?;
                }
            })
            .collect();

        quote! {
            fn #func_name<R>(s: #sessionmpst_name<
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
