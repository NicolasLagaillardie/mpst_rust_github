////////////////////////////////////////////
/// CHOOSE

/// Create the core for the choose_mpst macros
#[macro_export]
#[doc(hidden)]
macro_rules! create_choose_from_1_to_2_3 {
    ($session_1:ty, $session_2:ty, $session_3:ty, $role_1:ty, $role_2:ty, $role_3:ty, $receiver_1:ident, $receiver_2:ident, $sender:ident, $session:expr, $pat:path, $next:ident) => {{
        let (session_1_2, session_2_1) = <$session_1>::new();
        let (session_1_3, session_3_1) = <$session_2>::new();
        let (session_3_2, session_2_3) = <$session_3>::new();
        let (_, role_1) = <$role_1>::new();
        let (_, role_2) = <$role_2>::new();
        let (role_3, _) = <$role_3>::new();
        let (name_1, _) =
            <$receiver_1<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::Dual::new();
        let (name_2, _) =
            <$receiver_2<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::Dual::new();
        let (name_3, _) = $sender::<mpstthree::role::end::RoleEnd>::new();

        let choice_1 = mpstthree::sessionmpst::SessionMpst {
            session1: session_2_1,
            session2: session_2_3,
            stack: role_1,
            name: name_1,
        };

        let choice_2 = mpstthree::sessionmpst::SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_2,
            name: name_2,
        };

        let new_session_1 = mpstthree::binary::send($pat(choice_1), $session.session1);
        let new_session_2 = mpstthree::binary::send($pat(choice_2), $session.session2);
        let (_, new_queue) = $next($session.stack);

        let s = mpstthree::sessionmpst::SessionMpst {
            session1: new_session_1,
            session2: new_session_2,
            stack: new_queue,
            name: $session.name,
        };

        mpstthree::binary::cancel(s);

        mpstthree::sessionmpst::SessionMpst {
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
    ($session_1:ty, $session_2:ty, $session_3:ty, $role_1:ty, $role_2:ty, $role_3:ty, $receiver_1:ident, $receiver_2:ident, $sender:ident, $session:expr, $pat:path, $next:ident) => {{
        let (session_2_1, session_1_2) = <$session_1>::new();
        let (session_2_3, session_3_2) = <$session_2>::new();
        let (session_3_1, session_1_3) = <$session_3>::new();
        let (_, role_1) = <$role_1>::new();
        let (_, role_2) = <$role_2>::new();
        let (role_3, _) = <$role_3>::new();
        let (name_1, _) =
            <$receiver_1<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::Dual::new();
        let (name_2, _) =
            <$receiver_2<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::Dual::new();
        let (name_3, _) = $sender::<mpstthree::role::end::RoleEnd>::new();

        let choice_1 = mpstthree::sessionmpst::SessionMpst {
            session1: session_1_2,
            session2: session_1_3,
            stack: role_1,
            name: name_1,
        };

        let choice_2 = mpstthree::sessionmpst::SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_2,
            name: name_2,
        };

        let new_session_1 = mpstthree::binary::send($pat(choice_1), $session.session1);
        let new_session_2 = mpstthree::binary::send($pat(choice_2), $session.session2);
        let (_, new_queue) = $next($session.stack);

        let s = mpstthree::sessionmpst::SessionMpst {
            session1: new_session_1,
            session2: new_session_2,
            stack: new_queue,
            name: $session.name,
        };

        mpstthree::binary::cancel(s);

        mpstthree::sessionmpst::SessionMpst {
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
    ($session_1:ty, $session_2:ty, $session_3:ty, $role_1:ty, $role_2:ty, $role_3:ty, $receiver_1:ident, $receiver_2:ident, $sender:ident, $session:expr, $pat:path, $next:ident) => {{
        let (session_3_1, session_1_3) = <$session_1>::new();
        let (session_3_2, session_2_3) = <$session_2>::new();
        let (session_2_1, session_1_2) = <$session_3>::new();
        let (_, role_1) = <$role_1>::new();
        let (_, role_2) = <$role_2>::new();
        let (role_3, _) = <$role_3>::new();
        let (name_1, _) =
            <$receiver_1<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::Dual::new();
        let (name_2, _) =
            <$receiver_2<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::Dual::new();
        let (name_3, _) = $sender::<mpstthree::role::end::RoleEnd>::new();

        let choice_1 = mpstthree::sessionmpst::SessionMpst {
            session1: session_1_2,
            session2: session_1_3,
            stack: role_1,
            name: name_1,
        };

        let choice_2 = mpstthree::sessionmpst::SessionMpst {
            session1: session_2_1,
            session2: session_2_3,
            stack: role_2,
            name: name_2,
        };

        let new_session_1 = mpstthree::binary::send($pat(choice_1), $session.session1);
        let new_session_2 = mpstthree::binary::send($pat(choice_2), $session.session2);
        let (_, new_queue) = $next($session.stack);

        let s = mpstthree::sessionmpst::SessionMpst {
            session1: new_session_1,
            session2: new_session_2,
            stack: new_queue,
            name: $session.name,
        };

        mpstthree::binary::cancel(s);

        mpstthree::sessionmpst::SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_3,
            name: name_3,
        }
    }};
}

////////////////////////////////////////////

///  Create the *ChooseMpst* function to send a *Choose* right branch from the third role to the others.
///  Must be used with [`mpstthree::sessionmpst::SessionMpst`].
///
///  # Arguments
///  
///  * The name of the new *ChooseMpst* function
///  * The name of the dual of the first receiver
///  * The name of the dual of the second receiver
///  * The name of the broadcasting sender. This one should contain *toAll* according to the convention
///  * The name of related *next* function
///  * The name of the sender
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_broadcast_role, create_sessionmpst, create_choose_right_from_3_to_1_and_2};
///
///  create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
///  create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
///  create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
///
///  create_broadcast_role!(RoleAlltoC, next_all_to_c, RoleCtoAll, next_c_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_choose_right_from_3_to_1_and_2!(
///      choose_right_mpst_session_c_to_all,
///      RoleADual,
///      RoleBDual,
///      RoleCtoAll,
///      next_c_to_all,
///      RoleC
///  );
///  ```
///
///  [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_choose_right_from_3_to_1_and_2 {
    ($func_name:ident, $dual_1:ident, $dual_2:ident, $role_broadcast:ident, $next:ident, $sender:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::sessionmpst::SessionMpst<
                mpstthree::functionmpst::ChooseMpst<
                    S0,
                    S2,
                    S1,
                    S4,
                    R0,
                    R1,
                    $dual_1<mpstthree::role::end::RoleEnd>,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    <S0 as mpstthree::binary::Session>::Dual,
                    S3,
                    <S1 as mpstthree::binary::Session>::Dual,
                    S5,
                    R2,
                    R3,
                    $dual_2<mpstthree::role::end::RoleEnd>,
                >,
                $role_broadcast<R4, R5>,
                $sender<mpstthree::role::end::RoleEnd>,
            >,
        ) -> mpstthree::sessionmpst::SessionMpst<S4, S5, R5, $sender<mpstthree::role::end::RoleEnd>>
        where
            S0: mpstthree::binary::Session + 'a,
            S1: mpstthree::binary::Session + 'a,
            S2: mpstthree::binary::Session + 'a,
            S3: mpstthree::binary::Session + 'a,
            S4: mpstthree::binary::Session + 'a,
            S5: mpstthree::binary::Session + 'a,
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
                $dual_1,
                $dual_2,
                $sender,
                s,
                either::Either::Right,
                $next
            )
        }
    };
}

