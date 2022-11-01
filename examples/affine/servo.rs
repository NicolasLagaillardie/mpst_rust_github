use mpstthree::baker;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::end::RoleEnd;

use std::error::Error;

baker!(
    "rec_and_cancel",
    MeshedChannels,
    Constellation,
    Layout,
    Script
);

// Payload types

struct GetCurrentState;
struct DocumentLoading;
struct WebFontLoaded;
struct GetWebPageLoadState;
struct OutstandingWebFonts;

// Binary types

// Constellation thread

type CtoL = Send<GetWebPageLoadState, Recv<OutstandingWebFonts, End>>;
type CtoS = Send<GetCurrentState, Recv<DocumentLoading, End>>;

//Layout thread

type LtoC = Recv<GetWebPageLoadState, Send<OutstandingWebFonts, End>>;
type LtoS = Send<WebFontLoaded, End>;

// Script thread

type StoC = Recv<GetCurrentState, Send<DocumentLoading, End>>;
type StoL = Recv<WebFontLoaded, End>;

// Orderings

type OrderingC = RoleScript<RoleScript<RoleLayout<RoleLayout<RoleEnd>>>>;

type OrderingL = RoleScript<RoleConstellation<RoleConstellation<RoleEnd>>>;

type OrderingS = RoleConstellation<RoleConstellation<RoleLayout<RoleEnd>>>;

// MeshedChannels

type EndpointC = MeshedChannels<CtoL, CtoS, OrderingC, NameConstellation>;

type EndpointL = MeshedChannels<LtoC, LtoS, OrderingL, NameLayout>;

type EndpointS = MeshedChannels<StoC, StoL, OrderingS, NameScript>;

// Functions

/////////////////////////

// Functions related to endpoints
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = s.send(GetCurrentState {})?;

    let (_, s) = s.recv()?;

    // To "process" the information
    let s = s.send(GetWebPageLoadState {})?;

    let (_, s) = s.recv()?;

    s.close()?;
    Ok(())
}

/////////////////////////

fn endpoint_l(s: EndpointL) -> Result<(), Box<dyn Error>> {
    let s = s.send(WebFontLoaded {})?;

    let (_, s) = s.recv()?;

    let s = s.send(OutstandingWebFonts {})?;

    s.close()?;
    Ok(())
}

/////////////////////////

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    let s = s.send(DocumentLoading {})?;

    let (_, s) = s.recv()?;

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
