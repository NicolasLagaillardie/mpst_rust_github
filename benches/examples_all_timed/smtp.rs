#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use criterion::{black_box, Criterion};

use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
baker_timed!(MeshedChannels, C, S);

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
type Choose0fromCtoS = SendTimed<Branching0fromCtoS, End, 'a', 0, true, 1, true, false>;
type EndpointC0 = MeshedChannels<
    RecvTimed<(), Choose0fromCtoS, 'a', 0, true, 1, true, false>,
    RoleS<RoleBroadcast>,
    NameC,
>;

// S
enum Branching0fromCtoS {
    Continue(
        MeshedChannels<
            RecvTimed<
                (),
                RecvTimed<(), Choose1fromStoC, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            TwoRoleCBroadcast,
            NameS,
        >,
    ),
    Quit(MeshedChannels<RecvTimed<(), End, 'a', 0, true, 1, true, false>, RoleC<RoleEnd>, NameS>),
}
type Offer0fromCtoS = <Choose0fromCtoS as Session>::Dual;
type EndpointS0 = MeshedChannels<
    SendTimed<(), Offer0fromCtoS, 'a', 0, true, 1, true, false>,
    RoleC<RoleC<RoleEnd>>,
    NameS,
>;

// Step 1
// C
enum Branching1fromStoC {
    Continue(
        MeshedChannels<
            RecvTimed<(), Choose2fromCtoS, 'a', 0, true, 1, true, false>,
            RoleS<RoleBroadcast>,
            NameC,
        >,
    ),
    Loop(
        MeshedChannels<
            RecvTimed<
                (),
                RecvTimed<(), Offer1fromStoC, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            ThreeRoleS,
            NameC,
        >,
    ),
}
type Offer1fromStoC = <Choose1fromStoC as Session>::Dual;
type EndpointC1 = MeshedChannels<Offer1fromStoC, RoleS<RoleEnd>, NameC>;

// S
type Choose1fromStoC = SendTimed<Branching1fromStoC, End, 'a', 0, true, 1, true, false>;
type EndpointS1 = MeshedChannels<Choose1fromStoC, RoleBroadcast, NameS>;

// Step 2
// C
type Choose2fromCtoS = SendTimed<Branching2fromCtoS, End, 'a', 0, true, 1, true, false>;
type EndpointC2 = MeshedChannels<Choose2fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching2fromCtoS {
    Continue(
        MeshedChannels<
            RecvTimed<
                (),
                SendTimed<(), Offer3fromCtoS, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            ThreeRoleC,
            NameS,
        >,
    ),
    Quit(MeshedChannels<RecvTimed<(), End, 'a', 0, true, 1, true, false>, RoleC<RoleEnd>, NameS>),
}
type Offer2fromCtoS = <Choose2fromCtoS as Session>::Dual;
type EndpointS2 = MeshedChannels<Offer2fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 3
// C
type Choose3fromCtoS = SendTimed<Branching3fromCtoS, End, 'a', 0, true, 1, true, false>;
type EndpointC3 = MeshedChannels<Choose3fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching3fromCtoS {
    Continue(
        MeshedChannels<
            RecvTimed<(), Choose4fromStoC, 'a', 0, true, 1, true, false>,
            RoleC<RoleBroadcast>,
            NameS,
        >,
    ),
    Quit(MeshedChannels<RecvTimed<(), End, 'a', 0, true, 1, true, false>, RoleC<RoleEnd>, NameS>),
}
type Offer3fromCtoS = <Choose3fromCtoS as Session>::Dual;
type EndpointS3 = MeshedChannels<Offer3fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 4
// C
enum Branching4fromStoC {
    Continue(
        MeshedChannels<
            RecvTimed<(), Choose5fromCtoS, 'a', 0, true, 1, true, false>,
            RoleS<RoleBroadcast>,
            NameC,
        >,
    ),
    Loop(
        MeshedChannels<
            RecvTimed<(), Offer4fromStoC, 'a', 0, true, 1, true, false>,
            RoleS<RoleS<RoleEnd>>,
            NameC,
        >,
    ),
}
type Offer4fromStoC = <Choose4fromStoC as Session>::Dual;
type EndpointC4 = MeshedChannels<Offer4fromStoC, RoleS<RoleEnd>, NameC>;

// S
type Choose4fromStoC = SendTimed<Branching4fromStoC, End, 'a', 0, true, 1, true, false>;
type EndpointS4 = MeshedChannels<Choose4fromStoC, RoleBroadcast, NameS>;

