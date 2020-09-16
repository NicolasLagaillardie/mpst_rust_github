////////////////////////////////////////////
/// SEND

// create a function send_mpst for the first session
#[macro_export]
macro_rules! create_send_mpst_session_1 {
    ($func_name:ident, $role:ident, $next:ident, $name:ident) => {
        fn $func_name<T, S1, S2, R>(
            x: T,
            s: SessionMpst<Send<T, S1>, S2, $role<R>, $name<RoleEnd>>,
        ) -> SessionMpst<S1, S2, R, $name<RoleEnd>>
        where
            T: marker::Send,
            S1: Session,
            S2: Session,
            R: Role,
        {
            let new_session = send(x, s.session1);
            let new_queue = $next(s.stack);

            SessionMpst {
                session1: new_session,
                session2: s.session2,
                stack: new_queue,
                name: s.name,
            }
        }
    };
}

// create a function send_mpst for the second session
#[macro_export]
macro_rules! create_send_mpst_session_2 {
    ($func_name:ident, $role:ident, $next:ident, $name:ident) => {
        fn $func_name<T, S1, S2, R>(
            x: T,
            s: SessionMpst<S1, Send<T, S2>, $role<R>, $name<RoleEnd>>,
        ) -> SessionMpst<S1, S2, R, $name<RoleEnd>>
        where
            T: marker::Send,
            S1: Session,
            S2: Session,
            R: Role,
        {
            let new_session = send(x, s.session2);
            let new_queue = $next(s.stack);

            SessionMpst {
                session1: s.session1,
                session2: new_session,
                stack: new_queue,
                name: s.name,
            }
        }
    };
}

////////////////////////////////////////////
/// RECV

// create a function recv_mpst for the first session
#[macro_export]
macro_rules! create_recv_mpst_session_1 {
    ($func_name:ident, $role:ident, $next:ident, $name:ident) => {
        fn $func_name<T, S1, S2, R>(
            s: SessionMpst<Recv<T, S1>, S2, $role<R>, $name<RoleEnd>>,
        ) -> Result<(T, SessionMpst<S1, S2, R, $name<RoleEnd>>), Box<dyn Error>>
        where
            T: marker::Send,
            S1: Session,
            S2: Session,
            R: Role,
        {
            let (v, new_session) = recv(s.session1)?;
            let new_queue = $next(s.stack);
            let result = SessionMpst {
                session1: new_session,
                session2: s.session2,
                stack: new_queue,
                name: s.name,
            };

            Ok((v, result))
        }
    };
}

// create a function recv_mpst for the second session
#[macro_export]
macro_rules! create_recv_mpst_session_2 {
    ($func_name:ident, $role:ident, $next:ident, $name:ident) => {
        fn $func_name<T, S1, S2, R>(
            s: SessionMpst<S1, Recv<T, S2>, $role<R>, $name<RoleEnd>>,
        ) -> Result<(T, SessionMpst<S1, S2, R, $name<RoleEnd>>), Box<dyn Error>>
        where
            T: marker::Send,
            S1: Session,
            S2: Session,
            R: Role,
        {
            let (v, new_session) = recv(s.session2)?;
            let new_queue = $next(s.stack);
            let result = SessionMpst {
                session1: s.session1,
                session2: new_session,
                stack: new_queue,
                name: s.name,
            };

            Ok((v, result))
        }
    };
}

// create a function recv_mpst_all for the first session
#[macro_export]
macro_rules! create_recv_mpst_all_session_1 {
    ($func_name:ident, $role:ident, $next:ident, $name:ident) => {
        fn $func_name<T, S1, S2, R>(
            s: SessionMpst<Recv<T, S1>, S2, $role<R, R>, $name<RoleEnd>>,
        ) -> Result<(T, SessionMpst<S1, S2, R, $name<RoleEnd>>), Box<dyn Error>>
        where
            T: marker::Send,
            S1: Session,
            S2: Session,
            R: Role,
        {
            let (v, new_session) = recv(s.session1)?;
            let (new_queue, _) = $next(s.stack);
            let result = SessionMpst {
                session1: new_session,
                session2: s.session2,
                stack: new_queue,
                name: s.name,
            };

            Ok((v, result))
        }
    };
}

// create a function recv_mpst_all for the second session
#[macro_export]
macro_rules! create_recv_mpst_all_session_2 {
    ($func_name:ident, $role:ident, $next:ident, $name:ident) => {
        fn $func_name<T, S1, S2, R>(
            s: SessionMpst<S1, Recv<T, S2>, $role<R, R>, $name<RoleEnd>>,
        ) -> Result<(T, SessionMpst<S1, S2, R, $name<RoleEnd>>), Box<dyn Error>>
        where
            T: marker::Send,
            S1: Session,
            S2: Session,
            R: Role,
        {
            let (v, new_session) = recv(s.session2)?;
            let (new_queue, _) = $next(s.stack);
            let result = SessionMpst {
                session1: s.session1,
                session2: new_session,
                stack: new_queue,
                name: s.name,
            };

            Ok((v, result))
        }
    };
}

