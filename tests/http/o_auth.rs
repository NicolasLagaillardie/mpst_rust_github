use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_http_session_bundle,
    create_send_mpst_http_bundle, offer_http_mpst,
};

use hyper::{Body, Method, Request, StatusCode};
use rand::{thread_rng, Rng};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::marker;

// global protocol Proto(role A, role C, role S)
// {
//     choice at S
//     {
//         login(Int) from S to C;
//         password(Int) from C to A;
//         choice at A
//         {
//              Auth(Int) from A to S;
//              Auth(Int) from S to C;
//         }
//         or
//         {
//              Again(Int) from A to S;
//              Again(Int) from S to C;
//         }
//     }
//     or
//     {
//         cancel(Int) from S to C;
//         quit(Int) from C to A;
//     }
// }

// Create the new SessionMpst for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstThree, 3);

// Create new roles
// normal
create_multiple_normal_role_short!(A, C, S);

// Create new send functions
// A
create_send_mpst_http_bundle!(
    send_http_a_to_s, RoleS, 2 | =>
    RoleA, SessionMpstThree, 3
);
// C
create_send_mpst_http_bundle!(
    send_http_c_to_a, RoleA, 1 | =>
    RoleC, SessionMpstThree, 3
);
// S
create_send_mpst_http_bundle!(
    send_http_s_to_c, RoleC, 2 | =>
    RoleS, SessionMpstThree, 3
);