///  Create the *ChooseMpst* function to send a *Choose* left branch from the third role to the others.
///  Must be used with [`mpstthree::sessionmpst::SessionMpst`].
///
///  # Arguments
///  
///  * The name of the new *ChooseMpst* function
///  * The name of the dual of the first receiver
///  * The name of the dual of the second receiver
///  * The name of the broadcasting sender. This one should contain *toAll* according to the convention
///  * The name of related *next* function
///  * The name of the sender
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_broadcast_role, create_sessionmpst, create_choose_left_from_3_to_1_and_2};
///
///  create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
///  create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
///  create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
///
///  create_broadcast_role!(RoleAlltoC, next_all_to_c, RoleCtoAll, next_c_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_choose_left_from_3_to_1_and_2!(
///      choose_right_mpst_session_c_to_all,
///      RoleADual,
///      RoleBDual,
///      RoleCtoAll,
///      next_c_to_all,
///      RoleC
///  );
///  ```
///
///  [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_choose_left_from_3_to_1_and_2 {
    ($func_name:ident, $dual_1:ident, $dual_2:ident, $role_broadcast:ident, $next:ident, $sender:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::sessionmpst::SessionMpst<
                mpstthree::functionmpst::ChooseMpst<
                    S0,
                    S2,
                    S1,
                    S4,
                    R0,
                    R1,
                    $dual_1<mpstthree::role::end::RoleEnd>,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    <S0 as mpstthree::binary::Session>::Dual,
                    S3,
                    <S1 as mpstthree::binary::Session>::Dual,
                    S5,
                    R2,
                    R3,
                    $dual_2<mpstthree::role::end::RoleEnd>,
                >,
                $role_broadcast<R4, R5>,
                $sender<mpstthree::role::end::RoleEnd>,
            >,
        ) -> mpstthree::sessionmpst::SessionMpst<S2, S3, R4, $sender<mpstthree::role::end::RoleEnd>>
        where
            S0: mpstthree::binary::Session + 'a,
            S1: mpstthree::binary::Session + 'a,
            S2: mpstthree::binary::Session + 'a,
            S3: mpstthree::binary::Session + 'a,
            S4: mpstthree::binary::Session + 'a,
            S5: mpstthree::binary::Session + 'a,
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
                $dual_1,
                $dual_2,
                $sender,
                s,
                either::Either::Left,
                $next
            )
        }
    };
}

