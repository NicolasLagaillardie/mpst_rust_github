#![allow(clippy::type_complexity, dead_code)]
#![recursion_limit = "256"]

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::{choose, offer};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::thread::spawn;

////////////////////////////

type EndpointCInit = Recv<(), EndpointC0>;

type EndpointC0 = Recv<Branching0fromCtoS, End>;

type EndpointSInit = <EndpointCInit as Session>::Dual;

type EndpointS0 = <EndpointC0 as Session>::Dual;

enum Branching0fromCtoS {
    Continue(Recv<(), EndpointC1>),
    Quit(Recv<(), End>),
}

type EndpointC1 = Recv<Branching1fromCtoS, End>;

type EndpointS1 = <EndpointC1 as Session>::Dual;

enum Branching1fromCtoS {
    Continue(Recv<(), EndpointC2>),
    Loop(Recv<(), EndpointC1>),
}

type EndpointC2 = Recv<Branching2fromCtoS, End>;

type EndpointS2 = <EndpointC2 as Session>::Dual;

enum Branching2fromCtoS {
    Continue(Recv<(), Recv<(), EndpointC3>>),
    Quit(Recv<(), End>),
}

type EndpointC3 = Recv<Branching3fromCtoS, End>;

type EndpointS3 = <EndpointC3 as Session>::Dual;

enum Branching3fromCtoS {
    Continue(Recv<(), EndpointC4>),
    Quit(Recv<(), End>),
}

type EndpointC4 = Recv<Branching4fromCtoS, End>;

type EndpointS4 = <EndpointC4 as Session>::Dual;

enum Branching4fromCtoS {
    Continue(Recv<(), EndpointC5>),
    Loop(Recv<(), EndpointC4>),
}

type EndpointC5 = Recv<Branching5fromCtoS, End>;

type EndpointS5 = <EndpointC5 as Session>::Dual;

enum Branching5fromCtoS {
    Continue(Recv<(), EndpointC6>),
    Quit(Recv<(), End>),
}

type EndpointC6 = Recv<Branching6fromCtoS, End>;

type EndpointS6 = <EndpointC6 as Session>::Dual;

enum Branching6fromCtoS {
    Continue(Recv<(), EndpointC7>),
    Loop(Recv<(), EndpointC6>),
}

type EndpointC7 = Recv<Branching7fromCtoS, End>;

type EndpointS7 = <EndpointC7 as Session>::Dual;

enum Branching7fromCtoS {
    Continue(Recv<(), EndpointC8>),
    Quit(Recv<(), End>),
}

type EndpointC8 = Recv<Branching8fromCtoS, End>;

type EndpointS8 = <EndpointC8 as Session>::Dual;

enum Branching8fromCtoS {
    Continue(Recv<(), EndpointC9>),
    Loop(Recv<(), EndpointC7>),
}

type EndpointC9 = Recv<Branching9fromCtoS, End>;

type EndpointS9 = <EndpointC9 as Session>::Dual;

enum Branching9fromCtoS {
    Continue(Recv<(), Recv<(), Recv<(), Recv<(), EndpointC10>>>>),
    Loop(Recv<(), Recv<(), EndpointC9>>),
}

type EndpointC10 = Recv<Branching10fromCtoS, End>;

type EndpointS10 = <EndpointC10 as Session>::Dual;

enum Branching10fromCtoS {
    Data(Recv<(), Recv<(), EndpointC10>>),
    Subject(Recv<(), Recv<(), EndpointC10>>),
    End(Recv<(), Recv<(), EndpointC7>>),
}

///////////////////////////

fn binary_s_quit_0(s: EndpointSInit) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Quit, s);
    let s = send((), s);
    close(s)
}

fn binary_s_x_0(s: EndpointSInit) -> Result<EndpointS1, Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Loop, s);
    let s = send((), s);
    Ok(s)
}

fn binary_s_quit_2(s: EndpointSInit) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Quit, s);
    let s = send((), s);
    close(s)
}

fn binary_s_quit_3(s: EndpointSInit) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Quit, s);
    let s = send((), s);
    close(s)
}

