use binary::End;
use role::end::RoleEnd;
use sessionmpst::SessionMpst;
use std::error::Error;

/// Closes session one. Synchronises with the partner, and fails if the partner
/// has crashed.
pub fn close_mpst(s: SessionMpst<End, End, RoleEnd>) -> Result<(), Box<dyn Error>> {
    s.session1.sender.send(()).unwrap_or(());
    s.session2.sender.send(()).unwrap_or(());

    s.session1.receiver.recv()?;
    s.session2.receiver.recv()?;

    Ok(())
}
