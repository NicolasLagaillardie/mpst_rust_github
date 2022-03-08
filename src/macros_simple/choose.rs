//! This module contains the macros
//! for creating choose functions for three
//! of participants, whatever are their name.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"macros_simple"` feature.*

// Create the core for the choose_mpst macros
#[macro_export]
#[doc(hidden)]
macro_rules! create_choose_from_1_to_2_3 {
    (
        $session_1:ty,
        $session_2:ty,
        $session_3:ty,
        $role_1:ty,
        $role_2:ty,
        $role_3:ty,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $name_sender:ident,
        $session:expr,
        $pat:path
    ) => {{
        let (session_1_2, session_2_1) =
            <$session_1 as mpstthree::binary::struct_trait::session::Session>::new();
        let (session_1_3, session_3_1) =
            <$session_2 as mpstthree::binary::struct_trait::session::Session>::new();
        let (session_3_2, session_2_3) =
            <$session_3 as mpstthree::binary::struct_trait::session::Session>::new();
        let (_, role_1) = <$role_1 as mpstthree::role::Role>::new();
        let (_, role_2) = <$role_2 as mpstthree::role::Role>::new();
        let (role_3, _) = <$role_3 as mpstthree::role::Role>::new();
        let (name_1, _) =
            <$name_receiver_1 as mpstthree::name::Name>::Dual::new();
        let (name_2, _) =
            <$name_receiver_2 as mpstthree::name::Name>::Dual::new();
        let (name_3, _) = <$name_sender as mpstthree::name::Name>::new();

        let choice_1 = mpstthree::meshedchannels::MeshedChannels {
            session1: session_2_1,
            session2: session_2_3,
            stack: role_1,
            name: name_1,
        };

        let choice_2 = mpstthree::meshedchannels::MeshedChannels {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_2,
            name: name_2,
        };

        let new_session_1 = mpstthree::binary::send::send($pat(choice_1), $session.session1);
        let new_session_2 = mpstthree::binary::send::send($pat(choice_2), $session.session2);

        let s = mpstthree::meshedchannels::MeshedChannels {
            session1: new_session_1,
            session2: new_session_2,
            stack: $session.stack,
            name: $session.name,
        };

        mpstthree::binary::cancel::cancel(s);

        mpstthree::meshedchannels::MeshedChannels {
            session1: session_1_2,
            session2: session_1_3,
            stack: role_3,
            name: name_3,
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! create_choose_from_2_to_1_3 {
    (
        $session_1:ty,
        $session_2:ty,
        $session_3:ty,
        $role_1:ty,
        $role_2:ty,
        $role_3:ty,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $name_sender:ident,
        $session:expr,
        $pat:path
    ) => {{
        let (session_2_1, session_1_2) =
            <$session_1 as mpstthree::binary::struct_trait::session::Session>::new();
        let (session_2_3, session_3_2) =
            <$session_2 as mpstthree::binary::struct_trait::session::Session>::new();
        let (session_3_1, session_1_3) =
            <$session_3 as mpstthree::binary::struct_trait::session::Session>::new();
        let (_, role_1) = <$role_1 as mpstthree::role::Role>::new();
        let (_, role_2) = <$role_2 as mpstthree::role::Role>::new();
        let (role_3, _) = <$role_3 as mpstthree::role::Role>::new();
        let (name_1, _) =
            <$name_receiver_1 as mpstthree::name::Name>::Dual::new();
        let (name_2, _) =
            <$name_receiver_2 as mpstthree::name::Name>::Dual::new();
        let (name_3, _) = <$name_sender as mpstthree::name::Name>::new();

        let choice_1 = mpstthree::meshedchannels::MeshedChannels {
            session1: session_1_2,
            session2: session_1_3,
            stack: role_1,
            name: name_1,
        };

        let choice_2 = mpstthree::meshedchannels::MeshedChannels {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_2,
            name: name_2,
        };

        let new_session_1 = mpstthree::binary::send::send($pat(choice_1), $session.session1);
        let new_session_2 = mpstthree::binary::send::send($pat(choice_2), $session.session2);

        let s = mpstthree::meshedchannels::MeshedChannels {
            session1: new_session_1,
            session2: new_session_2,
            stack: $session.stack,
            name: $session.name,
        };

        mpstthree::binary::cancel::cancel(s);

        mpstthree::meshedchannels::MeshedChannels {
            session1: session_2_1,
            session2: session_2_3,
            stack: role_3,
            name: name_3,
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! create_choose_from_3_to_1_2 {
    (
        $session_1:ty,
        $session_2:ty,
        $session_3:ty,
        $role_1:ty,
        $role_2:ty,
        $role_3:ty,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $name_sender:ident,
        $session:expr,
        $pat:path
    ) => {{
        let (session_3_1, session_1_3) =
            <$session_1 as mpstthree::binary::struct_trait::session::Session>::new();
        let (session_3_2, session_2_3) =
            <$session_2 as mpstthree::binary::struct_trait::session::Session>::new();
        let (session_2_1, session_1_2) =
            <$session_3 as mpstthree::binary::struct_trait::session::Session>::new();
        let (_, role_1) = <$role_1 as mpstthree::role::Role>::new();
        let (_, role_2) = <$role_2 as mpstthree::role::Role>::new();
        let (role_3, _) = <$role_3 as mpstthree::role::Role>::new();
        let (name_1, _) =
            <
                <
                    $name_receiver_1 as mpstthree::name::Name
                >::Dual as mpstthree::name::Name
            >::new();
        let (name_2, _) =
            <
                <
                    $name_receiver_2 as mpstthree::name::Name
                >::Dual as mpstthree::name::Name
            >::new();
        let (name_3, _) = <$name_sender as mpstthree::name::Name>::new();

        let choice_1 = mpstthree::meshedchannels::MeshedChannels {
            session1: session_1_2,
            session2: session_1_3,
            stack: role_1,
            name: name_1,
        };

        let choice_2 = mpstthree::meshedchannels::MeshedChannels {
            session1: session_2_1,
            session2: session_2_3,
            stack: role_2,
            name: name_2,
        };

        let new_session_1 = mpstthree::binary::send::send($pat(choice_1), $session.session1);
        let new_session_2 = mpstthree::binary::send::send($pat(choice_2), $session.session2);

        let s = mpstthree::meshedchannels::MeshedChannels {
            session1: new_session_1,
            session2: new_session_2,
            stack: $session.stack,
            name: $session.name,
        };

        mpstthree::binary::cancel::cancel(s);

        mpstthree::meshedchannels::MeshedChannels {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_3,
            name: name_3,
        }
    }};
}

////////////////////////////////////////////

/// Create the *ChooseMpst* function to send a *Choose*
/// right branch from the third role to the others. Must be
/// used with [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the dual of the first receiver
/// * The name of the dual of the second receiver
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{
///     create_multiple_normal_role, create_broadcast_role,
///     create_choose_right_from_3_to_1_and_2
/// };
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleC, RoleCDual |
/// );
///
/// create_broadcast_role!(RoleAlltoC, RoleCtoAll);
///
/// create_choose_right_from_3_to_1_and_2!(
///     choose_right_mpst_session_c_to_all,
///     RoleADual,
///     RoleBDual,
///     RoleCtoAll,
///     RoleC
/// );
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_choose_right_from_3_to_1_and_2 {
    (
        $func_name:ident,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $role_broadcast:ident,
        $name_sender:ident
    ) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::meshedchannels::MeshedChannels<
                mpstthree::functionmpst::ChooseMpst<
                    S0,
                    S2,
                    S1,
                    S4,
                    R0,
                    R1,
                    $name_receiver_1,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    <S0 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    S3,
                    <S1 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    S5,
                    R2,
                    R3,
                    $name_receiver_2,
                >,
                $role_broadcast<R4, R5>,
                $name_sender,
            >,
        ) -> mpstthree::meshedchannels::MeshedChannels<
            S4,
            S5,
            R5,
            $name_sender,
        >
        where
            S0: mpstthree::binary::struct_trait::session::Session + 'a,
            S1: mpstthree::binary::struct_trait::session::Session + 'a,
            S2: mpstthree::binary::struct_trait::session::Session + 'a,
            S3: mpstthree::binary::struct_trait::session::Session + 'a,
            S4: mpstthree::binary::struct_trait::session::Session + 'a,
            S5: mpstthree::binary::struct_trait::session::Session + 'a,
            R0: mpstthree::role::Role + 'a,
            R1: mpstthree::role::Role + 'a,
            R2: mpstthree::role::Role + 'a,
            R3: mpstthree::role::Role + 'a,
            R4: mpstthree::role::Role + 'a,
            R5: mpstthree::role::Role + 'a,
        {
            mpstthree::create_choose_from_3_to_1_2!(
                S4,
                S5,
                S1,
                R1,
                R3,
                R5,
                $name_receiver_1,
                $name_receiver_2,
                $name_sender,
                s,
                either::Either::Right
            )
        }
    };
}

/// Create the *ChooseMpst* function to send a *Choose*
/// left branch from the third role to the others. Must be
/// used with [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the dual of the first receiver
/// * The name of the dual of the second receiver
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{
///     create_multiple_normal_role, create_broadcast_role,
///     create_choose_left_from_3_to_1_and_2
/// };
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleC, RoleCDual |
/// );
///
/// create_broadcast_role!(RoleAlltoC, RoleCtoAll);
///
/// create_choose_left_from_3_to_1_and_2!(
///     choose_right_mpst_session_c_to_all,
///     RoleADual,
///     RoleBDual,
///     RoleCtoAll,
///     RoleC
/// );
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_choose_left_from_3_to_1_and_2 {
    (
        $func_name:ident,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $role_broadcast:ident,
        $name_sender:ident
    ) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::meshedchannels::MeshedChannels<
                mpstthree::functionmpst::ChooseMpst<
                    S0,
                    S2,
                    S1,
                    S4,
                    R0,
                    R1,
                    $name_receiver_1,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    <S0 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    S3,
                    <S1 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    S5,
                    R2,
                    R3,
                    $name_receiver_2,
                >,
                $role_broadcast<R4, R5>,
                $name_sender,
            >,
        ) -> mpstthree::meshedchannels::MeshedChannels<
            S2,
            S3,
            R4,
            $name_sender,
        >
        where
            S0: mpstthree::binary::struct_trait::session::Session + 'a,
            S1: mpstthree::binary::struct_trait::session::Session + 'a,
            S2: mpstthree::binary::struct_trait::session::Session + 'a,
            S3: mpstthree::binary::struct_trait::session::Session + 'a,
            S4: mpstthree::binary::struct_trait::session::Session + 'a,
            S5: mpstthree::binary::struct_trait::session::Session + 'a,
            R0: mpstthree::role::Role + 'a,
            R1: mpstthree::role::Role + 'a,
            R2: mpstthree::role::Role + 'a,
            R3: mpstthree::role::Role + 'a,
            R4: mpstthree::role::Role + 'a,
            R5: mpstthree::role::Role + 'a,
        {
            mpstthree::create_choose_from_3_to_1_2!(
                S2,
                S3,
                S0,
                R0,
                R2,
                R4,
                $name_receiver_1,
                $name_receiver_2,
                $name_sender,
                s,
                either::Either::Left
            )
        }
    };
}

/// Create the *ChooseMpst* function to send a *Choose*
/// left branch from the first role to the others. Must be
/// used with [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the dual of the first receiver
/// * The name of the dual of the second receiver
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{
///     create_multiple_normal_role, create_broadcast_role,
///     create_choose_left_from_1_to_2_and_3
/// };
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleC, RoleCDual |
/// );
///
/// create_broadcast_role!(RoleAlltoA, RoleAtoAll);
///
/// create_choose_left_from_1_to_2_and_3!(
///     choose_right_mpst_session_a_to_all,
///     RoleBDual,
///     RoleCDual,
///     RoleAtoAll,
///     RoleA
/// );
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_choose_left_from_1_to_2_and_3 {
    (
        $func_name:ident,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $role_broadcast:ident,
        $name_sender:ident
    ) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::meshedchannels::MeshedChannels<
                mpstthree::functionmpst::ChooseMpst<
                    S2,
                    S0,
                    S4,
                    S1,
                    R0,
                    R1,
                    $name_receiver_1,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    S3,
                    <S0 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    S5,
                    <S1 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    R2,
                    R3,
                    $name_receiver_2,
                >,
                $role_broadcast<R4, R5>,
                $name_sender,
            >,
        ) -> mpstthree::meshedchannels::MeshedChannels<
            S2,
            S3,
            R4,
            $name_sender,
        >
        where
            S0: mpstthree::binary::struct_trait::session::Session + 'a,
            S1: mpstthree::binary::struct_trait::session::Session + 'a,
            S2: mpstthree::binary::struct_trait::session::Session + 'a,
            S3: mpstthree::binary::struct_trait::session::Session + 'a,
            S4: mpstthree::binary::struct_trait::session::Session + 'a,
            S5: mpstthree::binary::struct_trait::session::Session + 'a,
            R0: mpstthree::role::Role + 'a,
            R1: mpstthree::role::Role + 'a,
            R2: mpstthree::role::Role + 'a,
            R3: mpstthree::role::Role + 'a,
            R4: mpstthree::role::Role + 'a,
            R5: mpstthree::role::Role + 'a,
        {
            mpstthree::create_choose_from_1_to_2_3!(
                S2,
                S3,
                S0,
                R0,
                R2,
                R4,
                $name_receiver_1,
                $name_receiver_2,
                $name_sender,
                s,
                either::Either::Left
            )
        }
    };
}

/// Create the *ChooseMpst* function to send a *Choose*
/// right branch from the first role to the others. Must be
/// used with [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the dual of the first receiver
/// * The name of the dual of the second receiver
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{
///     create_multiple_normal_role, create_broadcast_role,
///     create_choose_right_from_1_to_2_and_3
/// };
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleC, RoleCDual |
/// );
///
/// create_broadcast_role!(RoleAlltoA, RoleAtoAll);
///
/// create_choose_right_from_1_to_2_and_3!(
///     choose_right_mpst_session_a_to_all,
///     RoleBDual,
///     RoleCDual,
///     RoleAtoAll,
///     RoleA
/// );
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_choose_right_from_1_to_2_and_3 {
    (
        $func_name:ident,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $role_broadcast:ident,
        $name_sender:ident
    ) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::meshedchannels::MeshedChannels<
                mpstthree::functionmpst::ChooseMpst<
                    S2,
                    S0,
                    S4,
                    S1,
                    R0,
                    R1,
                    $name_receiver_1,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    S3,
                    <S0 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    S5,
                    <S1 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    R2,
                    R3,
                    $name_receiver_2,
                >,
                $role_broadcast<R4, R5>,
                $name_sender,
            >,
        ) -> mpstthree::meshedchannels::MeshedChannels<
            S4,
            S5,
            R5,
            $name_sender,
        >
        where
            S0: mpstthree::binary::struct_trait::session::Session + 'a,
            S1: mpstthree::binary::struct_trait::session::Session + 'a,
            S2: mpstthree::binary::struct_trait::session::Session + 'a,
            S3: mpstthree::binary::struct_trait::session::Session + 'a,
            S4: mpstthree::binary::struct_trait::session::Session + 'a,
            S5: mpstthree::binary::struct_trait::session::Session + 'a,
            R0: mpstthree::role::Role + 'a,
            R1: mpstthree::role::Role + 'a,
            R2: mpstthree::role::Role + 'a,
            R3: mpstthree::role::Role + 'a,
            R4: mpstthree::role::Role + 'a,
            R5: mpstthree::role::Role + 'a,
        {
            mpstthree::create_choose_from_1_to_2_3!(
                S4,
                S5,
                S1,
                R1,
                R3,
                R5,
                $name_receiver_1,
                $name_receiver_2,
                $name_sender,
                s,
                either::Either::Right
            )
        }
    };
}

/// Create the *ChooseMpst* function to send a *Choose*
/// left branch from the second role to the others. Must be
/// used with [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the dual of the first receiver
/// * The name of the dual of the second receiver
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{
///     create_multiple_normal_role, create_broadcast_role,
///     create_choose_left_from_2_to_1_and_3
/// };
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleC, RoleCDual |
/// );
///
/// create_broadcast_role!(RoleAlltoB, RoleBtoAll);
///
/// create_choose_left_from_2_to_1_and_3!(
///     choose_right_mpst_session_b_to_all,
///     RoleADual,
///     RoleCDual,
///     RoleBtoAll,
///     RoleB
/// );
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_choose_left_from_2_to_1_and_3 {
    (
        $func_name:ident,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $role_broadcast:ident,
        $name_sender:ident
    ) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::meshedchannels::MeshedChannels<
                mpstthree::functionmpst::ChooseMpst<
                    S2,
                    S0,
                    S4,
                    S1,
                    R0,
                    R1,
                    $name_receiver_1,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    <S0 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    S3,
                    <S1 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    S5,
                    R2,
                    R3,
                    $name_receiver_2,
                >,
                $role_broadcast<R4, R5>,
                $name_sender,
            >,
        ) -> mpstthree::meshedchannels::MeshedChannels<
            S2,
            S3,
            R4,
            $name_sender,
        >
        where
            S0: mpstthree::binary::struct_trait::session::Session + 'a,
            S1: mpstthree::binary::struct_trait::session::Session + 'a,
            S2: mpstthree::binary::struct_trait::session::Session + 'a,
            S3: mpstthree::binary::struct_trait::session::Session + 'a,
            S4: mpstthree::binary::struct_trait::session::Session + 'a,
            S5: mpstthree::binary::struct_trait::session::Session + 'a,
            R0: mpstthree::role::Role + 'a,
            R1: mpstthree::role::Role + 'a,
            R2: mpstthree::role::Role + 'a,
            R3: mpstthree::role::Role + 'a,
            R4: mpstthree::role::Role + 'a,
            R5: mpstthree::role::Role + 'a,
        {
            mpstthree::create_choose_from_2_to_1_3!(
                S2,
                S3,
                S0,
                R0,
                R2,
                R4,
                $name_receiver_1,
                $name_receiver_2,
                $name_sender,
                s,
                either::Either::Left
            )
        }
    };
}

/// Create the *ChooseMpst* function to send a *Choose*
/// right branch from the second role to the others. Must
/// be used with [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new *ChooseMpst* function
/// * The name of the dual of the first receiver
/// * The name of the dual of the second receiver
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{
///     create_multiple_normal_role, create_broadcast_role,
///     create_choose_right_from_2_to_1_and_3
/// };
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleC, RoleCDual |
/// );
///
/// create_broadcast_role!(RoleAlltoB, RoleBtoAll);
///
/// create_choose_right_from_2_to_1_and_3!(
///     choose_right_mpst_session_b_to_all,
///     RoleADual,
///     RoleCDual,
///     RoleBtoAll,
///     RoleB
/// );
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_choose_right_from_2_to_1_and_3 {
    (
        $func_name:ident,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $role_broadcast:ident,
        $name_sender:ident
    ) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::meshedchannels::MeshedChannels<
                mpstthree::functionmpst::ChooseMpst<
                    S2,
                    S0,
                    S4,
                    S1,
                    R0,
                    R1,
                    $name_receiver_1,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    <S0 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    S3,
                    <S1 as mpstthree::binary::struct_trait::session::Session>::Dual,
                    S5,
                    R2,
                    R3,
                    $name_receiver_2,
                >,
                $role_broadcast<R4, R5>,
                $name_sender,
            >,
        ) -> mpstthree::meshedchannels::MeshedChannels<
            S4,
            S5,
            R5,
            $name_sender,
        >
        where
            S0: mpstthree::binary::struct_trait::session::Session + 'a,
            S1: mpstthree::binary::struct_trait::session::Session + 'a,
            S2: mpstthree::binary::struct_trait::session::Session + 'a,
            S3: mpstthree::binary::struct_trait::session::Session + 'a,
            S4: mpstthree::binary::struct_trait::session::Session + 'a,
            S5: mpstthree::binary::struct_trait::session::Session + 'a,
            R0: mpstthree::role::Role + 'a,
            R1: mpstthree::role::Role + 'a,
            R2: mpstthree::role::Role + 'a,
            R3: mpstthree::role::Role + 'a,
            R4: mpstthree::role::Role + 'a,
            R5: mpstthree::role::Role + 'a,
        {
            mpstthree::create_choose_from_2_to_1_3!(
                S4,
                S5,
                S1,
                R1,
                R3,
                R5,
                $name_receiver_1,
                $name_receiver_2,
                $name_sender,
                s,
                either::Either::Right
            )
        }
    };
}

/// Call both [create_choose_right_from_2_to_1_and_3]
/// and [create_choose_left_from_2_to_1_and_3].
/// Must be used with [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new right *ChooseMpst* function
/// * The name of the new left *ChooseMpst* function
/// * The name of the dual of the first receiver
/// * The name of the dual of the second receiver
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{
///     create_multiple_normal_role, create_broadcast_role,
///     create_choose_both_from_2_to_1_and_3
/// };
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleC, RoleCDual |
/// );
///
/// create_broadcast_role!(RoleAlltoB, RoleBtoAll);
///
/// create_choose_both_from_2_to_1_and_3!(
///     choose_right_mpst_session_b_to_all,
///     choose_left_mpst_session_b_to_all,
///     RoleADual,
///     RoleCDual,
///     RoleBtoAll,
///     RoleB
/// );
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [create_choose_right_from_2_to_1_and_3]: crate::create_choose_right_from_2_to_1_and_3
/// [create_choose_left_from_2_to_1_and_3]: crate::create_choose_left_from_2_to_1_and_3
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_choose_both_from_2_to_1_and_3 {
    (
        $func_name_right:ident,
        $func_name_left:ident,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $role_broadcast:ident,
        $name_sender:ident
    ) => {
        mpstthree::create_choose_right_from_2_to_1_and_3!(
            $func_name_right,
            $name_receiver_1,
            $name_receiver_2,
            $role_broadcast,
            $name_sender
        );
        mpstthree::create_choose_left_from_2_to_1_and_3!(
            $func_name_left,
            $name_receiver_1,
            $name_receiver_2,
            $role_broadcast,
            $name_sender
        );
    };
}

/// Call both [create_choose_right_from_1_to_2_and_3]
/// and [create_choose_left_from_1_to_2_and_3].
/// Must be used with [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new right *ChooseMpst* function
/// * The name of the new left *ChooseMpst* function
/// * The name of the dual of the first receiver
/// * The name of the dual of the second receiver
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{
///     create_multiple_normal_role, create_broadcast_role,
///     create_choose_both_from_1_to_2_and_3
/// };
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleC, RoleCDual |
/// );
///
/// create_broadcast_role!(RoleAlltoB, RoleBtoAll);
///
/// create_choose_both_from_1_to_2_and_3!(
///     choose_right_mpst_session_b_to_all,
///     choose_left_mpst_session_b_to_all,
///     RoleADual,
///     RoleCDual,
///     RoleBtoAll,
///     RoleB
/// );
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [create_choose_right_from_1_to_2_and_3]: crate::create_choose_right_from_1_to_2_and_3
/// [create_choose_left_from_1_to_2_and_3]: crate::create_choose_left_from_1_to_2_and_3
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_choose_both_from_1_to_2_and_3 {
    (
        $func_name_right:ident,
        $func_name_left:ident,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $role_broadcast:ident,
        $name_sender:ident
    ) => {
        mpstthree::create_choose_right_from_1_to_2_and_3!(
            $func_name_right,
            $name_receiver_1,
            $name_receiver_2,
            $role_broadcast,
            $name_sender
        );
        mpstthree::create_choose_left_from_1_to_2_and_3!(
            $func_name_left,
            $name_receiver_1,
            $name_receiver_2,
            $role_broadcast,
            $name_sender
        );
    };
}

/// Call both [create_choose_right_from_3_to_1_and_2]
/// and [create_choose_left_from_3_to_1_and_2].
/// Must be used with [`MeshedChannels`].
///
/// # Arguments
///
/// * The name of the new right *ChooseMpst* function
/// * The name of the new left *ChooseMpst* function
/// * The name of the dual of the first receiver
/// * The name of the dual of the second receiver
/// * The name of the broadcasting sender. This one should contain *toAll* according to the
///   convention
/// * The name of the sender
///
/// # Example
///
/// ```
/// use mpstthree::role::Role;
/// use mpstthree::{  
///     create_multiple_normal_role, create_broadcast_role,
///     create_choose_both_from_3_to_1_and_2
/// };
///
/// create_multiple_normal_role!(
///     RoleA, RoleADual |
///     RoleB, RoleBDual |
///     RoleC, RoleCDual |
/// );
///
/// create_broadcast_role!(RoleAlltoB, RoleBtoAll);
///
/// create_choose_both_from_3_to_1_and_2!(
///     choose_right_mpst_session_b_to_all,
///     choose_left_mpst_session_b_to_all,
///     RoleADual,
///     RoleCDual,
///     RoleBtoAll,
///     RoleB
/// );
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [create_choose_right_from_3_to_1_and_2]: crate::create_choose_right_from_3_to_1_and_2
/// [create_choose_left_from_3_to_1_and_2]: crate::create_choose_left_from_3_to_1_and_2
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! create_choose_both_from_3_to_1_and_2 {
    (
        $func_name_right:ident,
        $func_name_left:ident,
        $name_receiver_1:ident,
        $name_receiver_2:ident,
        $role_broadcast:ident,
        $name_sender:ident
    ) => {
        mpstthree::create_choose_right_from_3_to_1_and_2!(
            $func_name_right,
            $name_receiver_1,
            $name_receiver_2,
            $role_broadcast,
            $name_sender
        );
        mpstthree::create_choose_left_from_3_to_1_and_2!(
            $func_name_left,
            $name_receiver_1,
            $name_receiver_2,
            $role_broadcast,
            $name_sender
        );
    };
}

/// Choose among two different sessions.
/// Must be used with [`MeshedChannels`].
/// # Arguments
///
///  * The session to be used
///  * The first path to be sent
///  * The second path to be sent
///  * The *send* function to be used for the first path
///  * The *send* function to be used for the second path
///  * The first passive role
///  * The second passive role
///  * The active role
///
/// # Example
///
/// ```ignore
/// match xs.pop() {
///     Option::Some(_) => {
///         let s = choose_mpst_to_all!(
///             s,
///             CBranchesAtoC::Video,
///             CBranchesBtoC::Video, =>
///             RoleA,
///             RoleB, =>
///             RoleC
///         );
///         let s = send_mpst_c_to_a(1, s);
///         let (_, s) = recv_mpst_c_from_a(s)?;
///         client_recurs(s, xs, index + 1)
///     }
///     Option::None => {
///         let s = choose_mpst_to_all!(
///             s,
///             CBranchesAtoC::End,
///             CBranchesBtoC::End, =>
///             RoleA,
///             RoleB, =>
///             RoleC
///         );
///         close_mpst(s)
///     }
/// }
/// ```
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
///
/// *This macro is available only if MultiCrusty is built with
/// the `"macros_simple"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
macro_rules! choose_mpst_to_all {
    (
        $session: expr ,
        $( $label: path , )+ =>
        $( $receiver: ident , )+ =>
        $name_sender: ident
    ) => {{
        use mpstthree::meshedchannels::MeshedChannels;

        mpstthree::choose_mpst_multi_to_all!(
            $session,
            $( $label , )+ =>
            $( $receiver , )+ =>
            $name_sender,
            MeshedChannels,
            3
        )
    }};
}
