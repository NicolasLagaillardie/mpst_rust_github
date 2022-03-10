//! This module contains the *close* function

use crate::binary::struct_trait::end::End;
use crate::binary::struct_trait::end::Signal;
use crate::meshedchannels::MeshedChannels;
use crate::name::Name;
use crate::role::end::RoleEnd;
use std::error::Error;

/// Closes a [`MeshedChannels`].
///
/// Synchronises with all partners, and fails if one of the partners has
/// crashed.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, session::Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::role::Role;
/// use mpstthree::name::Name;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::end::RoleEnd;
///
/// use mpstthree::name::a::NameA;
///
/// use mpstthree::functionmpst::close::close_mpst;
///
/// // Creating the binary sessions
/// type AtoB = End;
/// type AtoC = End;
///
/// // Stack
/// type StackA = RoleEnd;
///
/// // From this point...
///
/// let (channel_ab, _) = AtoB::new();
/// let (channel_ac, _) = AtoC::new();
///
/// let (role_a, _) = StackA::new();
///
/// let (name_a, _) = NameA::new();
///
/// let sess = MeshedChannels {
///     session1: channel_ab,
///     session2: channel_ac,
///     stack: role_a,
///     name: name_a,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let _s = close_mpst(sess);
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
pub fn close_mpst<N>(s: MeshedChannels<End, End, RoleEnd, N>) -> Result<(), Box<dyn Error>>
where
    N: Name,
{
    s.session1.sender.send(Signal::Stop).unwrap_or(());
    s.session2.sender.send(Signal::Stop).unwrap_or(());

    s.session1.receiver.recv()?;
    s.session2.receiver.recv()?;

    Ok(())
}
