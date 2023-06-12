//! This module contains all the *choose* functions for three participants A, B and C

use crate::binary::cancel::cancel;
use crate::binary::send::send;
use crate::binary::struct_trait::session::Session;
use crate::functionmpst::ChooseMpst;
use crate::meshedchannels::MeshedChannels;
use crate::name::a::NameA;
use crate::name::b::NameB;
use crate::name::c::NameC;
use crate::name::Name;
use crate::role::a_to_all::RoleAtoAll;
use crate::role::b_to_all::RoleBtoAll;
use crate::role::c_to_all::RoleCtoAll;
use crate::role::Role;
use either::Either;

type ShortChooseMpstOne<S0, S1, S2, S4, R0, R1, N0> = ChooseMpst<S2, S0, S4, S1, R0, R1, N0>;
type ShortChooseMpstTwo<S0, S1, S3, S5, R0, R1, N0> =
    ChooseMpst<S3, <S0 as Session>::Dual, S5, <S1 as Session>::Dual, R0, R1, N0>;

type ShortMeshedChannelsAtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5> = MeshedChannels<
    ShortChooseMpstOne<S0, S1, S2, S4, R0, R1, NameB>,
    ShortChooseMpstTwo<S0, S1, S3, S5, R2, R3, NameC>,
    RoleAtoAll<R4, R5>,
    NameA,
>;
type ShortMeshedChannelsBtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5> = MeshedChannels<
    ShortChooseMpstOne<S0, S1, S2, S4, R0, R1, NameA>,
    ShortChooseMpstTwo<
        <S3 as Session>::Dual,
        <S5 as Session>::Dual,
        <S0 as Session>::Dual,
        <S1 as Session>::Dual,
        R2,
        R3,
        NameC,
    >,
    RoleBtoAll<R4, R5>,
    NameB,
>;
type ShortMeshedChannelsCtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5> = MeshedChannels<
    ShortChooseMpstOne<S2, S4, S0, S1, R0, R1, NameA>,
    ShortChooseMpstTwo<
        <S3 as Session>::Dual,
        <S5 as Session>::Dual,
        <S0 as Session>::Dual,
        <S1 as Session>::Dual,
        R2,
        R3,
        NameB,
    >,
    RoleCtoAll<R4, R5>,
    NameC,
>;

#[doc(hidden)]
macro_rules! choose_mpst_a {
    (
        $session_1:ty,
        $session_2:ty,
        $session_3:ty,
        $stack_1:ty,
        $stack_2:ty,
        $stack_3:ty,
        $receiver_1: ident,
        $receiver_2: ident,
        $sender: ident,
        $session:expr,
        $pat:path
    ) => {{
        let (session_1_2, session_2_1) = <$session_1 as Session>::new();
        let (session_1_3, session_3_1) = <$session_2 as Session>::new();
        let (session_3_2, session_2_3) = <$session_3 as Session>::new();

        let (_, stack_1) = <$stack_1>::new();
        let (_, stack_2) = <$stack_2>::new();
        let (stack_3, _) = <$stack_3>::new();
        let (name_1, _) = <$receiver_1 as Name>::new();
        let (name_2, _) = <$receiver_2 as Name>::new();
        let (name_3, _) = <$sender as Name>::new();

        let choice_1 = MeshedChannels {
            session1: session_2_1,
            session2: session_2_3,
            stack: stack_1,
            name: name_1,
        };

        let choice_2 = MeshedChannels {
            session1: session_3_1,
            session2: session_3_2,
            stack: stack_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);

        let s = MeshedChannels {
            session1: new_session_1,
            session2: new_session_2,
            stack: $session.stack,
            name: $session.name,
        };

        cancel(s);

        MeshedChannels {
            session1: session_1_2,
            session2: session_1_3,
            stack: stack_3,
            name: name_3,
        }
    }};
}

#[doc(hidden)]
macro_rules! choose_mpst_b {
    (
        $session_1:ty,
        $session_2:ty,
        $session_3:ty,
        $stack_1:ty,
        $stack_2:ty,
        $stack_3:ty,
        $receiver_1: ident,
        $receiver_2: ident,
        $sender: ident,
        $session:expr,
        $pat:path
    ) => {{
        let (session_2_1, session_1_2) = <$session_1 as Session>::new();
        let (session_3_1, session_1_3) = <$session_2 as Session>::new();
        let (session_2_3, session_3_2) = <$session_3 as Session>::new();

        let (_, stack_1) = <$stack_1>::new();
        let (_, stack_2) = <$stack_2>::new();
        let (stack_3, _) = <$stack_3>::new();
        let (name_1, _) = <$receiver_1 as Name>::new();
        let (name_2, _) = <$receiver_2 as Name>::new();
        let (name_3, _) = <$sender as Name>::new();

        let choice_1 = MeshedChannels {
            session1: session_1_2,
            session2: session_1_3,
            stack: stack_1,
            name: name_1,
        };

        let choice_2 = MeshedChannels {
            session1: session_3_1,
            session2: session_3_2,
            stack: stack_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);

        let s = MeshedChannels {
            session1: new_session_1,
            session2: new_session_2,
            stack: $session.stack,
            name: $session.name,
        };

        cancel(s);

        MeshedChannels {
            session1: session_2_1,
            session2: session_2_3,
            stack: stack_3,
            name: name_3,
        }
    }};
}