// Step 5
// C
type Choose5fromCtoS = SendTimed<Branching5fromCtoS, End, 'a', 0, true, 1, true, false>;
type EndpointC5 = MeshedChannels<Choose5fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching5fromCtoS {
    Continue(
        MeshedChannels<
            RecvTimed<(), Choose6fromStoC, 'a', 0, true, 1, true, false>,
            RoleC<RoleBroadcast>,
            NameS,
        >,
    ),
    Quit(MeshedChannels<RecvTimed<(), End, 'a', 0, true, 1, true, false>, RoleC<RoleEnd>, NameS>),
}
type Offer5fromCtoS = <Choose5fromCtoS as Session>::Dual;
type EndpointS5 = MeshedChannels<Offer5fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 6
// C
enum Branching6fromStoC {
    Continue(
        MeshedChannels<
            RecvTimed<(), Choose7fromCtoS, 'a', 0, true, 1, true, false>,
            RoleS<RoleBroadcast>,
            NameC,
        >,
    ),
    Loop(
        MeshedChannels<
            RecvTimed<(), Offer6fromStoC, 'a', 0, true, 1, true, false>,
            RoleS<RoleS<RoleEnd>>,
            NameC,
        >,
    ),
}
type Offer6fromStoC = <Choose6fromStoC as Session>::Dual;
type EndpointC6 = MeshedChannels<Offer6fromStoC, RoleS<RoleEnd>, NameC>;

// S
type Choose6fromStoC = SendTimed<Branching6fromStoC, End, 'a', 0, true, 1, true, false>;
type EndpointS6 = MeshedChannels<Choose6fromStoC, RoleBroadcast, NameS>;

// Step 7
// C
type Choose7fromCtoS = SendTimed<Branching7fromCtoS, End, 'a', 0, true, 1, true, false>;
type EndpointC7 = MeshedChannels<Choose7fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching7fromCtoS {
    Continue(
        MeshedChannels<
            RecvTimed<(), Choose8fromStoC, 'a', 0, true, 1, true, false>,
            RoleC<RoleBroadcast>,
            NameS,
        >,
    ),
    Quit(MeshedChannels<RecvTimed<(), End, 'a', 0, true, 1, true, false>, RoleC<RoleEnd>, NameS>),
}
type Offer7fromCtoS = <Choose7fromCtoS as Session>::Dual;
type EndpointS7 = MeshedChannels<Offer7fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 8
// C
enum Branching8fromStoC {
    Continue(
        MeshedChannels<
            RecvTimed<(), Choose9fromCtoS, 'a', 0, true, 1, true, false>,
            RoleS<RoleBroadcast>,
            NameC,
        >,
    ),
    Loop(
        MeshedChannels<
            RecvTimed<(), Choose7fromCtoS, 'a', 0, true, 1, true, false>,
            RoleS<RoleBroadcast>,
            NameC,
        >,
    ),
}
type Offer8fromStoC = <Choose8fromStoC as Session>::Dual;
type EndpointC8 = MeshedChannels<Offer8fromStoC, RoleS<RoleEnd>, NameC>;

// S
type Choose8fromStoC = SendTimed<Branching8fromStoC, End, 'a', 0, true, 1, true, false>;
type EndpointS8 = MeshedChannels<Choose8fromStoC, RoleBroadcast, NameS>;

// Step 9
// C
type Choose9fromCtoS = SendTimed<Branching9fromCtoS, End, 'a', 0, true, 1, true, false>;
type EndpointC9 = MeshedChannels<Choose9fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching9fromCtoS {
    Continue(MeshedChannels<FullOffer10fromCtoS, FiveRoleC, NameS>),
    Loop(
        MeshedChannels<
            RecvTimed<
                (),
                SendTimed<(), Offer9fromCtoS, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            ThreeRoleC,
            NameS,
        >,
    ),
}
type FullOffer10fromCtoS = RecvTimed<
    (),
    SendTimed<
        (),
        RecvTimed<
            (),
            RecvTimed<(), Offer10fromCtoS, 'a', 0, true, 1, true, false>,
            'a',
            0,
            true,
            1,
            true,
            false,
        >,
        'a',
        0,
        true,
        1,
        true,
        false,
    >,
    'a',
    0,
    true,
    1,
    true,
    false,
>;
type Offer9fromCtoS = <Choose9fromCtoS as Session>::Dual;
type EndpointS9 = MeshedChannels<Offer9fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 10
// C
type Choose10fromCtoS = SendTimed<Branching10fromCtoS, End, 'a', 0, true, 1, true, false>;
type EndpointC10 = MeshedChannels<Choose10fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching10fromCtoS {
    Data(
        MeshedChannels<
            RecvTimed<
                (),
                RecvTimed<(), Offer10fromCtoS, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            ThreeRoleC,
            NameS,
        >,
    ),
    Subject(
        MeshedChannels<
            RecvTimed<
                (),
                RecvTimed<(), Offer10fromCtoS, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            ThreeRoleC,
            NameS,
        >,
    ),
    End(
        MeshedChannels<
            RecvTimed<
                (),
                SendTimed<(), Offer7fromCtoS, 'a', 0, true, 1, true, false>,
                'a',
                0,
                true,
                1,
                true,
                false,
            >,
            ThreeRoleC,
            NameS,
        >,
    ),
}
type Offer10fromCtoS = <Choose10fromCtoS as Session>::Dual;
type EndpointS10 = MeshedChannels<Offer10fromCtoS, RoleC<RoleEnd>, NameS>;