///  Create the *ChooseMpst* function to send a *Choose* left branch from the first role to the others.
///  Must be used with [`mpstthree::sessionmpst::SessionMpst`].
///
///  # Arguments
///  
///  * The name of the new *ChooseMpst* function
///  * The name of the dual of the first receiver
///  * The name of the dual of the second receiver
///  * The name of the broadcasting sender. This one should contain *toAll* according to the convention
///  * The name of related *next* function
///  * The name of the sender
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_broadcast_role, create_sessionmpst, create_choose_left_from_1_to_2_and_3};
///
///  create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
///  create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
///  create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
///
///  create_broadcast_role!(RoleAlltoA, next_all_to_a, RoleAtoAll, next_a_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_choose_left_from_1_to_2_and_3!(
///      choose_right_mpst_session_a_to_all,
///      RoleBDual,
///      RoleCDual,
///      RoleAtoAll,
///      next_a_to_all,
///      RoleA
///  );
///  ```
///
///  [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_choose_left_from_1_to_2_and_3 {
    ($func_name:ident, $dual_1:ident, $dual_2:ident, $role_broadcast:ident, $next:ident, $sender:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::sessionmpst::SessionMpst<
                mpstthree::functionmpst::ChooseMpst<
                    S2,
                    S0,
                    S4,
                    S1,
                    R0,
                    R1,
                    $dual_1<mpstthree::role::end::RoleEnd>,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    S3,
                    <S0 as mpstthree::binary::Session>::Dual,
                    S5,
                    <S1 as mpstthree::binary::Session>::Dual,
                    R2,
                    R3,
                    $dual_2<mpstthree::role::end::RoleEnd>,
                >,
                $role_broadcast<R4, R5>,
                $sender<mpstthree::role::end::RoleEnd>,
            >,
        ) -> mpstthree::sessionmpst::SessionMpst<S2, S3, R4, $sender<mpstthree::role::end::RoleEnd>>
        where
            S0: mpstthree::binary::Session + 'a,
            S1: mpstthree::binary::Session + 'a,
            S2: mpstthree::binary::Session + 'a,
            S3: mpstthree::binary::Session + 'a,
            S4: mpstthree::binary::Session + 'a,
            S5: mpstthree::binary::Session + 'a,
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
                $dual_1,
                $dual_2,
                $sender,
                s,
                either::Either::Left,
                $next
            )
        }
    };
}

///  Create the *ChooseMpst* function to send a *Choose* right branch from the first role to the others.
///  Must be used with [`mpstthree::sessionmpst::SessionMpst`].
///
///  # Arguments
///  
///  * The name of the new *ChooseMpst* function
///  * The name of the dual of the first receiver
///  * The name of the dual of the second receiver
///  * The name of the broadcasting sender. This one should contain *toAll* according to the convention
///  * The name of related *next* function
///  * The name of the sender
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_broadcast_role, create_sessionmpst, create_choose_right_from_1_to_2_and_3};
///
///  create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
///  create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
///  create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
///
///  create_broadcast_role!(RoleAlltoA, next_all_to_a, RoleAtoAll, next_a_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_choose_right_from_1_to_2_and_3!(
///      choose_right_mpst_session_a_to_all,
///      RoleBDual,
///      RoleCDual,
///      RoleAtoAll,
///      next_a_to_all,
///      RoleA
///  );
///  ```
///
///  [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_choose_right_from_1_to_2_and_3 {
    ($func_name:ident, $dual_1:ident, $dual_2:ident, $role_broadcast:ident, $next:ident, $sender:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::sessionmpst::SessionMpst<
                mpstthree::functionmpst::ChooseMpst<
                    S2,
                    S0,
                    S4,
                    S1,
                    R0,
                    R1,
                    $dual_1<mpstthree::role::end::RoleEnd>,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    S3,
                    <S0 as mpstthree::binary::Session>::Dual,
                    S5,
                    <S1 as mpstthree::binary::Session>::Dual,
                    R2,
                    R3,
                    $dual_2<mpstthree::role::end::RoleEnd>,
                >,
                $role_broadcast<R4, R5>,
                $sender<mpstthree::role::end::RoleEnd>,
            >,
        ) -> mpstthree::sessionmpst::SessionMpst<S4, S5, R5, $sender<mpstthree::role::end::RoleEnd>>
        where
            S0: mpstthree::binary::Session + 'a,
            S1: mpstthree::binary::Session + 'a,
            S2: mpstthree::binary::Session + 'a,
            S3: mpstthree::binary::Session + 'a,
            S4: mpstthree::binary::Session + 'a,
            S5: mpstthree::binary::Session + 'a,
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
                $dual_1,
                $dual_2,
                $sender,
                s,
                either::Either::Right,
                $next
            )
        }
    };
}

