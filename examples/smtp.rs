// Unfinished, still in process of adding send_tcp and recv_tcp

#![allow(dead_code, unused_imports)]

use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::role::Role;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_multi_to_all, create_multiple_normal_role,
    create_recv_mpst_session_bundle, create_send_mpst_session_bundle, create_sessionmpst,
    offer_mpst,
};

use rand::{random, thread_rng, Rng};
use std::error::Error;
use std::marker;
use std::time::Duration;

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
//                                          {
//                                             235() from S to C;
//                                              rec Z1
//                                             {
//                                                 choice at C // Choice 7
//                                                  {
//                                                      Mail() from C to S; //Mail from:<a@b.com>
//                                                      choice at S // Choice 8
//                                                      {
//                                                          501() from S to C;
//                                                          continue Z1;
//                                                      }
//                                                      or
//                                                      {
//                                                          2502() from S to C;
//                                                          rec Z2
//                                                          {
//                                                              choice at C // Choice 9
//                                                              {
//                                                                  Rcpt() from C to S; //Rcpt to:<c@d.com>
//                                                                  choice at S // What is this choice???  // Choice X
//                                                                      {
//                                                                          2503() from S to C;
//                                                                          continue Z2;
//                                                                      }
//                                                              }
//                                                              or
//                                                              {
//                                                                  Data() from C to S;
//                                                                  354() from S to C;
//
//                                                                  too from C to S; //to:<you>
//
//                                                                  froom from C to S; //from:<me>
//                                                                  rec Z3
//                                                                  {
//                                                                      choice at C // Choice 10
//                                                                      {
//                                                                          DataLine() from C to S;
//                                                                          DataLine() from C to S;
//                                                                          continue Z3;
//                                                                      }
//                                                                      or
//                                                                      {
//                                                                          Subject() from C to S; //Subject:<my Subject>
//                                                                          Subject() from C to S; //Subject:<my Subject>
//                                                                          continue Z3;
//                                                                      }
//                                                                      or
//                                                                      {
//                                                                          EndOfData() from C to S; // CRLF.CRLF
//                                                                          2504() from S to C;
//                                                                          continue Z1;
//                                                                      }
//                                                                  }
//                                                              }
//                                                          }
//                                                      }
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
// C
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;
type EndpointC0 = SessionMpstTwo<Choose0fromCtoS, RoleBroadcast, NameC>;

enum Branching1fromStoC {
    Continue(SessionMpstTwo<Choose2fromCtoS, RoleBroadcast, NameC>),
    Quit(SessionMpstTwo<Offer1fromStoC, RoleEnd, NameC>),
}
type Offer1fromStoC = <Choose1fromStoC as Session>::Dual;
type EndpointC1 = SessionMpstTwo<Offer1fromStoC, RoleS<RoleEnd>, NameC>;

type Choose2fromCtoS = Send<Branching2fromCtoS, End>;
type EndpointC2 = SessionMpstTwo<Choose2fromCtoS, RoleBroadcast, NameC>;

type Choose3fromCtoS = Send<Branching3fromCtoS, End>;
type EndpointC3 = SessionMpstTwo<Choose3fromCtoS, RoleBroadcast, NameC>;

enum Branching4fromStoC {
    Continue(SessionMpstTwo<Choose5fromCtoS, RoleBroadcast, NameC>),
    Quit(SessionMpstTwo<Offer1fromStoC, RoleEnd, NameC>),
}
type Offer4fromStoC = <Choose4fromStoC as Session>::Dual;
type EndpointC4 = SessionMpstTwo<Offer4fromStoC, RoleS<RoleEnd>, NameC>;

type Choose5fromCtoS = Send<Branching5fromCtoS, End>;
type EndpointC5 = SessionMpstTwo<Choose5fromCtoS, RoleBroadcast, NameC>;

enum Branching6fromStoC {
    Continue(SessionMpstTwo<Choose7fromCtoS, RoleBroadcast, NameC>),
    Quit(SessionMpstTwo<Offer1fromStoC, RoleEnd, NameC>),
}
type Offer6fromStoC = <Choose6fromStoC as Session>::Dual;
type EndpointC6 = SessionMpstTwo<Offer6fromStoC, RoleS<RoleEnd>, NameC>;

type Choose7fromCtoS = Send<Branching7fromCtoS, End>;
type EndpointC7 = SessionMpstTwo<Choose7fromCtoS, RoleBroadcast, NameC>;

enum Branching8fromStoC {
    Continue(SessionMpstTwo<Choose9fromCtoS, RoleBroadcast, NameC>),
    Quit(SessionMpstTwo<Offer1fromStoC, RoleEnd, NameC>),
}
type Offer8fromStoC = <Choose8fromStoC as Session>::Dual;
type EndpointC8 = SessionMpstTwo<Offer8fromStoC, RoleS<RoleEnd>, NameC>;