////////////////////////////////////////////
/// OFFER

/// Get an offer on session 1
#[macro_export]
macro_rules! create_offer_mpst_session_1 {
    ($func_name:ident, $role:ident, $recv_func:ident, $name:ident) => {
        fn $func_name<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
            s: SessionMpst<
                OfferMpst<S1, S2, S3, S4, R1, R2, $name<RoleEnd>>,
                S5,
                $role<R3, R3>,
                $name<RoleEnd>,
            >,
            f: F,
            g: G,
        ) -> Result<U, Box<dyn Error + 'a>>
        where
            S1: Session,
            S2: Session,
            S3: Session,
            S4: Session,
            S5: Session,
            R1: Role,
            R2: Role,
            R3: Role,
            F: FnOnce(SessionMpst<S1, S2, R1, $name<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
            G: FnOnce(SessionMpst<S3, S4, R2, $name<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
        {
            let (e, s) = $recv_func(s)?;
            cancel(s);
            e.either(f, g)
        }
    };
}

/// Get an offer on session 2
#[macro_export]
macro_rules! create_offer_mpst_session_2 {
    ($func_name:ident, $role:ident, $recv_func:ident, $name:ident) => {
        fn $func_name<'a, S1, S2, S3, S4, S5, F, G, R1, R2, R3, U>(
            s: SessionMpst<
                S5,
                OfferMpst<S1, S2, S3, S4, R1, R2, $name<RoleEnd>>,
                $role<R3, R3>,
                $name<RoleEnd>,
            >,
            f: F,
            g: G,
        ) -> Result<U, Box<dyn Error + 'a>>
        where
            S1: Session,
            S2: Session,
            S3: Session,
            S4: Session,
            S5: Session,
            R1: Role,
            R2: Role,
            R3: Role,
            F: FnOnce(SessionMpst<S1, S2, R1, $name<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
            G: FnOnce(SessionMpst<S3, S4, R2, $name<RoleEnd>>) -> Result<U, Box<dyn Error + 'a>>,
        {
            let (e, s) = $recv_func(s)?;
            cancel(s);
            e.either(f, g)
        }
    };
}

////////////////////////////////////////////
/// CHOOSE

/// Create the core for the choose_mpst macros
#[macro_export]
macro_rules! create_choose_from_1_to_2_3 {
    ($session_1:ty, $session_2:ty, $session_3:ty, $role_1:ty, $role_2:ty, $role_3:ty, $receiver_1:ident, $receiver_2:ident, $sender:ident, $session:expr, $pat:path, $next:ident) => {{
        let (session_1_2, session_2_1) = <$session_1>::new();
        let (session_1_3, session_3_1) = <$session_2>::new();
        let (session_3_2, session_2_3) = <$session_3>::new();
        let (_, role_1) = <$role_1>::new();
        let (_, role_2) = <$role_2>::new();
        let (role_3, _) = <$role_3>::new();
        let (name_1, _) = <$receiver_1<RoleEnd> as Role>::Dual::new();
        let (name_2, _) = <$receiver_2<RoleEnd> as Role>::Dual::new();
        let (name_3, _) = $sender::<RoleEnd>::new();

        let choice_1 = SessionMpst {
            session1: session_2_1,
            session2: session_2_3,
            stack: role_1,
            name: name_1,
        };

        let choice_2 = SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);
        let (_, new_queue) = $next($session.stack);

        let s = SessionMpst {
            session1: new_session_1,
            session2: new_session_2,
            stack: new_queue,
            name: $session.name,
        };

        cancel(s);

        SessionMpst {
            session1: session_1_2,
            session2: session_1_3,
            stack: role_3,
            name: name_3,
        }
    }};
}

