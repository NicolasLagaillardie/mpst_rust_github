//! This module contains the macros
//! for creating choose functions for any number
//! of participants for timed protocols.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"baking_timed"` feature.*

/// Choose among different sessions that are provided.
///
/// # Arguments
///
///  * The session to be used
///  * The Hashmap with all the clocks to be used. It need to have a `&mut`.
///  * The different `enum` variants which represent the different branches to be sent to each
///    passive role
///  * The different passive roles
///  * The name of the sender
///  * The name of the *MeshedChannels* type that will be used
///  * The index of the active role
///
/// # Example
///
/// Available on the *cases/13_macro_multi_recursion* test.
///
/// ```ignore
/// match xs.pop() {
///    Option::Some(_) => {
///        let s = choose_timed_mpst_multi_to_all!(
///            s,
///            &mut all_clocks,
///            CBranchesAtoC::Video,
///            CBranchesBtoC::Video, =>
///            NameD,
///            MeshedChannels,
///            3
///        );
///        let s = s.send(all_clocks, 1)?;
///        let (_, s) = s.recv(all_clocks)?;
///        client_recurs(s, xs, index + 1)
///    }

///    Option::None => {
///        let s = choose_timed_mpst_multi_to_all!(
///            s,
///            &mut all_clocks,
///            CBranchesAtoC::End,
///            CBranchesBtoC::End, =>
///            NameD,
///            MeshedChannels,
///            3
///        );
///        s.close()
///    }
/// }
/// ```
/// 
/// *This macro is available only if MultiCrusty is built with
/// the `"baking_timed"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "baking_timed")))]
macro_rules! choose_timed_mpst_multi_to_all {
    (
        $session: expr,
        $all_clocks: expr,
        $( $label: path , )+ =>
        $sender: ident,
        $meshedchannels_name: ident,
        $exclusion: literal
    ) => {
        mpst_seq_proc::choose_timed_mpst_multi_to_all!(
            $session ,
            $all_clocks ,
            ( $( ( $label ) )+ ) ,
            $sender ,
            $meshedchannels_name ,
            $exclusion
        );
    }
}
