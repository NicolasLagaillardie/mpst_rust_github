#![allow(clippy::type_complexity)]

// Unfinished, still in process of adding send_tcp and recv_tcp

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_name,
    create_multiple_normal_role, create_recv_mpst_session_bundle, create_send_mpst_session_bundle,
    offer_mpst,
};

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new roles
// normal
create_multiple_normal_role!(
    RoleC, RoleCDual |
    RoleS, RoleSDual |
);

// Create new names
create_multiple_normal_name!(NameC, NameS);

// Create new send functions
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_s, RoleS, 1 | =>
    NameC, MeshedChannelsTwo, 2
);
// S
create_send_mpst_session_bundle!(
    send_mpst_s_to_c, RoleC, 1 | =>
    NameS, MeshedChannelsTwo, 2
);

// Create new recv functions and related types
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_s, RoleS, 1 | =>
    NameC, MeshedChannelsTwo, 2
);
// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_from_c, RoleC, 1 | =>
    NameS, MeshedChannelsTwo, 2
);

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsTwo, 2);

// Stacks
// C
type ThreeRoleC = RoleC<RoleC<RoleC<RoleEnd>>>;
type FiveRoleC = RoleC<RoleC<RoleC<RoleC<RoleC<RoleEnd>>>>>;
type TwoRoleCBroadcast = RoleC<RoleC<RoleBroadcast>>;

// S
type ThreeRoleS = RoleS<RoleS<RoleS<RoleEnd>>>;

// Types
// Step 0
// C
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;
type EndpointC0 = MeshedChannelsTwo<Recv<(), Choose0fromCtoS>, RoleS<RoleBroadcast>, NameC>;

