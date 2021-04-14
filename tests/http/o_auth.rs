// use mpstthree::binary::struct_trait::{End, Recv, Send};
// use mpstthree::role::broadcast::RoleBroadcast;
// use mpstthree::role::end::RoleEnd;
// use mpstthree::{
//     bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
//     create_multiple_normal_role_short, create_recv_http_session_bundle,
//     create_send_mpst_http_bundle, offer_http_mpst,
// };

// use hyper::{Body, Client, Method, Request, StatusCode};
// use hyper_tls::HttpsConnector;
// use rand::{thread_rng, Rng};
// use std::collections::hash_map::RandomState;
// use std::collections::HashMap;
// use std::error::Error;
// use std::fs;
// use std::marker;

// // TODO: update with the correct protocol

// // global protocol OAuth(role Auth, role Client, role Server) {

// //     Authorization(Approval) from Client to Auth; // Request Authorization Approval

// //     choice at A
// //     {
// //         Access(Token) from Auth to Client;

// //         rec Loop {
// //             choice at Client { // Client makes a choice

// //                 RequestPicture(Token) from Client to Server; // Client sends a request for a picture, giving its access token
// //                 SendPicture(Picture) from Server to Client; // Server sends the picture file to the client

// //                 continue Loop; // A Recursive call
// //             } or {
// //                 Close() from Client to Server; // Close the session between Client and Server
// //                 Close() from Client to Auth; // Close the session between Client and Auth
// //             }
// //         }

// //     } or {
// //         Close() from Auth to Client; // Close the session between Client and Auth
// //         Close() from Client to Server; // Close the session between Client and Server

// //     }
// // }

// // Create the new SessionMpst for three participants and the close and fork functions
// bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstThree, 3);

// // Create new roles
// // normal
// create_multiple_normal_role_short!(A, C, S);

// // Create new send functions
// // A
// create_send_mpst_http_bundle!(
//     send_http_a_to_s, RoleS, 2 | =>
//     RoleA, SessionMpstThree, 3
// );
// // C
// create_send_mpst_http_bundle!(
//     send_http_c_to_a, RoleA, 1 | =>
//     RoleC, SessionMpstThree, 3
// );
// // S
// create_send_mpst_http_bundle!(
//     send_http_s_to_c, RoleC, 2 | =>
//     RoleS, SessionMpstThree, 3
// );

// // Create new recv functions and related types
// // A
// create_recv_http_session_bundle!(
//     recv_http_a_to_c, RoleC, 1 |
//     recv_http_a_to_s, RoleS, 2 | =>
//     RoleA, SessionMpstThree, 3
// );
// // C
// create_recv_http_session_bundle!(
//     recv_http_c_to_a, RoleA, 1 |
//     recv_http_c_to_s, RoleS, 2 | =>
//     RoleC, SessionMpstThree, 3
// );
// // S
// create_recv_http_session_bundle!(
//     recv_http_s_to_a, RoleA, 1 | =>
//     RoleS, SessionMpstThree, 3
// );

// // Names
// type NameA = RoleA<RoleEnd>;
// type NameC = RoleC<RoleEnd>;
// type NameS = RoleS<RoleEnd>;

// // Types
// // C
// type Choose1fromCtoA<N> = Send<Branching1fromCtoA<N>, End>;
// type Choose1fromCtoS<N> = Send<Branching1fromCtoS<N>, End>;
// // A
// type Choose0fromAtoC<N> = Send<Branching0fromAtoC<N>, End>;
// type Choose0fromAtoS<N> = Send<Branching0fromAtoS<N>, End>;

