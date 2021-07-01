////////////////////////////////////////////
/// CHOICE

/// Cancels a session
#[macro_export]
macro_rules! send_cancel {
    ($func_name: ident, $name: ident, $sessionmpst_name: ident, $nsessions: literal, $msg: expr) => {
        mpst_seq::send_cancel!($func_name, $name, $sessionmpst_name, $nsessions, $msg);
    };
}

/// Broadcast a session from the first participant to
/// others. Creates the function that will be direcly sent
#[macro_export]
macro_rules! broadcast_cancel {
    ($session: expr, $nsessions: literal) => {
        mpst_seq::broadcast_cancel!($session, $nsessions);
    };
}