#[macro_export]
macro_rules! create_choose_from_2_to_1_3 {
    ($session_1:ty, $session_2:ty, $session_3:ty, $role_1:ty, $role_2:ty, $role_3:ty, $receiver_1:ident, $receiver_2:ident, $sender:ident, $session:expr, $pat:path, $next:ident) => {{
        let (session_2_1, session_1_2) = <$session_1>::new();
        let (session_2_3, session_3_2) = <$session_2>::new();
        let (session_3_1, session_1_3) = <$session_3>::new();
        let (_, role_1) = <$role_1>::new();
        let (_, role_2) = <$role_2>::new();
        let (role_3, _) = <$role_3>::new();
        let (name_1, _) = <$receiver_1<RoleEnd> as Role>::Dual::new();
        let (name_2, _) = <$receiver_2<RoleEnd> as Role>::Dual::new();
        let (name_3, _) = $sender::<RoleEnd>::new();

        let choice_1 = SessionMpst {
            session1: session_1_2,
            session2: session_1_3,
            stack: role_1,
            name: name_1,
        };

        let choice_2 = SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);
        let (_, new_queue) = $next($session.stack);

        let s = SessionMpst {
            session1: new_session_1,
            session2: new_session_2,
            stack: new_queue,
            name: $session.name,
        };

        cancel(s);

        SessionMpst {
            session1: session_2_1,
            session2: session_2_3,
            stack: role_3,
            name: name_3,
        }
    }};
}

#[macro_export]
macro_rules! create_choose_from_3_to_1_2 {
    ($session_1:ty, $session_2:ty, $session_3:ty, $role_1:ty, $role_2:ty, $role_3:ty, $receiver_1:ident, $receiver_2:ident, $sender:ident, $session:expr, $pat:path, $next:ident) => {{
        let (session_3_1, session_1_3) = <$session_1>::new();
        let (session_3_2, session_2_3) = <$session_2>::new();
        let (session_2_1, session_1_2) = <$session_3>::new();
        let (_, role_1) = <$role_1>::new();
        let (_, role_2) = <$role_2>::new();
        let (role_3, _) = <$role_3>::new();
        let (name_1, _) = <$receiver_1<RoleEnd> as Role>::Dual::new();
        let (name_2, _) = <$receiver_2<RoleEnd> as Role>::Dual::new();
        let (name_3, _) = $sender::<RoleEnd>::new();

        let choice_1 = SessionMpst {
            session1: session_1_2,
            session2: session_1_3,
            stack: role_1,
            name: name_1,
        };

        let choice_2 = SessionMpst {
            session1: session_2_1,
            session2: session_2_3,
            stack: role_2,
            name: name_2,
        };

        let new_session_1 = send($pat(choice_1), $session.session1);
        let new_session_2 = send($pat(choice_2), $session.session2);
        let (_, new_queue) = $next($session.stack);

        let s = SessionMpst {
            session1: new_session_1,
            session2: new_session_2,
            stack: new_queue,
            name: $session.name,
        };

        cancel(s);

        SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_3,
            name: name_3,
        }
    }};
}

/// create a function choose_mpst right from the 3rd role
#[macro_export]
macro_rules! create_choose_right_from_3_to_1_and_2 {
    ($func_name:ident, $role:ident, $dual_1:ident, $dual_2:ident, $next:ident, $sender:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: SessionMpst<
                ChooseMpst<S0, S2, S1, S4, R0, R1, $dual_1<RoleEnd>>,
                ChooseMpst<
                    <S0 as Session>::Dual,
                    S3,
                    <S1 as Session>::Dual,
                    S5,
                    R2,
                    R3,
                    $dual_2<RoleEnd>,
                >,
                $role<R4, R5>,
                $sender<RoleEnd>,
            >,
        ) -> SessionMpst<S4, S5, R5, $sender<RoleEnd>>
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
            create_choose_from_3_to_1_2!(
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
                Either::Right,
                $next
            )
        }
    };
}

// create a function choose_mpst left from the 3rd role
#[macro_export]
macro_rules! create_choose_left_from_3_to_1_and_2 {
    ($func_name:ident, $role:ident, $dual_1:ident, $dual_2:ident, $next:ident, $sender:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: SessionMpst<
                ChooseMpst<S0, S2, S1, S4, R0, R1, $dual_1<RoleEnd>>,
                ChooseMpst<
                    <S0 as Session>::Dual,
                    S3,
                    <S1 as Session>::Dual,
                    S5,
                    R2,
                    R3,
                    $dual_2<RoleEnd>,
                >,
                $role<R4, R5>,
                $sender<RoleEnd>,
            >,
        ) -> SessionMpst<S2, S3, R4, $sender<RoleEnd>>
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
            create_choose_from_3_to_1_2!(
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
                Either::Left,
                $next
            )
        }
    };
}