// S
enum Branching0fromCtoS {
    Continue(MeshedChannelsTwo<Recv<(), Recv<(), Choose1fromStoC>>, TwoRoleCBroadcast, NameS>),
    Quit(MeshedChannelsTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer0fromCtoS = <Choose0fromCtoS as Session>::Dual;
type EndpointS0 = MeshedChannelsTwo<Send<(), Offer0fromCtoS>, RoleC<RoleC<RoleEnd>>, NameS>;

// Step 1
// C
enum Branching1fromStoC {
    Continue(MeshedChannelsTwo<Recv<(), Choose2fromCtoS>, RoleS<RoleBroadcast>, NameC>),
    Loop(MeshedChannelsTwo<Recv<(), Recv<(), Offer1fromStoC>>, ThreeRoleS, NameC>),
}
type Offer1fromStoC = <Choose1fromStoC as Session>::Dual;
type EndpointC1 = MeshedChannelsTwo<Offer1fromStoC, RoleS<RoleEnd>, NameC>;

// S
type Choose1fromStoC = Send<Branching1fromStoC, End>;
type EndpointS1 = MeshedChannelsTwo<Choose1fromStoC, RoleBroadcast, NameS>;

// Step 2
// C
type Choose2fromCtoS = Send<Branching2fromCtoS, End>;
type EndpointC2 = MeshedChannelsTwo<Choose2fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching2fromCtoS {
    Continue(MeshedChannelsTwo<Recv<(), Send<(), Offer3fromCtoS>>, ThreeRoleC, NameS>),
    Quit(MeshedChannelsTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer2fromCtoS = <Choose2fromCtoS as Session>::Dual;
type EndpointS2 = MeshedChannelsTwo<Offer2fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 3
// C
type Choose3fromCtoS = Send<Branching3fromCtoS, End>;
type EndpointC3 = MeshedChannelsTwo<Choose3fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching3fromCtoS {
    Continue(MeshedChannelsTwo<Recv<(), Choose4fromStoC>, RoleC<RoleBroadcast>, NameS>),
    Quit(MeshedChannelsTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer3fromCtoS = <Choose3fromCtoS as Session>::Dual;
type EndpointS3 = MeshedChannelsTwo<Offer3fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 4
// C
enum Branching4fromStoC {
    Continue(MeshedChannelsTwo<Recv<(), Choose5fromCtoS>, RoleS<RoleBroadcast>, NameC>),
    Loop(MeshedChannelsTwo<Recv<(), Offer4fromStoC>, RoleS<RoleS<RoleEnd>>, NameC>),
}
type Offer4fromStoC = <Choose4fromStoC as Session>::Dual;
type EndpointC4 = MeshedChannelsTwo<Offer4fromStoC, RoleS<RoleEnd>, NameC>;

// S
type Choose4fromStoC = Send<Branching4fromStoC, End>;
type EndpointS4 = MeshedChannelsTwo<Choose4fromStoC, RoleBroadcast, NameS>;

// Step 5
// C
type Choose5fromCtoS = Send<Branching5fromCtoS, End>;
type EndpointC5 = MeshedChannelsTwo<Choose5fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching5fromCtoS {
    Continue(MeshedChannelsTwo<Recv<(), Choose6fromStoC>, RoleC<RoleBroadcast>, NameS>),
    Quit(MeshedChannelsTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer5fromCtoS = <Choose5fromCtoS as Session>::Dual;
type EndpointS5 = MeshedChannelsTwo<Offer5fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 6
// C
enum Branching6fromStoC {
    Continue(MeshedChannelsTwo<Recv<(), Choose7fromCtoS>, RoleS<RoleBroadcast>, NameC>),
    Loop(MeshedChannelsTwo<Recv<(), Offer6fromStoC>, RoleS<RoleS<RoleEnd>>, NameC>),
}
type Offer6fromStoC = <Choose6fromStoC as Session>::Dual;
type EndpointC6 = MeshedChannelsTwo<Offer6fromStoC, RoleS<RoleEnd>, NameC>;

// S
type Choose6fromStoC = Send<Branching6fromStoC, End>;
type EndpointS6 = MeshedChannelsTwo<Choose6fromStoC, RoleBroadcast, NameS>;

// Step 7
// C
type Choose7fromCtoS = Send<Branching7fromCtoS, End>;
type EndpointC7 = MeshedChannelsTwo<Choose7fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching7fromCtoS {
    Continue(MeshedChannelsTwo<Recv<(), Choose8fromStoC>, RoleC<RoleBroadcast>, NameS>),
    Quit(MeshedChannelsTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer7fromCtoS = <Choose7fromCtoS as Session>::Dual;
type EndpointS7 = MeshedChannelsTwo<Offer7fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 8
// C
enum Branching8fromStoC {
    Continue(MeshedChannelsTwo<Recv<(), Choose9fromCtoS>, RoleS<RoleBroadcast>, NameC>),
    Loop(MeshedChannelsTwo<Recv<(), Choose7fromCtoS>, RoleS<RoleBroadcast>, NameC>),
}
type Offer8fromStoC = <Choose8fromStoC as Session>::Dual;
type EndpointC8 = MeshedChannelsTwo<Offer8fromStoC, RoleS<RoleEnd>, NameC>;

// S
type Choose8fromStoC = Send<Branching8fromStoC, End>;
type EndpointS8 = MeshedChannelsTwo<Choose8fromStoC, RoleBroadcast, NameS>;

// Step 9
// C
type Choose9fromCtoS = Send<Branching9fromCtoS, End>;
type EndpointC9 = MeshedChannelsTwo<Choose9fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching9fromCtoS {
    Continue(MeshedChannelsTwo<FullOffer10fromCtoS, FiveRoleC, NameS>),
    Loop(MeshedChannelsTwo<Recv<(), Send<(), Offer9fromCtoS>>, ThreeRoleC, NameS>),
}
type FullOffer10fromCtoS = Recv<(), Send<(), Recv<(), Recv<(), Offer10fromCtoS>>>>;
type Offer9fromCtoS = <Choose9fromCtoS as Session>::Dual;
type EndpointS9 = MeshedChannelsTwo<Offer9fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 10
// C
type Choose10fromCtoS = Send<Branching10fromCtoS, End>;
type EndpointC10 = MeshedChannelsTwo<Choose10fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching10fromCtoS {
    Data(MeshedChannelsTwo<Recv<(), Recv<(), Offer10fromCtoS>>, ThreeRoleC, NameS>),
    Subject(MeshedChannelsTwo<Recv<(), Recv<(), Offer10fromCtoS>>, ThreeRoleC, NameS>),
    End(MeshedChannelsTwo<Recv<(), Send<(), Offer7fromCtoS>>, ThreeRoleC, NameS>),
}
type Offer10fromCtoS = <Choose10fromCtoS as Session>::Dual;
type EndpointS10 = MeshedChannelsTwo<Offer10fromCtoS, RoleC<RoleEnd>, NameS>;

// Functions
fn endpoint_c_0(s: EndpointC0) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_c_from_s(s)?;

    let expected: i32 = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromCtoS::Continue, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);
        let s = send_mpst_c_to_s((), s);

        endpoint_c_1(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromCtoS::Quit, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);

        close_mpst_multi(s)
    }
}

fn endpoint_c_1(s: EndpointC1) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_s, {
        Branching1fromStoC::Continue(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            endpoint_c_2(s)
        },
        Branching1fromStoC::Loop(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;
            let (_, s) = recv_mpst_c_from_s(s)?;

            endpoint_c_1(s)
        },
    })
}

fn endpoint_c_2(s: EndpointC2) -> Result<(), Box<dyn Error>> {
    let expected: i32 = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching2fromCtoS::Continue, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);
        let (_, s) = recv_mpst_c_from_s(s)?;

        endpoint_c_3(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching2fromCtoS::Quit, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);

        close_mpst_multi(s)
    }
}

fn endpoint_c_3(s: EndpointC3) -> Result<(), Box<dyn Error>> {
    let expected: i32 = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching3fromCtoS::Continue, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);

        endpoint_c_4(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching3fromCtoS::Quit, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);

        close_mpst_multi(s)
    }
}

fn endpoint_c_4(s: EndpointC4) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_s, {
        Branching4fromStoC::Continue(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            endpoint_c_5(s)
        },
        Branching4fromStoC::Loop(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            endpoint_c_4(s)
        },
    })
}

