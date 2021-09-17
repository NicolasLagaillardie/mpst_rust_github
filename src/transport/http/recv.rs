//! This module contains the functions and macros for
//! receiving a payload
//! for an HTTP connection, for at least two participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature.*

use crate::binary::struct_trait::{recv::Recv, session::Session};
use hyper::client::ResponseFuture;
use hyper::{Body, Response};
use std::boxed::Box;
use std::error::Error;
use std::marker;
use tokio::runtime::Runtime;

/// Send a value of type `T` over http. Returns the
/// continuation of the session `S`. May fail.
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_http"` feature.*
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_http")))
)]
pub fn recv_http<T, S>(
    s: Recv<T, S>,
    http: bool,
    req: ResponseFuture,
) -> Result<(T, S, Response<Body>), Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    // Await the response
    let resp = match http {
        true => {
            let rt = Runtime::new()?;
            rt.block_on(async move { req.await })?
        }
        false => Response::default(),
    };

    ////////////////

    let (v, s) = s.channel.recv()?;
    Ok((v, s, resp))
}

/// Creates a *recv* function to receive from a simple role on a given binary session type of a
/// MeshedChannels.
///
/// # Arguments
///
/// * The name of the new *recv* function
/// * The name of the sender
/// * The name of the receiver
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the MeshedChannels for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```ignore
/// use mpstthree::{create_multiple_normal_role, create_recv_http_session, create_meshedchannels};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_recv_http_session!(recv_mpst_d_from_a, RoleA, RoleD, MeshedChannels, 3, 1);
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_http"` feature.*
#[macro_export]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_http")))
)]
macro_rules! create_recv_http_session {
    (
        $func_name:ident,
        $sender:ident,
        $receiver:ident,
        $meshedchannels_name:ident,
        $nsessions:literal,
        $exclusion:literal
    ) => {
        mpst_seq_baking::create_recv_http_session!(
            $func_name,
            $sender,
            $receiver,
            $meshedchannels_name,
            $nsessions,
            $exclusion
        );
    };
}

/// Creates multiple *recv* functions to receive from a
/// simple role on a given binary session type of a
/// MeshedChannels.
///
/// # Arguments
///
/// * The name of the new *recv* functions
/// * The name of the senders
/// * The name of the receiver
/// * The index of the binary session types that will receive in the MeshedChannels for each
///   specific role. Index starts at 1.
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```ignore
/// use mpstthree::{create_multiple_normal_role, create_meshedchannels, create_recv_http_session_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
///
/// create_recv_http_session_bundle!(
///    recv_mpst_d_from_a,
///    RoleA,
///    1 |
///    recv_mpst_d_from_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    MeshedChannels,
///    3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"transport"` feature or the `"transport_http"` feature.*
#[macro_export]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_http")))
)]
macro_rules! create_recv_http_session_bundle {
    ($( $func_name: ident, $sender: ident, $exclusion: literal | )+ => $receiver: ident, $meshedchannels_name: ident, $nsessions: literal) => {
       $(
           mpstthree::create_recv_http_session!(
               $func_name,
               $sender,
               $receiver,
               $meshedchannels_name,
               $nsessions,
               $exclusion
            );
        )+
    }
}
