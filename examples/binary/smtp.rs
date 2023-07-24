#![allow(clippy::type_complexity)]

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::{choose, offer};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::thread::spawn;

// Types
// Step 0
// C
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;
type EndpointC0 = Recv<(), Choose0fromCtoS>;
// S
enum Branching0fromCtoS {
    Continue(Recv<(), Recv<(), Choose1fromStoC>>),
    Quit(Recv<(), End>),
}
type Offer0fromCtoS = <Choose0fromCtoS as Session>::Dual;
type EndpointS0 = Send<(), Offer0fromCtoS>;
// Step 1
// C
enum Branching1fromStoC {
    Continue(Recv<(), Choose2fromCtoS>),
    Loop(Recv<(), Recv<(), Offer1fromStoC>>),
}
type Offer1fromStoC = <Choose1fromStoC as Session>::Dual;
type EndpointC1 = Offer1fromStoC;
// S
type Choose1fromStoC = Send<Branching1fromStoC, End>;
type EndpointS1 = Choose1fromStoC;
// Step 2
// C
type Choose2fromCtoS = Send<Branching2fromCtoS, End>;
type EndpointC2 = Choose2fromCtoS;
// S
enum Branching2fromCtoS {
    Continue(Recv<(), Send<(), Offer3fromCtoS>>),
    Quit(Recv<(), End>),
}
type Offer2fromCtoS = <Choose2fromCtoS as Session>::Dual;
type EndpointS2 = Offer2fromCtoS;
// Step 3
// C
type Choose3fromCtoS = Send<Branching3fromCtoS, End>;
type EndpointC3 = Choose3fromCtoS;
// S
enum Branching3fromCtoS {
    Continue(Recv<(), Choose4fromStoC>),
    Quit(Recv<(), End>),
}
type Offer3fromCtoS = <Choose3fromCtoS as Session>::Dual;
type EndpointS3 = Offer3fromCtoS;
// Step 4
// C
enum Branching4fromStoC {
    Continue(Recv<(), Choose5fromCtoS>),
    Loop(Recv<(), Offer4fromStoC>),
}
type Offer4fromStoC = <Choose4fromStoC as Session>::Dual;
type EndpointC4 = Offer4fromStoC;
// S
type Choose4fromStoC = Send<Branching4fromStoC, End>;
type EndpointS4 = Choose4fromStoC;
// Step 5
// C
type Choose5fromCtoS = Send<Branching5fromCtoS, End>;
type EndpointC5 = Choose5fromCtoS;
// S
enum Branching5fromCtoS {
    Continue(Recv<(), Choose6fromStoC>),
    Quit(Recv<(), End>),
}
type Offer5fromCtoS = <Choose5fromCtoS as Session>::Dual;
type EndpointS5 = Offer5fromCtoS;
// Step 6
// C
enum Branching6fromStoC {
    Continue(Recv<(), Choose7fromCtoS>),
    Loop(Recv<(), Offer6fromStoC>),
}
type Offer6fromStoC = <Choose6fromStoC as Session>::Dual;
type EndpointC6 = Offer6fromStoC;
// S
type Choose6fromStoC = Send<Branching6fromStoC, End>;
type EndpointS6 = Choose6fromStoC;
// Step 7
// C
type Choose7fromCtoS = Send<Branching7fromCtoS, End>;
type EndpointC7 = Choose7fromCtoS;
// S
enum Branching7fromCtoS {
    Continue(Recv<(), Choose8fromStoC>),
    Quit(Recv<(), End>),
}
type Offer7fromCtoS = <Choose7fromCtoS as Session>::Dual;
type EndpointS7 = Offer7fromCtoS;
// Step 8
// C
enum Branching8fromStoC {
    Continue(Recv<(), Choose9fromCtoS>),
    Loop(Recv<(), Choose7fromCtoS>),
}
type Offer8fromStoC = <Choose8fromStoC as Session>::Dual;
type EndpointC8 = Offer8fromStoC;
// S
type Choose8fromStoC = Send<Branching8fromStoC, End>;
type EndpointS8 = Choose8fromStoC;
// Step 9
// C
type Choose9fromCtoS = Send<Branching9fromCtoS, End>;
type EndpointC9 = Choose9fromCtoS;
// S
enum Branching9fromCtoS {
    Continue(Recv<(), Send<(), Recv<(), Recv<(), Offer10fromCtoS>>>>),
    Loop(Recv<(), Send<(), Offer9fromCtoS>>),
}
type Offer9fromCtoS = <Choose9fromCtoS as Session>::Dual;
type EndpointS9 = Offer9fromCtoS;
// Step 10
// C
type Choose10fromCtoS = Send<Branching10fromCtoS, End>;
type EndpointC10 = Choose10fromCtoS;
// S
enum Branching10fromCtoS {
    Data(Recv<(), Recv<(), Offer10fromCtoS>>),
    Subject(Recv<(), Recv<(), Offer10fromCtoS>>),
    End(Recv<(), Send<(), Offer7fromCtoS>>),
}
type Offer10fromCtoS = <Choose10fromCtoS as Session>::Dual;
type EndpointS10 = Offer10fromCtoS;

