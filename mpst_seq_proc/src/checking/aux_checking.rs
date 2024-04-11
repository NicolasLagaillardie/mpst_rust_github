use proc_macro2::Span;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result};

#[derive(Debug)]
pub(crate) struct CheckingInput {
    choices: HashMap<String, Vec<String>>,
}

fn attempt_extraction(input: ParseStream) -> Result<Vec<String>> {
    let content;
    let _parentheses = syn::braced!(content in input);
    let token_stream = TokenStream::parse(&content)?;

    let mut temp_result: Vec<TokenStream> = Vec::new();
    for tt in token_stream.into_iter() {
        let elt = match tt {
            TokenTree::Group(g) => Some(g.stream()),
            TokenTree::Ident(g) => Some(quote! { #g }),
            _ => None,
        };
        if let Some(elt_tt) = elt {
            temp_result.push(elt_tt)
        }
    }

    let mut result = Vec::new();
    for elt in temp_result {
        // let ident: proc_macro2::Ident = ;
        if let Ok(temp_ident) = syn::parse2(elt) {
            let ident: proc_macro2::Ident = temp_ident;
            result.push(ident.to_string());
        }
    }

    Ok(result)
}

impl Parse for CheckingInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let state_branches = RandomState::new();
        let mut choices: HashMap<String, Vec<String>> = HashMap::with_hasher(state_branches);

        while let Ok(result) = attempt_extraction(input) {
            let vec_to_add = if let Some(temp_vec) = choices.get_mut(&result[0]) {
                temp_vec.append(&mut result[1..].to_vec());
                temp_vec.to_vec()
            } else {
                result[1..].to_vec()
            };
            choices.insert(result[0].to_string(), vec_to_add);
        }

        Ok(CheckingInput { choices })
    }
}

impl From<CheckingInput> for TokenStream {
    fn from(input: CheckingInput) -> TokenStream {
        input.expand()
    }
}

impl CheckingInput {
    fn expand(&self) -> TokenStream {
        let mut branches_receivers_hashmap: Vec<proc_macro2::TokenStream> = Vec::new();

        for key in self.choices.keys() {
            let name_key = Ident::new(key, Span::call_site());
            let fn_key = Ident::new(&key.to_lowercase(), Span::call_site());

            branches_receivers_hashmap.push(quote! {
                branches_receivers.insert(String::from(stringify!(#name_key)), #fn_key());
            });
        }

        quote! {
            let state_branches = std::collections::hash_map::RandomState::new();
            let mut branches_receivers: std::collections::HashMap<String, std::collections::HashMap<String, String>> =
                std::collections::HashMap::with_hasher(state_branches);

            #( #branches_receivers_hashmap )*
        }
    }
}