// Functions
fn endpoint_c_init(
    s: EndpointC0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    endpoint_c_0(s, 100, all_clocks)
}

fn endpoint_c_0(
    s: EndpointC0,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv(all_clocks)?;

    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching0fromCtoS::Quit);

            let s = s.send((), all_clocks)?;

            s.close()
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching0fromCtoS::Continue);

            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;

            endpoint_c_1(s, loops, all_clocks)
        }
    }
}

fn endpoint_c_1(
    s: EndpointC1,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching1fromStoC::Continue(s) => {
            let (_, s) = s.recv(all_clocks)?;

            endpoint_c_2(s, loops, all_clocks)
        },
        Branching1fromStoC::Loop(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            endpoint_c_1(s, loops, all_clocks)
        },
    })
}

fn endpoint_c_2(
    s: EndpointC2,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching2fromCtoS::Quit);

            let s = s.send((), all_clocks)?;

            s.close()
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching2fromCtoS::Continue);

            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            endpoint_c_3(s, loops, all_clocks)
        }
    }
}

fn endpoint_c_3(
    s: EndpointC3,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching3fromCtoS::Quit);
            let s = s.send((), all_clocks)?;
            s.close()
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching3fromCtoS::Continue);

            let s = s.send((), all_clocks)?;

            endpoint_c_4(s, loops, all_clocks)
        }
    }
}

fn endpoint_c_4(
    s: EndpointC4,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching4fromStoC::Continue(s) => {
            let (_, s) = s.recv(all_clocks)?;

            endpoint_c_5(s, loops, all_clocks)
        },
        Branching4fromStoC::Loop(s) => {
            let (_, s) = s.recv(all_clocks)?;

            endpoint_c_4(s, loops, all_clocks)
        },
    })
}

fn endpoint_c_5(
    s: EndpointC5,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching5fromCtoS::Quit);

            let s = s.send((), all_clocks)?;

            s.close()
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching5fromCtoS::Continue);

            let s = s.send((), all_clocks)?;

            endpoint_c_6(s, loops, all_clocks)
        }
    }
}

fn endpoint_c_6(
    s: EndpointC6,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching6fromStoC::Continue(s) => {
            let (_, s) = s.recv(all_clocks)?;

            endpoint_c_7(s, loops, all_clocks)
        },
        Branching6fromStoC::Loop(s) => {
            let (_, s) = s.recv(all_clocks)?;

            endpoint_c_6(s, loops, all_clocks)
        },
    })
}

fn endpoint_c_7(
    s: EndpointC7,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching7fromCtoS::Quit);

            let s = s.send((), all_clocks)?;

            s.close()
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching7fromCtoS::Continue);

            let s = s.send((), all_clocks)?;

            endpoint_c_8(s, loops, all_clocks)
        }
    }
}

fn endpoint_c_8(
    s: EndpointC8,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching8fromStoC::Continue(s) => {
            let (_, s) = s.recv(all_clocks)?;

            endpoint_c_9(s, loops, all_clocks)
        },
        Branching8fromStoC::Loop(s) => {
            let (_, s) = s.recv(all_clocks)?;

            endpoint_c_7(s, loops, all_clocks)
        },
    })
}

fn endpoint_c_9(
    s: EndpointC9,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching9fromCtoS::Loop);

            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            endpoint_c_9(s, loops, all_clocks)
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, all_clocks, Branching9fromCtoS::Continue);

            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;

            endpoint_c_10(s, loops, all_clocks)
        }
    }
}

fn endpoint_c_10(
    s: EndpointC10,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    if loops == 0 {
        let s = choose_mpst_c_to_all!(s, all_clocks, Branching10fromCtoS::End);

        let s = s.send((), all_clocks)?;
        let (_, s) = s.recv(all_clocks)?;

        endpoint_c_7(s, loops, all_clocks)
    } else if loops % 2 == 1 {
        let s = choose_mpst_c_to_all!(s, all_clocks, Branching10fromCtoS::Subject);

        let s = s.send((), all_clocks)?;
        let s = s.send((), all_clocks)?;

        endpoint_c_10(s, loops - 1, all_clocks)
    } else {
        let s = choose_mpst_c_to_all!(s, all_clocks, Branching10fromCtoS::Data);

        let s = s.send((), all_clocks)?;
        let s = s.send((), all_clocks)?;

        endpoint_c_10(s, loops - 1, all_clocks)
    }
}