///////////////////////////








// Functions
fn endpoint_c_init(s: EndpointC0) -> Result<(), Box<dyn Error>> {
    endpoint_c_0(s, 100)
}

fn endpoint_c_0(s: EndpointC0, loops: i32) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;

    match loops {
        0 => {
            let s = choose!(s, Branching0fromCtoS::Quit);

            let s = send((), s);

            close(s)
        }
        _ => {
            let s = choose!(s, Branching0fromCtoS::Continue);

            let s = send((), s);
            let s = send((), s);

            endpoint_c_1(s, loops)
        }
    }
}

fn endpoint_c_1(s: EndpointC1, loops: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching1fromStoC::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_2(s, loops)
        },
        Branching1fromStoC::Loop(s) => {
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_c_1(s, loops)
        },
    })
}

fn endpoint_c_2(s: EndpointC2, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose!(s, Branching2fromCtoS::Quit);

            let s = send((), s);

            close(s)
        }
        _ => {
            let s = choose!(s, Branching2fromCtoS::Continue);

            let s = send((), s);
            let (_, s) = recv(s)?;

            endpoint_c_3(s, loops)
        }
    }
}

fn endpoint_c_3(s: EndpointC3, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose!(s, Branching3fromCtoS::Quit);

            let s = send((), s);

            close(s)
        }
        _ => {
            let s = choose!(s, Branching3fromCtoS::Continue);

            let s = send((), s);

            endpoint_c_4(s, loops)
        }
    }
}

fn endpoint_c_4(s: EndpointC4, loops: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching4fromStoC::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_5(s, loops)
        },
        Branching4fromStoC::Loop(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_4(s, loops)
        },
    })
}

fn endpoint_c_5(s: EndpointC5, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose!(s, Branching5fromCtoS::Quit);

            let s = send((), s);

            close(s)
        }
        _ => {
            let s = choose!(s, Branching5fromCtoS::Continue);

            let s = send((), s);

            endpoint_c_6(s, loops)
        }
    }
}

fn endpoint_c_6(s: EndpointC6, loops: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching6fromStoC::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_7(s, loops)
        },
        Branching6fromStoC::Loop(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_6(s, loops)
        },
    })
}

fn endpoint_c_7(s: EndpointC7, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose!(s, Branching7fromCtoS::Quit);

            let s = send((), s);

            close(s)
        }
        _ => {
            let s = choose!(s, Branching7fromCtoS::Continue);

            let s = send((), s);

            endpoint_c_8(s, loops)
        }
    }
}

fn endpoint_c_8(s: EndpointC8, loops: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching8fromStoC::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_9(s, loops)
        },
        Branching8fromStoC::Loop(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_7(s, loops)
        },
    })
}

fn endpoint_c_9(s: EndpointC9, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose!(s, Branching9fromCtoS::Loop);

            let s = send((), s);
            let (_, s) = recv(s)?;

            endpoint_c_9(s, loops)
        }
        _ => {
            let s = choose!(s, Branching9fromCtoS::Continue);

            let s = send((), s);
            let (_, s) = recv(s)?;
            let s = send((), s);
            let s = send((), s);

            endpoint_c_10(s, loops)
        }
    }
}

fn endpoint_c_10(s: EndpointC10, loops: i32) -> Result<(), Box<dyn Error>> {
    if loops == 0 {
        let s = choose!(s, Branching10fromCtoS::End);

        let s = send((), s);
        let (_, s) = recv(s)?;

        endpoint_c_7(s, loops)
    } else if loops % 2 == 1 {
        let s = choose!(s, Branching10fromCtoS::Subject);

        let s = send((), s);
        let s = send((), s);

        endpoint_c_10(s, loops - 1)
    } else {
        let s = choose!(s, Branching10fromCtoS::Data);

        let s = send((), s);
        let s = send((), s);

        endpoint_c_10(s, loops - 1)
    }
}

