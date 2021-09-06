//! This module contains the functions and macros for
//! sending a payload
//! for an HTTP connection, for at least two participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature.*

use crate::binary::struct_trait::{send::Send, session::Session};
use hyper::client::ResponseFuture;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use std::boxed::Box;
use std::error::Error;
use std::marker;
use std::panic;

/// Send a value of type `T` over http. Returns the
/// continuation of the session `S`. May fail.
///
/// *This function is available only if MultiCrusty is built with
/// the `"transport"` feature.*
#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub fn send_http<T, S>(
    x: T,
    s: Send<T, S>,
    http: bool,
    method: Method,
    uri: &str,
    header: Vec<(&str, &str)>,
    body: &'static str,
) -> Result<(S, ResponseFuture), Box<dyn Error>>
where
    T: marker::Send,
    S: Session, {
    let (here, there) = S::new();

    let respfut = match http {
        true => {
            let mut temp = Request::builder().method(method).uri(uri);

            for elt in header {
                temp = temp.header(elt.0, elt.1);
            }

            let req = temp.body(Body::from(body))?;

            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, Body>(https);

            client.request(req)
        }
        false => {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, Body>(https);

            client.request(Request::default())
        }
    };

    ////////////////

    match s.channel.send((x, there)) {
        Ok(_) => Ok((here, respfut)),
        Err(e) => panic!("{}", e.to_string()),
    }
}

/// Creates a *send* function to send from a given binary session type of a MeshedChannels with more
/// than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* function
/// * The name of the receiver
/// * The name of the sender
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The index of the binary session type that will receive in the MeshedChannels for this specific
///   role. Index starts at 1.
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_send_http_session, create_meshedchannels};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_send_http_session!(send_http_d_to_a, RoleA, RoleD, MeshedChannels, 3, 1);
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"transport"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
macro_rules! create_send_http_session {
    (
        $func_name:ident,
        $receiver:ident,
        $sender:ident,
        $meshedchannels_name:ident,
        $nsessions:literal,
        $exclusion:literal
    ) => {
        mpst_seq::create_send_http_session!(
            $func_name,
            $receiver,
            $sender,
            $meshedchannels_name,
            $nsessions,
            $exclusion
        );
    };
}

/// Creates multiple *send* functions to send from a given binary session type of a MeshedChannels
/// with more than 3 participants.
///
/// # Arguments
///
/// * The name of the new *send* functions
/// * The name of the receivers
/// * The name of the senders
/// * The index of the binary session types that will receive in the MeshedChannels for this
///   specific role. Index starts at 1.
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_multiple_normal_role, create_meshedchannels, create_send_mpst_http_bundle};
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleD, RoleDDual |
/// );
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// create_send_mpst_http_bundle!(
///    send_http_d_to_a,
///    RoleA,
///    1 |
///    send_http_d_to_b,
///    RoleB,
///    2 | =>
///    RoleD,
///    MeshedChannels,
///    3
/// );
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"transport"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
macro_rules! create_send_mpst_http_bundle {
    ($( $func_name: ident, $receiver: ident, $exclusion: literal | )+ => $sender: ident, $meshedchannels_name: ident, $nsessions: literal) => {
       $(
           mpstthree::create_send_http_session!(
               $func_name,
               $receiver,
               $sender,
               $meshedchannels_name,
               $nsessions,
               $exclusion
            );
        )+
    }
}