type Choose9fromCtoS = Send<Branching9fromCtoS, End>;
type EndpointC9 = SessionMpstTwo<Choose9fromCtoS, RoleBroadcast, NameC>;

type Choose10fromCtoS = Send<Branching10fromCtoS, End>;
type EndpointC10 = SessionMpstTwo<Choose10fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching0fromCtoS {
    Continue(SessionMpstTwo<Choose1fromStoC, RoleBroadcast, NameS>),
    Quit(SessionMpstTwo<End, RoleEnd, NameS>),
}
type Offer0fromCtoS = <Choose0fromCtoS as Session>::Dual;
type EndpointS0 = SessionMpstTwo<Offer0fromCtoS, RoleC<RoleEnd>, NameS>;

type Choose1fromStoC = Send<Branching1fromStoC, End>;
type EndpointS1 = SessionMpstTwo<Choose1fromStoC, RoleBroadcast, NameS>;

enum Branching2fromCtoS {
    Continue(SessionMpstTwo<End, RoleEnd, NameS>),
    Quit(SessionMpstTwo<End, RoleEnd, NameS>),
}
type Offer2fromCtoS = <Choose2fromCtoS as Session>::Dual;
type EndpointS2 = SessionMpstTwo<Offer2fromCtoS, RoleC<RoleEnd>, NameS>;

enum Branching3fromCtoS {
    Continue(SessionMpstTwo<Choose4fromStoC, RoleBroadcast, NameS>),
    Quit(SessionMpstTwo<End, RoleEnd, NameS>),
}
type Offer3fromCtoS = <Choose3fromCtoS as Session>::Dual;
type EndpointS3 = SessionMpstTwo<Offer3fromCtoS, RoleC<RoleEnd>, NameS>;

type Choose4fromStoC = Send<Branching4fromStoC, End>;
type EndpointS4 = SessionMpstTwo<Choose4fromStoC, RoleBroadcast, NameS>;

enum Branching5fromCtoS {
    Continue(SessionMpstTwo<Choose6fromStoC, RoleBroadcast, NameS>),
    Quit(SessionMpstTwo<End, RoleEnd, NameS>),
}
type Offer5fromCtoS = <Choose5fromCtoS as Session>::Dual;
type EndpointS5 = SessionMpstTwo<Offer5fromCtoS, RoleC<RoleEnd>, NameS>;

type Choose6fromStoC = Send<Branching6fromStoC, End>;
type EndpointS6 = SessionMpstTwo<Choose6fromStoC, RoleBroadcast, NameS>;

enum Branching7fromCtoS {
    Continue(SessionMpstTwo<Choose8fromStoC, RoleBroadcast, NameS>),
    Quit(SessionMpstTwo<End, RoleEnd, NameS>),
}
type Offer7fromCtoS = <Choose7fromCtoS as Session>::Dual;
type EndpointS7 = SessionMpstTwo<Offer7fromCtoS, RoleC<RoleEnd>, NameS>;

type Choose8fromStoC = Send<Branching8fromStoC, End>;
type EndpointS8 = SessionMpstTwo<Choose8fromStoC, RoleBroadcast, NameS>;

enum Branching9fromCtoS {
    Continue(SessionMpstTwo<Offer10fromCtoS, RoleBroadcast, NameS>),
    Quit(SessionMpstTwo<End, RoleEnd, NameS>),
}
type Offer9fromCtoS = <Choose9fromCtoS as Session>::Dual;
type EndpointS9 = SessionMpstTwo<Offer9fromCtoS, RoleC<RoleEnd>, NameS>;

enum Branching10fromCtoS {
    Continue(SessionMpstTwo<End, RoleEnd, NameS>),
    Quit(SessionMpstTwo<End, RoleEnd, NameS>),
}
type Offer10fromCtoS = <Choose10fromCtoS as Session>::Dual;
type EndpointS10 = SessionMpstTwo<Offer10fromCtoS, RoleC<RoleEnd>, NameS>;

// Creating the MP sessions

// None
type EndpointNoneC = SessionMpstTwo<End, RoleEnd, NameC>;
type EndpointNoneS = SessionMpstTwo<End, RoleEnd, NameS>;

// Functions
// C
fn endpoint_c(s: EndpointNoneC) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}
// S
fn endpoint_s(s: EndpointNoneS) -> Result<(), Box<dyn Error>> {
    close_mpst_multi(s)
}

fn all_mpst() -> Result<(), Box<dyn Error>> {
    let (thread_c, thread_s) = fork_mpst(endpoint_c, endpoint_s);

    thread_c.join().unwrap();
    thread_s.join().unwrap();

    Ok(())
}

fn main() {
    assert!(all_mpst().is_ok());
}