fn binary_s_x_4(s: EndpointSInit) -> Result<EndpointS4, Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Continue, s);
    let s: Send<Branching4fromCtoS, End> = send((), s);
    let s = choose!(Branching4fromCtoS::Loop, s);
    let s = send((), s);
    Ok(s)
}

fn binary_s_quit_5(s: EndpointSInit) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Continue, s);
    let s: Send<Branching4fromCtoS, End> = send((), s);
    let s = choose!(Branching4fromCtoS::Continue, s);
    let s: Send<Branching5fromCtoS, End> = send((), s);
    let s = choose!(Branching5fromCtoS::Quit, s);
    let s = send((), s);
    close(s)
}

fn binary_s_x_6(s: EndpointSInit) -> Result<EndpointS6, Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Continue, s);
    let s: Send<Branching4fromCtoS, End> = send((), s);
    let s = choose!(Branching4fromCtoS::Continue, s);
    let s: Send<Branching5fromCtoS, End> = send((), s);
    let s = choose!(Branching5fromCtoS::Continue, s);
    let s: Send<Branching6fromCtoS, End> = send((), s);
    let s = choose!(Branching6fromCtoS::Loop, s);
    let s = send((), s);
    Ok(s)
}

fn binary_s_quit_7(s: EndpointSInit) -> Result<(), Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Continue, s);
    let s: Send<Branching4fromCtoS, End> = send((), s);
    let s = choose!(Branching4fromCtoS::Continue, s);
    let s: Send<Branching5fromCtoS, End> = send((), s);
    let s = choose!(Branching5fromCtoS::Continue, s);
    let s: Send<Branching6fromCtoS, End> = send((), s);
    let s = choose!(Branching6fromCtoS::Continue, s);
    let s: Send<Branching7fromCtoS, End> = send((), s);
    let s = choose!(Branching7fromCtoS::Quit, s);
    let s = send((), s);
    close(s)
}

fn binary_s_x_8(s: EndpointSInit) -> Result<EndpointS7, Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Continue, s);
    let s: Send<Branching4fromCtoS, End> = send((), s);
    let s = choose!(Branching4fromCtoS::Continue, s);
    let s: Send<Branching5fromCtoS, End> = send((), s);
    let s = choose!(Branching5fromCtoS::Continue, s);
    let s: Send<Branching6fromCtoS, End> = send((), s);
    let s = choose!(Branching6fromCtoS::Continue, s);
    let s: Send<Branching7fromCtoS, End> = send((), s);
    let s = choose!(Branching7fromCtoS::Continue, s);
    let s: Send<Branching8fromCtoS, End> = send((), s);
    let s = choose!(Branching8fromCtoS::Loop, s);
    let s = send((), s);
    Ok(s)
}