// // A
// enum Branching1fromCtoA<N: marker::Send> {
//     Request(
//         SessionMpstThree<
//             Recv<N, Recv<N, Choose1fromCtoA<N>>>,
//             End,
//             RoleC<RoleC<RoleBroadcast>>,
//             NameA,
//         >,
//     ),
//     Done(SessionMpstThree<Recv<N, End>, End, RoleC<RoleEnd>, NameA>),
// }
// type EndpointA<N> = SessionMpstThree<
//     Recv<N, Send<Branching0fromAtoC<N>, End>>,
//     Send<Branching0fromAtoS<N>, End>,
//     RoleC<RoleC<RoleBroadcast>>,
//     NameA,
// >;
// // C
// enum Branching0fromAtoC<N: marker::Send> {
//     Auth(
//         SessionMpstThree<
//             Recv<N, Recv<N, Choose1fromCtoA<N>>>,
//             Choose1fromCtoS<N>,
//             RoleA<RoleA<RoleBroadcast>>,
//             NameC,
//         >,
//     ),
//     Done(SessionMpstThree<Send<N, End>, Send<N, End>, RoleS<RoleA<RoleEnd>>, NameC>),
// }
// // S
// enum Branching0fromAtoS<N: marker::Send> {
//     Auth(SessionMpstThree<End, Recv<Branching1fromCtoS<N>, End>, RoleC<RoleEnd>, NameS>),
//     Done(SessionMpstThree<End, Recv<N, End>, RoleC<RoleEnd>, NameS>),
// }
// enum Branching1fromCtoS<N: marker::Send> {
//     Request(
//         SessionMpstThree<
//             End,
//             Recv<N, Send<N, Recv<Branching1fromCtoS<N>, End>>>,
//             RoleC<RoleC<RoleC<RoleEnd>>>,
//             NameS,
//         >,
//     ),
//     Done(SessionMpstThree<End, Recv<N, End>, RoleC<RoleEnd>, NameS>),
// }
// // Creating the MP sessions
// // A
// type EndpointA<N> = SessionMpstThree<
//     Recv<N, Send<Branching0fromAtoC<N>, End>>,
//     Send<Branching0fromAtoS<N>, End>,
//     RoleC<RoleC<RoleBroadcast>>,
//     NameA,
// >;
// // C
// type EndpointC<N> = SessionMpstThree<Recv<Branching0fromAtoC<N>, End>, End, RoleA<RoleA<RoleEnd>>, NameC>;
// // S
// type EndpointS<N> = SessionMpstThree<Recv<Branching0fromAtoS<N>, End>, End, RoleA<RoleEnd>, NameS>;

// create_fn_choose_mpst_multi_to_all_bundle!(
//     done_from_c_to_all, login_from_c_to_all, =>
//     Request, Done, =>
//     EndpointDoneS<i32>, EndpointLoginS<i32>, =>
//     Branching1fromCtoA::<i32>, Branching1fromCtoS::<i32>, =>
//     RoleA, RoleS, =>
//     RoleC, SessionMpstThree, 3, 2
// );

// create_fn_choose_mpst_multi_to_all_bundle!(
//     auth_from_a_to_all, again_from_a_to_all, =>
//     Auth, Done, =>
//     EndpointAuthA<i32>, EndpointAgainA<i32>, =>
//     Branching0fromAtoC::<i32>, Branching0fromAtoS::<i32>, =>
//     RoleC, RoleS, =>
//     RoleA, SessionMpstThree, 3, 1
// );

// // Functions
// fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
//     offer_http_mpst!(s, recv_http_a_to_s, {
//         Branching0fromStoA::Done(s) => {
//             let https = HttpsConnector::new();
//             let client = Client::builder().build::<_, Body>(https);
//             let (_, s, _resp) = recv_http_a_to_c(s, false, client.request(Request::default()))?;

//             close_mpst_multi(s)
//         },
//         Branching0fromStoA::Login(s) => {
//             choice_a(s)
//         },
//     })
// }

// fn choice_a(s: ChoiceA<i32>) -> Result<(), Box<dyn Error>> {
//     let (pwd, s, _resp) = recv_http_a_to_c(s, false, ResponseFuture::default())?;
//     let expected = thread_rng().gen_range(1..=3);

//     if pwd == expected {
//         let s = auth_from_a_to_all(s);

//         let (s, _req) = send_http_a_to_s(0, s, false, Method::GET, "", vec![("", "")], "");

//         close_mpst_multi(s)
//     } else {
//         let s = again_from_a_to_all(s);

//         let (s, _req) = send_http_a_to_s(1, s, false, Method::GET, "", vec![("", "")], "");

//         choice_a(s)
//     }
// }

// fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
//     offer_http_mpst!(s, recv_http_c_to_s, {
//         Branching0fromStoC::<i32>::Done(s) => {
//             let (quit, s, _resp) = recv_http_c_to_s(s, false, ResponseFuture::default())?;
//             let (s, _req) = send_http_c_to_a(quit, s, false, Method::GET, "", vec![("", "")], "");
//             close_mpst_multi(s)
//         },
//         Branching0fromStoC::<i32>::Login(s) => {

