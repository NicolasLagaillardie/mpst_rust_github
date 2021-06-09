/// Create a new SessionMST structuren, new roles and the baking environment.
///
/// # Arguments
///
/// * Name of the new SessionMST
/// * Names of the new roles
/// * [Optional] Name of the new fork function
///
/// # Basic example
///
/// ```
/// use mpstthree::bundle_impl;
///
/// bundle_impl!(SessionMpstThree => A, B, C);
/// ```
///
/// # Example with fork
///
/// ```
/// use mpstthree::bundle_impl;
///
/// bundle_impl!(SessionMpstThree => A, B, C => fork_mpst);
/// ```
#[macro_export]
macro_rules! bundle_impl {
    ( $sessionmpst_name: ident => $( $all_roles: ident),+ $(,)? ) => {
        mpst_seq::baking!(
            $sessionmpst_name,
            ( $( $all_roles , )+ )
        );
    };
    ( $sessionmpst_name: ident => $( $all_roles: ident),+ $(,)? => $fork_mpst: ident) => {
        mpst_seq::baking!(
            $sessionmpst_name ,
            ( $( $all_roles , )+ ) ,
            $fork_mpst
        );
    };
    (
        $sessionmpst_name: ident =>
        $( $all_roles: ident),+ $(,)? =>
        $fork_mpst: ident =>
        $(
            $fn_name: ident,
            $( $paths: path , )+
            $return_type: ty,
            $index: literal
        ),+
        $(,)?
    ) => {
        mpst_seq::baking!(
            $sessionmpst_name ,
            ( $( $all_roles , )+ ) ,
            $fork_mpst
        );
    };
}
