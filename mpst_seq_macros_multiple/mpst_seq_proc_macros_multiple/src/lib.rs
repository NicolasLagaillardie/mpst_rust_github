#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::usize;
use syn::parse_macro_input;

mod macros;

use macros::create_broadcast_role_short::CreateBroadcastRoleShort;
use macros::create_normal_role_short::CreateNormalRoleShort;
use macros::multiple::broadcast_cancel::BroadcastCancel;
use macros::multiple::choose::choose_mpst_create_multi_to_all::ChooseMultiCreateToAll;
use macros::multiple::choose::choose_mpst_multi_cancel_to_all::ChooseTypeMultiCancelToAll;
use macros::multiple::choose::create_choose_mpst_session_multi_left::ChooseTypeMultiLeft;
use macros::multiple::choose::create_choose_mpst_session_multi_right::ChooseTypeMultiRight;
use macros::multiple::choose::create_choose_type_multi::ChooseTypeMulti;
use macros::multiple::choose::create_fn_choose_mpst_cancel_multi_to_all_bundle::ChooseTypeCancelMultiToAllBundle;
use macros::multiple::choose::create_fn_choose_mpst_multi_to_all_bundle::ChooseTypeMultiToAllBundle;
use macros::multiple::close::close_mpst::CloseMpst;
use macros::multiple::close::close_mpst_cancel::CloseMpstCancel;
use macros::multiple::close::close_mpst_check_cancel::CloseMpstCheckCancel;
use macros::multiple::create_meshedchannels::CreateMeshedChannels;
use macros::multiple::fork_mpst_multi::ForkMPSTMulti;
use macros::multiple::offer::create_offer_mpst_session_multi::OfferMPSTSessionMulti;
use macros::multiple::offer::create_offer_type_multi::OfferTypeMulti;
use macros::multiple::recv::create_recv_mpst_session::CreateRecvMPSTSession;
use macros::multiple::recv::recv_mpst::RecvMPST;
use macros::multiple::send::create_send_check_cancel::CreateSendCheckCancel;
use macros::multiple::send::create_send_mpst_cancel::CreateSendMPSTCancel;
use macros::multiple::send::create_send_mpst_session::CreateSendMPSTSession;
use macros::multiple::send::send_cancel::SendCancel;
use macros::multiple::send::send_mpst::SendMPST;

//////////////////////////////////////

#[proc_macro]
pub fn send_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SendCancel);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_send_cancel(input: TokenStream) -> TokenStream {
    send_cancel(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn broadcast_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BroadcastCancel);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_broadcast_cancel(input: TokenStream) -> TokenStream {
    broadcast_cancel(input)
}

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
pub fn close_mpst(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CloseMpst);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_close_mpst(input: TokenStream) -> TokenStream {
    close_mpst(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn close_mpst_check_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CloseMpstCheckCancel);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_close_mpst_check_cancel(input: TokenStream) -> TokenStream {
    close_mpst_check_cancel(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn fork_mpst_multi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ForkMPSTMulti);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_fork_mpst_multi(input: TokenStream) -> TokenStream {
    fork_mpst_multi(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_choose_type_multi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeMulti);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_choose_type_multi(input: TokenStream) -> TokenStream {
    create_choose_type_multi(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_choose_mpst_session_multi_left(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeMultiLeft);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_choose_mpst_session_multi_left(input: TokenStream) -> TokenStream {
    create_choose_mpst_session_multi_left(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_choose_mpst_session_multi_right(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeMultiRight);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_choose_mpst_session_multi_right(input: TokenStream) -> TokenStream {
    create_choose_mpst_session_multi_right(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn choose_mpst_multi_cancel_to_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeMultiCancelToAll);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_choose_mpst_multi_cancel_to_all(input: TokenStream) -> TokenStream {
    choose_mpst_multi_cancel_to_all(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_fn_choose_mpst_multi_to_all_bundle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeMultiToAllBundle);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_fn_choose_mpst_multi_to_all_bundle(input: TokenStream) -> TokenStream {
    create_fn_choose_mpst_multi_to_all_bundle(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_fn_choose_mpst_cancel_multi_to_all_bundle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeCancelMultiToAllBundle);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_fn_choose_mpst_cancel_multi_to_all_bundle(input: TokenStream) -> TokenStream {
    create_fn_choose_mpst_cancel_multi_to_all_bundle(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_offer_type_multi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as OfferTypeMulti);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_offer_type_multi(input: TokenStream) -> TokenStream {
    create_offer_type_multi(input)
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
pub fn recv_mpst(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RecvMPST);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_recv_mpst(input: TokenStream) -> TokenStream {
    recv_mpst(input)
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
pub fn send_mpst(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SendMPST);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_send_mpst(input: TokenStream) -> TokenStream {
    send_mpst(input)
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

//////////////////////////////////////

#[proc_macro]
pub fn create_send_mpst_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateSendMPSTCancel);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_send_mpst_cancel(input: TokenStream) -> TokenStream {
    create_send_mpst_cancel(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_send_check_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateSendCheckCancel);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_send_check_cancel(input: TokenStream) -> TokenStream {
    create_send_check_cancel(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_meshedchannels(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateMeshedChannels);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_meshedchannels(input: TokenStream) -> TokenStream {
    create_meshedchannels(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn close_mpst_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CloseMpstCancel);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_close_mpst_cancel(input: TokenStream) -> TokenStream {
    close_mpst_cancel(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn choose_mpst_create_multi_to_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseMultiCreateToAll);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_choose_mpst_create_multi_to_all(input: TokenStream) -> TokenStream {
    choose_mpst_create_multi_to_all(input)
}
