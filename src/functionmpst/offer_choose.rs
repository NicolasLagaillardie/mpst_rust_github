////pub type Offer_Mpst_one<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>> = Recv<Either<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>>, End>;
//
////pub type Choose_Mpst_one<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>> = Send<Either<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>>, End>;
//
//pub fn offer_mpst_session_one_B_to_A<'a, S1, S2, S3, S4, F, G, R, R1, R2, U, C1, C2>(
//    s: SessionMpst<Offer_Mpst_one<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>>, S4, RoleBtoA<R>>,
//    f: F,
//    g: G,
//) -> Result<(U), Box<dyn Error + 'a>>
//where
//    S1: Session,
//    S2: Session,
//    S3: Session,
//    S4: Session,
//    R1: Role,
//    R2: Role,
//    R: Role,
//    F: FnOnce(SessionMpst<S1, S3, R1>) -> Result<U, Box<dyn Error + 'a>>,
//    G: FnOnce(SessionMpst<S2, S3, R2>) -> Result<U, Box<dyn Error + 'a>>,
//{
//    let (e, new_session) = recv_mpst_session_one_B_to_A(s)?;
//    cancel(new_session);
//    e.either(f, g)
//}
//
//pub fn choose_left_mpst_session_one_A_to_B<S1, S2, S3, R, R1, R2>(
//    s: SessionMpst<Choose_Mpst_one<SessionMpst<S1, S3, R1>, SessionMpst<S2, S3, R2>>, S4, RoleAtoB<R>>,
//) -> SessionMpst<S1, S3, R1>
//where
//    S1: Session,
//    S2: Session,
//    S3: Session,
//    S4: Session,
//    R: Role,
//    R1: Role,
//    R2: Role,
//{
//    let (here, there) = S1::new();
//    let new_session = send(Either::Left(there), s.session1);
//    cancel(new_session);
//    let new_queue = nextAtoB(s.queue);
//
//    let result = SessionMpst {
//        session1: here,
//        session2: s.session2,
//        queue: new_queue,
//    };
//
//    result
//}
//
//pub fn choose_right_mpst_session_one_A_to_B<S1, S2, S3, R>(
//    s: SessionMpst<Choose<S1, S2>, S3, RoleAtoB<R>>,
//) -> SessionMpst<S2, S3, R>
//where
//    S1: Session,
//    S2: Session,
//    S3: Session,
//    R: Role,
//{
//    let (here, there) = S2::new();
//    let new_session = send(Either::Right(there), s.session1);
//    cancel(new_session);
//    let new_queue = nextAtoB(s.queue);
//
//    let result = SessionMpst {
//        session1: here,
//        session2: s.session2,
//        queue: new_queue,
//    };
//
//    result
//}