///  Create the *ChooseMpst* function to send a *Choose* left branch from the second role to the others.
///  Must be used with [`mpstthree::sessionmpst::SessionMpst`].
///
///  # Arguments
///  
///  * The name of the new *ChooseMpst* function
///  * The name of the dual of the first receiver
///  * The name of the dual of the second receiver
///  * The name of the broadcasting sender. This one should contain *toAll* according to the convention
///  * The name of related *next* function
///  * The name of the sender
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_broadcast_role, create_sessionmpst, create_choose_left_from_2_to_1_and_3};
///
///  create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
///  create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
///  create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
///
///  create_broadcast_role!(RoleAlltoB, next_all_to_b, RoleBtoAll, next_b_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_choose_left_from_2_to_1_and_3!(
///      choose_right_mpst_session_b_to_all,
///      RoleADual,
///      RoleCDual,
///      RoleBtoAll,
///      next_b_to_all,
///      RoleB
///  );
///  ```
///
///  [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_choose_left_from_2_to_1_and_3 {
    ($func_name:ident, $dual_1:ident, $dual_2:ident, $role_broadcast:ident, $next:ident, $sender:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::sessionmpst::SessionMpst<
                mpstthree::functionmpst::ChooseMpst<
                    S2,
                    S0,
                    S4,
                    S1,
                    R0,
                    R1,
                    $dual_1<mpstthree::role::end::RoleEnd>,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    <S0 as mpstthree::binary::Session>::Dual,
                    S3,
                    <S1 as mpstthree::binary::Session>::Dual,
                    S5,
                    R2,
                    R3,
                    $dual_2<mpstthree::role::end::RoleEnd>,
                >,
                $role_broadcast<R4, R5>,
                $sender<mpstthree::role::end::RoleEnd>,
            >,
        ) -> mpstthree::sessionmpst::SessionMpst<S2, S3, R4, $sender<mpstthree::role::end::RoleEnd>>
        where
            S0: mpstthree::binary::Session + 'a,
            S1: mpstthree::binary::Session + 'a,
            S2: mpstthree::binary::Session + 'a,
            S3: mpstthree::binary::Session + 'a,
            S4: mpstthree::binary::Session + 'a,
            S5: mpstthree::binary::Session + 'a,
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
                $dual_1,
                $dual_2,
                $sender,
                s,
                either::Either::Left,
                $next
            )
        }
    };
}

