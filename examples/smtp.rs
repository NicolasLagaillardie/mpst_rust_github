// Unfinished, still in process of adding send_tcp and recv_tcp

// #![allow(dead_code, unused_imports)]

use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, offer_mpst,
};

use rand::{thread_rng, Rng};
use std::error::Error;

// global protocol Smtp(role S, role C)
// {
//     220() from S to C;
//     choice at C // Choice 0
//     {
//         Ehlo1() from C to S;
//         rec X
//         {
//             choice at S // Choice 1
//             {
//                 250d() from S to C;
//                 continue X;
//             }
//             or
//             {
//                 250() from S to C;
//                 choice at C // Choice 2
//                 {
//                     StartTls() from C to S;
//                     220() from S to C;
//                     // Do TLS handshake here: level below the application level protocol (like regular TCP handshake)
//                     choice at C // Choice 3
//                     {
//                          Ehlo2() from C to S;
//                          rec X
//                         {
//                             choice at S // Choice 4
//                             {
//                                 250d1() from S to C;
//                                 continue X;
//                             }
//                             or
//                             {
//                                 2501() from S to C;
//                                 rec Y
//                                 {
//                                     choice at C // Choice 5
//                                     {
//                                         Auth() from C to S;
//                                         choice at S // Choice 6
//                                         {
//                                             235() from S to C;
//                                             rec Z1
//                                             {
//                                                 choice at C // Choice 7
//                                                 {
//                                                     Mail() from C to S; //Mail from:<a@b.com>
//                                                     choice at S // Choice 8
//                                                     {
//                                                         501() from S to C;
//                                                         continue Z1;
//                                                     }
//                                                     or
//                                                     {
//                                                         2502() from S to C;
//                                                         rec Z2
//                                                         {
//                                                             choice at C // Choice 9
//                                                             {
//                                                                 Rcpt() from C to S; //Rcpt to:<c@d.com>
//                                                                 choice at S // What is this choice???  // Choice X
//                                                                 {
//                                                                     2503() from S to C;
//                                                                     continue Z2;
//                                                                 }
//                                                             }
//                                                             or
//                                                             {
//                                                                 Data() from C to S;
//                                                                 354() from S to C;

//                                                                 too from C to S; //to:<you>

//                                                                 froom from C to S; //from:<me>

//                                                                 rec Z3
//                                                                 {
//                                                                     choice at C // Choice 10
//                                                                     {
//                                                                         DataLine() from C to S;
//                                                                         DataLine() from C to S;
//                                                                         continue Z3;
//                                                                     }
//                                                                     or
//                                                                     {
//                                                                         Subject() from C to S; //Subject:<my Subject>
//                                                                         Subject() from C to S; //Subject:<my Subject>
//                                                                         continue Z3;
//                                                                     }
//                                                                     or
//                                                                     {
//                                                                         EndOfData() from C to S; // CRLF.CRLF
//                                                                         2504() from S to C;
//                                                                         continue Z1;
//                                                                     }
//                                                                 }
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 or
//                                                 {
//                                                      Quit5() from C to S;
//                                                      221() from S to C;
//                                                  }
//                                             }
//                                         }
//                                         or
//                                         {
//                                             535() from S to C;
//                                              continue Y;
//                                         }
//                                         //.. 501 Invalid base64 Data
//                                      }
//                                     or
//                                     {
//                                         Quit4() from C to S
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                     or
//                     {
//                         Quit3() from C to S;
//                     }
//                 }
//                 or
//                 {
//                     Quit2() from C to S;
//                 }
//             }
//         }
//     }
//     or
//     {
//         Quit1() from C to S;
//     }
// }

// Create new roles
// normal
create_multiple_normal_role!(
    RoleC, RoleCDual |
    RoleS, RoleSDual |
);

