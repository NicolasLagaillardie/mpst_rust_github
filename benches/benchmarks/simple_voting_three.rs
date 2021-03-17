#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use rand::{thread_rng, Rng};
use std::error::Error;
use std::marker;
use std::time::Duration;

// global protopol SimpleVoting(role VOTER, role SERVER){
//     Authenticate(String) from VOTER to SERVER;
//     choice at SERVER {
//         Ok(String) from SERVER to VOTER;
//         choice at VOTER {
//             Yes(String) from VOTER to SERVER;
//         } or {
//             No(String) from VOTER to SERVER;
//         }
//         Result(Int) from SERVER to VOTER;
//     } or {
//         Reject(String) from SERVER to VOTER;
//     }
// }

// Create the new SessionMpst for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstThree, 3);

// Create new Roles
// normal
create_multiple_normal_role_short!(Voter, Pawn, Server);

// Create new send functions
// SERVER
create_send_mpst_session_bundle!(
    send_mpst_server_to_pawn, RolePawn, next_pawn, 1 |
    send_mpst_server_to_voter, RoleVoter, next_voter, 2 | =>
    RoleServer, SessionMpstThree, 3
);
// VOTER
create_send_mpst_session_bundle!(
    send_mpst_voter_to_pawn, RolePawn, next_pawn, 1 |
    send_mpst_voter_to_server, RoleServer, next_server, 2 | =>
    RoleVoter, SessionMpstThree, 3
);

// Create new recv functions and related types
// PAWN
create_recv_mpst_session_bundle!(
    recv_mpst_pawn_to_server, RoleServer, next_server, 1 |
    recv_mpst_pawn_to_voter, RoleVoter, next_voter, 2 | =>
    RolePawn, SessionMpstThree, 3
);
// SERVER
create_recv_mpst_session_bundle!(
    recv_mpst_server_to_pawn, RolePawn, next_pawn, 1 |
    recv_mpst_server_to_voter, RoleVoter, next_voter, 2 | =>
    RoleServer, SessionMpstThree, 3
);
// VOTER
create_recv_mpst_session_bundle!(
    recv_mpst_voter_to_pawn, RolePawn, next_pawn, 1 |
    recv_mpst_voter_to_server, RoleServer, next_server, 2 | =>
    RoleVoter, SessionMpstThree, 3
);

// Names
type NamePawn = RolePawn<RoleEnd>;
type NameServer = RoleServer<RoleEnd>;
type NameVoter = RoleVoter<RoleEnd>;

// Types
// SERVER
type Choose0fromStoP = Send<Branching0fromStoP, End>;
type Choose0fromStoV<N> = Send<Branching0fromStoV<N>, End>;
// VOTER
type Choose1fromVtoP = Send<Branching1fromVtoP, End>;
type Choose1fromVtoS<N> = Send<Branching1fromVtoS<N>, End>;

// VOTER
enum Branching0fromStoV<N: marker::Send> {
    Auth(
        SessionMpstThree<
            Choose1fromVtoP,
            Recv<N, Choose1fromVtoS<N>>,
            RoleServer<RolePawn<RoleServer<RoleEnd>>>,
            NameVoter,
        >,
    ),
    Reject(SessionMpstThree<End, Recv<N, End>, RoleServer<RoleEnd>, NameVoter>),
}
// PAWN
enum Branching0fromStoP {
    Auth(SessionMpstThree<End, Choice1fromPtoV, RoleVoter<RoleEnd>, NamePawn>),
    Reject(SessionMpstThree<End, End, RoleEnd, NamePawn>),
}
enum Branching1fromVtoP {
    Yes(SessionMpstThree<End, End, RoleEnd, NamePawn>),
    No(SessionMpstThree<End, End, RoleEnd, NamePawn>),
}
type Choice1fromPtoV = Recv<Branching1fromVtoP, End>;
// SERVER
enum Branching1fromVtoS<N: marker::Send> {
    Yes(SessionMpstThree<End, Recv<N, End>, RoleVoter<RoleEnd>, NameServer>),
    No(SessionMpstThree<End, Recv<N, End>, RoleVoter<RoleEnd>, NameServer>),
}
type Choice1fromStoV<N> = Recv<Branching1fromVtoS<N>, End>;

// Creating the MP sessions
// VOTER
type ChoiceVoter<N> = SessionMpstThree<
    Choose1fromVtoP,
    Recv<N, Choose1fromVtoS<N>>,
    RoleServer<RolePawn<RoleServer<RoleEnd>>>,
    NameVoter,
>;
type EndpointVoter<N> = SessionMpstThree<
    End,
    Send<N, Recv<Branching0fromStoV<N>, End>>,
    RoleServer<RoleServer<RoleEnd>>,
    NameVoter,
>;
// PAWN
type ChoicePawn = SessionMpstThree<End, Choice1fromPtoV, RoleVoter<RoleEnd>, NamePawn>;
type EndpointPawn =
    SessionMpstThree<Recv<Branching0fromStoP, End>, End, RoleServer<RoleEnd>, NamePawn>;
