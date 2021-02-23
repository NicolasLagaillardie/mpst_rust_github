//! This module contains the *close* function

use crate::binary::End;
use crate::binary::Signal;
use crate::role::end::RoleEnd;
use crate::role::Role;
use crate::sessionmpst::SessionMpst;
use std::error::Error;

/// Closes a [`mpstthree::sessionmpst::Sessionmpst`](../sessionmpst/struct.SessionMpst.html). Synchronises with all partners, and fails if one of the partners
/// has crashed.
///
/// # Example
///
/// ```
/// use mpstthree::binary::{End, Session};
/// use mpstthree::sessionmpst::SessionMpst;
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
/// // Queue
/// type QueueA = RoleEnd;
///
/// // Name
/// type NameA = RoleA<RoleEnd>;
///
/// // Creating the MP sessions
/// type EndpointA = SessionMpst<AtoB, AtoC, QueueA, NameA>;
///
/// // From this point...
/// let (channel_ab, _) = AtoB::new();
/// let (channel_ac, _) = AtoC::new();
///
/// let (role_a, _) = QueueA::new();
///
/// let (name_a, _) = NameA::new();
///
/// let sess = SessionMpst {
///    session1: channel_ab,
///    session2: channel_ac,
///    stack: role_a,
///    name: name_a,
/// };
/// // ...to this point, should not be written in general. Please look at [`mpstthree::fork`](../fork/index.html).
///
/// let s = close_mpst(sess);
/// ```
pub fn close_mpst<R>(s: SessionMpst<End, End, RoleEnd, R>) -> Result<(), Box<dyn Error>>
where
    R: Role,
{
    s.session1.sender.send(Signal::Stop).unwrap_or(());
    s.session2.sender.send(Signal::Stop).unwrap_or(());

    s.session1.receiver.recv()?;
    s.session2.receiver.recv()?;

    Ok(())
}
