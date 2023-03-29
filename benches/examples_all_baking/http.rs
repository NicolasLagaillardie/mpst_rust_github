#![allow(clippy::type_complexity, dead_code)]

use criterion::{black_box, Criterion};

use mpstthree::baker;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

use rand::{thread_rng, Rng};

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
type ClientToProxyOneTCP = Send<OpenTCPConnection, End>; // Send OpenTCPConnection to ProxyOne
type ClientToProxyTwoTCP = End; // No communications with ProxyTwo
type ClientToServerTCP = Recv<OpenTCPConnectionByServerToClient, End>; // Receive choice from Server

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
            End,              // No communication with Server
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
            Recv<RequestGet, End>, // Receive request from ProxyOne
            Send<RequestGet, Recv<ResponseByServerToProxyTwo, End>>, // Forward request to Server and receive choice from Server
            RoleProxyOne<RoleServer<RoleServer<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    RequestPut(
        MeshedChannels<
            End,                                                     // No communication with Client
            Recv<RequestPut, End>, // Receive request from ProxyOne
            Send<RequestPut, Recv<ResponseByServerToProxyTwo, End>>, // Forward request to Server and receive choice from Server
            RoleProxyOne<RoleServer<RoleServer<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    RequestPost(
        MeshedChannels<
            End,                                                      // No communication with Client
            Recv<RequestPost, End>, // Receive request from ProxyOne
            Send<RequestPost, Recv<ResponseByServerToProxyTwo, End>>, // Forward request to Server and receive choice from Server
            RoleProxyOne<RoleServer<RoleServer<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    Close(
        MeshedChannels<
            End,              // No communication with Client
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
            RoleProxyTwo<RoleBroadcast>,
            NameServer,
        >,
    ),
    RequestPut(
        MeshedChannels<
            Send<ResponseByServerToClient, End>,   // Send choice to Client
            Send<ResponseByServerToProxyOne, End>, // Send choice to ProxyOne
            Recv<RequestPut, Send<ResponseByServerToProxyTwo, End>>, // Receive request from ProxyTwo and send choice to ProxyTwo
            RoleProxyTwo<RoleBroadcast>,
            NameServer,
        >,
    ),
    RequestPost(
        MeshedChannels<
            Send<ResponseByServerToClient, End>,   // Send choice to Client
            Send<ResponseByServerToProxyOne, End>, // Send choice to ProxyOne
            Recv<RequestPost, Send<ResponseByServerToProxyTwo, End>>, // Receive request from ProxyTwo and send choice to ProxyTwo
            RoleProxyTwo<RoleBroadcast>,
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

// Answer to OpenTCPConnection
// For Client
enum OpenTCPConnectionByServerToClient {
    Fail(
        MeshedChannels<
            Recv<Fail, End>, // Recv Fail from ProxyOne
            End,             // No communication with ProxyTwo
            End,             // No communication with Server
            RoleProxyOne<RoleEnd>,
            NameClient,
        >,
    ),
    Success(
        MeshedChannels<
            Recv<Success, Send<RequestByClientToProxyOne, End>>, // Recv Success from ProxyOne and send choice
            Send<RequestByClientToProxyTwo, End>,                // Send choice to ProxyTwo
            Send<RequestByClientToServer, End>,                  // Send choice to Server
            RoleProxyOne<RoleBroadcast>,
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
            RoleProxyTwo<RoleClient<RoleEnd>>,
            NameProxyOne,
        >,
    ),
    Success(
        MeshedChannels<
            Send<Success, Recv<RequestByClientToProxyOne, End>>, // Forward Success to Client and receive choice
            Recv<Success, End>,                                  // Receive choice from ProxyTwo
            End,                                                 // No communication with Server
            RoleProxyTwo<RoleClient<RoleClient<RoleEnd>>>,
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
            RoleServer<RoleProxyOne<RoleEnd>>,
            NameProxyTwo,
        >,
    ),
    Success(
        MeshedChannels<
            Recv<RequestByClientToProxyTwo, End>, // Receive choice from Client
            Send<Success, End>,                   // Forward Success to ProxyOne
            Recv<Success, End>,                   // Receive Success from Server
            RoleServer<RoleProxyOne<RoleClient<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
}

// Answer to Resquest / Send Response
// For Client
enum ResponseByServerToClient {
    Response200(
        MeshedChannels<
            Recv<Response200, Send<RequestByClientToProxyOne, End>>, // Receive Response200 from ProxyOne and send choice
            Send<RequestByClientToProxyTwo, End>,                    // Send choice to ProxyTwo
            Send<RequestByClientToServer, End>,                      // Send choice to Server
            RoleProxyOne<RoleBroadcast>,
            NameClient,
        >,
    ),
    Response404(
        MeshedChannels<
            Recv<Response404, Send<RequestByClientToProxyOne, End>>, // Receive Response404 from ProxyOne and send choice
            Send<RequestByClientToProxyTwo, End>,                    // Send choice to ProxyTwo
            Send<RequestByClientToServer, End>,                      // Send choice to Server
            RoleProxyOne<RoleBroadcast>,
            NameClient,
        >,
    ),
    Response418(
        MeshedChannels<
            Recv<Response418, Send<RequestByClientToProxyOne, End>>, // Receive Response418 from ProxyOne and send choice
            Send<RequestByClientToProxyTwo, End>,                    // Send choice to ProxyTwo
            Send<RequestByClientToServer, End>,                      // Send choice to Server
            RoleProxyOne<RoleBroadcast>,
            NameClient,
        >,
    ),
}

// For ProxyOne
enum ResponseByServerToProxyOne {
    Response200(
        MeshedChannels<
            Send<Response200, Recv<RequestByClientToProxyOne, End>>, // Forward Response200 to Client ProxyOne and receive choice
            Recv<Response200, End>, // Receive Response200 from ProxyTwo
            End,                    // No communication with Server
            RoleProxyTwo<RoleClient<RoleClient<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
    Response404(
        MeshedChannels<
            Send<Response404, Recv<RequestByClientToProxyOne, End>>, // Forward Response404 to Client ProxyOne and receive choice
            Recv<Response404, End>, // Receive Response404 from ProxyTwo
            End,                    // No communication with Server
            RoleProxyTwo<RoleClient<RoleClient<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
    Response418(
        MeshedChannels<
            Send<Response418, Recv<RequestByClientToProxyOne, End>>, // Forward Response418 to Client ProxyOne and receive choice
            Recv<Response418, End>, // Receive Response418 from ProxyTwo
            End,                    // No communication with Server
            RoleProxyTwo<RoleClient<RoleClient<RoleEnd>>>,
            NameProxyOne,
        >,
    ),
}

// For ProxyTwo
enum ResponseByServerToProxyTwo {
    Response200(
        MeshedChannels<
            Recv<RequestByClientToProxyTwo, End>, // Receive choice from Client
            Send<Response200, End>,               // Forward Response200 to ProxyOne
            Recv<Response200, End>,               // Receive Response200 from Server
            RoleServer<RoleProxyOne<RoleClient<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    Response404(
        MeshedChannels<
            Recv<RequestByClientToProxyTwo, End>, // Receive choice from Client
            Send<Response404, End>,               // Forward Response404 to ProxyOne
            Recv<Response404, End>,               // Receive Response404 from Server
            RoleServer<RoleProxyOne<RoleClient<RoleEnd>>>,
            NameProxyTwo,
        >,
    ),
    Response418(
        MeshedChannels<
            Recv<RequestByClientToProxyTwo, End>, // Receive choice from Client
            Send<Response418, End>,               // Forward Response418 to ProxyOne
            Recv<Response418, End>,               // Receive Response418 from Server
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
    Send<RequestByClientToProxyOne, End>,
    Send<RequestByClientToProxyTwo, End>,
    Send<RequestByClientToServer, End>,
    RoleBroadcast,
    NameClient,
>;

type EndpointClientClose =
    MeshedChannels<Send<Close, End>, End, End, RoleProxyOne<RoleEnd>, NameClient>;

type EndpointClientRequestGet = MeshedChannels<
    Send<RequestGet, End>,
    End,
    Recv<ResponseByServerToClient, End>,
    RoleProxyOne<RoleServer<RoleEnd>>,
    NameClient,
>;

type EndpointClientRequestPut = MeshedChannels<
    Send<RequestPut, End>,
    End,
    Recv<ResponseByServerToClient, End>,
    RoleProxyOne<RoleServer<RoleEnd>>,
    NameClient,
>;

type EndpointClientRequestPost = MeshedChannels<
    Send<RequestPost, End>,
    End,
    Recv<ResponseByServerToClient, End>,
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
    Recv<RequestByClientToProxyOne, End>,
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
    Recv<RequestByClientToProxyTwo, End>,
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

type EndpointServerFail =
    MeshedChannels<End, End, Send<Fail, End>, RoleProxyTwo<RoleEnd>, NameServer>;

type EndpointServerSuccess = MeshedChannels<
    Recv<RequestByClientToServer, End>,
    End,
    Send<Success, End>,
    RoleProxyTwo<RoleClient<RoleEnd>>,
    NameServer,
>;

type RecursServer =
    MeshedChannels<Recv<RequestByClientToServer, End>, End, End, RoleClient<RoleEnd>, NameServer>;

type EndpointServerResponse200 = MeshedChannels<
    Recv<RequestByClientToServer, End>,
    End,
    Send<Response200, End>,
    RoleProxyTwo<RoleClient<RoleEnd>>,
    NameServer,
>;

type EndpointServerResponse404 = MeshedChannels<
    Recv<RequestByClientToServer, End>,
    End,
    Send<Response404, End>,
    RoleProxyTwo<RoleClient<RoleEnd>>,
    NameServer,
>;

type EndpointServerResponse418 = MeshedChannels<
    Recv<RequestByClientToServer, End>,
    End,
    Send<Response418, End>,
    RoleProxyTwo<RoleClient<RoleEnd>>,
    NameServer,
>;

// Functions

/////////////////////////

// Functions related to endpoints
fn endpoint_client(s: EndpointClient) -> Result<(), Box<dyn Error>> {
    let s = s.send(OpenTCPConnection {})?;

    offer_mpst!(s, {
        OpenTCPConnectionByServerToClient::Fail(s) => {

                        let (_, s) = s.recv()?;
            s.close()
        },
        OpenTCPConnectionByServerToClient::Success(s) => {

                        let (_, s) = s.recv()?;
            recurs_client(s, 100)
        },
    })
}

// Functions related to endpoints
fn recurs_client(s: RecursClient, loops: i32) -> Result<(), Box<dyn Error>> {
    if loops == 0 {
        let s: EndpointClientClose = choose_mpst_client_to_all!(
            s,
            RequestByClientToProxyOne::Close,
            RequestByClientToProxyTwo::Close,
            RequestByClientToServer::Close
        );
        let s = s.send(Close {})?;

        s.close()
    } else {
        match thread_rng().gen_range(1..3) {
            1 => {
                let s: EndpointClientRequestGet = choose_mpst_client_to_all!(
                    s,
                    RequestByClientToProxyOne::RequestGet,
                    RequestByClientToProxyTwo::RequestGet,
                    RequestByClientToServer::RequestGet
                );

                let s = s.send(RequestGet {
                    version_protocol: 5,
                    header: String::from("GET"),
                })?;

                offer_mpst!(s, {
                    ResponseByServerToClient::Response200(s) => {
                        let (_, s) = s.recv()?;
                        recurs_client(s, loops -1)
                    },
                    ResponseByServerToClient::Response404(s) => {
                        let (_, s) = s.recv()?;
                        recurs_client(s, loops -1)
                    },
                    ResponseByServerToClient::Response418(s) => {
                        let (_, s) = s.recv()?;
                        recurs_client(s, loops -1)
                    },
                })
            }
            2 => {
                let s: EndpointClientRequestPut = choose_mpst_client_to_all!(
                    s,
                    RequestByClientToProxyOne::RequestPut,
                    RequestByClientToProxyTwo::RequestPut,
                    RequestByClientToServer::RequestPut
                );

                let s = s.send(RequestPut {
                    version_protocol: 5,
                    header: String::from("PUT"),
                })?;

                offer_mpst!(s, {
                    ResponseByServerToClient::Response200(s) => {
                        let (_, s) = s.recv()?;
                        recurs_client(s, loops -1)
                    },
                    ResponseByServerToClient::Response404(s) => {
                        let (_, s) = s.recv()?;
                        recurs_client(s, loops -1)
                    },
                    ResponseByServerToClient::Response418(s) => {
                        let (_, s) = s.recv()?;
                        recurs_client(s, loops -1)
                    },
                })
            }
            3 => {
                let s: EndpointClientRequestPost = choose_mpst_client_to_all!(
                    s,
                    RequestByClientToProxyOne::RequestPost,
                    RequestByClientToProxyTwo::RequestPost,
                    RequestByClientToServer::RequestPost
                );

                let s = s.send(RequestPost {
                    version_protocol: 5,
                    header: String::from("POST"),
                })?;

                offer_mpst!(s, {
                    ResponseByServerToClient::Response200(s) => {
                        let (_, s) = s.recv()?;
                        recurs_client(s, loops -1)
                    },
                    ResponseByServerToClient::Response404(s) => {
                        let (_, s) = s.recv()?;
                        recurs_client(s, loops -1)
                    },
                    ResponseByServerToClient::Response418(s) => {
                        let (_, s) = s.recv()?;
                        recurs_client(s, loops -1)
                    },
                })
            }
            _ => panic!("Error, unexpected input"),
        }
    }
}

/////////////////////////

fn endpoint_proxyone(s: EndpointProxyOne) -> Result<(), Box<dyn Error>> {
    let (opentcpconnection, s) = s.recv()?;

    let s = s.send(opentcpconnection)?;

    offer_mpst!(s, {
            OpenTCPConnectionByServerToProxyOne::Fail(s) => {

                                let (fail, s) = s.recv()?;
                let s = s.send(fail)?;
                s.close()
            },
            OpenTCPConnectionByServerToProxyOne::Success(s) => {

                                let (fail, s) = s.recv()?;
                let s = s.send(fail)?;

                recurs_proxyone(s)
            },
    })
}

fn recurs_proxyone(s: RecursProxyOne) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        RequestByClientToProxyOne::RequestGet(s) => {

                        let (request, s) = s.recv()?;
            let s = s.send(request)?;

            offer_mpst!(s, {
                ResponseByServerToProxyOne::Response200(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxyone(s)
                },
                ResponseByServerToProxyOne::Response404(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxyone(s)
                },
                ResponseByServerToProxyOne::Response418(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxyone(s)
                },
            })
        },
        RequestByClientToProxyOne::RequestPut(s) => {

                        let (request, s) = s.recv()?;
            let s = s.send(request)?;

            offer_mpst!(s, {
                ResponseByServerToProxyOne::Response200(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxyone(s)
                },
                ResponseByServerToProxyOne::Response404(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxyone(s)
                },
                ResponseByServerToProxyOne::Response418(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxyone(s)
                },
            })
        },
        RequestByClientToProxyOne::RequestPost(s) => {

                        let (request, s) = s.recv()?;
            let s = s.send(request)?;

            offer_mpst!(s, {
                ResponseByServerToProxyOne::Response200(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxyone(s)
                },
                ResponseByServerToProxyOne::Response404(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxyone(s)
                },
                ResponseByServerToProxyOne::Response418(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxyone(s)
                },
            })
        },
        RequestByClientToProxyOne::Close(s) => {

                        let (request, s) = s.recv()?;
            let s = s.send(request)?;
            s.close()
        },
    })
}

/////////////////////////

fn endpoint_proxytwo(s: EndpointProxyTwo) -> Result<(), Box<dyn Error>> {
    let (opentcpconnection, s) = s.recv()?;

    let s = s.send(opentcpconnection)?;
    offer_mpst!(s, {
        OpenTCPConnectionByServerToProxyTwo::Fail(s) => {

                        let (fail, s) = s.recv()?;
            let s = s.send(fail)?;
            s.close()
        },
        OpenTCPConnectionByServerToProxyTwo::Success(s) => {

                        let (fail, s) = s.recv()?;
            let s = s.send(fail)?;
            recurs_proxytwo(s)
        },
    })
}

fn recurs_proxytwo(s: RecursProxyTwo) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        RequestByClientToProxyTwo::RequestGet(s) => {

                        let (request, s) = s.recv()?;
            let s = s.send(request)?;

            offer_mpst!(s, {
                ResponseByServerToProxyTwo::Response200(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxytwo(s)
                },
                ResponseByServerToProxyTwo::Response404(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxytwo(s)
                },
                ResponseByServerToProxyTwo::Response418(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxytwo(s)
                },
            })
        },
        RequestByClientToProxyTwo::RequestPut(s) => {

                        let (request, s) = s.recv()?;
            let s = s.send(request)?;

            offer_mpst!(s, {
                ResponseByServerToProxyTwo::Response200(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxytwo(s)
                },
                ResponseByServerToProxyTwo::Response404(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxytwo(s)
                },
                ResponseByServerToProxyTwo::Response418(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxytwo(s)
                },
            })
        },
        RequestByClientToProxyTwo::RequestPost(s) => {

                        let (request, s) = s.recv()?;
            let s = s.send(request)?;

            offer_mpst!(s, {
                ResponseByServerToProxyTwo::Response200(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxytwo(s)
                },
                ResponseByServerToProxyTwo::Response404(s) => {

                                        let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxytwo(s)
                },
                ResponseByServerToProxyTwo::Response418(s) => {

                    println!("ProxyTwo Request GET");
                    let (response, s) = s.recv()?;
                    let s = s.send(response)?;

                    recurs_proxytwo(s)
                },
            })
        },
        RequestByClientToProxyTwo::Close(s) => {

                        let (request, s) = s.recv()?;
            let s = s.send(request)?;
            s.close()
        },
    })
}

/////////////////////////

fn endpoint_server(s: EndpointServer) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;

    // 10 percent chance of failure
    match thread_rng().gen_range(1..10) {
        1 => {
            let s: EndpointServerFail = choose_mpst_server_to_all!(
                s,
                OpenTCPConnectionByServerToClient::Fail,
                OpenTCPConnectionByServerToProxyOne::Fail,
                OpenTCPConnectionByServerToProxyTwo::Fail
            );

            let s = s.send(Fail {})?;

            s.close()
        }
        _ => {
            let s: EndpointServerSuccess = choose_mpst_server_to_all!(
                s,
                OpenTCPConnectionByServerToClient::Success,
                OpenTCPConnectionByServerToProxyOne::Success,
                OpenTCPConnectionByServerToProxyTwo::Success
            );

            let s = s.send(Success {})?;

            recurs_server(s)
        }
    }
}

fn recurs_server(s: RecursServer) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        RequestByClientToServer::RequestGet(s) => {
            let (_, s) = s.recv()?;

            match thread_rng().gen_range(1..3) {
                1 => {
                    let s: EndpointServerResponse200 = choose_mpst_server_to_all!(
                        s,
                        ResponseByServerToClient::Response200,
                        ResponseByServerToProxyOne::Response200,
                        ResponseByServerToProxyTwo::Response200
                    );

                    let s = s.send(Response200 {
                        version_protocol: 5,
                        status_message: String::from("OK"),
                        header: String::from("200")
                    })?;

                    recurs_server(s)
                }
                2 => {
                    let s: EndpointServerResponse404 = choose_mpst_server_to_all!(
                        s,
                        ResponseByServerToClient::Response404,
                        ResponseByServerToProxyOne::Response404,
                        ResponseByServerToProxyTwo::Response404
                    );

                    let s = s.send(Response404 {
                        version_protocol: 5,
                        status_message: String::from("Resource not found"),
                        header: String::from("404")
                    })?;

                    recurs_server(s)
                }
                3 => {
                    let s: EndpointServerResponse418 = choose_mpst_server_to_all!(
                        s,
                        ResponseByServerToClient::Response418,
                        ResponseByServerToProxyOne::Response418,
                        ResponseByServerToProxyTwo::Response418
                    );

                    let s = s.send(Response418 {
                        version_protocol: 5,
                        status_message: String::from("I'm a tea pot"),
                        header: String::from("418")
                    })?;

                    recurs_server(s)
                }
                _ => panic!("Error, unexpected number")
            }
        },
        RequestByClientToServer::RequestPut(s) => {
            let (_, s) = s.recv()?;

            match thread_rng().gen_range(1..3) {
                1 => {
                    let s: EndpointServerResponse200 = choose_mpst_server_to_all!(
                        s,
                        ResponseByServerToClient::Response200,
                        ResponseByServerToProxyOne::Response200,
                        ResponseByServerToProxyTwo::Response200
                    );

                    let s = s.send(Response200 {
                        version_protocol: 5,
                        status_message: String::from("OK"),
                        header: String::from("200")
                    })?;

                    recurs_server(s)
                }
                2 => {
                    let s: EndpointServerResponse404 = choose_mpst_server_to_all!(
                        s,
                        ResponseByServerToClient::Response404,
                        ResponseByServerToProxyOne::Response404,
                        ResponseByServerToProxyTwo::Response404
                    );

                    let s = s.send(Response404 {
                        version_protocol: 5,
                        status_message: String::from("Resource not found"),
                        header: String::from("404")
                    })?;

                    recurs_server(s)
                }
                3 => {
                    let s: EndpointServerResponse418 = choose_mpst_server_to_all!(
                        s,
                        ResponseByServerToClient::Response418,
                        ResponseByServerToProxyOne::Response418,
                        ResponseByServerToProxyTwo::Response418
                    );

                    let s = s.send(Response418 {
                        version_protocol: 5,
                        status_message: String::from("I'm a tea pot"),
                        header: String::from("418")
                    })?;

                    recurs_server(s)
                }
                _ => panic!("Error, unexpected number")
            }
        },
        RequestByClientToServer::RequestPost(s) => {
            let (_, s) = s.recv()?;

            match thread_rng().gen_range(1..3) {
                1 => {
                    let s: EndpointServerResponse200 = choose_mpst_server_to_all!(
                        s,
                        ResponseByServerToClient::Response200,
                        ResponseByServerToProxyOne::Response200,
                        ResponseByServerToProxyTwo::Response200
                    );

                    let s = s.send(Response200 {
                        version_protocol: 5,
                        status_message: String::from("OK"),
                        header: String::from("200")
                    })?;

                    recurs_server(s)
                }
                2 => {
                    let s: EndpointServerResponse404 = choose_mpst_server_to_all!(
                        s,
                        ResponseByServerToClient::Response404,
                        ResponseByServerToProxyOne::Response404,
                        ResponseByServerToProxyTwo::Response404
                    );

                    let s = s.send(Response404 {
                        version_protocol: 5,
                        status_message: String::from("Resource not found"),
                        header: String::from("404")
                    })?;

                    recurs_server(s)
                }
                3 => {
                    let s: EndpointServerResponse418 = choose_mpst_server_to_all!(
                        s,
                        ResponseByServerToClient::Response418,
                        ResponseByServerToProxyOne::Response418,
                        ResponseByServerToProxyTwo::Response418
                    );

                    let s = s.send(Response418 {
                        version_protocol: 5,
                        status_message: String::from("I'm a tea pot"),
                        header: String::from("418")
                    })?;

                    recurs_server(s)
                }
                _ => panic!("Error, unexpected number")
            }
        },
        RequestByClientToServer::Close(s) => {
            let (_, s) = s.recv()?;

            s.close()
        },
    })
}

////////////////////////////////////////

fn all_mpst() {
    let (thread_client, thread_proxyone, thread_proxytwo, thread_server) = fork_mpst(
        black_box(endpoint_client),
        black_box(endpoint_proxyone),
        black_box(endpoint_proxytwo),
        black_box(endpoint_server),
    );

    thread_client.join().unwrap();
    thread_proxyone.join().unwrap();
    thread_proxytwo.join().unwrap();
    thread_server.join().unwrap();
}

/////////////////////////

pub fn http_main(c: &mut Criterion) {
    c.bench_function("HTTP baking", |b| b.iter(all_mpst));
}