fn endpoint_c_5(s: EndpointC5) -> Result<(), Box<dyn Error>> {
    let expected: i32 = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching5fromCtoS::Continue, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);

        endpoint_c_6(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching5fromCtoS::Quit, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);

        close_mpst_multi(s)
    }
}

fn endpoint_c_6(s: EndpointC6) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_s, {
        Branching6fromStoC::Continue(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            endpoint_c_7(s)
        },
        Branching6fromStoC::Loop(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            endpoint_c_6(s)
        },
    })
}

fn endpoint_c_7(s: EndpointC7) -> Result<(), Box<dyn Error>> {
    let expected: i32 = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching7fromCtoS::Continue, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);

        endpoint_c_8(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching7fromCtoS::Quit, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);

        close_mpst_multi(s)
    }
}

fn endpoint_c_8(s: EndpointC8) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_s, {
        Branching8fromStoC::Continue(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            endpoint_c_9(s)
        },
        Branching8fromStoC::Loop(s) => {
            let (_, s) = recv_mpst_c_from_s(s)?;

            endpoint_c_7(s)
        },
    })
}

fn endpoint_c_9(s: EndpointC9) -> Result<(), Box<dyn Error>> {
    let expected: i32 = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching9fromCtoS::Continue, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);
        let (_, s) = recv_mpst_c_from_s(s)?;
        let s = send_mpst_c_to_s((), s);
        let s = send_mpst_c_to_s((), s);

        endpoint_c_10(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching9fromCtoS::Loop, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);
        let (_, s) = recv_mpst_c_from_s(s)?;

        endpoint_c_9(s)
    }
}

fn endpoint_c_10(s: EndpointC10) -> Result<(), Box<dyn Error>> {
    let expected: i32 = thread_rng().gen_range(1..=3);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching10fromCtoS::Data, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);
        let s = send_mpst_c_to_s((), s);

        endpoint_c_10(s)
    } else if expected == 2 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching10fromCtoS::Subject, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);
        let s = send_mpst_c_to_s((), s);

        endpoint_c_10(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching10fromCtoS::End, =>
            NameC, MeshedChannelsTwo, 1
        );

        let s = send_mpst_c_to_s((), s);
        let (_, s) = recv_mpst_c_from_s(s)?;

        endpoint_c_7(s)
    }
}

