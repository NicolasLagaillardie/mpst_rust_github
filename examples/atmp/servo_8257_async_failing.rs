use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

generate_atmp!(MeshedChannels, Constellation, Layout, Script);

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
    'a',
    0,
    true,
    5,
    true,
    ' ',
    RecvTimed<OutstandingWebFonts, 'a', 0, true, 5, true, ' ', End>,
>;
type CtoS = SendTimed<
    GetCurrentState,
    'a',
    0,
    true,
    5,
    true,
    ' ',
    RecvTimed<DocumentLoading, 'a', 0, true, 5, true, ' ', End>,
>;

//Layout thread

type LtoC = RecvTimed<
    GetWebPageLoadState,
    'a',
    0,
    true,
    5,
    true,
    ' ',
    SendTimed<OutstandingWebFonts, 'a', 0, true, 5, true, ' ', End>,
>;
type LtoS = SendTimed<WebFontLoaded, 'a', 0, true, 5, true, ' ', End>;

// Script thread

type StoC = RecvTimed<
    GetCurrentState,
    'a',
    0,
    true,
    5,
    true,
    ' ',
    SendTimed<DocumentLoading, 'a', 0, true, 5, true, ' ', End>,
>;
type StoL = RecvTimed<WebFontLoaded, 'a', 0, true, 5, true, ' ', End>;

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

    let (_, s) = s.recv(all_clocks)?;

    // To "process" the information
    sleep(Duration::from_secs(1));

    let s = s.send(GetWebPageLoadState {}, all_clocks)?;

    let (_, s) = s.recv(all_clocks)?;
    s.close()
}

/////////////////////////

fn endpoint_l(s: EndpointL, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    sleep(Duration::from_secs(1));

    let s = s.send(WebFontLoaded {}, all_clocks)?;

    let (_, s) = s.recv(all_clocks)?;

    let s = s.send(OutstandingWebFonts {}, all_clocks)?;

    s.close()
}

/////////////////////////

fn endpoint_s(s: EndpointS, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    sleep(Duration::from_secs(1));

    let (_, s) = s.recv(all_clocks)?;

    let s = s.send(DocumentLoading {}, all_clocks)?;

    let (_, s) = s.recv(all_clocks)?;

    // Simulate new fonts loaded AFTER the script thread said the page was loaded
    sleep(Duration::from_secs(2));

    s.close()
}

////////////////////////////////////////

fn main() {
    let (thread_c, thread_l, thread_s) = fork_mpst(endpoint_c, endpoint_l, endpoint_s);

    thread_c.join().unwrap();
    thread_l.join().unwrap();
    thread_s.join().unwrap();
}
