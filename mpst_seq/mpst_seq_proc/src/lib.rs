#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::usize;
use syn::parse_macro_input;

mod functionmpst;

use functionmpst::recv_all_aux_simple::RecvAllAuxSimple;
use functionmpst::recv_aux_simple::RecvAuxSimple;
use functionmpst::send_aux_simple::SendAuxSimple;

mod choose_mpst_multi_to_all;

use choose_mpst_multi_to_all::ChooseMultiToAll;

//////////////////////////////////////

#[proc_macro]
pub fn recv_aux_simple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RecvAuxSimple);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_recv_aux_simple(input: TokenStream) -> TokenStream {
    recv_aux_simple(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn recv_all_aux_simple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RecvAllAuxSimple);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_recv_all_aux_simple(input: TokenStream) -> TokenStream {
    recv_all_aux_simple(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn send_aux_simple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SendAuxSimple);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_send_aux_simple(input: TokenStream) -> TokenStream {
    send_aux_simple(input)
}


//////////////////////////////////////

#[proc_macro]
pub fn choose_mpst_multi_to_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseMultiToAll);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_choose_mpst_multi_to_all(input: TokenStream) -> TokenStream {
    choose_mpst_multi_to_all(input)
}
