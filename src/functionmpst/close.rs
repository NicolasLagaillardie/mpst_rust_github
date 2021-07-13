//! This module contains the *close* function

use crate::binary::struct_trait::End;
use crate::binary::struct_trait::Signal;
use crate::meshedchannels::MeshedChannels;
use crate::role::end::RoleEnd;
use crate::role::Role;
use std::error::Error;

/// Closes a [`MeshedChannels`].
/// 
/// Synchronises with all partners, and fails if one of the partners has
/// crashed.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{End, Session};
/// use mpstthree::meshedchannels::MeshedChannels;
/// use mpstthree::role::Role;
///
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
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
/// // Name
/// type NameA = RoleA<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointA = MeshedChannels<AtoB, AtoC, StackA, NameA>;
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
///   session1: channel_ab,
///   session2: channel_ac,
///   stack: role_a,
///   name: name_a,
/// };
///
/// // ...to this point, should not be written in general. Please look at the *fork* function.
///
/// let s = close_mpst(sess);
/// ```
/// 
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
pub fn close_mpst<R>(s: MeshedChannels<End, End, RoleEnd, R>) -> Result<(), Box<dyn Error>>
where
    R: Role,
{
    s.session1.sender.send(Signal::Stop).unwrap_or(());
    s.session2.sender.send(Signal::Stop).unwrap_or(());

    s.session1.receiver.recv()?;
    s.session2.receiver.recv()?;

    Ok(())
}
