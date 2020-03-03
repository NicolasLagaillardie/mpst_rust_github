use binary::End;
use role::end::RoleEnd;
use sessionmpst::SessionMpst;
use std::error::Error;

/// Closes a `SessionMpst`. Synchronises with all partners, and fails if one of the partners
/// has crashed.
pub fn close_mpst(s: SessionMpst<End, End, RoleEnd>) -> Result<(), Box<dyn Error>> {
    s.session1.sender.send(()).unwrap_or(());
    s.session2.sender.send(()).unwrap_or(());

    s.session1.receiver.recv()?;
    s.session2.receiver.recv()?;

    Ok(())
}
