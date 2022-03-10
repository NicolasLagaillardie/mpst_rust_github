//! This module contains the functions
//! for cancelling binary sessions.

use std::mem;

/// Cancels a session. Always succeeds. If the partner calls `recv` or `close` after cancellation,
/// those calls fail.
///
/// # Example
///
/// ```
/// use mpstthree::binary::cancel::cancel;
/// use mpstthree::binary::struct_trait::end::End;
/// use mpstthree::binary::struct_trait::session::Session;
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::role::a::RoleA;
/// use mpstthree::name::a::NameA;
/// use mpstthree::name::Name;
/// use mpstthree::role::end::RoleEnd;
///
/// let (s, _s_dual) = MeshedChannels::<End, End, RoleEnd, NameA>::new();
/// cancel(s);
/// ```
pub fn cancel<T>(s: T) {
    mem::drop(s);
}
