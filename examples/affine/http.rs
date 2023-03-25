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
// We embed the method directly
// in the label of the message
struct OpenTCPConnection {
    // method: String,
    version_protocol: String,
    header: String,
}
struct RequestGet {
    // method: String,
    version_protocol: String,
    header: String,
}
struct RequestPost {
    // method: String,
    version_protocol: String,
    header: String,
}
struct RequestPut {
    // method: String,
    version_protocol: String,
    header: String,
}

// Response by the server
// We embed the status directly
// in the label of the message
struct Response200 {
    version_protocol: String,
    // status_code: String,
    status_message: String,
    header: String,
}
struct Response404 {
    version_protocol: String,
    // status_code: String,
    status_message: String,
    header: String,
}
struct Response418 {
    version_protocol: String,
    // status_code: String,
    status_message: String,
    header: String,
}

struct Fail;

struct Close;

// Binary types

// Client thread
// Try to open TCP communication
type ClientToProxyOneTCP = Send<OpenTCPConnection, End>; // Send OpenTCPConnection to ProxyOne
type ClientToProxyTwoTCP = End; // No communications with ProxyTwo
type ClientToServerTCP = Recv<ForwardOpenTCPConnectionbyProxyOne, End>; // Receive choice from Server

// For ProxyOne
enum RequestByClientToProxyOne {
    RequestGet(
        MeshedChannels<
            Recv<RequestGet, End>,                 // Receive request from Client
            Send<RequestGet, End>,                 // Forward request to ProxyTwo
            Recv<ResponseByServerToProxyOne, End>, // Receive choice from Server
            RoleClient<RoleProxyTwo<RoleServer<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
    RequestPut(
        MeshedChannels<
            Recv<RequestPut, End>,                 // Receive request from Client
            Send<RequestPut, End>,                 // Forward request to ProxyTwo
            Recv<ResponseByServerToProxyOne, End>, // Receive choice from Server
            RoleClient<RoleProxyTwo<RoleServer<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
    RequestPost(
        MeshedChannels<
            Recv<RequestPost, End>,                // Receive request from Client
            Send<RequestPost, End>,                // Forward request to ProxyTwo
            Recv<ResponseByServerToProxyOne, End>, // Receive choice from Server
            RoleClient<RoleProxyTwo<RoleServer<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
    Close(
        MeshedChannels<
            Recv<Close, End>, // Receive Close from Client
            Send<Close, End>, // Forward Close to ProxyTwo
            End,
            RoleClient<RoleProxyTwo<RoleEnd>>,
            NameProxyOne,
        >,
    ),
}

// For ProxyTwo
enum RequestByClientToProxyTwo {
    RequestGet(
        MeshedChannels<
            End,
            Recv<RequestGet, End>, // Receive request from ProxyOne
            Send<RequestGet, Recv<ResponseByServerToProxyOne, End>>, // Forward request to Server and receive choice from Server
            RoleProxyOne<RoleServer<RoleServer<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    RequestPut(
        MeshedChannels<
            End,
            Recv<RequestPut, End>, // Receive request from ProxyOne
            Send<RequestPut, Recv<ResponseByServerToProxyOne, End>>, // Forward request to Server and receive choice from Server
            RoleProxyOne<RoleServer<RoleServer<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    RequestPost(
        MeshedChannels<
            End,
            Recv<RequestPost, End>, // Receive request from ProxyOne
            Send<RequestPost, Recv<ResponseByServerToProxyOne, End>>, // Forward request to Server and receive choice from Server
            RoleProxyOne<RoleServer<RoleServer<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    Close(
        MeshedChannels<
            End,
            Recv<Close, End>, // Receive Close from ProxyOne
            Send<Close, End>, // Forward Close to Server
            RoleProxyOne<RoleServer<RoleEnd>>,
            NameProxyTwo,
        >,
    ),
}

// For Server
enum RequestByClientToServer {
    RequestGet(
        MeshedChannels<
            Send<ResponseByServerToClient, End>,   // Send choice to Client
            Send<ResponseByServerToProxyOne, End>, // Send choice to ProxyOne
            Recv<RequestGet, Send<ResponseByServerToProxyTwo, End>>, // Receive request from ProxyTwo and send choice to ProxyTwo
            RoleProxyTwo<RoleClient<RoleProxyOne<RoleProxyTwo<RoleEnd>>>>,
            NameServer,
        >,
    ),
    RequestPut(
        MeshedChannels<
            Send<ResponseByServerToClient, End>,   // Send choice to Client
            Send<ResponseByServerToProxyOne, End>, // Send choice to ProxyOne
            Recv<RequestPut, Send<ResponseByServerToProxyTwo, End>>, // Receive request from ProxyTwo and send choice to ProxyTwo
            RoleProxyTwo<RoleClient<RoleProxyOne<RoleProxyTwo<RoleEnd>>>>,
            NameServer,
        >,
    ),
    RequestPost(
        MeshedChannels<
            Send<ResponseByServerToClient, End>,   // Send choice to Client
            Send<ResponseByServerToProxyOne, End>, // Send choice to ProxyOne
            Recv<RequestPost, Send<ResponseByServerToProxyTwo, End>>, // Receive request from ProxyTwo and send choice to ProxyTwo
            RoleProxyTwo<RoleClient<RoleProxyOne<RoleProxyTwo<RoleEnd>>>>,
            NameServer,
        >,
    ),
    Close(
        MeshedChannels<
            End,
            End,
            Recv<Close, End>, // Receive Close from ProxyTwo
            RoleProxyTwo<RoleEnd>,
            NameServer,
        >,
    ),
}

// ProxyOne thread
// Open TCP communication by Client
type ProxyOneToClientTCP = Recv<OpenTCPConnection, End>; // Receive OpenTCPConnection from Client
type ProxyOneToProxyTwoTCP = Send<OpenTCPConnection, End>; // Forward OpenTCPConnection to ProxyTwo
type ProxyOneToServerTCP = Recv<OpenTCPConnectionByServerToProxyOne, End>; // Receive choice from Server

// ProxyTwo thread
type ProxyTwoToClientTCP = End; // No communication with Client
type ProxyTwoToProxyOneTCP = Recv<OpenTCPConnection, End>; // Receive OpenTCPConnection from ProxyOne
type ProxyTwoToServerTCP = Send<OpenTCPConnection, Recv<OpenTCPConnectionByServerToProxyTwo, End>>; // Forward OpenTCPConnection to Server and receive choice

// Server thread
type ServerToClientTCP = Send<OpenTCPConnectionByServerToClient, End>; // Send choice to Client
type ServerToProxyOneTCP = Send<OpenTCPConnectionByServerToProxyOne, End>; // Send choice to ProxyOne
type ServerToProxyTwoTCP = Recv<OpenTCPConnection, Send<OpenTCPConnectionByServerToProxyTwo, End>>; // Receive OpenTCPConnection from ProxyTwo and send choice

// For Client
enum OpenTCPConnectionByServerToClient {
    Fail(
        MeshedChannels<
            Recv<Fail, End>, // Recv Fail from ProxyOne
            End,             // No communication with ProxyTwo
            End,             // No communication with Server
            RoleProxyOne<End>,
            NameClient,
        >,
    ),
    Success(
        MeshedChannels<
            Recv<Success, Send<RequestByClientToProxyOne, End>>, // Recv Success from ProxyOne and send choice
            Send<RequestByClientToProxyTwo, End>,                // Send choice to ProxyTwo
            Send<RequestByClientToServer, End>,                  // Send choice to Server
            RoleProxyOne<RoleProxyOne<RoleProxyTwo<RoleServer<End>>>>,
            NameClient,
        >,
    ),
}

// For ProxyOne
enum OpenTCPConnectionByServerToProxyOne {
    Fail(
        MeshedChannels<
            Send<Fail, End>, // Forward Fail to Client
            Recv<Fail, End>, // Recv Fail from ProxyTwo
            End,             // End connection
            RoleProxyTwo<RoleClient<End>>,
            NameProxyOne,
        >,
    ),
    Success(
        MeshedChannels<
            Send<Success, Recv<RequestByClientToProxyOne, End>>, // Forward Success to Client and receive choice
            Recv<Success, End>,                                  // Receive choice from ProxyTwo
            End,                                                 // No communication with Server
            RoleProxyTwo<RoleClient<RoleClient<End>>>,
            NameProxyOne,
        >,
    ),
}

// For ProxyTwo
enum OpenTCPConnectionByServerToProxyTwo {
    Fail(
        MeshedChannels<
            End,             // No communication with Client
            Send<Fail, End>, // Forward Fail from Server
            Recv<Fail, End>, // Recv Fail from Server
            RoleServer<RoleProxyOne<End>>,
            NameProxyTwo,
        >,
    ),
    Success(
        MeshedChannels<
            Recv<RequestByClientToProxyTwo, End>, // Receive choice from Client
            Send<Success, End>,                   // Forward Success to ProxyOne
            Recv<Success, End>,                   // Receive Success from Server
            RoleServer<RoleProxyOne<RoleClient<End>>>,
            NameProxyTwo,
        >,
    ),
}

// Orderings

type OrderingClient = RoleProxyOne<RoleBroadcast>;

type OrderingProxyOne = RoleClient<RoleProxyTwo<RoleClient<RoleEnd>>>;

type OrderingProxyTwo = RoleProxyOne<RoleServer<RoleClient<RoleEnd>>>;

type OrderingServer = RoleProxyTwo<RoleClient<RoleEnd>>;

// MeshedChannels

type EndpointClient =
    MeshedChannels<ClientToProxyOneTCP, ClientToProxyTwoTCP, ClientToServerTCP, NameClient>;

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