// SERVER
type ChoiceServer<N> = SessionMpstThree<End, Choice1fromStoV<N>, RoleVoter<RoleEnd>, NameServer>;
type EndpointServer<N> = SessionMpstThree<
    Choose0fromStoP,
    Recv<N, Choose0fromStoV<N>>,
    RoleVoter<RolePawn<RoleVoter<RoleEnd>>>,
    NameServer,
>;

// Functions
fn simple_five_endpoint_voter(s: EndpointVoter<i32>) -> Result<(), Box<dyn Error>> {
    let auth = thread_rng().gen_range(1..=3);

    let s = send_mpst_voter_to_server(auth, s);

    offer_mpst!(s, recv_mpst_voter_to_server, {
        Branching0fromStoV::Reject(s) => {

            let (_, s) = recv_mpst_voter_to_server(s)?;

            close_mpst_multi(s)
        },
        Branching0fromStoV::Auth(s) => {
            choice_voter(s)
        },
    })
}

fn choice_voter(s: ChoiceVoter<i32>) -> Result<(), Box<dyn Error>> {
    let (ok, s) = recv_mpst_voter_to_server(s)?;

    let expected = thread_rng().gen_range(1..=3);

    if ok == expected {
        let s = choose_mpst_multi_to_all!(
            s,
            send_mpst_voter_to_pawn,
            send_mpst_voter_to_server, =>
            Branching1fromVtoP::Yes,
            Branching1fromVtoS::<i32>::Yes, =>
            RolePawn,
            RoleServer, =>
            RoleVoter,
            SessionMpstThree,
            3,
            3
        );

        let s = send_mpst_voter_to_server(1, s);

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            send_mpst_voter_to_pawn,
            send_mpst_voter_to_server, =>
            Branching1fromVtoP::No,
            Branching1fromVtoS::<i32>::No, =>
            RolePawn,
            RoleServer, =>
            RoleVoter,
            SessionMpstThree,
            3,
            3
        );

        let s = send_mpst_voter_to_server(0, s);

        close_mpst_multi(s)
    }
}

fn simple_five_endpoint_pawn(s: EndpointPawn) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_pawn_to_server, {
        Branching0fromStoP::Reject(s) => {
            close_mpst_multi(s)
        },
        Branching0fromStoP::Auth(s) => {
            choice_pawn(s)
        },
    })
}

fn choice_pawn(s: ChoicePawn) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_pawn_to_voter, {
        Branching1fromVtoP::Yes(s) => {
            close_mpst_multi(s)
        },
        Branching1fromVtoP::No(s) => {
            close_mpst_multi(s)
        },
    })
}

fn simple_five_endpoint_server(s: EndpointServer<i32>) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..=3);

    let (auth, s) = recv_mpst_server_to_voter(s)?;

    if choice == auth {
        let s = choose_mpst_multi_to_all!(
            s,
            send_mpst_server_to_pawn,
            send_mpst_server_to_voter, =>
            Branching0fromStoP::Reject,
            Branching0fromStoV::<i32>::Reject, =>
            RolePawn,
            RoleVoter, =>
            RoleServer,
            SessionMpstThree,
            3,
            2
        );

        let s = send_mpst_server_to_voter(0, s);

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            send_mpst_server_to_pawn,
            send_mpst_server_to_voter, =>
            Branching0fromStoP::Auth,
            Branching0fromStoV::<i32>::Auth, =>
            RolePawn,
            RoleVoter, =>
            RoleServer,
            SessionMpstThree,
            3,
            2
        );

        let s = send_mpst_server_to_voter(1, s);

        choice_server(s)
    }
}

fn choice_server(s: ChoiceServer<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_server_to_voter, {
        Branching1fromVtoS::<i32>::Yes(s) => {

            let (answer, s) = recv_mpst_server_to_voter(s)?;

            assert_eq!(answer, 1);

            close_mpst_multi(s)
        },
        Branching1fromVtoS::<i32>::No(s) => {

            let (answer, s) = recv_mpst_server_to_voter(s)?;

            assert_eq!(answer, 0);

            close_mpst_multi(s)
        },
    })
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_pawn, thread_server, thread_voter) = fork_mpst(
        black_box(simple_five_endpoint_pawn),
        black_box(simple_five_endpoint_server),
        black_box(simple_five_endpoint_voter),
    );

    thread_voter.join()?;
    thread_server.join()?;
    thread_pawn.join()?;

    Ok(())
}

/////////////////////////

fn simple_voting_mpst(c: &mut Criterion) {
    c.bench_function(&format!("Simple voting MPST"), |b| b.iter(|| all_mpst()));
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(30, 0))
}

criterion_group! {
    name = simple_voting;
    config = long_warmup();
    targets = simple_voting_mpst,
}

criterion_main!(simple_voting);
