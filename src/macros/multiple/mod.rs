pub mod cancel;
pub mod choose;
pub mod close;
pub mod fork;
pub mod meshedchannels;
pub mod offer;
pub mod recv;
pub mod send;

/// Creates thestructure SessionMPST
/// [`mpstthree::create_meshedchannels`](../macro.create_meshedchannels.html),
/// the [`mpstthree::close_mpst`](../macro.close_mpst.html) and
/// [`mpstthree::fork_mpst_multi`](../macro.fork_mpst_multi.html).
///
/// # Arguments
///
/// * The name of the new *close* function
/// * The name of the new *fork* function
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::bundle_struct_fork_close_multi;
///
/// bundle_struct_fork_close_multi!(close_mpst, fork_mpst, MeshedChannels, 3);
/// ```
#[macro_export]
macro_rules! bundle_struct_fork_close_multi {
    ($func_name_close: ident, $func_name_fork: ident, $meshedchannels_name: ident, $nsessions: literal) => {
        mpstthree::create_meshedchannels!($meshedchannels_name, $nsessions);
        mpstthree::close_mpst!($func_name_close, $meshedchannels_name, $nsessions);
        mpstthree::fork_mpst_multi!($func_name_fork, $meshedchannels_name, $nsessions);
    };
}

/// Creates thestructure SessionMPST
/// [`mpstthree::create_meshedchannels`](../macro.create_meshedchannels.html),
/// the [`mpstthree::close_mpst_cancel`](../macro.close_mpst_cancel.html) and
/// [`mpstthree::fork_mpst_multi`](../macro.fork_mpst_multi.html)
/// functions to be used with more than 2 participants.
/// It checks the send sides of the channels along the recv sides.
///
/// # Arguments
///
/// * The name of the new *close* function
/// * The name of the new *fork* function
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::bundle_struct_fork_close_multi_cancel;
///
/// bundle_struct_fork_close_multi_cancel!(close_mpst, fork_mpst, MeshedChannels, 3);
/// ```
#[macro_export]
macro_rules! bundle_struct_fork_close_multi_cancel {
    ($func_name_close: ident, $func_name_fork: ident, $meshedchannels_name: ident, $nsessions: literal) => {
        mpstthree::create_meshedchannels!($meshedchannels_name, $nsessions);
        mpstthree::close_mpst_cancel!($func_name_close, $meshedchannels_name, $nsessions);
        mpstthree::fork_mpst_multi!($func_name_fork, $meshedchannels_name, $nsessions);
    };
}
