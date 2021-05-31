#[macro_export]
macro_rules! bundle_impl {
    ( $sessionmpst_name: ident => $($all_roles: ident),+ $(,)? ) => {
        mpst_seq::baking!($sessionmpst_name, ($($all_roles,)+));
    };
}
