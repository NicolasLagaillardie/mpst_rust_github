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

type CtoL = Send<GetWebPageLoadState, Recv<OutstandingWebFonts, End>>;
type CtoS = Send<GetCurrentState, Recv<DocumentLoading, End>>;

type LtoC = Recv<GetWebPageLoadState, Send<OutstandingWebFonts, End>>;
type LtoS = Send<WebFontLoaded, End>;

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

/////////////////////////

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = s.send(GetCurrentState {})?;
    let (_, s) = s.recv()?;
    let s = s.send(GetWebPageLoadState {})?;
    let (_, s) = s.recv()?;
    s.close()
}

/////////////////////////

fn endpoint_l(s: EndpointL) -> Result<(), Box<dyn Error>> {
    let s = s.send(WebFontLoaded {})?;
    let (_, s) = s.recv()?;
    let s = s.send(OutstandingWebFonts {})?;
    s.close()
}

/////////////////////////

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    let s = s.send(DocumentLoading {})?;
    let (_, s) = s.recv()?;
    s.close()
}

/////////////////////////

fn main() {
    checking();

    let (thread_c, thread_l, thread_s) = fork_mpst(endpoint_c, endpoint_l, endpoint_s);

    assert!(thread_c.join().is_ok());
    assert!(thread_l.join().is_ok());
    assert!(thread_s.join().is_ok());
}

/////////////////////////

// Check for bottom-up approach
fn checking() {
    let (graphs, kmc) =
        mpstthree::checker_concat!("servo_8257_original", EndpointC, EndpointL, EndpointS).unwrap();

    println!(
        "graph C: {:?}",
        petgraph::dot::Dot::new(&graphs["RoleConstellation"])
    );
    println!("\n/////////////////////////\n");
    println!(
        "graph L: {:?}",
        petgraph::dot::Dot::new(&graphs["RoleLayout"])
    );
    println!("\n/////////////////////////\n");
    println!(
        "graph S: {:?}",
        petgraph::dot::Dot::new(&graphs["RoleScript"])
    );
    println!("\n/////////////////////////\n");
    println!("min kMC: {kmc:?}");
}
