#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::usize;
use syn::parse_macro_input;

mod macros;

use macros::recv::create_recv_http_session::CreateRecvHttpSession;
use macros::send::create_send_http_session::CreateSendHttpSession;
use macros::choose::choose_mpst_multi_http_to_all::ChooseTypeMultiHttpToAll;

//////////////////////////////////////

#[proc_macro]
pub fn choose_mpst_multi_http_to_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeMultiHttpToAll);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_choose_mpst_multi_http_to_all(input: TokenStream) -> TokenStream {
    choose_mpst_multi_http_to_all(input)
}


//////////////////////////////////////

#[proc_macro]
pub fn create_recv_http_session(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateRecvHttpSession);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_recv_http_session(input: TokenStream) -> TokenStream {
    create_recv_http_session(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_send_http_session(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateSendHttpSession);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_send_http_session(input: TokenStream) -> TokenStream {
    create_send_http_session(input)
}


