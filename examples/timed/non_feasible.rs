#![allow(clippy::type_complexity, dead_code)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_timed;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

use rand::{thread_rng, Rng};

// Create roles and implementations
generate_timed!(MeshedChannels, Client, ProxyOne, ProxyTwo, Server);

// Payload types

// Request by the client
// We embed the method directly
// in the label of the message
struct OpenTCPConnection;
struct RequestGet {
    // method: String,
    version_protocol: i32,
    header: String,
}
struct RequestPost {
    // method: String,
    version_protocol: i32,
    header: String,
}
struct RequestPut {
    // method: String,
    version_protocol: i32,
    header: String,
}

// Response by the server
// We embed the status directly
// in the label of the message
struct Response200 {
    version_protocol: i32,
    // status_code: String,
    status_message: String,
    header: String,
}
struct Response404 {
    version_protocol: i32,
    // status_code: String,
    status_message: String,
    header: String,
}
struct Response418 {
    version_protocol: i32,
    // status_code: String,
    status_message: String,
    header: String,
}

struct Fail;

struct Success;

struct Close;

// Binary types

// Client thread
// Try to open TCP communication
type ClientToProxyOneTCP = SendTimed<OpenTCPConnection, 'a', 0, true, 10, true, ' ', End>; // SendTimed OpenTCPConnection to ProxyOne
type ClientToProxyTwoTCP = End; // No communications with ProxyTwo
type ClientToServerTCP =
    RecvTimed<OpenTCPConnectionByServerToClient, 'a', 0, true, 10, true, ' ', End>; // Receive choice from Server

