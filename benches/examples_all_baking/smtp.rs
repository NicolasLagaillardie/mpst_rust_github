#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::baker;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
baker!("rec_and_cancel", MeshedChannelsTwo, C, S);

// Types
// Step 0
// C
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;
type EndpointC0 = MeshedChannelsTwo<Recv<(), Choose0fromCtoS>, RoleS<RoleBroadcast>, NameC>;

// S
enum Branching0fromCtoS {
    Continue(
        MeshedChannelsTwo<Recv<(), Recv<(), Choose1fromStoC>>, RoleC<RoleC<RoleBroadcast>>, NameS>,
    ),
    Quit(MeshedChannelsTwo<Recv<(), End>, RoleC<RoleEnd>, NameS>),
}
type Offer0fromCtoS = <Choose0fromCtoS as Session>::Dual;
type EndpointS0 = MeshedChannelsTwo<Send<(), Offer0fromCtoS>, RoleC<RoleC<RoleEnd>>, NameS>;

// Step 1
// C
enum Branching1fromStoC {
    Continue(MeshedChannelsTwo<Recv<(), Choose2fromCtoS>, RoleS<RoleBroadcast>, NameC>),
    Loop(
        MeshedChannelsTwo<Recv<(), Recv<(), Offer1fromStoC>>, RoleS<RoleS<RoleS<RoleEnd>>>, NameC>,
    ),
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
    Continue(
        MeshedChannelsTwo<Recv<(), Send<(), Offer3fromCtoS>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>,
    ),
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
    Continue(
        MeshedChannelsTwo<
            Recv<(), Send<(), Recv<(), Recv<(), Offer10fromCtoS>>>>,
            RoleC<RoleC<RoleC<RoleC<RoleC<RoleEnd>>>>>,
            NameS,
        >,
    ),
    Loop(
        MeshedChannelsTwo<Recv<(), Send<(), Offer9fromCtoS>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>,
    ),
}
type Offer9fromCtoS = <Choose9fromCtoS as Session>::Dual;
type EndpointS9 = MeshedChannelsTwo<Offer9fromCtoS, RoleC<RoleEnd>, NameS>;

// Step 10
// C
type Choose10fromCtoS = Send<Branching10fromCtoS, End>;
type EndpointC10 = MeshedChannelsTwo<Choose10fromCtoS, RoleBroadcast, NameC>;

// S
enum Branching10fromCtoS {
    Data(
        MeshedChannelsTwo<Recv<(), Recv<(), Offer10fromCtoS>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>,
    ),
    Subject(
        MeshedChannelsTwo<Recv<(), Recv<(), Offer10fromCtoS>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>,
    ),
    End(MeshedChannelsTwo<Recv<(), Send<(), Offer7fromCtoS>>, RoleC<RoleC<RoleC<RoleEnd>>>, NameS>),
}
type Offer10fromCtoS = <Choose10fromCtoS as Session>::Dual;
type EndpointS10 = MeshedChannelsTwo<Offer10fromCtoS, RoleC<RoleEnd>, NameS>;

// Functions
fn endpoint_c_init(s: EndpointC0) -> Result<(), Box<dyn Error>> {
    endpoint_c_0(s, 100)
}

fn endpoint_c_0(s: EndpointC0, loops: i32) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;

    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branching0fromCtoS::Quit);

            let s = s.send(())?;

            s.close()
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, Branching0fromCtoS::Continue);

            let s = s.send(())?;
            let s = s.send(())?;

            endpoint_c_1(s, loops)
        }
    }
}

fn endpoint_c_1(s: EndpointC1, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromStoC::Continue(s) => {
            let (_, s) = s.recv()?;

            endpoint_c_2(s, loops)
        },
        Branching1fromStoC::Loop(s) => {
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;

            endpoint_c_1(s, loops)
        },
    })
}

