//! This module contains the macros
//! for creating MeshedChannels for any number
//! of participants.

/// Creates a MeshedChannels for more than 3 participants.
///
/// # Arguments
///
/// * The name of the *MeshedChannels* type that will be used
/// * The number of participants (all together)
///
/// # Example
///
/// ```
/// use mpstthree::create_meshedchannels;
/// use mpstthree::role::Role;
///
/// create_meshedchannels!(MeshedChannels, 3);
/// ```
#[macro_export]
macro_rules! create_meshedchannels {
    ($meshedchannels_name: ident, $nsessions: literal) => {
        mpst_seq::create_meshedchannels!($meshedchannels_name, $nsessions);
    };
}
