////////////////////////////////////////////
/// FORK

/// Creates the _fork_ function to be used with more than 3
/// participants. It must be used with
/// [`mpstthree::fork_simple`](../macro.fork_simple.html)
///
/// # Arguments
///
/// * The name of the new *fork* function
/// * The name of the *simple fork* function
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::{create_meshedchannels, fork_mpst_multi};
///
/// create_meshedchannels!(MeshedChannels, 3);
///
/// fork_mpst_multi!(fork_mpst, MeshedChannels, 3);
/// ```
#[macro_export]
macro_rules! fork_mpst_multi {
    ($func_name: ident, $meshedchannels_name: ident, $nsessions: literal) => {
        mpst_seq::fork_mpst_multi!($func_name, $meshedchannels_name, $nsessions);
    };
}
