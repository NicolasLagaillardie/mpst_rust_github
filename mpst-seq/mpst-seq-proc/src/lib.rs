#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::usize;
use syn::parse_macro_input;

mod functionmpst;

use functionmpst::recv_all_aux_simple::RecvAllAuxSimpleMacroInput;
use functionmpst::recv_aux_simple::RecvAuxSimpleMacroInput;
use functionmpst::send_aux_simple::SendAuxSimpleMacroInput;

mod macros;

use macros::baking::BakingMacroInput;
use macros::create_broadcast_role_short::CreateBroadcastRoleShortMacroInput;
use macros::create_normal_role_short::CreateNormalRoleShortMacroInput;
use macros::multiple::broadcast_cancel::BroadcastCancelMacroInput;
use macros::multiple::choose_mpst_multi_cancel_to_all::ChooseTypeMultiCancelToAllMacroInput;
use macros::multiple::choose_mpst_multi_http_to_all::ChooseTypeMultiHttpToAllMacroInput;
use macros::multiple::choose_mpst_multi_to_all::ChooseTypeMultiToAllMacroInput;
use macros::multiple::close_mpst::CloseMpstMacroInput;
use macros::multiple::close_mpst_check_cancel::CloseMpstCheckCancelMacroInput;
use macros::multiple::create_choose_mpst_session_multi_left::ChooseTypeMultiLeftMacroInput;
use macros::multiple::create_choose_mpst_session_multi_right::ChooseTypeMultiRightMacroInput;
use macros::multiple::create_choose_type_multi::ChooseTypeMultiMacroInput;
use macros::multiple::create_fn_choose_mpst_cancel_multi_to_all_bundle::ChooseTypeCancelMultiToAllBundleMacroInput;
use macros::multiple::create_fn_choose_mpst_multi_to_all_bundle::ChooseTypeMultiToAllBundleMacroInput;
use macros::multiple::create_offer_mpst_session_multi::OfferMPSTSessionMultiMacroInput;
use macros::multiple::create_offer_type_multi::OfferTypeMultiMacroInput;
use macros::multiple::create_recv_http_session::CreateRecvHttpSessionMacroInput;
use macros::multiple::create_recv_mpst_session::CreateRecvMPSTSessionMacroInput;
use macros::multiple::fork_mpst_multi::ForkMPSTMultiMacroInput;
use macros::multiple::recv_mpst::RecvMPSTMacroInput;
use macros::multiple::send_cancel::SendCancelMacroInput;

mod basic;

use basic::SeqMacroInput;

//////////////////////////////////////

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_seq(input: TokenStream) -> TokenStream {
    seq(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn recv_aux_simple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RecvAuxSimpleMacroInput);
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
    let input = parse_macro_input!(input as RecvAllAuxSimpleMacroInput);
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
    let input = parse_macro_input!(input as SendAuxSimpleMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_send_aux_simple(input: TokenStream) -> TokenStream {
    send_aux_simple(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn send_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SendCancelMacroInput);
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
    let input = parse_macro_input!(input as BroadcastCancelMacroInput);
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
    let input = parse_macro_input!(input as CreateNormalRoleShortMacroInput);
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
    let input = parse_macro_input!(input as CreateBroadcastRoleShortMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_broadcast_role_short(input: TokenStream) -> TokenStream {
    create_broadcast_role_short(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn baking(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BakingMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_baking(input: TokenStream) -> TokenStream {
    baking(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn close_mpst(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CloseMpstMacroInput);
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
    let input = parse_macro_input!(input as CloseMpstCheckCancelMacroInput);
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
    let input = parse_macro_input!(input as ForkMPSTMultiMacroInput);
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
    let input = parse_macro_input!(input as ChooseTypeMultiMacroInput);
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
    let input = parse_macro_input!(input as ChooseTypeMultiLeftMacroInput);
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
    let input = parse_macro_input!(input as ChooseTypeMultiRightMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_choose_mpst_session_multi_right(input: TokenStream) -> TokenStream {
    create_choose_mpst_session_multi_right(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn choose_mpst_multi_to_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeMultiToAllMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_choose_mpst_multi_to_all(input: TokenStream) -> TokenStream {
    choose_mpst_multi_to_all(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn choose_mpst_multi_cancel_to_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeMultiCancelToAllMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_choose_mpst_multi_cancel_to_all(input: TokenStream) -> TokenStream {
    choose_mpst_multi_cancel_to_all(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn choose_mpst_multi_http_to_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeMultiHttpToAllMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_choose_mpst_multi_http_to_all(input: TokenStream) -> TokenStream {
    choose_mpst_multi_http_to_all(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_fn_choose_mpst_multi_to_all_bundle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChooseTypeMultiToAllBundleMacroInput);
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
    let input = parse_macro_input!(input as ChooseTypeCancelMultiToAllBundleMacroInput);
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
    let input = parse_macro_input!(input as OfferTypeMultiMacroInput);
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
    let input = parse_macro_input!(input as OfferMPSTSessionMultiMacroInput);
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
    let input = parse_macro_input!(input as RecvMPSTMacroInput);
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
    let input = parse_macro_input!(input as CreateRecvMPSTSessionMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_recv_mpst_session(input: TokenStream) -> TokenStream {
    create_recv_mpst_session(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_recv_http_session(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateRecvHttpSessionMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_recv_http_session(input: TokenStream) -> TokenStream {
    create_recv_http_session(input)
}
