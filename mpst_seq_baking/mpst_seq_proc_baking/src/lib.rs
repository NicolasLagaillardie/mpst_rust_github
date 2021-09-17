#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::usize;
use syn::parse_macro_input;

mod macros;

use macros::baking::Baking;
use macros::baking_with_cancel::BakingWithCancel;
use macros::baking_with_enum::BakingWithEnum;
use macros::baking_with_enum_and_cancel::BakingWithEnumAndCancel;

//////////////////////////////////////

#[proc_macro]
pub fn baking(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Baking);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_baking(input: TokenStream) -> TokenStream {
    baking(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn baking_with_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BakingWithEnum);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_baking_with_enum(input: TokenStream) -> TokenStream {
    baking_with_enum(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn baking_with_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BakingWithCancel);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_baking_with_cancel(input: TokenStream) -> TokenStream {
    baking_with_cancel(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn baking_with_enum_and_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BakingWithEnumAndCancel);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_baking_with_enum_and_cancel(input: TokenStream) -> TokenStream {
    baking_with_enum_and_cancel(input)
}