#[doc(hidden)]
macro_rules! choose_mpst_c {
    (
        $session_1:ty,
        $session_2:ty,
        $session_3:ty,
        $stack_1:ty,
        $stack_2:ty,
        $stack_3:ty,
        $receiver_1: ident,
        $receiver_2: ident,
        $sender: ident,
        $session:expr,
        $pat:path
    ) => {{
        let (session_2_1, session_1_2) = <$session_1 as Session>::new();
        let (session_3_1, session_1_3) = <$session_2 as Session>::new();
        let (session_3_2, session_2_3) = <$session_3 as Session>::new();

        let (_, stack_1) = <$stack_1>::new();
        let (_, stack_2) = <$stack_2>::new();
        let (stack_3, _) = <$stack_3>::new();
        let (name_1, _) = <$receiver_1 as Name>::new();
        let (name_2, _) = <$receiver_2 as Name>::new();
        let (name_3, _) = <$sender as Name>::new();

        let choice_1 = MeshedChannels {
            session1: session_1_2,
            session2: session_1_3,
            stack: stack_1,
            name: name_1,
        };

        let choice_2 = MeshedChannels {
            session1: session_2_1,
            session2: session_2_3,
            stack: stack_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);

        let s = MeshedChannels {
            session1: new_session_1,
            session2: new_session_2,
            stack: $session.stack,
            name: $session.name,
        };

        cancel(s);

        MeshedChannels {
            session1: session_3_1,
            session2: session_3_2,
            stack: stack_3,
            name: name_3,
        }
    }};
}

/// Given a choice from A, to other processes, between two
/// [`MeshedChannels`], choose the
/// first option for each.
///
/// A has to encapsulate all possible
/// [`MeshedChannels`] for each other
/// role. This function creates the 6 new binary
/// [`Session`], the 3 new
/// [`Role`] related to each first option
/// then the related [`mpstthree::meshedchannels::
/// MeshedChannels`]. It then sends those options to the
/// related processes.
///
/// * S0: dual session from B to C on left branch
/// * S1: dual session from B to C on right branch
/// * S2: session from A to B on left branch
/// * S3: session from A to C on left branch
/// * S4: session from A to B on right branch
/// * S5: session from A to C on right branch
///
/// * R0: dual stack of B on left branch
/// * R1: dual stack of B on right branch
/// * R2: dual stack of C on left branch
/// * R3: dual stack of C on right branch
/// * R4: stack of A on left branch
/// * R5: stack of A on right branch
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [`Session`]: crate::binary::struct_trait::session::Session
/// [`Role`]: crate::role::Role
pub fn choose_left_mpst_session_a_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortMeshedChannelsAtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> MeshedChannels<S2, S3, R4, NameA>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    choose_mpst_a!(S2, S3, S0, R0, R2, R4, NameB, NameC, NameA, s, Either::Left)
}

/// Given a choice from A, to other processes, between two
/// [`MeshedChannels`], choose the
/// second option for each.
///
/// A has to encapsulate all possible
/// [`MeshedChannels`] for each other
/// role. This function creates the 6 new binary
/// [`Session`], the 3 new
/// [`Role`] related to each second option
/// then the related [`mpstthree::meshedchannels::
/// MeshedChannels`]. It then sends those options to the
/// related processes.
///
/// * S0: dual session from B to C on left branch
/// * S1: dual session from B to C on right branch
/// * S2: session from A to B on left branch
/// * S3: session from A to C on left branch
/// * S4: session from A to B on right branch
/// * S5: session from A to C on right branch
///
/// * R0: dual stack of B on left branch
/// * R1: dual stack of B on right branch
/// * R2: dual stack of C on left branch
/// * R3: dual stack of C on right branch
/// * R4: stack of A on left branch
/// * R5: stack of A on right branch
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [`Session`]: crate::binary::struct_trait::session::Session
/// [`Role`]: crate::role::Role
pub fn choose_right_mpst_session_a_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortMeshedChannelsAtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> MeshedChannels<S4, S5, R5, NameA>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    choose_mpst_a!(
        S4,
        S5,
        S1,
        R1,
        R3,
        R5,
        NameB,
        NameC,
        NameA,
        s,
        Either::Right
    )
}

/// Given a choice from B, to other processes, between two
/// [`MeshedChannels`], choose the
/// first option for each.
///
/// B has to encapsulate all possible
/// [`MeshedChannels`] for each other
/// role. This function creates the 6 new binary
/// [`Session`], the 3 new
/// [`Role`] related to each first option
/// then the related [`mpstthree::meshedchannels::
/// MeshedChannels`]. It then sends those options to the
/// related processes.
///
/// * S0: dual session from A to C on left branch
/// * S1: dual session from A to C on right branch
/// * S2: session from B to A on left branch
/// * S3: session from B to C on left branch
/// * S4: session from B to A on right branch
/// * S5: session from B to C on right branch
///
/// * R0: dual stack of A on left branch
/// * R1: dual stack of A on right branch
/// * R2: dual stack of C on left branch
/// * R3: dual stack of C on right branch
/// * R4: stack of B on left branch
/// * R5: stack of B on right branch
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [`Session`]: crate::binary::struct_trait::session::Session
/// [`Role`]: crate::role::Role
pub fn choose_left_mpst_session_b_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortMeshedChannelsBtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> MeshedChannels<S2, S3, R4, NameB>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    choose_mpst_b!(S2, S0, S3, R0, R2, R4, NameA, NameC, NameB, s, Either::Left)
}