fn endpoint_c_2(s: EndpointC2, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branching2fromCtoS::Quit);

            let s = s.send(())?;

            s.close()
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, Branching2fromCtoS::Continue);

            let s = s.send(())?;
            let (_, s) = s.recv()?;

            endpoint_c_3(s, loops)
        }
    }
}

fn endpoint_c_3(s: EndpointC3, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branching3fromCtoS::Quit);

            let s = s.send(())?;

            s.close()
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, Branching3fromCtoS::Continue);

            let s = s.send(())?;

            endpoint_c_4(s, loops)
        }
    }
}

fn endpoint_c_4(s: EndpointC4, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching4fromStoC::Continue(s) => {
            let (_, s) = s.recv()?;

            endpoint_c_5(s, loops)
        },
        Branching4fromStoC::Loop(s) => {
            let (_, s) = s.recv()?;

            endpoint_c_4(s, loops)
        },
    })
}

fn endpoint_c_5(s: EndpointC5, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branching5fromCtoS::Quit);

            let s = s.send(())?;

            s.close()
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, Branching5fromCtoS::Continue);

            let s = s.send(())?;

            endpoint_c_6(s, loops)
        }
    }
}

fn endpoint_c_6(s: EndpointC6, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching6fromStoC::Continue(s) => {
            let (_, s) = s.recv()?;

            endpoint_c_7(s, loops)
        },
        Branching6fromStoC::Loop(s) => {
            let (_, s) = s.recv()?;

            endpoint_c_6(s, loops)
        },
    })
}

fn endpoint_c_7(s: EndpointC7, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branching7fromCtoS::Quit);

            let s = s.send(())?;

            s.close()
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, Branching7fromCtoS::Continue);

            let s = s.send(())?;

            endpoint_c_8(s, loops)
        }
    }
}

fn endpoint_c_8(s: EndpointC8, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching8fromStoC::Continue(s) => {
            let (_, s) = s.recv()?;

            endpoint_c_9(s, loops)
        },
        Branching8fromStoC::Loop(s) => {
            let (_, s) = s.recv()?;

            endpoint_c_7(s, loops)
        },
    })
}

fn endpoint_c_9(s: EndpointC9, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_c_to_all!(s, Branching9fromCtoS::Loop);

            let s = s.send(())?;
            let (_, s) = s.recv()?;

            endpoint_c_9(s, loops)
        }
        _ => {
            let s = choose_mpst_c_to_all!(s, Branching9fromCtoS::Continue);

            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let s = s.send(())?;

            endpoint_c_10(s, loops)
        }
    }
}

fn endpoint_c_10(s: EndpointC10, loops: i32) -> Result<(), Box<dyn Error>> {
    if loops == 0 {
        let s = choose_mpst_c_to_all!(s, Branching10fromCtoS::End);

        let s = s.send(())?;
        let (_, s) = s.recv()?;

        endpoint_c_7(s, loops)
    } else if loops % 2 == 1 {
        let s = choose_mpst_c_to_all!(s, Branching10fromCtoS::Subject);

        let s = s.send(())?;
        let s = s.send(())?;

        endpoint_c_10(s, loops - 1)
    } else {
        let s = choose_mpst_c_to_all!(s, Branching10fromCtoS::Data);

        let s = s.send(())?;
        let s = s.send(())?;

        endpoint_c_10(s, loops - 1)
    }
}

///
fn endpoint_s_init(s: EndpointS0) -> Result<(), Box<dyn Error>> {
    endpoint_s_0(s, 100)
}

fn endpoint_s_0(s: EndpointS0, loops: i32) -> Result<(), Box<dyn Error>> {
    let s = s.send(())?;

    offer_mpst!(s, {
        Branching0fromCtoS::Quit(s) => {
            let (_, s) = s.recv()?;

            s.close()
        },
        Branching0fromCtoS::Continue(s) => {
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;

            endpoint_s_1(s, loops)
        },
    })
}

