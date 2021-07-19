use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::Result;

#[derive(Debug)]
pub struct CheckingInput {
    choices: HashMap<String, Vec<String>>,
}

fn attempt_extraction(input: ParseStream) -> Result<Vec<String>> {
    let content;
    let _parentheses = syn::braced!(content in input);
    let token_stream = TokenStream::parse(&content)?;

    let mut temp_result: Vec<TokenStream> = Vec::new();
    for tt in token_stream.into_iter() {
        let elt = match tt {
            proc_macro2::TokenTree::Group(g) => Some(g.stream()),
            _ => None,
        };
        if let Some(elt_tt) = elt {
            temp_result.push(elt_tt)
        }
    }

    let mut result = Vec::new();
    for elt in temp_result {
        let ident: Ident = syn::parse2(elt)?;
        result.push(ident.to_string());
    }

    Ok(result)
}

impl Parse for CheckingInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let state_branches = RandomState::new();
        let mut choices: HashMap<String, Vec<String>> = HashMap::with_hasher(state_branches);

        loop {
            match attempt_extraction(input) {
                Ok(result) => choices.insert(result[0].clone(), result[1..].to_vec()),
                Err(_) => break,
            };
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
        let choices = self.choices.clone();

        let mut display: Vec<proc_macro2::TokenStream> = Vec::new();
        let mut new_hashmap: Vec<proc_macro2::TokenStream> = Vec::new();

        for (key, value) in choices {
            let name_key = syn::Ident::new(&key, proc_macro2::Span::call_site());
            let fn_key = syn::Ident::new(&key.to_lowercase(), proc_macro2::Span::call_site());

            let branches: Vec<proc_macro2::TokenStream> = value
                .iter()
                .map(|branch| {
                    let branch_ident = syn::Ident::new(&branch, proc_macro2::Span::call_site());
                    quote! {
                        #name_key::#branch_ident(s) => {
                            write!(f, "{}\n{}", stringify!(#branch_ident), type_of(&s))
                        }
                    }
                })
                .collect();

            display.push(quote! {
                impl std::fmt::Display for #name_key {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        match self {
                            #( #branches )*
                        }
                    }
                }
            });

            let branches_hashmap: Vec<proc_macro2::TokenStream> = value.iter()
            .map(|branch| {
                let temp = syn::Ident::new(&format!("temp_{}", branch).to_lowercase(), proc_macro2::Span::call_site());
                let branch_ident = syn::Ident::new(&branch, proc_macro2::Span::call_site());
                let branch_name = syn::Ident::new(&branch.to_lowercase(), proc_macro2::Span::call_site());
                quote! {
                    let #temp =
                        (#name_key::#branch_ident(<_ as mpstthree::binary::struct_trait::Session>::new().0))
                            .to_string();

                    let #branch_name = #temp
                        .split('\n')
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>();

                    all_branches.insert(String::from(#branch_name[0]), String::from(#branch_name[1]));
                }
            })
            .collect();

            new_hashmap.push(quote! {
                fn #fn_key() -> std::collections::HashMap<String, String> {
                    let state_branches_receivers = std::collections::hash_map::RandomState::new();
                    let mut all_branches: std::collections::HashMap<String, String> =
                        std::collections::HashMap::with_hasher(state_branches_receivers);

                    #( #branches_hashmap )*

                    all_branches
                }

                branches_receivers.insert(String::from(stringify!(#name_key)), #fn_key());
            });
        }

        quote! {
            let state_branches = std::collections::hash_map::RandomState::new();
            let mut branches_receivers: std::collections::HashMap<String, std::collections::HashMap<String, String>> =
                std::collections::HashMap::with_hasher(state_branches);

            #( #display )*
            #( #new_hashmap )*
        }
    }
}
