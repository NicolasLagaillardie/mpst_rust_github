#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::usize;
use syn::parse_macro_input;

mod common_functions;

mod functionmpst;

use functionmpst::recv_all_aux_simple::RecvAllAuxSimple;
use functionmpst::recv_aux_simple::RecvAuxSimple;
use functionmpst::send_aux_simple::SendAuxSimple;

mod choose_mpst_multi_to_all;

use choose_mpst_multi_to_all::ChooseMultiToAll;

mod macros_simple;

use macros_simple::offer::create_offer_mpst_session_multi::OfferMPSTSessionMulti;
use macros_simple::recv::create_recv_mpst_session::CreateRecvMPSTSession;
use macros_simple::roles::create_broadcast_role_short::CreateBroadcastRoleShort;
use macros_simple::roles::create_normal_role_short::CreateNormalRoleShort;
use macros_simple::names::create_normal_name_short::CreateNormalNameShort;
use macros_simple::send::create_send_mpst_session::CreateSendMPSTSession;

mod macros_multiple;

use macros_multiple::multiple::broadcast_cancel::BroadcastCancel;
use macros_multiple::multiple::choose::choose_mpst_create_multi_to_all::ChooseMultiCreateToAll;
use macros_multiple::multiple::choose::choose_mpst_multi_cancel_to_all::ChooseTypeMultiCancelToAll;
use macros_multiple::multiple::choose::create_choose_mpst_session_multi_left::ChooseTypeMultiLeft;
use macros_multiple::multiple::choose::create_choose_mpst_session_multi_right::ChooseTypeMultiRight;
use macros_multiple::multiple::choose::create_choose_type_multi::ChooseTypeMulti;
use macros_multiple::multiple::choose::create_fn_choose_mpst_cancel_multi_to_all_bundle::ChooseTypeCancelMultiToAllBundle;
use macros_multiple::multiple::choose::create_fn_choose_mpst_multi_to_all_bundle::ChooseTypeMultiToAllBundle;
use macros_multiple::multiple::close::close_mpst::CloseMpst;
use macros_multiple::multiple::close::close_mpst_cancel::CloseMpstCancel;
use macros_multiple::multiple::close::close_mpst_check_cancel::CloseMpstCheckCancel;
use macros_multiple::multiple::create_meshedchannels::CreateMeshedChannels;
use macros_multiple::multiple::fork_mpst_multi::ForkMPSTMulti;
use macros_multiple::multiple::offer::create_offer_type_multi::OfferTypeMulti;
use macros_multiple::multiple::send::create_send_check_cancel::CreateSendCheckCancel;
use macros_multiple::multiple::send::create_send_mpst_cancel::CreateSendMPSTCancel;
use macros_multiple::multiple::send::send_cancel::SendCancel;

mod interleaved;

use interleaved::close_mpst_interleaved::CloseMpstInterleaved;
use interleaved::fork_mpst_multi_interleaved::ForkMPSTMultiInterleaved;
use interleaved::fork_mpst_multi_solo::ForkMPSTMultiSolo;

mod macros_http;

use macros_http::choose::choose_mpst_multi_http_to_all::ChooseTypeMultiHttpToAll;
use macros_http::recv::create_recv_http_session::CreateRecvHttpSession;
use macros_http::send::create_send_http_session::CreateSendHttpSession;

mod checking;

use checking::aux_checking::CheckingInput;
use checking::branching::branching_variants;

mod baking;

use baking::baking_basic::Baking;
use baking::baking_interleaved_with_enum_and_cancel::BakingInterleavedWithEnumAndCancel;
use baking::baking_with_cancel::BakingWithCancel;
use baking::baking_with_enum::BakingWithEnum;
use baking::baking_with_enum_and_cancel::BakingWithEnumAndCancel;

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
pub fn create_normal_name_short(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateNormalNameShort);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_normal_name_short(input: TokenStream) -> TokenStream {
    create_normal_name_short(input)
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

//////////////////////////////////////

#[proc_macro]
pub fn fork_mpst_multi_solo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ForkMPSTMultiSolo);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_fork_mpst_multi_solo(input: TokenStream) -> TokenStream {
    fork_mpst_multi_solo(input)
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

#[proc_macro_attribute]
pub fn branching(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut out = input.clone();

    let ty = parse_macro_input!(input as syn::Item);
    assert!(args.is_empty());

    if let Err(e) = branching_variants(ty) {
        out.extend(TokenStream::from(e.to_compile_error()));
    }
    out
}

//////////////////////////////////////

#[proc_macro]
pub fn baking_interleaved_with_enum_and_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BakingInterleavedWithEnumAndCancel);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_baking_interleaved_with_enum_and_cancel(input: TokenStream) -> TokenStream {
    baking_interleaved_with_enum_and_cancel(input)
}
