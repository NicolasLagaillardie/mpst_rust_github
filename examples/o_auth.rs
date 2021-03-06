use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use rand::{thread_rng, Rng};
use std::error::Error;
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
//              again(Int) from A to S;
//              again(Int) from S to C;
//         }
//     }
//     or
//     {
//         cancel(Int) from S to C;
//         quit(Int) from C to A;
//     }
// }

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new roles
// normal
create_multiple_normal_role_short!(A, C, S);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_c, RoleC, 1 |
    send_mpst_a_to_s, RoleS, 2 | =>
    RoleA, MeshedChannelsThree, 3
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_a, RoleA, 1 | =>
    RoleC, MeshedChannelsThree, 3
);
// S
create_send_mpst_session_bundle!(
    send_mpst_s_to_a, RoleA, 1 |
    send_mpst_s_to_c, RoleC, 2 | =>
    RoleS, MeshedChannelsThree, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_c, RoleC, 1 |
    recv_mpst_a_from_s, RoleS, 2 | =>
    RoleA, MeshedChannelsThree, 3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_a, RoleA, 1 |
    recv_mpst_c_from_s, RoleS, 2 | =>
    RoleC, MeshedChannelsThree, 3
);
// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_from_a, RoleA, 1 |
    recv_mpst_s_from_c, RoleC, 2 | =>
    RoleS, MeshedChannelsThree, 3
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
type Choose1fromAtoC<N> = <Choice1fromCtoA<N> as Session>::Dual;
type Choose1fromAtoS<N> = <Choice1fromStoA<N> as Session>::Dual;

// A
enum Branching0fromStoA<N: marker::Send> {
    Login(
        MeshedChannelsThree<
            Recv<N, Choose1fromAtoC<N>>,
            Choose1fromAtoS<N>,
            RoleC<RoleBroadcast>,
            NameA,
        >,
    ),
    Done(MeshedChannelsThree<Recv<N, End>, End, RoleC<RoleEnd>, NameA>),
}

// C
enum Branching0fromStoC<N: marker::Send> {
    Login(MeshedChannelsThree<Send<N, Choice1fromCtoA<N>>, Recv<N, End>, RoleSAA, NameC>),
    Done(MeshedChannelsThree<Send<N, End>, Recv<N, End>, RoleSA, NameC>),
}

type RoleSA = RoleS<RoleA<RoleEnd>>;

enum Branching1fromAtoC<N: marker::Send> {
    Auth(MeshedChannelsThree<End, Recv<N, End>, RoleS<RoleEnd>, NameC>),
    Again(MeshedChannelsThree<Send<N, Choice1fromCtoA<N>>, Recv<N, End>, RoleSAA, NameC>),
}
type RoleSAA = RoleS<RoleA<RoleA<RoleEnd>>>;
type Choice1fromCtoA<N> = Recv<Branching1fromAtoC<N>, End>;

// S
enum Branching1fromAtoS<N: marker::Send> {
    Auth(MeshedChannelsThree<Recv<N, End>, Send<N, End>, RoleAC, NameS>),
    Again(MeshedChannelsThree<Recv<N, Choice1fromStoA<N>>, Send<N, End>, RoleACA, NameS>),
}
type RoleAC = RoleA<RoleC<RoleEnd>>;
type RoleACA = RoleA<RoleC<RoleA<RoleEnd>>>;
type Choice1fromStoA<N> = Recv<Branching1fromAtoS<N>, End>;

// Creating the MP sessions
// A
type ChoiceA<N> = MeshedChannelsThree<
    Recv<N, Choose1fromAtoC<N>>,
    Choose1fromAtoS<N>,
    RoleC<RoleBroadcast>,
    NameA,
>;
type EndpointA<N> =
    MeshedChannelsThree<End, Recv<Branching0fromStoA<N>, End>, RoleS<RoleEnd>, NameA>;

// C
type ChoiceC<N> =
    MeshedChannelsThree<Send<N, Choice1fromCtoA<N>>, End, RoleA<RoleA<RoleEnd>>, NameC>;
type EndpointC<N> =
    MeshedChannelsThree<End, Recv<Branching0fromStoC<N>, End>, RoleS<RoleEnd>, NameC>;

// S
type ChoiceS<N> = MeshedChannelsThree<Choice1fromStoA<N>, End, RoleA<RoleEnd>, NameS>;
type EndpointS<N> =
    MeshedChannelsThree<Choose0fromStoA<N>, Choose0fromStoC<N>, RoleBroadcast, NameS>;

// Functions
fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_s, {
        Branching0fromStoA::Done(s) => {
            let (_, s) = recv_mpst_a_from_c(s)?;

            close_mpst_multi(s)
        },
        Branching0fromStoA::Login(s) => {
            choice_a(s)
        },
    })
}

fn choice_a(s: ChoiceA<i32>) -> Result<(), Box<dyn Error>> {
    let (pwd, s) = recv_mpst_a_from_c(s)?;

    let expected = thread_rng().gen_range(1..=3);

    if pwd == expected {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching1fromAtoC::<i32>::Auth,
            Branching1fromAtoS::<i32>::Auth, =>
            RoleC,
            RoleS, =>
            RoleA,
            MeshedChannelsThree,
            1
        );

        let s = send_mpst_a_to_s(0, s);

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching1fromAtoC::<i32>::Again,
            Branching1fromAtoS::<i32>::Again, =>
            RoleC,
            RoleS, =>
            RoleA,
            MeshedChannelsThree,
            1
        );

        let s = send_mpst_a_to_s(1, s);

        choice_a(s)
    }
}

fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_s, {
        Branching0fromStoC::<i32>::Done(s) => {
            let (quit, s) = recv_mpst_c_from_s(s)?;
            let s = send_mpst_c_to_a(quit, s);
            close_mpst_multi(s)
        },
        Branching0fromStoC::<i32>::Login(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            choice_c(s)
        },
    })
}

fn choice_c(s: ChoiceC<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_a(thread_rng().gen_range(1..=3), s);

    offer_mpst!(s, recv_mpst_c_from_a, {
        Branching1fromAtoC::<i32>::Auth(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            close_mpst_multi(s)
        },
        Branching1fromAtoC::<i32>::Again(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            choice_c(s)
        },
    })
}

fn endpoint_s(s: EndpointS<i32>) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..=3);

    if choice != 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromStoA::<i32>::Done,
            Branching0fromStoC::<i32>::Done, =>
            RoleA,
            RoleC, =>
            RoleS,
            MeshedChannelsThree,
            3
        );

        let s = send_mpst_s_to_c(0, s);

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromStoA::<i32>::Login,
            Branching0fromStoC::<i32>::Login, =>
            RoleA,
            RoleC, =>
            RoleS,
            MeshedChannelsThree,
            3
        );

        let s = send_mpst_s_to_c(1, s);

        choice_s(s)
    }
}

fn choice_s(s: ChoiceS<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_a, {
        Branching1fromAtoS::<i32>::Auth(s) => {
            let (success, s) = recv_mpst_s_from_a(s)?;

            let s = send_mpst_s_to_c(success, s);

            close_mpst_multi(s)
        },
        Branching1fromAtoS::<i32>::Again(s) => {
            let (fail, s) = recv_mpst_s_from_a(s)?;

            let s = send_mpst_s_to_c(fail, s);

            choice_s(s)
        },
    })
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_a, thread_c, thread_s) = fork_mpst(endpoint_a, endpoint_c, endpoint_s);

    thread_a.join()?;
    thread_c.join()?;
    thread_s.join()?;

    Ok(())
}

fn main() {
    assert!(all_mpst().is_ok());
}