// create a function choose_mpst left from the 1st role
#[macro_export]
macro_rules! create_choose_left_from_1_to_2_and_3 {
    ($func_name:ident, $role:ident, $dual_1:ident, $dual_2:ident, $next:ident, $name:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: SessionMpst<
                ChooseMpst<S2, S0, S4, S1, R0, R1, $dual_1<RoleEnd>>,
                ChooseMpst<
                    S3,
                    <S0 as Session>::Dual,
                    S5,
                    <S1 as Session>::Dual,
                    R2,
                    R3,
                    $dual_2<RoleEnd>,
                >,
                $role<R4, R5>,
                $name<RoleEnd>,
            >,
        ) -> SessionMpst<S2, S3, R4, $name<RoleEnd>>
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
            create_choose_from_1_to_2_3!(
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
                Either::Left,
                $next
            )
        }
    };
}

// create a function choose_mpst right from the 1st role
#[macro_export]
macro_rules! create_choose_right_from_1_to_2_and_3 {
    ($func_name:ident, $role:ident, $dual_1:ident, $dual_2:ident, $next:ident, $name:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: SessionMpst<
                ChooseMpst<S2, S0, S4, S1, R0, R1, $dual_1<RoleEnd>>,
                ChooseMpst<
                    S3,
                    <S0 as Session>::Dual,
                    S5,
                    <S1 as Session>::Dual,
                    R2,
                    R3,
                    $dual_2<RoleEnd>,
                >,
                $role<R4, R5>,
                $name<RoleEnd>,
            >,
        ) -> SessionMpst<S4, S5, R5, $name<RoleEnd>>
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
            create_choose_from_1_to_2_3!(
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
                Either::Right,
                $next
            )
        }
    };
}

// create a function choose_mpst left from the 2nd role
#[macro_export]
macro_rules! create_choose_left_from_2_to_1_and_3 {
    ($func_name:ident, $role:ident, $dual_1:ident, $dual_2:ident, $next:ident, $name:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: SessionMpst<
                ChooseMpst<S2, S0, S4, S1, R0, R1, $dual_1<RoleEnd>>,
                ChooseMpst<
                    S3,
                    <S0 as Session>::Dual,
                    S5,
                    <S1 as Session>::Dual,
                    R2,
                    R3,
                    $dual_2<RoleEnd>,
                >,
                $role<R4, R5>,
                $name<RoleEnd>,
            >,
        ) -> SessionMpst<S2, S3, R4, $name<RoleEnd>>
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
            create_choose_from_2_to_1_3!(
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
                Either::Left,
                $next
            )
        }
    };
}

// create a function choose_mpst right from the 2nd role
#[macro_export]
macro_rules! create_choose_right_from_2_to_1_and_3 {
    ($func_name:ident, $role:ident, $dual_1:ident, $dual_2:ident, $next:ident, $name:ident) => {
        fn $func_name<'a, S0, S1, S2, S3, S4, S5, R0, R1, R2, R3, R4, R5>(
            s: SessionMpst<
                ChooseMpst<S2, S0, S4, S1, R0, R1, $dual_1<RoleEnd>>,
                ChooseMpst<
                    S3,
                    <S0 as Session>::Dual,
                    S5,
                    <S1 as Session>::Dual,
                    R2,
                    R3,
                    $dual_2<RoleEnd>,
                >,
                $role<R4, R5>,
                $name<RoleEnd>,
            >,
        ) -> SessionMpst<S4, S5, R5, $name<RoleEnd>>
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
            create_choose_from_2_to_1_3!(
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
                Either::Right,
                $next
            )
        }
    };
}

/// Choose between many different sessions wrapped in an `enum`
#[macro_export]
macro_rules! choose_mpst_to_all {
    ($session:expr, $label_1:path, $label_2:path, $fn_send_1:ident, $fn_send_2:ident, $receiver_1:ident, $receiver_2:ident, $sender:ident) => {{
        let (session_1_3, session_3_1) = <_ as Session>::new();
        let (session_2_3, session_3_2) = <_ as Session>::new();
        let (session_1_2, session_2_1) = <_ as Session>::new();
        let (role_1, _) = <_ as Role>::new();
        let (role_2, _) = <_ as Role>::new();
        let (role_3, _) = <_ as Role>::new();
        let (name_1, _) = <$receiver_1<RoleEnd> as Role>::new();
        let (name_2, _) = <$receiver_2<RoleEnd> as Role>::new();
        let (name_3, _) = <$sender<RoleEnd> as Role>::new();

        let s = $fn_send_1(
            $label_1(SessionMpst {
                session1: session_1_2,
                session2: session_1_3,
                stack: role_1,
                name: name_1,
            }),
            $session,
        );
        let s = $fn_send_2(
            $label_2(SessionMpst {
                session1: session_2_1,
                session2: session_2_3,
                stack: role_2,
                name: name_2,
            }),
            s,
        );

        cancel(s);

        SessionMpst {
            session1: session_3_1,
            session2: session_3_2,
            stack: role_3,
            name: name_3,
        }
    }};
}
