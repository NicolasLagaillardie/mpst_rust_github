#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::usize;
use syn::parse_macro_input;

mod close_mpst_interleaved;

use close_mpst_interleaved::CloseMpstInterleaved;

mod fork_mpst_multi_interleaved;

use fork_mpst_multi_interleaved::ForkMPSTMultiInterleaved;

//////////////////////////////////////

#[proc_macro]
pub fn fork_mpst_multi_interleaved(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ForkMPSTMultiInterleaved);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_fork_mpst_multi_interleaved(input: TokenStream) -> TokenStream {
    fork_mpst_multi_interleaved(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn close_mpst_interleaved(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CloseMpstInterleaved);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_close_mpst_interleaved(input: TokenStream) -> TokenStream {
    close_mpst_interleaved(input)
}