//             /////////////
//             // Get the tokens

//             match fs::read_to_string("imgur.env") {
//                 Ok(contents) => {
//                     let lines: Vec<&str> = contents.split("\n").collect();
//                     let hasher = RandomState::new();
//                     let mut ids: HashMap<&str, &str> = HashMap::with_hasher(hasher);
//                     for line in lines {
//                         let temp: Vec<&str> = line.split("=").collect();
//                         ids.insert(temp[0], temp[1]);
//                     }

//                     let req = Request::builder()
//                         .method(Method::GET)
//                         .uri(ids["CREDITS_URL"])
//                         .header("content-type", ids["CONTENT_TYPE"])
//                         .header(
//                             "Authorization",
//                             format!("{} {}", ids["TOKEN_TYPE"], ids["ACCESS_TOKEN"]),
//                         )
//                         .header("User-Agent", ids["USER_AGENT"])
//                         .header("Accept", ids["ACCEPT"])
//                         .header("Connection", ids["CONNECTION"])
//                         .body(Body::default())?;

//                     /////////////
//                     let (_, s, resp) = recv_http_c_to_s(s, true, req)?;

//                     assert_eq!(resp.status(), StatusCode::from_u16(200).unwrap());

//                     choice_c(s)
//                 }
//                 Err(_) => {
//                     let (_, s, _resp) = recv_http_c_to_s(s, false, ResponseFuture::default())?;

//                     choice_c(s)
//                 }
//             }
//         },
//     })
// }

// fn choice_c(s: ChoiceC<i32>) -> Result<(), Box<dyn Error>> {
//     let (s, _req) = send_http_c_to_a(
//         thread_rng().gen_range(1..=3),
//         s,
//         false,
//         Method::GET,
//         "",
//         vec![("", "")],
//         "",
//     );

//     offer_http_mpst!(s, recv_http_c_to_a, {
//         Branching1fromAtoC::<i32>::Auth(s) => {
//             let (_, s, _resp) = recv_http_c_to_s(s, false, Request::default())?;

//             close_mpst_multi(s)
//         },
//         Branching1fromAtoC::<i32>::Again(s) => {
//             let (_, s, _resp) = recv_http_c_to_s(s, false, Request::default())?;

//             choice_c(s)
//         },
//     })
// }

// fn endpoint_s(s: EndpointS<i32>) -> Result<(), Box<dyn Error>> {
//     let choice = thread_rng().gen_range(1..=6);

//     if choice == 1 {
//         let s = done_from_s_to_all(s);

//         let (s, _req) = send_http_s_to_c(0, s, false, Method::GET, "", vec![("", "")], "");

//         close_mpst_multi(s)
//     } else {
//         let s = login_from_s_to_all(s);

//         let (s, _req) = send_http_s_to_c(1, s, false, Method::GET, "", vec![("", "")], "");

//         choice_s(s)
//     }
// }

// fn choice_s(s: ChoiceS<i32>) -> Result<(), Box<dyn Error>> {
//     offer_http_mpst!(s, recv_http_s_to_a, {
//         Branching1fromAtoS::<i32>::Auth(s) => {
//             let (success, s, _resp) = recv_http_s_to_a(s, false, Request::default())?;
//             let (s, _req) = send_http_s_to_c(success, s, false, Method::GET, "", vec![("", "")], "");

//             close_mpst_multi(s)
//         },
//         Branching1fromAtoS::<i32>::Again(s) => {
//             let (fail, s, _resp) = recv_http_s_to_a(s, false, Request::default())?;
//             let (s, _req) = send_http_s_to_c(fail, s, false, Method::GET, "", vec![("", "")], "");

//             choice_s(s)
//         },
//     })
// }

// /////////////////////////

// pub fn main() {
//     let (thread_a, thread_c, thread_s) = fork_mpst(
//         endpoint_a,
//         endpoint_c,
//         endpoint_s,
//     );

//     assert!(thread_a.join().is_ok());
//     assert!(thread_c.join().is_ok());
//     assert!(thread_s.join().is_ok());
// }

pub fn main() {}
