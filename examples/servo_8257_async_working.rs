// Test for affine timed protocols
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

type OrderingC = RoleLayout<RoleLayout<RoleScript<RoleScript<RoleEnd>>>>;

type OrderingL = RoleConstellation<RoleConstellation<RoleScript<RoleEnd>>>;

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

    // To "process" the information
    sleep(Duration::from_secs(1));

    let s = s.send(GetCurrentState {}, all_clocks)?;
    println!("send GetCurrentState from C");

    let (_, s) = s.recv(all_clocks)?;
    println!("recv DocumentLoading on C");
    println!(
        "C: Current page loaded at {:?} s",
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

    let (_, s) = s.recv(all_clocks)?;
    println!("recv GetWebPageLoadState on L");

    let s = s.send(OutstandingWebFonts {}, all_clocks)?;
    println!("send OutstandingWebFonts from L");

    let s = s.send(WebFontLoaded {}, all_clocks)?;
    println!("send WebFontLoaded from L");
    println!(
        "L: new font loaded at {:?} s",
        all_clocks.get(&'a').unwrap().elapsed().as_secs()
    );

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

pub fn main() {
    let (thread_c, thread_l, thread_s) = fork_mpst(endpoint_c, endpoint_l, endpoint_s);

    assert!(thread_c.join().is_ok());
    assert!(thread_l.join().is_ok());
    assert!(thread_s.join().is_ok());
}