///
fn endpoint_s_init(s: EndpointS0) -> Result<(), Box<dyn Error>> {
    endpoint_s_0(s, 100)
}

fn endpoint_s_0(s: EndpointS0, loops: i32) -> Result<(), Box<dyn Error>> {
    let s = send((), s);

    offer!(s, {
        Branching0fromCtoS::Quit(s) => {
            let (_, s) = recv(s)?;

            close(s)
        },
        Branching0fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_s_1(s, loops)
        },
    })
}

fn endpoint_s_1(s: EndpointS1, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose!(s, Branching1fromStoC::Loop);

            let s = send((), s);
            let s = send((), s);

            endpoint_s_1(s, loops)
        }
        _ => {
            let s = choose!(s, Branching1fromStoC::Continue);

            let s = send((), s);

            endpoint_s_2(s, loops)
        }
    }
}

fn endpoint_s_2(s: EndpointS2, loops: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching2fromCtoS::Quit(s) => {
            let (_, s) = recv(s)?;

            close(s)
        },
        Branching2fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);

            endpoint_s_3(s, loops)
        },
    })
}

fn endpoint_s_3(s: EndpointS3, loops: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching3fromCtoS::Quit(s) => {
            let (_, s) = recv(s)?;

            close(s)
        },
        Branching3fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_s_4(s, loops)
        },
    })
}

fn endpoint_s_4(s: EndpointS4, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose!(s, Branching4fromStoC::Loop);

            let s = send((), s);

            endpoint_s_4(s, loops)
        }
        _ => {
            let s = choose!(s, Branching4fromStoC::Continue);

            let s = send((), s);

            endpoint_s_5(s, loops)
        }
    }
}

fn endpoint_s_5(s: EndpointS5, loops: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching5fromCtoS::Quit(s) => {
            let (_, s) = recv(s)?;

            close(s)
        },
        Branching5fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_s_6(s, loops)
        },
    })
}

fn endpoint_s_6(s: EndpointS6, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose!(s, Branching6fromStoC::Loop);

            let s = send((), s);

            endpoint_s_6(s, loops)
        }
        _ => {
            let s = choose!(s, Branching6fromStoC::Continue);

            let s = send((), s);

            endpoint_s_7(s, loops)
        }
    }
}

fn endpoint_s_7(s: EndpointS7, loops: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching7fromCtoS::Quit(s) => {
            let (_, s) = recv(s)?;

            close(s)
        },
        Branching7fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_s_8(s, loops)
        },
    })
}

fn endpoint_s_8(s: EndpointS8, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s = choose!(s, Branching8fromStoC::Loop);

            let s = send((), s);

            endpoint_s_7(s, loops)
        }
        _ => {
            let s = choose!(s, Branching8fromStoC::Continue);

            let s = send((), s);

            endpoint_s_9(s, loops)
        }
    }
}

fn endpoint_s_9(s: EndpointS9, loops: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching9fromCtoS::Loop(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);

            endpoint_s_9(s, loops)
        },
        Branching9fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_s_10(s, loops)
        },
    })
}

fn endpoint_s_10(s: EndpointS10, loops: i32) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching10fromCtoS::Data(s) => {
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_s_10(s, loops - 1)
        },
        Branching10fromCtoS::Subject(s) => {
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_s_10(s, loops - 1)
        },
        Branching10fromCtoS::End(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);

            endpoint_s_7(s, loops - 1)
        },
    })
}

////////////////////////////

fn main() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    let (thread, session) = fork_with_thread_id(binary_a);

    let session = send((), session);
    let session = send((), session);
    let session = send((), session);

    threads.push(thread);
    sessions.push(session);

    let main = spawn(move || {
        for _ in 0..LOOPS {
            let choice = thread_rng().gen_range(1..=2);

            if choice != 1 {
                sessions = sessions
                    .into_iter()
                    .map(|s| binary_up_b(s).unwrap())
                    .collect::<Vec<_>>();
            } else {
                sessions = sessions
                    .into_iter()
                    .map(|s| binary_failure_b(s).unwrap())
                    .collect::<Vec<_>>();
            }
        }

        sessions
            .into_iter()
            .for_each(|s| binary_close_b(s).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

static LOOPS: i32 = 100;