///

fn endpoint_s_0(s: EndpointS0) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_s_to_c((), s);

    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching0fromCtoS::Quit(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;

            close_mpst_multi(s)
        },
        Branching0fromCtoS::Continue(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;
            let (_, s) = recv_mpst_s_from_c(s)?;

            endpoint_s_1(s)
        },
    })
}

fn endpoint_s_1(s: EndpointS1) -> Result<(), Box<dyn Error>> {
    let expected: i32 = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching1fromStoC::Continue, =>
            NameS, MeshedChannelsTwo, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_2(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching1fromStoC::Loop, =>
            NameS, MeshedChannelsTwo, 2
        );

        let s = send_mpst_s_to_c((), s);
        let s = send_mpst_s_to_c((), s);

        endpoint_s_1(s)
    }
}

fn endpoint_s_2(s: EndpointS2) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching2fromCtoS::Quit(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;

            close_mpst_multi(s)
        },
        Branching2fromCtoS::Continue(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;
            let s = send_mpst_s_to_c((), s);

            endpoint_s_3(s)
        },
    })
}

fn endpoint_s_3(s: EndpointS3) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching3fromCtoS::Quit(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;

            close_mpst_multi(s)
        },
        Branching3fromCtoS::Continue(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;

            endpoint_s_4(s)
        },
    })
}

fn endpoint_s_4(s: EndpointS4) -> Result<(), Box<dyn Error>> {
    let expected: i32 = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching4fromStoC::Continue, =>
            NameS, MeshedChannelsTwo, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_5(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching4fromStoC::Loop, =>
            NameS, MeshedChannelsTwo, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_4(s)
    }
}

fn endpoint_s_5(s: EndpointS5) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching5fromCtoS::Quit(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;

            close_mpst_multi(s)
        },
        Branching5fromCtoS::Continue(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;

            endpoint_s_6(s)
        },
    })
}

fn endpoint_s_6(s: EndpointS6) -> Result<(), Box<dyn Error>> {
    let expected: i32 = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching6fromStoC::Continue, =>
            NameS, MeshedChannelsTwo, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_7(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching6fromStoC::Loop, =>
            NameS, MeshedChannelsTwo, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_6(s)
    }
}

fn endpoint_s_7(s: EndpointS7) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching7fromCtoS::Quit(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;

            close_mpst_multi(s)
        },
        Branching7fromCtoS::Continue(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;

            endpoint_s_8(s)
        },
    })
}

fn endpoint_s_8(s: EndpointS8) -> Result<(), Box<dyn Error>> {
    let expected: i32 = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching8fromStoC::Continue, =>
            NameS, MeshedChannelsTwo, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_9(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching8fromStoC::Loop, =>
            NameS, MeshedChannelsTwo, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_7(s)
    }
}

fn endpoint_s_9(s: EndpointS9) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching9fromCtoS::Loop(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;
            let s = send_mpst_s_to_c((), s);

            endpoint_s_9(s)
        },
        Branching9fromCtoS::Continue(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;
            let s = send_mpst_s_to_c((), s);
            let (_, s) = recv_mpst_s_from_c(s)?;
            let (_, s) = recv_mpst_s_from_c(s)?;

            endpoint_s_10(s)
        },
    })
}

fn endpoint_s_10(s: EndpointS10) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_s_from_c, {
        Branching10fromCtoS::Data(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;
            let (_, s) = recv_mpst_s_from_c(s)?;

            endpoint_s_10(s)
        },
        Branching10fromCtoS::Subject(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;
            let (_, s) = recv_mpst_s_from_c(s)?;

            endpoint_s_10(s)
        },
        Branching10fromCtoS::End(s) => {
            let (_, s) = recv_mpst_s_from_c(s)?;
            let s = send_mpst_s_to_c((), s);

            endpoint_s_7(s)
        },
    })
}

///

fn main() {
    let (thread_c, thread_s) = fork_mpst(endpoint_c_0, endpoint_s_0);

    assert!(thread_c.join().is_ok());
    assert!(thread_s.join().is_ok());
}