///
fn endpoint_s_init(
    s: EndpointS0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    endpoint_s_0(s, 100, all_clocks)
}

fn endpoint_s_0(
    s: EndpointS0,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    let s = s.send((), all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Branching0fromCtoS::Quit(s) => {
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
        Branching0fromCtoS::Continue(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            endpoint_s_1(s, loops, all_clocks)
        },
    })
}

fn endpoint_s_1(
    s: EndpointS1,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_s_to_all!(s, all_clocks, Branching1fromStoC::Loop);

            let s = s.send((), all_clocks)?;
            let s = s.send((), all_clocks)?;

            endpoint_s_1(s, loops, all_clocks)
        }
        _ => {
            let s = choose_mpst_s_to_all!(s, all_clocks, Branching1fromStoC::Continue);

            let s = s.send((), all_clocks)?;

            endpoint_s_2(s, loops, all_clocks)
        }
    }
}

fn endpoint_s_2(
    s: EndpointS2,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching2fromCtoS::Quit(s) => {
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
        Branching2fromCtoS::Continue(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;

            endpoint_s_3(s, loops, all_clocks)
        },
    })
}

fn endpoint_s_3(
    s: EndpointS3,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching3fromCtoS::Quit(s) => {
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
        Branching3fromCtoS::Continue(s) => {
            let (_, s) = s.recv(all_clocks)?;

            endpoint_s_4(s, loops, all_clocks)
        },
    })
}

fn endpoint_s_4(
    s: EndpointS4,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_s_to_all!(s, all_clocks, Branching4fromStoC::Loop);

            let s = s.send((), all_clocks)?;

            endpoint_s_4(s, loops, all_clocks)
        }
        _ => {
            let s = choose_mpst_s_to_all!(s, all_clocks, Branching4fromStoC::Continue);

            let s = s.send((), all_clocks)?;

            endpoint_s_5(s, loops, all_clocks)
        }
    }
}

fn endpoint_s_5(
    s: EndpointS5,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching5fromCtoS::Quit(s) => {
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
        Branching5fromCtoS::Continue(s) => {
            let (_, s) = s.recv(all_clocks)?;

            endpoint_s_6(s, loops, all_clocks)
        },
    })
}

fn endpoint_s_6(
    s: EndpointS6,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_s_to_all!(s, all_clocks, Branching6fromStoC::Loop);
            let s = s.send((), all_clocks)?;
            endpoint_s_6(s, loops, all_clocks)
        }
        _ => {
            let s = choose_mpst_s_to_all!(s, all_clocks, Branching6fromStoC::Continue);

            let s = s.send((), all_clocks)?;

            endpoint_s_7(s, loops, all_clocks)
        }
    }
}

fn endpoint_s_7(
    s: EndpointS7,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching7fromCtoS::Quit(s) => {
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
        Branching7fromCtoS::Continue(s) => {
            let (_, s) = s.recv(all_clocks)?;

            endpoint_s_8(s, loops, all_clocks)
        },
    })
}

fn endpoint_s_8(
    s: EndpointS8,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_s_to_all!(s, all_clocks, Branching8fromStoC::Loop);
            let s = s.send((), all_clocks)?;
            endpoint_s_7(s, loops, all_clocks)
        }
        _ => {
            let s = choose_mpst_s_to_all!(s, all_clocks, Branching8fromStoC::Continue);

            let s = s.send((), all_clocks)?;

            endpoint_s_9(s, loops, all_clocks)
        }
    }
}

fn endpoint_s_9(
    s: EndpointS9,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching9fromCtoS::Loop(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;

            endpoint_s_9(s, loops, all_clocks)
        },
        Branching9fromCtoS::Continue(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            endpoint_s_10(s, loops, all_clocks)
        },
    })
}

fn endpoint_s_10(
    s: EndpointS10,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching10fromCtoS::Data(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            endpoint_s_10(s, loops - 1, all_clocks)
        },
        Branching10fromCtoS::Subject(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            endpoint_s_10(s, loops - 1, all_clocks)
        },
        Branching10fromCtoS::End(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let s = s.send((), all_clocks)?;

            endpoint_s_7(s, loops - 1, all_clocks)
        },
    })
}

///

fn all_mpst() {
    let (thread_c, thread_s) = fork_mpst(black_box(endpoint_c_init), black_box(endpoint_s_init));

    thread_c.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

pub fn smtp_main(c: &mut Criterion) {
    c.bench_function("Timed SMTP baking", |b| b.iter(all_mpst));
}
