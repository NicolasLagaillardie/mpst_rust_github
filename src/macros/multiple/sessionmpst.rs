////////////////////////////////////////////
/// SESSIONMPST

/// Creates a SessionMpst for more than 3 participants.
///
/// # Arguments
///
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::create_sessionmpst;
/// use mpstthree::role::Role;
///
/// create_sessionmpst!(SessionMpst, 3);
/// ```
#[macro_export]
macro_rules! create_sessionmpst {
    ($sessionmpst_name: ident, $nsessions: literal) => {
        mpst_seq::create_sessionmpst!($sessionmpst_name, $nsessions);
    }
}