// Create new send functions
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_s, RoleS, 1 | =>
    RoleC, SessionMpstTwo, 2
);
// S
create_send_mpst_session_bundle!(
    send_mpst_s_to_c, RoleC, 1 | =>
    RoleS, SessionMpstTwo, 2
);

// Create new recv functions and related types
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_s, RoleS, 1 | =>
    RoleC, SessionMpstTwo, 2
);
// S
create_recv_mpst_session_bundle!(
    recv_mpst_s_from_c, RoleC, 1 | =>
    RoleS, SessionMpstTwo, 2
);

// Create the new SessionMpst for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstTwo, 2);

// Names
type NameC = RoleC<RoleEnd>;
type NameS = RoleS<RoleEnd>;

// Types
// Step 0
// C
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;
type EndpointC0 = SessionMpstTwo<Recv<(), Choose0fromCtoS>, RoleS<RoleBroadcast>, NameC>;
// S
enum Branching0fromCtoS {
    Continue(
        SessionMpstTwo<Recv<(), Recv<(), Choose1fromStoC>>, RoleC<RoleC<RoleBroadcast>>, NameS>,
    ),
    Quit(SessionMpstTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer0fromCtoS = <Choose0fromCtoS as Session>::Dual;
type EndpointS0 = SessionMpstTwo<Send<(), Offer0fromCtoS>, RoleC<RoleC<RoleEnd>>, NameS>;

// Step 1
// C
enum Branching1fromStoC {
    Continue(SessionMpstTwo<Recv<(), Choose2fromCtoS>, RoleS<RoleBroadcast>, NameC>),
    Loop(SessionMpstTwo<Recv<(), Recv<(), Offer1fromStoC>>, RoleS<RoleS<RoleS<RoleEnd>>>, NameC>),
}
type Offer1fromStoC = <Choose1fromStoC as Session>::Dual;
type EndpointC1 = SessionMpstTwo<Offer1fromStoC, RoleS<RoleEnd>, NameC>;
// S
type Choose1fromStoC = Send<Branching1fromStoC, End>;
type EndpointS1 = SessionMpstTwo<Choose1fromStoC, RoleBroadcast, NameS>;

// Step 2
// C
type Choose2fromCtoS = Send<Branching2fromCtoS, End>;
type EndpointC2 = SessionMpstTwo<Choose2fromCtoS, RoleBroadcast, NameC>;
// S
enum Branching2fromCtoS {
    Continue(
        SessionMpstTwo<Recv<(), Send<(), Offer3fromCtoS>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>,
    ),
    Quit(SessionMpstTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer2fromCtoS = <Choose2fromCtoS as Session>::Dual;
type EndpointS2 = SessionMpstTwo<Offer2fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 3
// C
type Choose3fromCtoS = Send<Branching3fromCtoS, End>;
type EndpointC3 = SessionMpstTwo<Choose3fromCtoS, RoleBroadcast, NameC>;
// S
enum Branching3fromCtoS {
    Continue(SessionMpstTwo<Recv<(), Choose4fromStoC>, RoleC<RoleBroadcast>, NameS>),
    Quit(SessionMpstTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer3fromCtoS = <Choose3fromCtoS as Session>::Dual;
type EndpointS3 = SessionMpstTwo<Offer3fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 4
// C
enum Branching4fromStoC {
    Continue(SessionMpstTwo<Recv<(), Choose5fromCtoS>, RoleS<RoleBroadcast>, NameC>),
    Loop(SessionMpstTwo<Recv<(), Offer4fromStoC>, RoleS<RoleS<RoleEnd>>, NameC>),
}
type Offer4fromStoC = <Choose4fromStoC as Session>::Dual;
type EndpointC4 = SessionMpstTwo<Offer4fromStoC, RoleS<RoleEnd>, NameC>;
// S
type Choose4fromStoC = Send<Branching4fromStoC, End>;
type EndpointS4 = SessionMpstTwo<Choose4fromStoC, RoleBroadcast, NameS>;

// Step 5
// C
type Choose5fromCtoS = Send<Branching5fromCtoS, End>;
type EndpointC5 = SessionMpstTwo<Choose5fromCtoS, RoleBroadcast, NameC>;
// S
enum Branching5fromCtoS {
    Continue(SessionMpstTwo<Recv<(), Choose6fromStoC>, RoleC<RoleBroadcast>, NameS>),
    Quit(SessionMpstTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer5fromCtoS = <Choose5fromCtoS as Session>::Dual;
type EndpointS5 = SessionMpstTwo<Offer5fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 6
// C
enum Branching6fromStoC {
    Continue(SessionMpstTwo<Recv<(), Choose7fromCtoS>, RoleS<RoleBroadcast>, NameC>),
    Loop(SessionMpstTwo<Recv<(), Offer6fromStoC>, RoleS<RoleS<RoleEnd>>, NameC>),
}
type Offer6fromStoC = <Choose6fromStoC as Session>::Dual;
type EndpointC6 = SessionMpstTwo<Offer6fromStoC, RoleS<RoleEnd>, NameC>;
// S
type Choose6fromStoC = Send<Branching6fromStoC, End>;
type EndpointS6 = SessionMpstTwo<Choose6fromStoC, RoleBroadcast, NameS>;

// Step 7
// C
type Choose7fromCtoS = Send<Branching7fromCtoS, End>;
type EndpointC7 = SessionMpstTwo<Choose7fromCtoS, RoleBroadcast, NameC>;
// S
enum Branching7fromCtoS {
    Continue(SessionMpstTwo<Recv<(), Choose8fromStoC>, RoleC<RoleBroadcast>, NameS>),
    Quit(SessionMpstTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer7fromCtoS = <Choose7fromCtoS as Session>::Dual;
type EndpointS7 = SessionMpstTwo<Offer7fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 8
// C
enum Branching8fromStoC {
    Continue(SessionMpstTwo<Recv<(), Choose9fromCtoS>, RoleS<RoleBroadcast>, NameC>),
    Loop(SessionMpstTwo<Recv<(), Choose7fromCtoS>, RoleS<RoleBroadcast>, NameC>),
}
type Offer8fromStoC = <Choose8fromStoC as Session>::Dual;
type EndpointC8 = SessionMpstTwo<Offer8fromStoC, RoleS<RoleEnd>, NameC>;
// S
type Choose8fromStoC = Send<Branching8fromStoC, End>;
type EndpointS8 = SessionMpstTwo<Choose8fromStoC, RoleBroadcast, NameS>;

// Step 9
// C
type Choose9fromCtoS = Send<Branching9fromCtoS, End>;
type EndpointC9 = SessionMpstTwo<Choose9fromCtoS, RoleBroadcast, NameC>;
// S
enum Branching9fromCtoS {
    Continue(
        SessionMpstTwo<
            Recv<(), Send<(), Recv<(), Recv<(), Offer10fromCtoS>>>>,
            RoleC<RoleC<RoleC<RoleC<RoleC<RoleEnd>>>>>,
            NameS,
        >,
    ),
    Loop(SessionMpstTwo<Recv<(), Send<(), Offer9fromCtoS>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>),
}
type Offer9fromCtoS = <Choose9fromCtoS as Session>::Dual;
type EndpointS9 = SessionMpstTwo<Offer9fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 10
// C
type Choose10fromCtoS = Send<Branching10fromCtoS, End>;
type EndpointC10 = SessionMpstTwo<Choose10fromCtoS, RoleBroadcast, NameC>;
// S
enum Branching10fromCtoS {
    Data(SessionMpstTwo<Recv<(), Recv<(), Offer10fromCtoS>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>),
    Subject(
        SessionMpstTwo<Recv<(), Recv<(), Offer10fromCtoS>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>,
    ),
    End(SessionMpstTwo<Recv<(), Send<(), Offer7fromCtoS>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>),
}
type Offer10fromCtoS = <Choose10fromCtoS as Session>::Dual;
type EndpointS10 = SessionMpstTwo<Offer10fromCtoS, RoleC<RoleEnd>, NameS>;

// Functions
fn endpoint_c_0(s: EndpointC0) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_c_from_s(s)?;

    let expected = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromCtoS::Continue, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
        );

        let s = send_mpst_c_to_s((), s);
        let s = send_mpst_c_to_s((), s);

        endpoint_c_1(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching0fromCtoS::Quit, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
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
    let expected = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching2fromCtoS::Continue, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
        );

        let s = send_mpst_c_to_s((), s);
        let (_, s) = recv_mpst_c_from_s(s)?;

        endpoint_c_3(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching2fromCtoS::Quit, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
        );

        let s = send_mpst_c_to_s((), s);

        close_mpst_multi(s)
    }
}

fn endpoint_c_3(s: EndpointC3) -> Result<(), Box<dyn Error>> {
    let expected = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching3fromCtoS::Continue, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
        );

        let s = send_mpst_c_to_s((), s);

        endpoint_c_4(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching3fromCtoS::Quit, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
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
    let expected = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching5fromCtoS::Continue, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
        );

        let s = send_mpst_c_to_s((), s);

        endpoint_c_6(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching5fromCtoS::Quit, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
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
    let expected = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching7fromCtoS::Continue, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
        );

        let s = send_mpst_c_to_s((), s);

        endpoint_c_8(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching7fromCtoS::Quit, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
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
    let expected = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching9fromCtoS::Continue, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
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
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
        );

        let s = send_mpst_c_to_s((), s);
        let (_, s) = recv_mpst_c_from_s(s)?;

        endpoint_c_9(s)
    }
}

fn endpoint_c_10(s: EndpointC10) -> Result<(), Box<dyn Error>> {
    let expected = thread_rng().gen_range(1..=3);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching10fromCtoS::Data, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
        );

        let s = send_mpst_c_to_s((), s);
        let s = send_mpst_c_to_s((), s);

        endpoint_c_10(s)
    } else if expected == 2 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching10fromCtoS::Subject, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
        );

        let s = send_mpst_c_to_s((), s);
        let s = send_mpst_c_to_s((), s);

        endpoint_c_10(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching10fromCtoS::End, =>
            RoleS, =>
            RoleC, SessionMpstTwo, 2, 1
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
    let expected = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching1fromStoC::Continue, =>
            RoleC, =>
            RoleS, SessionMpstTwo, 2, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_2(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching1fromStoC::Loop, =>
            RoleC, =>
            RoleS, SessionMpstTwo, 2, 2
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
    let expected = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching4fromStoC::Continue, =>
            RoleC, =>
            RoleS, SessionMpstTwo, 2, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_5(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching4fromStoC::Loop, =>
            RoleC, =>
            RoleS, SessionMpstTwo, 2, 2
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
    let expected = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching6fromStoC::Continue, =>
            RoleC, =>
            RoleS, SessionMpstTwo, 2, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_7(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching6fromStoC::Loop, =>
            RoleC, =>
            RoleS, SessionMpstTwo, 2, 2
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
    let expected = thread_rng().gen_range(1..=2);

    if expected == 1 {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching8fromStoC::Continue, =>
            RoleC, =>
            RoleS, SessionMpstTwo, 2, 2
        );

        let s = send_mpst_s_to_c((), s);

        endpoint_s_9(s)
    } else {
        let s = choose_mpst_multi_to_all!(
            s,
            Branching8fromStoC::Loop, =>
            RoleC, =>
            RoleS, SessionMpstTwo, 2, 2
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

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_c, thread_s) = fork_mpst(endpoint_c_0, endpoint_s_0);

    thread_c.join().unwrap();
    thread_s.join().unwrap();

    Ok(())
}

fn main() {
    assert!(all_mpst().is_ok());
}
