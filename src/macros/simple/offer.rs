////////////////////////////////////////////
/// OFFER

/// Get an offer on session 1
#[macro_export]
macro_rules! create_offer_mpst_session_1 {
    ($func_name:ident, $role:ident, $recv_func:ident, $name:ident) => {
        mpstthree::create_offer_mpst_session_multi!(
            $func_name,
            OfferMpst,
            $role,
            $recv_func,
            $name,
            SessionMpst,
            3,
            1
        );
    };
}

/// Get an offer on session 2
#[macro_export]
macro_rules! create_offer_mpst_session_2 {
    ($func_name:ident, $role:ident, $recv_func:ident, $name:ident) => {
        mpstthree::create_offer_mpst_session_multi!(
            $func_name,
            OfferMpst,
            $role,
            $recv_func,
            $name,
            SessionMpst,
            3,
            2
        );
    };
}
