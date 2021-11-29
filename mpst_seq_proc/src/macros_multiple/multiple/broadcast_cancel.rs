use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

#[derive(Debug)]
pub struct BroadcastCancel {
    session: syn::Expr,
    n_sessions: syn::LitInt,
}

impl Parse for BroadcastCancel {
    fn parse(input: ParseStream) -> Result<Self> {
        let session = syn::Expr::parse(input)?;
        <Token![,]>::parse(input)?;

        let n_sessions = syn::LitInt::parse(input)?;

        Ok(BroadcastCancel {
            session,
            n_sessions,
        })
    }
}

impl From<BroadcastCancel> for proc_macro2::TokenStream {
    fn from(input: BroadcastCancel) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl BroadcastCancel {
    fn expand(&self) -> proc_macro2::TokenStream {
        let session = self.session.clone();
        let n_sessions = (self.n_sessions).base10_parse::<usize>().unwrap();

        let bool_session: Vec<syn::Ident> = (1..n_sessions)
            .map(|i| format_ident!("bool_session{}", i))
            .collect();

        let field_session: Vec<syn::Ident> = (1..n_sessions)
            .map(|i| format_ident!("session{}", i))
            .collect();

        let send_sessions = quote! { #( s.#field_session.sender.send(mpstthree::binary::struct_trait::end::Signal::Cancel).unwrap_or(()); )* };

        quote! {
            {
                #(
                    let mut #bool_session = true;
                )*

                let mut s = #session;

                let (size, mut s) = s.field_names();

                if size.len() != #n_sessions - 1 {
                    panic!("Wrong number for $n_sessions: expected {:?}, found {:?}", size.len(), #n_sessions)
                } else {
                    while #( #bool_session || )* false {
                        #(
                            match s.#field_session.receiver.try_recv() {
                                Ok(mpstthree::binary::struct_trait::end::Signal::Cancel) => {
                                    #send_sessions
                                    mpstthree::binary::cancel::cancel(s);
                                    panic!("Error");
                                }
                                Ok(mpstthree::binary::struct_trait::end::Signal::Stop) => match #bool_session {
                                    true => {
                                        s.#field_session.sender.send(mpstthree::binary::struct_trait::end::Signal::Stop).unwrap_or(());
                                        #bool_session = false;
                                    }
                                    false => panic!("Close already sent"),
                                }
                                Ok(mpstthree::binary::struct_trait::end::Signal::Offer(channel)) => {
                                    s.#field_session = channel;
                                }
                                _ => {}
                            };
                        )*
                    }
                };

                Ok(())
            }
        }
    }
}
