#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::usize;
use syn::parse_macro_input;

mod macros;

use macros::roles::create_broadcast_role_short::CreateBroadcastRoleShort;
use macros::roles::create_normal_role_short::CreateNormalRoleShort;
use macros::offer::create_offer_mpst_session_multi::OfferMPSTSessionMulti;
use macros::recv::create_recv_mpst_session::CreateRecvMPSTSession;
use macros::send::create_send_mpst_session::CreateSendMPSTSession;

//////////////////////////////////////

#[proc_macro]
pub fn create_normal_role_short(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateNormalRoleShort);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_normal_role_short(input: TokenStream) -> TokenStream {
    create_normal_role_short(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_broadcast_role_short(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateBroadcastRoleShort);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_broadcast_role_short(input: TokenStream) -> TokenStream {
    create_broadcast_role_short(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_offer_mpst_session_multi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as OfferMPSTSessionMulti);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_offer_mpst_session_multi(input: TokenStream) -> TokenStream {
    create_offer_mpst_session_multi(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_recv_mpst_session(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateRecvMPSTSession);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_recv_mpst_session(input: TokenStream) -> TokenStream {
    create_recv_mpst_session(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_send_mpst_session(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateSendMPSTSession);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_send_mpst_session(input: TokenStream) -> TokenStream {
    create_send_mpst_session(input)
}