/// Given a choice from B, to other processes, between two
/// [`MeshedChannels`], choose the
/// second option for each.
///
/// B has to encapsulate all possible
/// [`MeshedChannels`] for each other
/// role. This function creates the 6 new binary
/// [`Session`], the 3 new
/// [`Role`] related to each second option
/// then the related [`mpstthree::meshedchannels::
/// MeshedChannels`]. It then sends those options to the
/// related processes.
///
/// * S0: dual session from A to C on left branch
/// * S1: dual session from A to C on right branch
/// * S2: session from B to A on left branch
/// * S3: session from B to C on left branch
/// * S4: session from B to A on right branch
/// * S5: session from B to C on right branch
///
/// * R0: dual stack of A on left branch
/// * R1: dual stack of A on right branch
/// * R2: dual stack of C on left branch
/// * R3: dual stack of C on right branch
/// * R4: stack of B on left branch
/// * R5: stack of B on right branch
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [`Session`]: crate::binary::struct_trait::session::Session
/// [`Role`]: crate::role::Role
pub fn choose_right_mpst_session_b_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortMeshedChannelsBtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> MeshedChannels<S4, S5, R5, NameB>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    choose_mpst_b!(
        S4,
        S1,
        S5,
        R1,
        R3,
        R5,
        NameA,
        NameC,
        NameB,
        s,
        Either::Right
    )
}

/// Given a choice from C, to other processes, between two
/// [`MeshedChannels`], choose the
/// first option for each.
///
/// C has to encapsulate all possible
/// [`MeshedChannels`] for each other
/// role. This function creates the 6 new binary
/// [`Session`], the 3 new
/// [`Role`] related to each first option
/// then the related [`mpstthree::meshedchannels::
/// MeshedChannels`]. It then sends those options to the
/// related processes.
///
/// * S0: dual session from A to B on left branch
/// * S1: dual session from A to B on right branch
/// * S2: session from C to A on left branch
/// * S3: session from C to B on left branch
/// * S4: session from C to A on right branch
/// * S5: session from C to B on right branch
///
/// * R0: dual stack of A on left branch
/// * R1: dual stack of A on right branch
/// * R2: dual stack of B on left branch
/// * R3: dual stack of B on right branch
/// * R4: stack of C on left branch
/// * R5: stack of C on right branch
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [`Session`]: crate::binary::struct_trait::session::Session
/// [`Role`]: crate::role::Role
pub fn choose_left_mpst_session_c_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortMeshedChannelsCtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> MeshedChannels<S2, S3, R4, NameC>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    choose_mpst_c!(S0, S2, S3, R0, R2, R4, NameA, NameB, NameC, s, Either::Left)
}

/// Given a choice from C, to other processes, between two
/// [`MeshedChannels`], choose the
/// second option for each.
///
/// C has to encapsulate all possible
/// [`MeshedChannels`] for each other
/// role. This function creates the 6 new binary
/// [`Session`], the 3 new
/// [`Role`] related to each second option
/// then the related [`mpstthree::meshedchannels::
/// MeshedChannels`]. It then sends those options to the
/// related processes.
///
/// * S0: dual session from A to B on left branch
/// * S1: dual session from A to B on right branch
/// * S2: session from C to A on left branch
/// * S3: session from C to B on left branch
/// * S4: session from C to A on right branch
/// * S5: session from C to B on right branch
///
/// * R0: dual stack of A on left branch
/// * R1: dual stack of A on right branch
/// * R2: dual stack of B on left branch
/// * R3: dual stack of B on right branch
/// * R4: stack of C on left branch
/// * R5: stack of C on right branch
///
/// [`MeshedChannels`]: crate::meshedchannels::MeshedChannels
/// [`Session`]: crate::binary::struct_trait::session::Session
/// [`Role`]: crate::role::Role
pub fn choose_right_mpst_session_c_to_all<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
    s: ShortMeshedChannelsCtoAll<S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>,
) -> MeshedChannels<S4, S5, R5, NameC>
where
    S0: Session + 'a,
    S1: Session + 'a,
    S2: Session + 'a,
    S3: Session + 'a,
    S4: Session + 'a,
    S5: Session + 'a,
    R0: Role + 'a,
    R1: Role + 'a,
    R2: Role + 'a,
    R3: Role + 'a,
    R4: Role + 'a,
    R5: Role + 'a,
{
    choose_mpst_c!(
        S1,
        S4,
        S5,
        R1,
        R3,
        R5,
        NameA,
        NameB,
        NameC,
        s,
        Either::Right
    )
}
