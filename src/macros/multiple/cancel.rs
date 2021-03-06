////////////////////////////////////////////
/// CHOICE

/// Creates a function that will cancel a session and send a `Cancel` signal to the broadcasting role.
///
/// # Arguments
///
/// * The name of the new function type
/// * The name of the role that will send the `Cancel` signal
/// * The *MeshedChannels* type that will be used
/// * The number of participants (all together)
/// * The message that will be displayed
#[macro_export]
macro_rules! send_cancel {
    ($func_name: ident, $name: ident, $meshedchannels_name: ident, $nsessions: literal, $msg: expr) => {
        mpst_seq::send_cancel!($func_name, $name, $meshedchannels_name, $nsessions, $msg);
    };
}

/// Indefinitely loops to check all sessions if there
/// is a `Cancel` signal and broadcast if it present.
/// Will also close correctly if a `Stop` signal
/// is received.
///
/// # Arguments
///
/// * The session that will be used
/// * The number of participants (all together)
#[macro_export]
macro_rules! broadcast_cancel {
    ($session: expr, $nsessions: literal) => {
        mpst_seq::broadcast_cancel!($session, $nsessions);
    };
}