// Create new recv functions and related types
// A
create_recv_http_session_bundle!(
    recv_http_a_to_c, RoleC, 1 |
    recv_http_a_to_s, RoleS, 2 | =>
    RoleA, SessionMpstThree, 3
);
// C
create_recv_http_session_bundle!(
    recv_http_c_to_a, RoleA, 1 |
    recv_http_c_to_s, RoleS, 2 | =>
    RoleC, SessionMpstThree, 3
);
// S
create_recv_http_session_bundle!(
    recv_http_s_to_a, RoleA, 1 | =>
    RoleS, SessionMpstThree, 3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameS = RoleS<RoleEnd>;

// Types
// S
type Choose0fromStoA<N> = Send<Branching0fromStoA<N>, End>;
type Choose0fromStoC<N> = Send<Branching0fromStoC<N>, End>;
// A
type Choose1fromAtoC<N> = Send<Branching1fromAtoC<N>, End>;
type Choose1fromAtoS<N> = Send<Branching1fromAtoS<N>, End>;

// A
enum Branching0fromStoA<N: marker::Send> {
    Login(
        SessionMpstThree<
            Recv<N, Choose1fromAtoC<N>>,
            Choose1fromAtoS<N>,
            RoleC<RoleBroadcast>,
            NameA,
        >,
    ),
    Done(SessionMpstThree<Recv<N, End>, End, RoleC<RoleEnd>, NameA>),
}
type EndpointAuthA<N> = SessionMpstThree<End, Send<N, End>, RoleS<RoleEnd>, NameA>;
type EndpointAgainA<N> = SessionMpstThree<
    Recv<N, Choose1fromAtoC<N>>,
    Send<N, Choose1fromAtoS<N>>,
    RoleS<RoleC<RoleBroadcast>>,
    NameA,
>;
// C
enum Branching0fromStoC<N: marker::Send> {
    Login(
        SessionMpstThree<
            Send<N, Choice1fromCtoA<N>>,
            Recv<N, End>,
            RoleS<RoleA<RoleA<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(SessionMpstThree<Send<N, End>, Recv<N, End>, RoleS<RoleA<RoleEnd>>, NameC>),
}
enum Branching1fromAtoC<N: marker::Send> {
    Auth(SessionMpstThree<End, Recv<N, End>, RoleS<RoleEnd>, NameC>),
    Again(
        SessionMpstThree<
            Send<N, Choice1fromCtoA<N>>,
            Recv<N, End>,
            RoleS<RoleA<RoleA<RoleEnd>>>,
            NameC,
        >,
    ),
}
type Choice1fromCtoA<N> = Recv<Branching1fromAtoC<N>, End>;
// S
enum Branching1fromAtoS<N: marker::Send> {
    Auth(SessionMpstThree<Recv<N, End>, Send<N, End>, RoleA<RoleC<RoleEnd>>, NameS>),
    Again(
        SessionMpstThree<
            Recv<N, Choice1fromStoA<N>>,
            Send<N, End>,
            RoleA<RoleC<RoleA<RoleEnd>>>,
            NameS,
        >,
    ),
}
type Choice1fromStoA<N> = Recv<Branching1fromAtoS<N>, End>;
type EndpointDoneS<N> = SessionMpstThree<End, Send<N, End>, RoleC<RoleEnd>, NameS>;
type EndpointLoginS<N> =
    SessionMpstThree<Recv<Branching1fromAtoS<N>, End>, Send<N, End>, RoleC<RoleA<RoleEnd>>, NameS>;

// Creating the MP sessions
// A
type ChoiceA<N> =
    SessionMpstThree<Recv<N, Choose1fromAtoC<N>>, Choose1fromAtoS<N>, RoleC<RoleBroadcast>, NameA>;
type EndpointA<N> = SessionMpstThree<End, Recv<Branching0fromStoA<N>, End>, RoleS<RoleEnd>, NameA>;
// C
type ChoiceC<N> = SessionMpstThree<Send<N, Choice1fromCtoA<N>>, End, RoleA<RoleA<RoleEnd>>, NameC>;
type EndpointC<N> = SessionMpstThree<End, Recv<Branching0fromStoC<N>, End>, RoleS<RoleEnd>, NameC>;
// S
type ChoiceS<N> = SessionMpstThree<Choice1fromStoA<N>, End, RoleA<RoleEnd>, NameS>;
type EndpointS<N> = SessionMpstThree<Choose0fromStoA<N>, Choose0fromStoC<N>, RoleBroadcast, NameS>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_s_to_all, login_from_s_to_all, =>
    Done, Login, =>
    EndpointDoneS<i32>, EndpointLoginS<i32>, =>
    Branching0fromStoA::<i32>, Branching0fromStoC::<i32>, =>
    RoleA, RoleC, =>
    RoleS, SessionMpstThree, 3, 3
);

create_fn_choose_mpst_multi_to_all_bundle!(
    auth_from_a_to_all, again_from_a_to_all, =>
    Auth, Again, =>
    EndpointAuthA<i32>, EndpointAgainA<i32>, =>
    Branching1fromAtoC::<i32>, Branching1fromAtoS::<i32>, =>
    RoleC, RoleS, =>
    RoleA, SessionMpstThree, 3, 1
);

// Functions
fn simple_three_endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    offer_http_mpst!(s, recv_http_a_to_s, {
        Branching0fromStoA::Done(s) => {
            let (_, s, _resp) = recv_http_a_to_c(s, false, Request::default())?;

            close_mpst_multi(s)
        },
        Branching0fromStoA::Login(s) => {
            choice_a(s)
        },
    })
}

fn choice_a(s: ChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    let (pwd, s, _resp) = recv_http_a_to_c(s, false, Request::default())?;
    let expected = thread_rng().gen_range(1..=3);

    if pwd == expected {
        let s = auth_from_a_to_all(s);

        let (s, _req) = send_http_a_to_s(0, s, false, Method::GET, "", vec![("", "")], "");

        close_mpst_multi(s)
    } else {
        let s = again_from_a_to_all(s);

        let (s, _req) = send_http_a_to_s(1, s, false, Method::GET, "", vec![("", "")], "");

        choice_a(s)
    }
}