fn endpoint_s_1(s: EndpointS1, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_s_to_all!(s, Branching1fromStoC::Loop);

            let s = s.send(())?;
            let s = s.send(())?;

            endpoint_s_1(s, loops)
        }
        _ => {
            let s = choose_mpst_s_to_all!(s, Branching1fromStoC::Continue);

            let s = s.send(())?;

            endpoint_s_2(s, loops)
        }
    }
}

fn endpoint_s_2(s: EndpointS2, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching2fromCtoS::Quit(s) => {
            let (_, s) = s.recv()?;

            s.close()
        },
        Branching2fromCtoS::Continue(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;

            endpoint_s_3(s, loops)
        },
    })
}

fn endpoint_s_3(s: EndpointS3, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching3fromCtoS::Quit(s) => {
            let (_, s) = s.recv()?;

            s.close()
        },
        Branching3fromCtoS::Continue(s) => {
            let (_, s) = s.recv()?;

            endpoint_s_4(s, loops)
        },
    })
}

fn endpoint_s_4(s: EndpointS4, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_s_to_all!(s, Branching4fromStoC::Loop);

            let s = s.send(())?;

            endpoint_s_4(s, loops)
        }
        _ => {
            let s = choose_mpst_s_to_all!(s, Branching4fromStoC::Continue);

            let s = s.send(())?;

            endpoint_s_5(s, loops)
        }
    }
}

fn endpoint_s_5(s: EndpointS5, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching5fromCtoS::Quit(s) => {
            let (_, s) = s.recv()?;

            s.close()
        },
        Branching5fromCtoS::Continue(s) => {
            let (_, s) = s.recv()?;

            endpoint_s_6(s, loops)
        },
    })
}

fn endpoint_s_6(s: EndpointS6, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_s_to_all!(s, Branching6fromStoC::Loop);

            let s = s.send(())?;

            endpoint_s_6(s, loops)
        }
        _ => {
            let s = choose_mpst_s_to_all!(s, Branching6fromStoC::Continue);

            let s = s.send(())?;

            endpoint_s_7(s, loops)
        }
    }
}

fn endpoint_s_7(s: EndpointS7, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching7fromCtoS::Quit(s) => {
            let (_, s) = s.recv()?;

            s.close()
        },
        Branching7fromCtoS::Continue(s) => {
            let (_, s) = s.recv()?;

            endpoint_s_8(s, loops)
        },
    })
}

fn endpoint_s_8(s: EndpointS8, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose_mpst_s_to_all!(s, Branching8fromStoC::Loop);

            let s = s.send(())?;

            endpoint_s_7(s, loops)
        }
        _ => {
            let s = choose_mpst_s_to_all!(s, Branching8fromStoC::Continue);

            let s = s.send(())?;

            endpoint_s_9(s, loops)
        }
    }
}

fn endpoint_s_9(s: EndpointS9, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching9fromCtoS::Loop(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;

            endpoint_s_9(s, loops)
        },
        Branching9fromCtoS::Continue(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;

            endpoint_s_10(s, loops)
        },
    })
}

fn endpoint_s_10(s: EndpointS10, loops: i32) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching10fromCtoS::Data(s) => {
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;

            endpoint_s_10(s, loops - 1)
        },
        Branching10fromCtoS::Subject(s) => {
            let (_, s) = s.recv()?;
            let (_, s) = s.recv()?;

            endpoint_s_10(s, loops - 1)
        },
        Branching10fromCtoS::End(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(())?;

            endpoint_s_7(s, loops - 1)
        },
    })
}

/////////////////////////

fn aux() {
    let (thread_c, thread_s) = fork_mpst(black_box(endpoint_c_init), black_box(endpoint_s_init));

    thread_c.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

pub fn smtp(c: &mut Criterion) {
    c.bench_function("SMTP baking", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = smtp,
}

criterion_main! {
    bench
}