///  Create the *ChooseMpst* function to send a *Choose* right branch from the second role to the others.
///  Must be used with [`mpstthree::sessionmpst::SessionMpst`].
///
///  # Arguments
///  
///  * The name of the new *ChooseMpst* function
///  * The name of the dual of the first receiver
///  * The name of the dual of the second receiver
///  * The name of the broadcasting sender. This one should contain *toAll* according to the convention
///  * The name of related *next* function
///  * The name of the sender
///  
///  # Example
///  
///  ```
///  use mpstthree::role::Role;
///  use mpstthree::{create_normal_role, create_broadcast_role, create_sessionmpst, create_choose_right_from_2_to_1_and_3};
///
///  create_normal_role!(RoleA, next_a, RoleADual, next_a_dual);
///  create_normal_role!(RoleB, next_b, RoleBDual, next_b_dual);
///  create_normal_role!(RoleC, next_c, RoleCDual, next_c_dual);
///
///  create_broadcast_role!(RoleAlltoB, next_all_to_b, RoleBtoAll, next_b_to_all);
///
///  create_sessionmpst!(SessionMpst, 3);
///
///  create_choose_right_from_2_to_1_and_3!(
///      choose_right_mpst_session_b_to_all,
///      RoleADual,
///      RoleCDual,
///      RoleBtoAll,
///      next_b_to_all,
///      RoleB
///  );
///  ```
///
///  [`mpstthree::sessionmpst::SessionMpst`]: ../sessionmpst/struct.SessionMpst.html.
#[macro_export]
macro_rules! create_choose_right_from_2_to_1_and_3 {
    ($func_name:ident, $dual_1:ident, $dual_2:ident, $role_broadcast:ident, $next:ident, $sender:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: mpstthree::sessionmpst::SessionMpst<
                mpstthree::functionmpst::ChooseMpst<
                    S2,
                    S0,
                    S4,
                    S1,
                    R0,
                    R1,
                    $dual_1<mpstthree::role::end::RoleEnd>,
                >,
                mpstthree::functionmpst::ChooseMpst<
                    <S0 as mpstthree::binary::Session>::Dual,
                    S3,
                    <S1 as mpstthree::binary::Session>::Dual,
                    S5,
                    R2,
                    R3,
                    $dual_2<mpstthree::role::end::RoleEnd>,
                >,
                $role_broadcast<R4, R5>,
                $sender<mpstthree::role::end::RoleEnd>,
            >,
        ) -> mpstthree::sessionmpst::SessionMpst<S4, S5, R5, $sender<mpstthree::role::end::RoleEnd>>
        where
            S0: mpstthree::binary::Session + 'a,
            S1: mpstthree::binary::Session + 'a,
            S2: mpstthree::binary::Session + 'a,
            S3: mpstthree::binary::Session + 'a,
            S4: mpstthree::binary::Session + 'a,
            S5: mpstthree::binary::Session + 'a,
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
                $dual_1,
                $dual_2,
                $sender,
                s,
                either::Either::Right,
                $next
            )
        }
    };
}

///  Choose among two different sessions
///  
///  # Arguments
///  
///   * The session to be used
///   * The first path to be sent
///   * The second path to be sent
///   * The *send* function to be used for the first path
///   * The *send* function to be used for the second path
///   * The first passive role
///   * The second passive role
///   * The active role
///  
///  # Example
///  
///  ```ignore
///  match xs.pop() {
///      Option::Some(_) => {
///          let s = choose_mpst_to_all!(
///              s,
///              CBranchesAtoC::Video,
///              CBranchesBtoC::Video,
///              send_mpst_c_to_a,
///              send_mpst_c_to_b,
///              RoleA,
///              RoleB,
///              RoleC
///          );
///          let s = send_mpst_c_to_a(1, s);
///          let (_, s) = recv_mpst_c_to_a(s)?;
///          client_recurs(s, xs, index + 1)
///      }
///      Option::None => {
///          let s = choose_mpst_to_all!(
///              s,
///              CBranchesAtoC::End,
///              CBranchesBtoC::End,
///              send_mpst_c_to_a,
///              send_mpst_c_to_b,
///              RoleA,
///              RoleB,
///              RoleC
///          );
///          close_mpst(s)?;
///          Ok(())
///      }
///  }
///  ```
#[macro_export]
macro_rules! choose_mpst_to_all {
    ($session:expr, $label_1:path, $label_2:path, $fn_send_1:ident, $fn_send_2:ident, $receiver_1:ident, $receiver_2:ident, $sender:ident) => {{
        let (session_1_3, session_3_1) = <_ as mpstthree::binary::Session>::new();
        let (session_2_3, session_3_2) = <_ as mpstthree::binary::Session>::new();
        let (session_1_2, session_2_1) = <_ as mpstthree::binary::Session>::new();
        let (role_1, _) = <_ as Role>::new();
        let (role_2, _) = <_ as Role>::new();
        let (role_3, _) = <_ as Role>::new();
        let (name_1, _) = <$receiver_1<mpstthree::role::end::RoleEnd> as Role>::new();
        let (name_2, _) = <$receiver_2<mpstthree::role::end::RoleEnd> as Role>::new();
        let (name_3, _) = <$sender<mpstthree::role::end::RoleEnd> as Role>::new();

        let s = $fn_send_1(
            $label_1(mpstthree::sessionmpst::SessionMpst {
                session1: session_1_2,
                session2: session_1_3,
                stack: role_1,
                name: name_1,
            }),
            $session,
        );
        let s = $fn_send_2(
            $label_2(mpstthree::sessionmpst::SessionMpst {
                session1: session_2_1,
                session2: session_2_3,
                stack: role_2,
                name: name_2,
            }),
            s,
        );

        mpstthree::binary::cancel(s);

        mpstthree::sessionmpst::SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_3,
            name: name_3,
        }
    }};
}
