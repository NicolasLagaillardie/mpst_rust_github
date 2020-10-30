////////////////////////////////////////////
/// SEND

// create a function send_mpst for the first session
#[macro_export]
macro_rules! create_send_mpst_session_1 {
    ($func_name:ident, $role:ident, $next:ident, $name:ident) => {
        mpstthree::create_send_mpst_session!($func_name, $role, $next, $name, SessionMpst, 3, 1);
    };
}

// create a function send_mpst for the second session
#[macro_export]
macro_rules! create_send_mpst_session_2 {
    ($func_name:ident, $role:ident, $next:ident, $name:ident) => {
        mpstthree::create_send_mpst_session!($func_name, $role, $next, $name, SessionMpst, 3, 2);
    };
}
