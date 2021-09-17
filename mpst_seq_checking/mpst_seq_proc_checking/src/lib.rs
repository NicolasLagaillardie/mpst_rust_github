#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::usize;
use syn::parse_macro_input;

mod checking;

use checking::CheckingInput;

//////////////////////////////////////

#[proc_macro]
pub fn checking(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CheckingInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_checking(input: TokenStream) -> TokenStream {
    checking(input)
}