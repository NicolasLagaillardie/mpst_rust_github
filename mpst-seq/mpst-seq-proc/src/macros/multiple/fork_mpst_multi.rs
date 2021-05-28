use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct ForkMPSTMultiMacroInput {
    func_name: syn::Ident,
    sessionmpst_name: syn::Ident,
    nsessions: syn::LitInt,
}

impl Parse for ForkMPSTMultiMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let sessionmpst_name = syn::Ident::parse(input)?;
        <Token![,]>::parse(input)?;

        let nsessions = syn::LitInt::parse(input)?;

        Ok(ForkMPSTMultiMacroInput {
            func_name,
            sessionmpst_name,
            nsessions,
        })
    }
}

impl From<ForkMPSTMultiMacroInput> for proc_macro2::TokenStream {
    fn from(input: ForkMPSTMultiMacroInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl ForkMPSTMultiMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let func_name = self.func_name.clone();
        let sessionmpst_name = self.sessionmpst_name.clone();
        let nsessions = (self.nsessions).base10_parse::<usize>().unwrap();

        let sessions: Vec<proc_macro2::TokenStream> = (1..nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_struct: Vec<proc_macro2::TokenStream> = (1..nsessions)
            .map(|i| {
                let temp_ident =
                    syn::Ident::new(&format!("S{}", i), proc_macro2::Span::call_site());
                quote! {
                    #temp_ident : mpstthree::binary::struct_trait::Session + 'static,
                }
            })
            .collect();

        quote! {
            fn $func_name<
                #(
                    S#K:0,
                )14:0
                #(
                    R#K:0,
                )0:0
                #(
                    N#K:0,
                )0:0
                #(
                    F#K:0,
                )0:0
            >(
                #(
                    f#K:0: F#K:0,
                )0:0
            ) -> (
                #(
                    std::thread::JoinHandle<()>,
                )0:0
            )
            where
                #(
                    R#K:0: mpstthree::role::Role + 'static,
                )0:0
                #(
                    N#K:0: mpstthree::role::Role + 'static,
                )0:0
                #(
                    S#K:0: mpstthree::binary::struct_trait::Session + 'static,
                )14:0
                #(
                    F#K:0: FnOnce($sessionmpst_name<
                        ~(
                            S~K:6,
                        )(
                            <S~K:6 as mpstthree::binary::struct_trait::Session>::Dual,
                        )4*
                        R#K:0, N#K:0>) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                )0:0
            {
                #(
                    let (channel_#K:12, channel_#K:13) = <S#K:0 as mpstthree::binary::struct_trait::Session>::new(); // channel_(get from matrix), channel_(opposite get from matrix) = S(i)
                )14:0

                #(
                    let (role_#K:0, _) = R#K:0::new();
                )0:0

                #(
                    let (name_#K:0, _) = N#K:0::new();
                )0:0

                #(
                    let sessionmpst_#K:0 = $sessionmpst_name {
                        ~(
                            session#K:1 : channel_~K:5,
                        )(
                            session#K:1 : channel_~K:5,
                        )4*
                        stack: role_#K:0,
                        name: name_#K:0,
                    };
                )0:0

                (
                    #(
                        std::thread::spawn(move || {
                            std::panic::set_hook(Box::new(|_info| {
                                // do nothing
                            }));
                            match f#K:0(sessionmpst_#K:0) {
                                Ok(()) => (),
                                Err(e) => panic!("{:?}", e),
                            }
                        }),
                    )0:0
                )
            }
        }
    }
}
