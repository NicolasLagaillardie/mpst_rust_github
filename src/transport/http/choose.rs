//! This module contains the macros
//! for creating choose functions
//! for an HTTP connection, for at least two participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"transport"` feature or the `"transport_http"` feature.*

/// Choose among different sessions that are provided.
///
/// # Arguments
///
///  * The session to be used
///  * The different `enum` variants which represent the different branches to be sent to each
///    passive role
///  * The different passive roles
///  * The name of the sender
///  * The name of the *MeshedChannels* type that will be used
///  * The number of participants (all together)
///
/// # Example
///
/// ```ignore
/// match xs.pop() {
///    Option::Some(_) => {
///        let s = choose_mpst_multi_http_to_all!(
///            s,
///            CBranchesAtoC::Video,
///            CBranchesBtoC::Video, =>
///            RoleA,
///            RoleB, =>
///            RoleD,
///            MeshedChannels,
///            3
///        );
///        let s = send_http_d_to_a(1, s);
///        let (_, s) = recv_http_d_to_a(s)?;
///        client_recurs(s, xs, index + 1)
///    }

///    Option::None => {
///        let s = choose_mpst_multi_http_to_all!(
///            s,
///            CBranchesAtoC::End,
///            CBranchesBtoC::End, =>
///            RoleA,
///            RoleB, =>
///            RoleD,
///            MeshedChannels,
///            3
///        );
///        close_mpst_multi(s)
///    }
/// }
/// ```
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_multiple"` feature.*
#[macro_export]
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(feature = "transport", feature = "transport_http")))
)]
macro_rules! choose_mpst_multi_http_to_all {
    (
        $session: expr,
        $( $label: path , )+ =>
        $( $receiver: ident , )+ =>
        $sender: ident,
        $meshedchannels_name: ident,
        $exclusion: literal
    ) => {
        mpst_seq_proc::choose_mpst_multi_http_to_all!(
            $session ,
            ( $( ( $label ) )+ ) ,
            ( $( ( $receiver ) )+ ) ,
            $sender ,
            $meshedchannels_name ,
            $exclusion
        );
    }
}
