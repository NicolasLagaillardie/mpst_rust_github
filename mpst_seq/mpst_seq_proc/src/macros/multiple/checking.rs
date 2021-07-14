use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Result;

#[derive(Debug)]
pub struct CheckingInput {
    session: syn::Type,
}

impl Parse for CheckingInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let session = syn::Type::parse(input)?;

        println!("{:?}", session.clone());
        println!("");

        // match session.clone() {
        //     syn::Expr::Group(group) => match *group.expr {
        //         syn::Expr::MethodCall(method) => match *method.receiver {
        //             syn::Expr::Path(path) => match path.path {
        //                 syn::Path {
        //                     segments,
        //                     ..
        //                 } => println!("{:?}", segments),
        //             },
        //             _ => println!("sad3"),
        //         },
        //         _ => println!("sad2"),
        //     },
        //     syn::Expr::Struct(expr) => println!("{:?}", expr),
        //     _ => println!("sad1"),
        // };

        // println!("");

        Ok(CheckingInput { session })
    }
}

impl From<CheckingInput> for proc_macro2::TokenStream {
    fn from(input: CheckingInput) -> proc_macro2::TokenStream {
        input.expand()
    }
}

impl CheckingInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let _ = self.session.clone();

        quote! {}
    }
}