fn simple_three_endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    offer_http_mpst!(s, recv_http_c_to_s, {
        Branching0fromStoC::<i32>::Done(s) => {
            let (quit, s, _resp) = recv_http_c_to_s(s, false, Request::default())?;
            let (s, _req) = send_http_c_to_a(quit, s, false, Method::GET, "", vec![("", "")], "");
            close_mpst_multi(s)
        },
        Branching0fromStoC::<i32>::Login(s) => {

            /////////////
            // Get the tokens

            match fs::read_to_string("imgur.env") {
                Ok(contents) => {
                    let lines: Vec<&str> = contents.split("\n").collect();
                    let hasher = RandomState::new();
                    let mut ids: HashMap<&str, &str> = HashMap::with_hasher(hasher);
                    for line in lines {
                        let temp: Vec<&str> = line.split("=").collect();
                        ids.insert(temp[0], temp[1]);
                    }

                    let req = Request::builder()
                        .method(Method::GET)
                        .uri(ids["CREDITS_URL"])
                        .header("content-type", ids["CONTENT_TYPE"])
                        .header(
                            "Authorization",
                            format!("{} {}", ids["TOKEN_TYPE"], ids["ACCESS_TOKEN"]),
                        )
                        .header("User-Agent", ids["USER_AGENT"])
                        .header("Accept", ids["ACCEPT"])
                        .header("Connection", ids["CONNECTION"])
                        .body(Body::default())?;

                    /////////////
                    let (_, s, resp) = recv_http_c_to_s(s, true, req)?;

                    assert_eq!(resp.status(), StatusCode::from_u16(200).unwrap());

                    choice_c(s)
                }
                Err(_) => {
                    let (_, s, _resp) = recv_http_c_to_s(s, false, Request::default())?;

                    choice_c(s)
                }
            }
        },
    })
}

fn choice_c(s: ChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    let (s, _req) = send_http_c_to_a(
        thread_rng().gen_range(1..=3),
        s,
        false,
        Method::GET,
        "",
        vec![("", "")],
        "",
    );

    offer_http_mpst!(s, recv_http_c_to_a, {
        Branching1fromAtoC::<i32>::Auth(s) => {
            let (_, s, _resp) = recv_http_c_to_s(s, false, Request::default())?;

            close_mpst_multi(s)
        },
        Branching1fromAtoC::<i32>::Again(s) => {
            let (_, s, _resp) = recv_http_c_to_s(s, false, Request::default())?;

            choice_c(s)
        },
    })
}

fn simple_three_endpoint_s(s: EndpointS<i32>) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..=6);

    if choice == 1 {
        let s = done_from_s_to_all(s);

        let (s, _req) = send_http_s_to_c(0, s, false, Method::GET, "", vec![("", "")], "");

        close_mpst_multi(s)
    } else {
        let s = login_from_s_to_all(s);

        let (s, _req) = send_http_s_to_c(1, s, false, Method::GET, "", vec![("", "")], "");

        choice_s(s)
    }
}

fn choice_s(s: ChoiceS<i32>) -> Result<(), Box<dyn Error>> {
    offer_http_mpst!(s, recv_http_s_to_a, {
        Branching1fromAtoS::<i32>::Auth(s) => {
            let (success, s, _resp) = recv_http_s_to_a(s, false, Request::default())?;
            let (s, _req) = send_http_s_to_c(success, s, false, Method::GET, "", vec![("", "")], "");

            close_mpst_multi(s)
        },
        Branching1fromAtoS::<i32>::Again(s) => {
            let (fail, s, _resp) = recv_http_s_to_a(s, false, Request::default())?;
            let (s, _req) = send_http_s_to_c(fail, s, false, Method::GET, "", vec![("", "")], "");

            choice_s(s)
        },
    })
}

/////////////////////////

pub fn main() {
    let (thread_a, thread_c, thread_s) = fork_mpst(
        simple_three_endpoint_a,
        simple_three_endpoint_c,
        simple_three_endpoint_s,
    );

    assert!(thread_a.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_s.join().is_ok());
}