fn binary_s_x_9(s: EndpointSInit) -> Result<EndpointS9, Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Continue, s);
    let s: Send<Branching4fromCtoS, End> = send((), s);
    let s = choose!(Branching4fromCtoS::Continue, s);
    let s: Send<Branching5fromCtoS, End> = send((), s);
    let s = choose!(Branching5fromCtoS::Continue, s);
    let s: Send<Branching6fromCtoS, End> = send((), s);
    let s = choose!(Branching6fromCtoS::Continue, s);
    let s: Send<Branching7fromCtoS, End> = send((), s);
    let s = choose!(Branching7fromCtoS::Continue, s);
    let s: Send<Branching8fromCtoS, End> = send((), s);
    let s = choose!(Branching8fromCtoS::Continue, s);
    let s: Send<Branching9fromCtoS, End> = send((), s);
    let s = choose!(Branching9fromCtoS::Loop, s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn binary_s_end_10(s: EndpointSInit) -> Result<EndpointS7, Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Continue, s);
    let s: Send<Branching4fromCtoS, End> = send((), s);
    let s = choose!(Branching4fromCtoS::Continue, s);
    let s: Send<Branching5fromCtoS, End> = send((), s);
    let s = choose!(Branching5fromCtoS::Continue, s);
    let s: Send<Branching6fromCtoS, End> = send((), s);
    let s = choose!(Branching6fromCtoS::Continue, s);
    let s: Send<Branching7fromCtoS, End> = send((), s);
    let s = choose!(Branching7fromCtoS::Continue, s);
    let s: Send<Branching8fromCtoS, End> = send((), s);
    let s = choose!(Branching8fromCtoS::Continue, s);
    let s: Send<Branching9fromCtoS, End> = send((), s);
    let s = choose!(Branching9fromCtoS::Continue, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s: Send<Branching10fromCtoS, End> = send((), s);
    let s = choose!(Branching10fromCtoS::End, s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn binary_s_data_10(s: EndpointSInit) -> Result<EndpointS10, Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Continue, s);
    let s: Send<Branching4fromCtoS, End> = send((), s);
    let s = choose!(Branching4fromCtoS::Continue, s);
    let s: Send<Branching5fromCtoS, End> = send((), s);
    let s = choose!(Branching5fromCtoS::Continue, s);
    let s: Send<Branching6fromCtoS, End> = send((), s);
    let s = choose!(Branching6fromCtoS::Continue, s);
    let s: Send<Branching7fromCtoS, End> = send((), s);
    let s = choose!(Branching7fromCtoS::Continue, s);
    let s: Send<Branching8fromCtoS, End> = send((), s);
    let s = choose!(Branching8fromCtoS::Continue, s);
    let s: Send<Branching9fromCtoS, End> = send((), s);
    let s = choose!(Branching9fromCtoS::Continue, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s: Send<Branching10fromCtoS, End> = send((), s);
    let s = choose!(Branching10fromCtoS::Data, s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn binary_s_subject_10(s: EndpointSInit) -> Result<EndpointS10, Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Continue, s);
    let s: Send<Branching4fromCtoS, End> = send((), s);
    let s = choose!(Branching4fromCtoS::Continue, s);
    let s: Send<Branching5fromCtoS, End> = send((), s);
    let s = choose!(Branching5fromCtoS::Continue, s);
    let s: Send<Branching6fromCtoS, End> = send((), s);
    let s = choose!(Branching6fromCtoS::Continue, s);
    let s: Send<Branching7fromCtoS, End> = send((), s);
    let s = choose!(Branching7fromCtoS::Continue, s);
    let s: Send<Branching8fromCtoS, End> = send((), s);
    let s = choose!(Branching8fromCtoS::Continue, s);
    let s: Send<Branching9fromCtoS, End> = send((), s);
    let s = choose!(Branching9fromCtoS::Continue, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s: Send<Branching10fromCtoS, End> = send((), s);
    let s = choose!(Branching10fromCtoS::Subject, s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn binary_s_init(s: EndpointSInit) -> Result<EndpointS10, Box<dyn Error>> {
    let s = send((), s);
    let s = choose!(Branching0fromCtoS::Continue, s);
    let s: Send<Branching1fromCtoS, End> = send((), s);
    let s = choose!(Branching1fromCtoS::Continue, s);
    let s: Send<Branching2fromCtoS, End> = send((), s);
    let s = choose!(Branching2fromCtoS::Continue, s);
    let s = send((), s);
    let s: Send<Branching3fromCtoS, End> = send((), s);
    let s = choose!(Branching3fromCtoS::Continue, s);
    let s: Send<Branching4fromCtoS, End> = send((), s);
    let s = choose!(Branching4fromCtoS::Continue, s);
    let s: Send<Branching5fromCtoS, End> = send((), s);
    let s = choose!(Branching5fromCtoS::Continue, s);
    let s: Send<Branching6fromCtoS, End> = send((), s);
    let s = choose!(Branching6fromCtoS::Continue, s);
    let s: Send<Branching7fromCtoS, End> = send((), s);
    let s = choose!(Branching7fromCtoS::Continue, s);
    let s: Send<Branching8fromCtoS, End> = send((), s);
    let s = choose!(Branching8fromCtoS::Continue, s);
    let s: Send<Branching9fromCtoS, End> = send((), s);
    let s = choose!(Branching9fromCtoS::Continue, s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn recurs_data_s_10(s: EndpointS10) -> Result<EndpointS10, Box<dyn Error>> {
    let s = choose!(Branching10fromCtoS::Data, s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn recurs_subject_s_10(s: EndpointS10) -> Result<EndpointS10, Box<dyn Error>> {
    let s = choose!(Branching10fromCtoS::Subject, s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn close_s_10(s: EndpointS10) -> Result<(), Box<dyn Error>> {
    let s = choose!(Branching10fromCtoS::End, s);
    let s = send((), s);
    let s: Send<Branching7fromCtoS, End> = send((), s);
    let s = choose!(Branching7fromCtoS::Quit, s);
    let s = send((), s);
    close(s)
}

////////////////////////////

fn endpoint_c_init(s: EndpointCInit) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv(s)?;

    endpoint_c_0(s)
}

fn endpoint_c_0(s: EndpointC0) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching0fromCtoS::Quit(s) => {
            let (_, s) = recv(s)?;

            close(s)
        },
        Branching0fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_1(s)
        },
    })
}

fn endpoint_c_1(s: EndpointC1) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching1fromCtoS::Loop(s) => {

            let (_, s) = recv(s)?;

            endpoint_c_1(s)
        },
        Branching1fromCtoS::Continue(s) => {

            let (_, s) = recv(s)?;

            endpoint_c_2(s)
        },
    })
}

fn endpoint_c_2(s: EndpointC2) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching2fromCtoS::Quit(s) => {
            let (_, s) = recv(s)?;

            close(s)
        },
        Branching2fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_c_3(s)
        },
    })
}

