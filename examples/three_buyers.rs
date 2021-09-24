use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use rand::{random, thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

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
    send_mpst_c_to_a, RoleA, 1 |
    send_mpst_c_to_s, RoleS, 2 | =>
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
// A
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;

// A
enum Branching0fromCtoA {
    Accept(MeshedChannelsThree<Recv<i32, End>, End, RoleC<RoleEnd>, NameA>),
    Quit(MeshedChannelsThree<Recv<i32, End>, End, RoleC<RoleEnd>, NameA>),
}
// S
enum Branching0fromCtoS {
    Accept(MeshedChannelsThree<End, Recv<i32, Send<i32, End>>, TwoRoleC, NameS>),
    Quit(MeshedChannelsThree<End, Recv<i32, End>, RoleC<RoleEnd>, NameS>),
}
type TwoRoleC = RoleC<RoleC<RoleEnd>>;

// Creating the MP sessions
// A
type EndpointA = MeshedChannelsThree<
    Send<i32, Recv<Branching0fromCtoA, End>>,
    Send<i32, Recv<i32, End>>,
    RoleS<RoleS<TwoRoleC>>,
    NameA,
>;

// C
type EndpointC = MeshedChannelsThree<
    Recv<i32, Choose0fromCtoA>,
    Recv<i32, Choose0fromCtoS>,
    RoleS<RoleA<RoleBroadcast>>,
    NameC,
>;

// S
type EndpointS = MeshedChannelsThree<
    Recv<i32, Send<i32, End>>,
    Send<i32, Recv<Branching0fromCtoS, End>>,
    RoleA<RoleA<RoleC<RoleC<RoleEnd>>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_s(random(), s);
    let (_empty2, s) = recv_mpst_a_from_s(s)?;
    let s = send_mpst_a_to_c(random(), s);
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Accept(s) => {
            let (_ok, s) = recv_mpst_a_from_c(s)?;
            close_mpst_multi(s)
        },
        Branching0fromCtoA::Quit(s) => {
            let (_ok, s) = recv_mpst_a_from_c(s)?;
            close_mpst_multi(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let (_empty3, s) = recv_mpst_c_from_s(s)?;
    let (_empty4, s) = recv_mpst_c_from_a(s)?;

    let choice = thread_rng().gen_range(1..=3);

    if choice != 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromCtoA::Accept,
            Branching0fromCtoS::Accept, =>
            RoleA,
            RoleS, =>
            RoleC,
            MeshedChannelsThree,
            2
        );

        let s = send_mpst_c_to_a(random(), s);
        let s = send_mpst_c_to_s(random(), s);
        let (_empty5, s) = recv_mpst_c_from_s(s)?;

        close_mpst_multi(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromCtoA::Quit,
            Branching0fromCtoS::Quit, =>
            RoleA,
            RoleS, =>
            RoleC,
            MeshedChannelsThree,
            2
        );
        let s = send_mpst_c_to_s(random(), s);
        let s = send_mpst_c_to_a(random(), s);
        close_mpst_multi(s)
    }
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    let (_empty1, s) = recv_mpst_s_from_a(s)?;
    let s = send_mpst_s_to_a(random(), s);
    let s = send_mpst_s_to_c(random(), s);
    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching0fromCtoS::Accept(s) => {
            let (_ok, s) = recv_mpst_s_from_c(s)?;
            let s = send_mpst_s_to_c(random(), s);
            close_mpst_multi(s)
        },
        Branching0fromCtoS::Quit(s) => {
            let (_ok, s) = recv_mpst_s_from_c(s)?;
            close_mpst_multi(s)
        },
    })
}

/////////////////////////////////////////

fn main() {
    let (thread_a, thread_c, thread_s) = fork_mpst(endpoint_a, endpoint_c, endpoint_s);

    assert!(thread_a.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_s.join().is_ok());
}
