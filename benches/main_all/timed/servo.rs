#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    dead_code
)]

use criterion::{black_box, Criterion};

use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

baker_timed!(MeshedChannels, Constellation, Layout, Script);

// Payload types

struct GetCurrentState;
struct DocumentLoading;
struct WebFontLoaded;
struct GetWebPageLoadState;
struct OutstandingWebFonts;

// Binary types

// Constellation thread

type CtoL = SendTimed<
    GetWebPageLoadState,
    RecvTimed<OutstandingWebFonts, End, 'a', 0, true, 5, true, false>,
    'a',
    0,
    true,
    5,
    true,
    false,
>;
type CtoS = SendTimed<
    GetCurrentState,
    RecvTimed<DocumentLoading, End, 'a', 0, true, 5, true, false>,
    'a',
    0,
    true,
    5,
    true,
    false,
>;

//Layout thread

type LtoC = RecvTimed<
    GetWebPageLoadState,
    SendTimed<OutstandingWebFonts, End, 'a', 0, true, 5, true, false>,
    'a',
    0,
    true,
    5,
    true,
    false,
>;
type LtoS = SendTimed<WebFontLoaded, End, 'a', 0, true, 5, true, false>;

// Script thread

type StoC = RecvTimed<
    GetCurrentState,
    SendTimed<DocumentLoading, End, 'a', 0, true, 5, true, false>,
    'a',
    0,
    true,
    5,
    true,
    false,
>;
type StoL = RecvTimed<WebFontLoaded, End, 'a', 0, true, 5, true, false>;

// Orderings

type OrderingC = RoleScript<RoleScript<RoleLayout<RoleLayout<RoleEnd>>>>;

type OrderingL = RoleScript<RoleConstellation<RoleConstellation<RoleEnd>>>;

type OrderingS = RoleConstellation<RoleConstellation<RoleLayout<RoleEnd>>>;

// MeshedChannels

type EndpointC = MeshedChannels<CtoL, CtoS, OrderingC, NameConstellation>;

type EndpointL = MeshedChannels<LtoC, LtoS, OrderingL, NameLayout>;

type EndpointS = MeshedChannels<StoC, StoL, OrderingS, NameScript>;

/////////////////////////

// Functions related to endpoints
fn endpoint_c(s: EndpointC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    sleep(Duration::from_secs(1));

    let s = s.send(GetCurrentState {}, all_clocks)?;
    println!("send GetCurrentState from C");

    let (_, s) = s.recv(all_clocks)?;
    println!("recv DocumentLoading on C");
    println!(
        "C: Current page loaded at {:?} s",
        all_clocks.get(&'a').unwrap().elapsed().as_secs()
    );

    // To "process" the information
    sleep(Duration::from_secs(1));

    let s = s.send(GetWebPageLoadState {}, all_clocks)?;
    println!("send GetWebPageLoadState from C");

    let (_, s) = s.recv(all_clocks)?;
    println!("recv OutstandingWebFonts on C");
    println!(
        "C: OutstandingWebFonts = 0 at {:?} s",
        all_clocks.get(&'a').unwrap().elapsed().as_secs()
    );
    println!(
        "C: Fonts are loaded at {:?} s",
        all_clocks.get(&'a').unwrap().elapsed().as_secs()
    );

    s.close()?;
    println!("C closed");
    Ok(())
}

/////////////////////////

fn endpoint_l(s: EndpointL, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    sleep(Duration::from_secs(1));

    let s = s.send(WebFontLoaded {}, all_clocks)?;
    println!("send WebFontLoaded from L");
    println!(
        "L: new font loaded at {:?} s",
        all_clocks.get(&'a').unwrap().elapsed().as_secs()
    );

    let (_, s) = s.recv(all_clocks)?;
    println!("recv GetWebPageLoadState on L");

    let s = s.send(OutstandingWebFonts {}, all_clocks)?;
    println!("send OutstandingWebFonts from L");

    s.close()?;
    println!("L closed");
    Ok(())
}

/////////////////////////

fn endpoint_s(s: EndpointS, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    sleep(Duration::from_secs(1));

    let (_, s) = s.recv(all_clocks)?;
    println!("recv GetCurrentState on S");

    let s = s.send(DocumentLoading {}, all_clocks)?;
    println!("send DocumentLoading from S");

    let (_, s) = s.recv(all_clocks)?;
    println!("recv WebFontLoaded on S");

    // Simulate new fonts loaded AFTER the script thread said the page was loaded
    println!("S: WebFontLoaded = false");
    println!(
        "S: starts redrawing the page at {:?} s",
        all_clocks.get(&'a').unwrap().elapsed().as_secs()
    );
    sleep(Duration::from_secs(2));
    println!(
        "S: finishes redrawing the page at {:?} s",
        all_clocks.get(&'a').unwrap().elapsed().as_secs()
    );

    s.close()?;
    println!("S closed");
    Ok(())
}

////////////////////////////////////////

fn all_mpst() {
    let (thread_c, thread_l, thread_s) = fork_mpst(black_box(endpoint_c), black_box(endpoint_l), black_box(endpoint_s));

    thread_c.join().unwrap();
    thread_l.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

pub fn servo_main(c: &mut Criterion) {
    c.bench_function("Travel Servo", |b| b.iter(all_mpst));
}
