// ////////////////////////////////////////////
// /// SEND

// // create a function send_mpst for the first session
// #[macro_export]
// macro_rules! create_send_mpst_session_multiple {
//     ($func_name:ident, $role:ident, $next:ident,
//         $struct_name:ident, $($session_name: ident, $session_type: ident, )*) => {
//         fn $func_name<T, $($session_type, )* R>(
//             x: T,
//             s: $struct_name<$($session_type, )* $role<R>>,
//         ) -> SessionMpst<$($session_type, )* R>
//         where
//             T: marker::Send,
//             $(
//                 $session_type: Session,
//             )*
//             R: Role,
//         {
//             let new_session = send(x, s.session1);
//             let new_queue = $next(s.stack);

//             $struct_name {
//                 session1: new_session,
//                 session2: s.session2,
//                 stack: new_queue,
//             }
//         }
//     };
// }

// ////////////////////////////////////////////
// /// RECV

// // create a function recv_mpst for the first session
// #[macro_export]
// macro_rules! create_recv_mpst_session_multiple {
//     ($func_name:ident, $role:ident, $next:ident,
//         $struct_name:ident, $($session_name: ident, $session_type: ident, )*) => {
//         fn $func_name<T, S1, S2, R>(
//             s: SessionMpst<Recv<T, S1>, S2, $role<R>>,
//         ) -> Result<(T, SessionMpst<S1, S2, R>), Box<dyn Error>>
//         where
//             T: marker::Send,
//             S1: Session,
//             S2: Session,
//             R: Role,
//         {
//             let (v, new_session) = recv(s.session1)?;
//             let new_queue = $next(s.stack);
//             let result = SessionMpst {
//                 session1: new_session,
//                 session2: s.session2,
//                 stack: new_queue,
//             };

//             Ok((v, result))
//         }
//     };
// }
