////////////////////////////////////////////
/// CLOSE

/// Create the close function to be used with more than 3
/// participants.
///
/// # Arguments
///
/// * The name of the new *close* function
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{close_mpst, create_sessionmpst};
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// close_mpst!(close_mpst_multi, SessionMpst, 3);
/// ```
#[macro_export]
macro_rules! close_mpst {
    ($func_name: ident, $sessionmpst_name: ident, $nsessions: literal) => {
        mpst_seq::close_mpst!($func_name, $sessionmpst_name, $nsessions);
    };
}

/// Create the close function to be used with more than 3
/// participants.
///
/// # Arguments
///
/// * The name of the new *close* function
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{close_mpst, create_sessionmpst};
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// close_mpst!(close_mpst_multi, SessionMpst, 3);
/// ```
#[macro_export]
macro_rules! close_mpst_check_cancel {
    ($func_name: ident, $sessionmpst_name: ident, $nsessions: literal) => {
        mpst_seq::close_mpst_check_cancel!($func_name, $sessionmpst_name, $nsessions);
    };
}

/// Create the close function to be used with more than 3
/// participants.
/// It is used for checking the send sides upon closing.
///
/// # Arguments
///
/// * The name of the new *close* function
/// * The name of the *SessionMpst* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{close_mpst_cancel, create_sessionmpst};
///
/// create_sessionmpst!(SessionMpst, 3);
///
/// close_mpst_cancel!(close_mpst_multi, SessionMpst, 3);
/// ```
#[macro_export]
macro_rules! close_mpst_cancel {
    ($func_name: ident, $sessionmpst_name: ident, $nsessions: literal) => {
        mpst_seq::close_mpst_cancel!($func_name, $sessionmpst_name, $nsessions);
    };
}
