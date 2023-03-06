use mpstthree::baker;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// Create roles and implementations
baker!(
    "rec_and_cancel",
    MeshedChannels,
    Client,
    ProxyOne,
    ProxyTwo,
    Server
);

// Payload types

// Request by the client
struct Request {
    method: str,
    version_protocol: str,
    header: str
}
// Response by the server
struct Response {
    version_protocol: str,
    status_code: str,
    status_message: str,
    header: str
}

struct Close;

// Binary types

// Open TCP communication
// Client thread
type ClientToProxyOneTCP = Send<OpenTCPConnectionByClient, End>;
type ClientToProxyTwoTCP = End; // No communications with ProxyTwo
type ClientToServerTCP = End; // No communications with Server

enum OpenTCPConnectionByClient {
    Success(
        MeshedChannels<
            Send<RequestByClient, End>,
            End,
            End,
            RoleProxyOne<RoleEnd>,
            NameClient,
        >
    ),
    Fail(
        MeshedChannels<
            Close,
            End,
            End,
            RoleEnd,
            NameClient,
        >
    )
}

//ProxyOne thread
type ProxyOneToClientTCP = <ClientToProxyOneTCP as Session>::Dual;
type ProxyOneToProxyTwoTCP = Send<OpenTCPConnectionByProxyOneTCP, End>;
type ProxyOneToServerTCP = End; // No communications with Server

// ProxyTwo thread
type ProxyTwoToClientTCP = <ClientToProxyTwoTCP as Session>::Dual; // No communications with Client
type ProxyTwoToProxyOneTCP = <ProxyOneToProxyTwoTCP as Session>::Dual;
type ProxyTwoToServerTCP = Send<OpenTCPConnectionByProxyTwoTCP, End>;

// Server thread
type ServerToClientTCP = <ClientToServerTCP as Session>::Dual; // No communications with Client
type ServerToProxyOneTCP = <ProxyOneToServerTCP as Session>::Dual; // No communications with ProxyOne
type ServerToProxyTwoTCP = <ProxyTwoToServerTCP as Session>::Dual;



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
