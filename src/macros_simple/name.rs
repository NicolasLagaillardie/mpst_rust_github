//! This module contains the macros
//! for creating new participants.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_simple"` feature.*

/// Create a new [`Name`].
///
/// # Arguments
///
/// * The name of the new [`Name`]
///
/// # Example
///
/// ```
/// use mpstthree::create_normal_name;
///
/// // Create the names *A* and *ADual*
/// create_normal_name!(A, ADual);
/// ```
///
/// [`Name`]: crate::name::Name
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_normal_name {
    ($name:ident) => {
        ////////////////////////////////////////////
        /// The Name

        #[derive(Debug)]
        struct $name {
            #[doc(hidden)]
            sender: crossbeam_channel::Sender<()>,
            #[doc(hidden)]
            receiver: crossbeam_channel::Receiver<()>,
        }

        ////////////////////////////////////////////
        /// The Name functions

        impl mpstthree::name::Name for $name {
            type Dual = $name;

            #[doc(hidden)]
            fn new() -> (Self, Self::Dual) {
                let (sender1, receiver1) = crossbeam_channel::bounded::<()>(1);
                let (sender2, receiver2) = crossbeam_channel::bounded::<()>(1);

                (
                    $name {
                        sender: sender1,
                        receiver: receiver2,
                    },
                    $name {
                        sender: sender2,
                        receiver: receiver1,
                    },
                )
            }

            #[doc(hidden)]
            fn head_str() -> String {
                String::from(stringify!($name))
            }

            #[doc(hidden)]
            fn tail_str() -> String {
                "".to_string()
            }

            #[doc(hidden)]
            fn self_head_str(&self) -> String {
                String::from(stringify!($name))
            }

            #[doc(hidden)]
            fn self_tail_str(&self) -> String {
                "".to_string()
            }
        }
    };
}

/// Create multiple new [`Name`].
///
/// # Arguments
///
/// * The name of the new [`Name`]
///
/// # Example
///
/// ```
/// use mpstthree::create_multiple_normal_name;
///
/// // Create the names *A*, *ADual*, *B* and *BDual*
/// create_multiple_normal_name!(
///    A, ADual |
///    B, BDual |
/// );
/// ```
///
/// [`Name`]: crate::name::Name
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_multiple_normal_name {
    ($( $name: ident),+ $(,)? ) => {
        $(mpstthree::create_normal_name!($name);)+
     }
}

///////////////////////////////

/// Create a new [`Name`].
/// When a name *X* is given, the Names created are
///
///  * NameX
///  * NameXDual
///
/// If you want to specify other names, please use [`create_normal_name`].
///
/// # Arguments
///
/// * The name of the new [`Name`]
///
/// # Example
///
/// ```
/// use mpstthree::create_normal_name_short;
///
/// // Create the names *NameA* and *NameADual*
/// create_normal_name_short!(A);
/// ```
///
/// [`Name`]: crate::name::Name
/// [`create_normal_name`]: crate::create_normal_name
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_normal_name_short {
    ($name:ident) => {
        mpst_seq::create_normal_name_short!($name);
    };
}

/// Create multiple new [`Name`].
/// When a name *X* is given, the Names created are
///
///  * NameX
///  * NameXDual
///
/// If you want to specify other names, please use [`create_multiple_normal_name`].
///
/// # Arguments
///
/// * The names of the new [`Name`]
///
/// # Example
///
/// ```
/// use mpstthree::create_multiple_normal_name_short;
///
/// // Create the names *NameA*, *NameB*, *NameC*,
/// // *NameADual*, *NameBDual* and *NameCDual*
/// create_multiple_normal_name_short!(A, B, C);
/// ```
///
/// [`Name`]: crate::name::Name
/// [`create_multiple_normal_name`]: crate::create_multiple_normal_name
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_multiple_normal_name_short {
    ($( $name: ident),+ $(,)? ) => {
        $(
            mpstthree::create_normal_name_short!($name);
        )+
     }
}
