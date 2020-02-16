use role::end::{next_end, RoleEnd};
use binary::{End, close};
use std::error::Error;
use sessionmpst::SessionMpst;

/// Closes session one. Synchronises with the partner, and fails if the partner
/// has crashed.
pub fn close_mpst(s: SessionMpst<End, End, RoleEnd>) -> Result<(), Box<dyn Error>> {
    close(s.session1)?;
    close(s.session2)?;
    next_end(s.queue)?;

    Ok(())
}
