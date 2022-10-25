use mpstthree::baker_timed;
use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

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
    RecvTimed<OutstandingWebFonts, End, 'a', 0, true, 1, true, false>,
    'a',
    0,
    true,
    1,
    true,
    false,
>;
type CtoS = SendTimed<
    GetCurrentState,
    RecvTimed<DocumentLoading, End, 'a', 0, true, 1, true, false>,
    'a',
    0,
    true,
    1,
    true,
    false,
>;

//Layout thread

type LtoC = RecvTimed<
    GetWebPageLoadState,
    SendTimed<OutstandingWebFonts, End, 'a', 0, true, 1, true, false>,
    'a',
    0,
    true,
    1,
    true,
    false,
>;
type LtoS = SendTimed<WebFontLoaded, End, 'a', 0, true, 1, true, false>;

// Script thread

type StoC = RecvTimed<
    GetCurrentState,
    SendTimed<DocumentLoading, End, 'a', 0, true, 1, true, false>,
    'a',
    0,
    true,
    1,
    true,
    false,
>;
type StoL = RecvTimed<WebFontLoaded, End, 'a', 0, true, 1, true, false>;

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

    let s = s.send(GetCurrentState {}, all_clocks)?;

    let (_, s) = s.recv(all_clocks)?;

    // To "process" the information
    let s = s.send(GetWebPageLoadState {}, all_clocks)?;

    let (_, s) = s.recv(all_clocks)?;

    s.close()?;
    Ok(())
}

/////////////////////////

fn endpoint_l(s: EndpointL, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(WebFontLoaded {}, all_clocks)?;

    let (_, s) = s.recv(all_clocks)?;

    let s = s.send(OutstandingWebFonts {}, all_clocks)?;

    s.close()?;
    Ok(())
}

/////////////////////////

fn endpoint_s(s: EndpointS, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv(all_clocks)?;
    let s = s.send(DocumentLoading {}, all_clocks)?;

    let (_, s) = s.recv(all_clocks)?;

    s.close()?;
    Ok(())
}

////////////////////////////////////////

fn main() {
    let (thread_c, thread_l, thread_s) = fork_mpst(endpoint_c, endpoint_l, endpoint_s);

    assert!(thread_c.join().is_ok());
    assert!(thread_l.join().is_ok());
    assert!(thread_s.join().is_ok());
}