fn endpoint_c_3(s: EndpointC3) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching3fromCtoS::Quit(s) => {
            let (_, s) = recv(s)?;

            close(s)
        },
        Branching3fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_4(s)
        },
    })
}

fn endpoint_c_4(s: EndpointC4) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching4fromCtoS::Loop(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_4(s)
        },
        Branching4fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_5(s)
        },
    })
}

fn endpoint_c_5(s: EndpointC5) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching5fromCtoS::Quit(s) => {
            let (_, s) = recv(s)?;

            close(s)
        },
        Branching5fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_6(s)
        },
    })
}

fn endpoint_c_6(s: EndpointC6) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching6fromCtoS::Loop(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_6(s)
        },
        Branching6fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_7(s)
        },
    })
}

fn endpoint_c_7(s: EndpointC7) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching7fromCtoS::Quit(s) => {
            let (_, s) = recv(s)?;

            close(s)
        },
        Branching7fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_8(s)
        },
    })
}

fn endpoint_c_8(s: EndpointC8) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching8fromCtoS::Loop(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_7(s)
        },
        Branching8fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;

            endpoint_c_9(s)
        },
    })
}

fn endpoint_c_9(s: EndpointC9) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching9fromCtoS::Loop(s) => {
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_c_9(s)
        },
        Branching9fromCtoS::Continue(s) => {
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_c_10(s)
        },
    })
}

fn endpoint_c_10(s: EndpointC10) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        Branching10fromCtoS::Data(s) => {
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_c_10(s)
        },
        Branching10fromCtoS::Subject(s) => {
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_c_10(s)
        },
        Branching10fromCtoS::End(s) => {
            let (_, s) = recv(s)?;
            let (_, s) = recv(s)?;

            endpoint_c_7(s)
        },
    })
}

////////////////////////////

fn main() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    let (thread, session) = fork_with_thread_id(endpoint_c_init);

    let session = binary_s_init(session).unwrap();

    threads.push(thread);
    sessions.push(session);

    let main = spawn(move || {
        for _ in 0..LOOPS {
            let choice = thread_rng().gen_range(1..=2);

            if choice != 1 {
                sessions = sessions
                    .into_iter()
                    .map(|s| recurs_data_s_10(s).unwrap())
                    .collect::<Vec<_>>();
            } else {
                sessions = sessions
                    .into_iter()
                    .map(|s| recurs_subject_s_10(s).unwrap())
                    .collect::<Vec<_>>();
            }
        }

        sessions.into_iter().for_each(|s| close_s_10(s).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

static LOOPS: i32 = 100;