// For ProxyOne
enum RequestByClientToProxyOne {
    RequestGet(
        MeshedChannels<
            RecvTimed<RequestGet, 'a', 0, true, 10, true, ' ', End>, // Receive request from Client
            SendTimed<RequestGet, 'a', 0, true, 10, true, ' ', End>, // Forward request to ProxyTwo
            RecvTimed<ResponseByServerToProxyOne, 'a', 0, true, 10, true, ' ', End>, // Receive choice from Server
            RoleClient<RoleProxyTwo<RoleServer<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
    RequestPut(
        MeshedChannels<
            RecvTimed<RequestPut, 'a', 0, true, 10, true, ' ', End>, // Receive request from Client
            SendTimed<RequestPut, 'a', 0, true, 10, true, ' ', End>, // Forward request to ProxyTwo
            RecvTimed<ResponseByServerToProxyOne, 'a', 0, true, 10, true, ' ', End>, // Receive choice from Server
            RoleClient<RoleProxyTwo<RoleServer<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
    RequestPost(
        MeshedChannels<
            RecvTimed<RequestPost, 'a', 0, true, 10, true, ' ', End>, // Receive request from Client
            SendTimed<RequestPost, 'a', 0, true, 10, true, ' ', End>, // Forward request to ProxyTwo
            RecvTimed<ResponseByServerToProxyOne, 'a', 0, true, 10, true, ' ', End>, // Receive choice from Server
            RoleClient<RoleProxyTwo<RoleServer<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
    Close(
        MeshedChannels<
            RecvTimed<Close, 'a', 0, true, 10, true, ' ', End>, // Receive Close from Client
            SendTimed<Close, 'a', 0, true, 10, true, ' ', End>, // Forward Close to ProxyTwo
            End,                                                // No communication with Server
            RoleClient<RoleProxyTwo<RoleEnd>>,
            NameProxyOne,
        >,
    ),
}

// For ProxyTwo
enum RequestByClientToProxyTwo {
    RequestGet(
        MeshedChannels<
            End,                                                     // No communication with Client
            RecvTimed<RequestGet, 'a', 0, true, 10, true, ' ', End>, // Receive request from ProxyOne
            SendTimed<
                RequestGet,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                RecvTimed<ResponseByServerToProxyTwo, 'a', 0, true, 10, true, ' ', End>,
            >, // Forward request to Server and receive choice from Server
            RoleProxyOne<RoleServer<RoleServer<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    RequestPut(
        MeshedChannels<
            End,                                                     // No communication with Client
            RecvTimed<RequestPut, 'a', 0, true, 10, true, ' ', End>, // Receive request from ProxyOne
            SendTimed<
                RequestPut,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                RecvTimed<ResponseByServerToProxyTwo, 'a', 0, true, 10, true, ' ', End>,
            >, // Forward request to Server and receive choice from Server
            RoleProxyOne<RoleServer<RoleServer<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    RequestPost(
        MeshedChannels<
            End,                                                      // No communication with Client
            RecvTimed<RequestPost, 'a', 0, true, 10, true, ' ', End>, // Receive request from ProxyOne
            SendTimed<
                RequestPost,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                RecvTimed<ResponseByServerToProxyTwo, 'a', 0, true, 10, true, ' ', End>,
            >, // Forward request to Server and receive choice from Server
            RoleProxyOne<RoleServer<RoleServer<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    Close(
        MeshedChannels<
            End,                                                // No communication with Client
            RecvTimed<Close, 'a', 0, true, 10, true, ' ', End>, // Receive Close from ProxyOne
            SendTimed<Close, 'a', 0, true, 10, true, ' ', End>, // Forward Close to Server
            RoleProxyOne<RoleServer<RoleEnd>>,
            NameProxyTwo,
        >,
    ),
}

// For Server
enum RequestByClientToServer {
    RequestGet(
        MeshedChannels<
            SendTimed<ResponseByServerToClient, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to Client
            SendTimed<ResponseByServerToProxyOne, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to ProxyOne
            RecvTimed<
                RequestGet,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                SendTimed<ResponseByServerToProxyTwo, 'a', 0, true, 10, true, ' ', End>,
            >, // Receive request from ProxyTwo and send choice to ProxyTwo
            RoleProxyTwo<RoleBroadcast>,
            NameServer,
        >,
    ),
    RequestPut(
        MeshedChannels<
            SendTimed<ResponseByServerToClient, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to Client
            SendTimed<ResponseByServerToProxyOne, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to ProxyOne
            RecvTimed<
                RequestPut,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                SendTimed<ResponseByServerToProxyTwo, 'a', 0, true, 10, true, ' ', End>,
            >, // Receive request from ProxyTwo and send choice to ProxyTwo
            RoleProxyTwo<RoleBroadcast>,
            NameServer,
        >,
    ),
    RequestPost(
        MeshedChannels<
            SendTimed<ResponseByServerToClient, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to Client
            SendTimed<ResponseByServerToProxyOne, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to ProxyOne
            RecvTimed<
                RequestPost,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                SendTimed<ResponseByServerToProxyTwo, 'a', 0, true, 10, true, ' ', End>,
            >, // Receive request from ProxyTwo and send choice to ProxyTwo
            RoleProxyTwo<RoleBroadcast>,
            NameServer,
        >,
    ),
    Close(
        MeshedChannels<
            End,
            End,
            RecvTimed<Close, 'a', 0, true, 10, true, ' ', End>, // Receive Close from ProxyTwo
            RoleProxyTwo<RoleEnd>,
            NameServer,
        >,
    ),
}

// ProxyOne thread
// Open TCP communication by Client
type ProxyOneToClientTCP = RecvTimed<OpenTCPConnection, 'a', 0, true, 10, true, ' ', End>; // Receive OpenTCPConnection from Client
type ProxyOneToProxyTwoTCP = SendTimed<OpenTCPConnection, 'a', 0, true, 10, true, ' ', End>; // Forward OpenTCPConnection to ProxyTwo
type ProxyOneToServerTCP =
    RecvTimed<OpenTCPConnectionByServerToProxyOne, 'a', 0, true, 10, true, ' ', End>; // Receive choice from Server

// ProxyTwo thread
type ProxyTwoToClientTCP = End; // No communication with Client
type ProxyTwoToProxyOneTCP = RecvTimed<OpenTCPConnection, 'a', 0, true, 10, true, ' ', End>; // Receive OpenTCPConnection from ProxyOne
type ProxyTwoToServerTCP = SendTimed<
    OpenTCPConnection,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    RecvTimed<OpenTCPConnectionByServerToProxyTwo, 'a', 0, true, 10, true, ' ', End>,
>; // Forward OpenTCPConnection to Server and receive choice

// Server thread
type ServerToClientTCP =
    SendTimed<OpenTCPConnectionByServerToClient, 'a', 0, true, 10, true, ' ', End>; // SendTimed choice to Client
type ServerToProxyOneTCP =
    SendTimed<OpenTCPConnectionByServerToProxyOne, 'a', 0, true, 10, true, ' ', End>; // SendTimed choice to ProxyOne
type ServerToProxyTwoTCP = RecvTimed<
    OpenTCPConnection,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    SendTimed<OpenTCPConnectionByServerToProxyTwo, 'a', 0, true, 10, true, ' ', End>,
>; // Receive OpenTCPConnection from ProxyTwo and send choice

// Answer to OpenTCPConnection
// For Client
enum OpenTCPConnectionByServerToClient {
    Fail(
        MeshedChannels<
            RecvTimed<Fail, 'a', 0, true, 10, true, ' ', End>, // RecvTimed Fail from ProxyOne
            End,                                               // No communication with ProxyTwo
            End,                                               // No communication with Server
            RoleProxyOne<RoleEnd>,
            NameClient,
        >,
    ),
    Success(
        MeshedChannels<
            RecvTimed<
                Success,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                SendTimed<RequestByClientToProxyOne, 'a', 0, true, 10, true, ' ', End>,
            >, // RecvTimed Success from ProxyOne and send choice
            SendTimed<RequestByClientToProxyTwo, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to ProxyTwo
            SendTimed<RequestByClientToServer, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to Server
            RoleProxyOne<RoleBroadcast>,
            NameClient,
        >,
    ),
}

// For ProxyOne
enum OpenTCPConnectionByServerToProxyOne {
    Fail(
        MeshedChannels<
            SendTimed<Fail, 'a', 0, true, 10, true, ' ', End>, // Forward Fail to Client
            RecvTimed<Fail, 'a', 0, true, 10, true, ' ', End>, // RecvTimed Fail from ProxyTwo
            End,                                               // End connection
            RoleProxyTwo<RoleClient<RoleEnd>>,
            NameProxyOne,
        >,
    ),
    Success(
        MeshedChannels<
            SendTimed<
                Success,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                RecvTimed<RequestByClientToProxyOne, 'a', 0, true, 10, true, ' ', End>,
            >, // Forward Success to Client and receive choice
            RecvTimed<Success, 'a', 0, true, 10, true, ' ', End>, // Receive choice from ProxyTwo
            End,                                                  // No communication with Server
            RoleProxyTwo<RoleClient<RoleClient<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
}

// For ProxyTwo
enum OpenTCPConnectionByServerToProxyTwo {
    Fail(
        MeshedChannels<
            End,                                               // No communication with Client
            SendTimed<Fail, 'a', 0, true, 10, true, ' ', End>, // Forward Fail from Server
            RecvTimed<Fail, 'a', 0, true, 10, true, ' ', End>, // RecvTimed Fail from Server
            RoleServer<RoleProxyOne<RoleEnd>>,
            NameProxyTwo,
        >,
    ),
    Success(
        MeshedChannels<
            RecvTimed<RequestByClientToProxyTwo, 'a', 0, true, 10, true, ' ', End>, // Receive choice from Client
            SendTimed<Success, 'a', 0, true, 10, true, ' ', End>, // Forward Success to ProxyOne
            RecvTimed<Success, 'a', 0, true, 10, true, ' ', End>, // Receive Success from Server
            RoleServer<RoleProxyOne<RoleClient<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
}

// Answer to Resquest / SendTimed Response
// For Client
enum ResponseByServerToClient {
    Response200(
        MeshedChannels<
            RecvTimed<
                Response200,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                SendTimed<RequestByClientToProxyOne, 'a', 0, true, 10, true, ' ', End>,
            >, // Receive Response200 from ProxyOne and send choice
            SendTimed<RequestByClientToProxyTwo, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to ProxyTwo
            SendTimed<RequestByClientToServer, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to Server
            RoleProxyOne<RoleBroadcast>,
            NameClient,
        >,
    ),
    Response404(
        MeshedChannels<
            RecvTimed<
                Response404,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                SendTimed<RequestByClientToProxyOne, 'a', 0, true, 10, true, ' ', End>,
            >, // Receive Response404 from ProxyOne and send choice
            SendTimed<RequestByClientToProxyTwo, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to ProxyTwo
            SendTimed<RequestByClientToServer, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to Server
            RoleProxyOne<RoleBroadcast>,
            NameClient,
        >,
    ),
    Response418(
        MeshedChannels<
            RecvTimed<
                Response418,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                SendTimed<RequestByClientToProxyOne, 'a', 0, true, 10, true, ' ', End>,
            >, // Receive Response418 from ProxyOne and send choice
            SendTimed<RequestByClientToProxyTwo, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to ProxyTwo
            SendTimed<RequestByClientToServer, 'a', 0, true, 10, true, ' ', End>, // SendTimed choice to Server
            RoleProxyOne<RoleBroadcast>,
            NameClient,
        >,
    ),
}

// For ProxyOne
enum ResponseByServerToProxyOne {
    Response200(
        MeshedChannels<
            SendTimed<
                Response200,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                RecvTimed<RequestByClientToProxyOne, 'a', 0, true, 10, true, ' ', End>,
            >, // Forward Response200 to Client ProxyOne and receive choice
            RecvTimed<Response200, 'a', 0, true, 10, true, ' ', End>, // Receive Response200 from ProxyTwo
            End, // No communication with Server
            RoleProxyTwo<RoleClient<RoleClient<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
    Response404(
        MeshedChannels<
            SendTimed<
                Response404,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                RecvTimed<RequestByClientToProxyOne, 'a', 0, true, 10, true, ' ', End>,
            >, // Forward Response404 to Client ProxyOne and receive choice
            RecvTimed<Response404, 'a', 0, true, 10, true, ' ', End>, // Receive Response404 from ProxyTwo
            End, // No communication with Server
            RoleProxyTwo<RoleClient<RoleClient<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
    Response418(
        MeshedChannels<
            SendTimed<
                Response418,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                RecvTimed<RequestByClientToProxyOne, 'a', 0, true, 10, true, ' ', End>,
            >, // Forward Response418 to Client ProxyOne and receive choice
            RecvTimed<Response418, 'a', 0, true, 10, true, ' ', End>, // Receive Response418 from ProxyTwo
            End, // No communication with Server
            RoleProxyTwo<RoleClient<RoleClient<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
}

// For ProxyTwo
enum ResponseByServerToProxyTwo {
    Response200(
        MeshedChannels<
            RecvTimed<RequestByClientToProxyTwo, 'a', 0, true, 10, true, ' ', End>, // Receive choice from Client
            SendTimed<Response200, 'a', 0, true, 10, true, ' ', End>, // Forward Response200 to ProxyOne
            RecvTimed<Response200, 'a', 0, true, 10, true, ' ', End>, // Receive Response200 from Server
            RoleServer<RoleProxyOne<RoleClient<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    Response404(
        MeshedChannels<
            RecvTimed<RequestByClientToProxyTwo, 'a', 0, true, 10, true, ' ', End>, // Receive choice from Client
            SendTimed<Response404, 'a', 0, true, 10, true, ' ', End>, // Forward Response404 to ProxyOne
            RecvTimed<Response404, 'a', 0, true, 10, true, ' ', End>, // Receive Response404 from Server
            RoleServer<RoleProxyOne<RoleClient<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    Response418(
        MeshedChannels<
            RecvTimed<RequestByClientToProxyTwo, 'a', 0, true, 10, true, ' ', End>, // Receive choice from Client
            SendTimed<Response418, 'a', 0, true, 10, true, ' ', End>, // Forward Response418 to ProxyOne
            RecvTimed<Response418, 'a', 0, true, 10, true, ' ', End>, // Receive Response418 from Server
            RoleServer<RoleProxyOne<RoleClient<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
}

// Orderings

type OrderingClient = RoleProxyOne<RoleServer<RoleEnd>>;

type OrderingProxyOne = RoleClient<RoleProxyTwo<RoleServer<RoleEnd>>>;

type OrderingProxyTwo = RoleProxyOne<RoleServer<RoleServer<RoleEnd>>>;

type OrderingServer = RoleProxyTwo<RoleBroadcast>;

// MeshedChannels
// CLient
type EndpointClient = MeshedChannels<
    ClientToProxyOneTCP,
    ClientToProxyTwoTCP,
    ClientToServerTCP,
    OrderingClient,
    NameClient,
>;

type RecursClient = MeshedChannels<
    SendTimed<RequestByClientToProxyOne, 'a', 0, true, 10, true, ' ', End>,
    SendTimed<RequestByClientToProxyTwo, 'a', 0, true, 10, true, ' ', End>,
    SendTimed<RequestByClientToServer, 'a', 0, true, 10, true, ' ', End>,
    RoleBroadcast,
    NameClient,
>;

type EndpointClientClose = MeshedChannels<
    SendTimed<Close, 'a', 0, true, 10, true, ' ', End>,
    End,
    End,
    RoleProxyOne<RoleEnd>,
    NameClient,
>;

type EndpointClientRequestGet = MeshedChannels<
    SendTimed<RequestGet, 'a', 0, true, 10, true, ' ', End>,
    End,
    RecvTimed<ResponseByServerToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleProxyOne<RoleServer<RoleEnd>>,
    NameClient,
>;

type EndpointClientRequestPut = MeshedChannels<
    SendTimed<RequestPut, 'a', 0, true, 10, true, ' ', End>,
    End,
    RecvTimed<ResponseByServerToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleProxyOne<RoleServer<RoleEnd>>,
    NameClient,
>;

type EndpointClientRequestPost = MeshedChannels<
    SendTimed<RequestPost, 'a', 0, true, 10, true, ' ', End>,
    End,
    RecvTimed<ResponseByServerToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleProxyOne<RoleServer<RoleEnd>>,
    NameClient,
>;

// ProxyOne
type EndpointProxyOne = MeshedChannels<
    ProxyOneToClientTCP,
    ProxyOneToProxyTwoTCP,
    ProxyOneToServerTCP,
    OrderingProxyOne,
    NameProxyOne,
>;

type RecursProxyOne = MeshedChannels<
    RecvTimed<RequestByClientToProxyOne, 'a', 0, true, 10, true, ' ', End>,
    End,
    End,
    RoleClient<RoleEnd>,
    NameProxyOne,
>;

// ProxyTwo
type EndpointProxyTwo = MeshedChannels<
    ProxyTwoToClientTCP,
    ProxyTwoToProxyOneTCP,
    ProxyTwoToServerTCP,
    OrderingProxyTwo,
    NameProxyTwo,
>;

type RecursProxyTwo = MeshedChannels<
    RecvTimed<RequestByClientToProxyTwo, 'a', 0, true, 10, true, ' ', End>,
    End,
    End,
    RoleClient<RoleEnd>,
    NameProxyTwo,
>;

// Server
type EndpointServer = MeshedChannels<
    ServerToClientTCP,
    ServerToProxyOneTCP,
    ServerToProxyTwoTCP,
    OrderingServer,
    NameServer,
>;

type EndpointServerFail = MeshedChannels<
    End,
    End,
    SendTimed<Fail, 'a', 0, true, 10, true, ' ', End>,
    RoleProxyTwo<RoleEnd>,
    NameServer,
>;

type EndpointServerSuccess = MeshedChannels<
    RecvTimed<RequestByClientToServer, 'a', 0, true, 10, true, ' ', End>,
    End,
    SendTimed<Success, 'a', 0, true, 10, true, ' ', End>,
    RoleProxyTwo<RoleClient<RoleEnd>>,
    NameServer,
>;

type RecursServer = MeshedChannels<
    RecvTimed<RequestByClientToServer, 'a', 0, true, 10, true, ' ', End>,
    End,
    End,
    RoleClient<RoleEnd>,
    NameServer,
>;

type EndpointServerResponse200 = MeshedChannels<
    RecvTimed<RequestByClientToServer, 'a', 0, true, 10, true, ' ', End>,
    End,
    SendTimed<Response200, 'a', 0, true, 10, true, ' ', End>,
    RoleProxyTwo<RoleClient<RoleEnd>>,
    NameServer,
>;

type EndpointServerResponse404 = MeshedChannels<
    RecvTimed<RequestByClientToServer, 'a', 0, true, 10, true, ' ', End>,
    End,
    SendTimed<Response404, 'a', 0, true, 10, true, ' ', End>,
    RoleProxyTwo<RoleClient<RoleEnd>>,
    NameServer,
>;

type EndpointServerResponse418 = MeshedChannels<
    RecvTimed<RequestByClientToServer, 'a', 0, true, 10, true, ' ', End>,
    End,
    SendTimed<Response418, 'a', 0, true, 10, true, ' ', End>,
    RoleProxyTwo<RoleClient<RoleEnd>>,
    NameServer,
>;

// Functions

/////////////////////////

// Functions related to endpoints
fn endpoint_client(
    s: EndpointClient,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(OpenTCPConnection {}, all_clocks)?;

    offer_mpst!(s, all_clocks, {
        OpenTCPConnectionByServerToClient::Fail(s) => {
            let (_, s) = s.recv(all_clocks)?;
            s.close()
        },
        OpenTCPConnectionByServerToClient::Success(s) => {
            let (_, s) = s.recv(all_clocks)?;
            recurs_client(s, 100, all_clocks)
        },
    })
}

// Functions related to endpoints
fn recurs_client(
    s: RecursClient,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    if loops == 0 {
        let s: EndpointClientClose = choose_mpst_client_to_all!(
            s,
            all_clocks,
            RequestByClientToProxyOne::Close,
            RequestByClientToProxyTwo::Close,
            RequestByClientToServer::Close
        );
        let s = s.send(Close {}, all_clocks)?;
        s.close()
    } else {
        match thread_rng().gen_range(1..3) {
            1 => {
                let s: EndpointClientRequestGet = choose_mpst_client_to_all!(
                    s,
                    all_clocks,
                    RequestByClientToProxyOne::RequestGet,
                    RequestByClientToProxyTwo::RequestGet,
                    RequestByClientToServer::RequestGet
                );

                let s = s.send(
                    RequestGet {
                        version_protocol: 5,
                        header: String::from("GET"),
                    },
                    all_clocks,
                )?;

                offer_mpst!(s, all_clocks, {
                    ResponseByServerToClient::Response200(s) => {
                        let (_, s) = s.recv(all_clocks)?;
                        recurs_client(s, loops -1, all_clocks)
                    },
                    ResponseByServerToClient::Response404(s) => {
                        let (_, s) = s.recv(all_clocks)?;
                        recurs_client(s, loops -1, all_clocks)
                    },
                    ResponseByServerToClient::Response418(s) => {
                        let (_, s) = s.recv(all_clocks)?;
                        recurs_client(s, loops -1, all_clocks)
                    },
                })
            }
            2 => {
                let s: EndpointClientRequestPut = choose_mpst_client_to_all!(
                    s,
                    all_clocks,
                    RequestByClientToProxyOne::RequestPut,
                    RequestByClientToProxyTwo::RequestPut,
                    RequestByClientToServer::RequestPut
                );

                let s = s.send(
                    RequestPut {
                        version_protocol: 5,
                        header: String::from("PUT"),
                    },
                    all_clocks,
                )?;

                offer_mpst!(s, all_clocks, {
                    ResponseByServerToClient::Response200(s) => {
                        let (_, s) = s.recv(all_clocks)?;
                        recurs_client(s, loops -1, all_clocks)
                    },
                    ResponseByServerToClient::Response404(s) => {
                        let (_, s) = s.recv(all_clocks)?;
                        recurs_client(s, loops -1, all_clocks)
                    },
                    ResponseByServerToClient::Response418(s) => {
                        let (_, s) = s.recv(all_clocks)?;
                        recurs_client(s, loops -1, all_clocks)
                    },
                })
            }
            3 => {
                let s: EndpointClientRequestPost = choose_mpst_client_to_all!(
                    s,
                    all_clocks,
                    RequestByClientToProxyOne::RequestPost,
                    RequestByClientToProxyTwo::RequestPost,
                    RequestByClientToServer::RequestPost
                );

                let s = s.send(
                    RequestPost {
                        version_protocol: 5,
                        header: String::from("POST"),
                    },
                    all_clocks,
                )?;

                offer_mpst!(s, all_clocks, {
                    ResponseByServerToClient::Response200(s) => {
                        let (_, s) = s.recv(all_clocks)?;
                        recurs_client(s, loops -1, all_clocks)
                    },
                    ResponseByServerToClient::Response404(s) => {
                        let (_, s) = s.recv(all_clocks)?;
                        recurs_client(s, loops -1, all_clocks)
                    },
                    ResponseByServerToClient::Response418(s) => {
                        let (_, s) = s.recv(all_clocks)?;
                        recurs_client(s, loops -1, all_clocks)
                    },
                })
            }
            _ => panic!("Error, unexpected input"),
        }
    }
}

/////////////////////////

fn endpoint_proxyone(
    s: EndpointProxyOne,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (opentcpconnection, s) = s.recv(all_clocks)?;

    let s = s.send(opentcpconnection, all_clocks)?;

    offer_mpst!(s, all_clocks, {
            OpenTCPConnectionByServerToProxyOne::Fail(s) => {
                let (fail, s) = s.recv(all_clocks)?;
                let s = s.send(fail, all_clocks)?;
                s.close()
            },
            OpenTCPConnectionByServerToProxyOne::Success(s) => {
                let (fail, s) = s.recv(all_clocks)?;
                let s = s.send(fail, all_clocks)?;

                recurs_proxyone(s, all_clocks)
            },
    })
}

fn recurs_proxyone(
    s: RecursProxyOne,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        RequestByClientToProxyOne::RequestGet(s) => {
            let (request, s) = s.recv(all_clocks)?;
            let s = s.send(request, all_clocks)?;

            offer_mpst!(s, all_clocks, {
                ResponseByServerToProxyOne::Response200(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxyone(s, all_clocks)
                },
                ResponseByServerToProxyOne::Response404(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxyone(s, all_clocks)
                },
                ResponseByServerToProxyOne::Response418(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxyone(s, all_clocks)
                },
            })
        },
        RequestByClientToProxyOne::RequestPut(s) => {
            let (request, s) = s.recv(all_clocks)?;
            let s = s.send(request, all_clocks)?;

            offer_mpst!(s, all_clocks, {
                ResponseByServerToProxyOne::Response200(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxyone(s, all_clocks)
                },
                ResponseByServerToProxyOne::Response404(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxyone(s, all_clocks)
                },
                ResponseByServerToProxyOne::Response418(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxyone(s, all_clocks)
                },
            })
        },
        RequestByClientToProxyOne::RequestPost(s) => {
            let (request, s) = s.recv(all_clocks)?;
            let s = s.send(request, all_clocks)?;

            offer_mpst!(s, all_clocks, {
                ResponseByServerToProxyOne::Response200(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxyone(s, all_clocks)
                },
                ResponseByServerToProxyOne::Response404(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxyone(s, all_clocks)
                },
                ResponseByServerToProxyOne::Response418(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxyone(s, all_clocks)
                },
            })
        },
        RequestByClientToProxyOne::Close(s) => {
            let (request, s) = s.recv(all_clocks)?;
            let s = s.send(request, all_clocks)?;
            s.close()
        },
    })
}

/////////////////////////

fn endpoint_proxytwo(
    s: EndpointProxyTwo,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (opentcpconnection, s) = s.recv(all_clocks)?;

    let s = s.send(opentcpconnection, all_clocks)?;
    offer_mpst!(s, all_clocks, {
        OpenTCPConnectionByServerToProxyTwo::Fail(s) => {
            let (fail, s) = s.recv(all_clocks)?;
            let s = s.send(fail, all_clocks)?;
            s.close()
        },
        OpenTCPConnectionByServerToProxyTwo::Success(s) => {
            let (fail, s) = s.recv(all_clocks)?;
            let s = s.send(fail, all_clocks)?;
            recurs_proxytwo(s, all_clocks)
        },
    })
}

fn recurs_proxytwo(
    s: RecursProxyTwo,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        RequestByClientToProxyTwo::RequestGet(s) => {
            let (request, s) = s.recv(all_clocks)?;
            let s = s.send(request, all_clocks)?;

            offer_mpst!(s, all_clocks, {
                ResponseByServerToProxyTwo::Response200(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxytwo(s, all_clocks)
                },
                ResponseByServerToProxyTwo::Response404(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxytwo(s, all_clocks)
                },
                ResponseByServerToProxyTwo::Response418(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxytwo(s, all_clocks)
                },
            })
        },
        RequestByClientToProxyTwo::RequestPut(s) => {
            let (request, s) = s.recv(all_clocks)?;
            let s = s.send(request, all_clocks)?;

            offer_mpst!(s, all_clocks, {
                ResponseByServerToProxyTwo::Response200(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxytwo(s, all_clocks)
                },
                ResponseByServerToProxyTwo::Response404(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxytwo(s, all_clocks)
                },
                ResponseByServerToProxyTwo::Response418(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxytwo(s, all_clocks)
                },
            })
        },
        RequestByClientToProxyTwo::RequestPost(s) => {
            let (request, s) = s.recv(all_clocks)?;
            let s = s.send(request, all_clocks)?;

            offer_mpst!(s, all_clocks, {
                ResponseByServerToProxyTwo::Response200(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxytwo(s, all_clocks)
                },
                ResponseByServerToProxyTwo::Response404(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxytwo(s, all_clocks)
                },
                ResponseByServerToProxyTwo::Response418(s) => {
                    let (response, s) = s.recv(all_clocks)?;
                    let s = s.send(response, all_clocks)?;

                    recurs_proxytwo(s, all_clocks)
                },
            })
        },
        RequestByClientToProxyTwo::Close(s) => {
            let (request, s) = s.recv(all_clocks)?;
            let s = s.send(request, all_clocks)?;
            s.close()
        },
    })
}

/////////////////////////

fn endpoint_server(
    s: EndpointServer,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv(all_clocks)?;

    // 10 percent chance of failure
    match thread_rng().gen_range(1..10) {
        1 => {
            let s: EndpointServerFail = choose_mpst_server_to_all!(
                s,
                all_clocks,
                OpenTCPConnectionByServerToClient::Fail,
                OpenTCPConnectionByServerToProxyOne::Fail,
                OpenTCPConnectionByServerToProxyTwo::Fail
            );

            let s = s.send(Fail {}, all_clocks)?;

            s.close()
        }
        _ => {
            let s: EndpointServerSuccess = choose_mpst_server_to_all!(
                s,
                all_clocks,
                OpenTCPConnectionByServerToClient::Success,
                OpenTCPConnectionByServerToProxyOne::Success,
                OpenTCPConnectionByServerToProxyTwo::Success
            );

            let s = s.send(Success {}, all_clocks)?;

            recurs_server(s, all_clocks)
        }
    }
}

fn recurs_server(
    s: RecursServer,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        RequestByClientToServer::RequestGet(s) => {
            let (_, s) = s.recv(all_clocks)?;

            match thread_rng().gen_range(1..3) {
                1 => {
                    let s: EndpointServerResponse200 = choose_mpst_server_to_all!(
                        s,
                        all_clocks,
                        ResponseByServerToClient::Response200,
                        ResponseByServerToProxyOne::Response200,
                        ResponseByServerToProxyTwo::Response200
                    );

                    let s = s.send(Response200 {
                        version_protocol: 5,
                        status_message: String::from("OK"),
                        header: String::from("200")
                    }, all_clocks)?;

                    recurs_server(s, all_clocks)
                }
                2 => {
                    let s: EndpointServerResponse404 = choose_mpst_server_to_all!(
                        s,
                        all_clocks,
                        ResponseByServerToClient::Response404,
                        ResponseByServerToProxyOne::Response404,
                        ResponseByServerToProxyTwo::Response404
                    );

                    let s = s.send(Response404 {
                        version_protocol: 5,
                        status_message: String::from("Resource not found"),
                        header: String::from("404")
                    }, all_clocks)?;

                    recurs_server(s, all_clocks)
                }
                3 => {
                    let s: EndpointServerResponse418 = choose_mpst_server_to_all!(
                        s,
                        all_clocks,
                        ResponseByServerToClient::Response418,
                        ResponseByServerToProxyOne::Response418,
                        ResponseByServerToProxyTwo::Response418
                    );

                    let s = s.send(Response418 {
                        version_protocol: 5,
                        status_message: String::from("I'm a tea pot"),
                        header: String::from("418")
                    }, all_clocks)?;

                    recurs_server(s, all_clocks)
                }
                _ => panic!("Error, unexpected number")
            }
        },
        RequestByClientToServer::RequestPut(s) => {
            let (_, s) = s.recv(all_clocks)?;

            match thread_rng().gen_range(1..3) {
                1 => {
                    let s: EndpointServerResponse200 = choose_mpst_server_to_all!(
                        s,
                        all_clocks,
                        ResponseByServerToClient::Response200,
                        ResponseByServerToProxyOne::Response200,
                        ResponseByServerToProxyTwo::Response200
                    );

                    let s = s.send(Response200 {
                        version_protocol: 5,
                        status_message: String::from("OK"),
                        header: String::from("200")
                    }, all_clocks)?;

                    recurs_server(s, all_clocks)
                }
                2 => {
                    let s: EndpointServerResponse404 = choose_mpst_server_to_all!(
                        s,
                        all_clocks,
                        ResponseByServerToClient::Response404,
                        ResponseByServerToProxyOne::Response404,
                        ResponseByServerToProxyTwo::Response404
                    );

                    let s = s.send(Response404 {
                        version_protocol: 5,
                        status_message: String::from("Resource not found"),
                        header: String::from("404")
                    }, all_clocks)?;

                    recurs_server(s, all_clocks)
                }
                3 => {
                    let s: EndpointServerResponse418 = choose_mpst_server_to_all!(
                        s,
                        all_clocks,
                        ResponseByServerToClient::Response418,
                        ResponseByServerToProxyOne::Response418,
                        ResponseByServerToProxyTwo::Response418
                    );

                    let s = s.send(Response418 {
                        version_protocol: 5,
                        status_message: String::from("I'm a tea pot"),
                        header: String::from("418")
                    }, all_clocks)?;

                    recurs_server(s, all_clocks)
                }
                _ => panic!("Error, unexpected number")
            }
        },
        RequestByClientToServer::RequestPost(s) => {
            let (_, s) = s.recv(all_clocks)?;

            match thread_rng().gen_range(1..3) {
                1 => {
                    let s: EndpointServerResponse200 = choose_mpst_server_to_all!(
                        s,
                        all_clocks,
                        ResponseByServerToClient::Response200,
                        ResponseByServerToProxyOne::Response200,
                        ResponseByServerToProxyTwo::Response200
                    );

                    let s = s.send(Response200 {
                        version_protocol: 5,
                        status_message: String::from("OK"),
                        header: String::from("200")
                    }, all_clocks)?;

                    recurs_server(s, all_clocks)
                }
                2 => {
                    let s: EndpointServerResponse404 = choose_mpst_server_to_all!(
                        s,
                        all_clocks,
                        ResponseByServerToClient::Response404,
                        ResponseByServerToProxyOne::Response404,
                        ResponseByServerToProxyTwo::Response404
                    );

                    let s = s.send(Response404 {
                        version_protocol: 5,
                        status_message: String::from("Resource not found"),
                        header: String::from("404")
                    }, all_clocks)?;

                    recurs_server(s, all_clocks)
                }
                3 => {
                    let s: EndpointServerResponse418 = choose_mpst_server_to_all!(
                        s,
                        all_clocks,
                        ResponseByServerToClient::Response418,
                        ResponseByServerToProxyOne::Response418,
                        ResponseByServerToProxyTwo::Response418
                    );

                    let s = s.send(Response418 {
                        version_protocol: 5,
                        status_message: String::from("I'm a tea pot"),
                        header: String::from("418")
                    }, all_clocks)?;

                    recurs_server(s, all_clocks)
                }
                _ => panic!("Error, unexpected number")
            }
        },
    })
}

////////////////////////////////////////

fn main() {
    let (thread_client, thread_proxyone, thread_proxytwo, thread_server) = fork_mpst(
        endpoint_client,
        endpoint_proxyone,
        endpoint_proxytwo,
        endpoint_server,
    );

    assert!(thread_client.join().is_ok());
    assert!(thread_proxyone.join().is_ok());
    assert!(thread_proxytwo.join().is_ok());
    assert!(thread_server.join().is_ok());
}
