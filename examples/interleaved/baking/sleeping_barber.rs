#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{generate, offer_mpst};

use rand::{thread_rng, Rng};

use std::error::Error;

generate!(
    "interleaved",
    MeshedChannelsBarber,
    Barber,
    ShopBarber,
    2,
    MeshedChannelsClient,
    Client,
    ShopClient,
    2,
    fork_mpst
);

// Number of seats
static SEATS: usize = 3;
static LOOPS: usize = 100;

// Struct for labels
struct Full;
struct Seat;
struct Available;
struct Ready;
struct Customer;
struct Description;
struct Haircut;
struct Pay;

// Barber
type Recurs0fromShopBarberToBarber = Recv<Branching0fromShopBarberToBarber, End>;

enum Branching0fromShopBarberToBarber {
    Done(MeshedChannelsBarber<End, RoleEnd, NameBarber>),
    Available(
        MeshedChannelsBarber<
            Send<Available, Recurs1fromShopBarberToBarber>,
            RoleShopBarber<RoleShopBarber<RoleEnd>>,
            NameBarber,
        >,
    ),
}
type Recurs1fromShopBarberToBarber = Recv<Branching1fromShopBarberToBarber, End>;

enum Branching1fromShopBarberToBarber {
    Available(
        MeshedChannelsBarber<
            Recv<Customer, Recurs2fromShopBarberToBarber>,
            RoleShopBarber<RoleShopBarber<RoleEnd>>,
            NameBarber,
        >,
    ),
}
type Recurs2fromShopBarberToBarber = Recv<Branching2fromShopBarberToBarber, End>;

enum Branching2fromShopBarberToBarber {
    Available(
        MeshedChannelsBarber<
            Recv<Description, Recurs3fromShopBarberToBarber>,
            RoleShopBarber<RoleShopBarber<RoleEnd>>,
            NameBarber,
        >,
    ),
}
type Recurs3fromShopBarberToBarber = Recv<Branching3fromShopBarberToBarber, End>;

enum Branching3fromShopBarberToBarber {
    Available(
        MeshedChannelsBarber<
            Send<Haircut, Recurs4fromShopBarberToBarber>,
            RoleShopBarber<RoleShopBarber<RoleEnd>>,
            NameBarber,
        >,
    ),
}
type Recurs4fromShopBarberToBarber = Recv<Branching4fromShopBarberToBarber, End>;

enum Branching4fromShopBarberToBarber {
    Available(
        MeshedChannelsBarber<
            Recv<Pay, Recurs5fromShopBarberToBarber>,
            RoleShopBarber<RoleShopBarber<RoleEnd>>,
            NameBarber,
        >,
    ),
}
type Recurs5fromShopBarberToBarber = Recv<Branching0fromShopBarberToBarber, End>;

// ShopBarber
type Choose0fromShopBarberToBarber = <Recurs0fromShopBarberToBarber as Session>::Dual;
type Choose1fromShopBarberToBarber = <Recurs1fromShopBarberToBarber as Session>::Dual;
type Choose2fromShopBarberToBarber = <Recurs2fromShopBarberToBarber as Session>::Dual;
type Choose3fromShopBarberToBarber = <Recurs3fromShopBarberToBarber as Session>::Dual;
type Choose4fromShopBarberToBarber = <Recurs4fromShopBarberToBarber as Session>::Dual;
type Choose5fromShopBarberToBarber = <Recurs5fromShopBarberToBarber as Session>::Dual;

// Client
type Recurs0fromShopClientToClient = Recv<Branching0fromShopClientToClient, End>;

enum Branching0fromShopClientToClient {
    Done(MeshedChannelsClient<End, RoleEnd, NameClient>),
    Full(
        MeshedChannelsClient<
            Recv<Full, Recv<Branching0fromShopClientToClient, End>>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
    Available(
        MeshedChannelsClient<
            Recv<Seat, Recurs1fromShopClientToClient>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
}
type Recurs1fromShopClientToClient = Recv<Branching1fromShopClientToClient, End>;

enum Branching1fromShopClientToClient {
    Available(
        MeshedChannelsClient<
            Recv<Ready, Recurs2fromShopClientToClient>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
}
type Recurs2fromShopClientToClient = Recv<Branching2fromShopClientToClient, End>;

enum Branching2fromShopClientToClient {
    Available(
        MeshedChannelsClient<
            Send<Description, Recurs3fromShopClientToClient>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
}
type Recurs3fromShopClientToClient = Recv<Branching3fromShopClientToClient, End>;

enum Branching3fromShopClientToClient {
    Available(
        MeshedChannelsClient<
            Recv<Haircut, Recurs4fromShopClientToClient>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
}
type Recurs4fromShopClientToClient = Recv<Branching4fromShopClientToClient, End>;

enum Branching4fromShopClientToClient {
    Available(
        MeshedChannelsClient<
            Send<Pay, Recurs5fromShopClientToClient>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
}
type Recurs5fromShopClientToClient = Recv<Branching0fromShopClientToClient, End>;

// ShopClient
type Choose0fromShopClientToClient = <Recurs0fromShopClientToClient as Session>::Dual;
type Choose1fromShopClientToClient = <Recurs1fromShopClientToClient as Session>::Dual;
type Choose2fromShopClientToClient = <Recurs2fromShopClientToClient as Session>::Dual;
type Choose3fromShopClientToClient = <Recurs3fromShopClientToClient as Session>::Dual;
type Choose4fromShopClientToClient = <Recurs4fromShopClientToClient as Session>::Dual;
type Choose5fromShopClientToClient = <Recurs5fromShopClientToClient as Session>::Dual;

// Creating the MP sessions
// Barber
type EndpointBarber0 =
    MeshedChannelsBarber<Recurs0fromShopBarberToBarber, RoleShopBarber<RoleEnd>, NameBarber>;
type EndpointBarber1 =
    MeshedChannelsBarber<Recurs1fromShopBarberToBarber, RoleShopBarber<RoleEnd>, NameBarber>;
type EndpointBarber2 =
    MeshedChannelsBarber<Recurs2fromShopBarberToBarber, RoleShopBarber<RoleEnd>, NameBarber>;
type EndpointBarber3 =
    MeshedChannelsBarber<Recurs3fromShopBarberToBarber, RoleShopBarber<RoleEnd>, NameBarber>;
type EndpointBarber4 =
    MeshedChannelsBarber<Recurs4fromShopBarberToBarber, RoleShopBarber<RoleEnd>, NameBarber>;

// ShopBarber
type EndpointShopBarber0 =
    MeshedChannelsBarber<Choose0fromShopBarberToBarber, RoleBroadcast, NameShopBarber>;
type EndpointShopBarberDone = MeshedChannelsBarber<End, RoleEnd, NameShopBarber>;
// Recurs
type EndpointShopBarberRecurs1 =
    MeshedChannelsBarber<Choose1fromShopBarberToBarber, RoleBroadcast, NameShopBarber>;
type EndpointShopBarberRecurs2 =
    MeshedChannelsBarber<Choose2fromShopBarberToBarber, RoleBroadcast, NameShopBarber>;
type EndpointShopBarberRecurs3 =
    MeshedChannelsBarber<Choose3fromShopBarberToBarber, RoleBroadcast, NameShopBarber>;
type EndpointShopBarberRecurs4 =
    MeshedChannelsBarber<Choose4fromShopBarberToBarber, RoleBroadcast, NameShopBarber>;
// Full branches
type EndpointShopBarberAvailabe = MeshedChannelsBarber<
    Recv<Available, Choose1fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberCustomer = MeshedChannelsBarber<
    Send<Customer, Choose2fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberDescription = MeshedChannelsBarber<
    Send<Description, Choose3fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberHaircut = MeshedChannelsBarber<
    Recv<Haircut, Choose4fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberPay = MeshedChannelsBarber<
    Send<Pay, Choose5fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;

// Client
type EndpointClient0 = MeshedChannelsClient<
    Recv<Branching0fromShopClientToClient, End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient1 = MeshedChannelsClient<
    Recv<Branching1fromShopClientToClient, End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient2 = MeshedChannelsClient<
    Recv<Branching2fromShopClientToClient, End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient3 = MeshedChannelsClient<
    Recv<Branching3fromShopClientToClient, End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient4 = MeshedChannelsClient<
    Recv<Branching4fromShopClientToClient, End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;

// ShopClient
type EndpointShopClient0 =
    MeshedChannelsClient<Choose0fromShopClientToClient, RoleBroadcast, NameShopClient>;
type EndpointShopClientDone = MeshedChannelsClient<End, RoleEnd, NameShopClient>;
type EndpointShopClientFull = MeshedChannelsClient<
    Send<Full, Choose5fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
// Recurs
type EndpointShopClientRecurs1 =
    MeshedChannelsClient<Choose1fromShopClientToClient, RoleBroadcast, NameShopClient>;
type EndpointShopClientRecurs2 =
    MeshedChannelsClient<Choose2fromShopClientToClient, RoleBroadcast, NameShopClient>;
type EndpointShopClientRecurs3 =
    MeshedChannelsClient<Choose3fromShopClientToClient, RoleBroadcast, NameShopClient>;
type EndpointShopClientRecurs4 =
    MeshedChannelsClient<Choose4fromShopClientToClient, RoleBroadcast, NameShopClient>;
// Full branches
type EndpointShopClientSeat = MeshedChannelsClient<
    Send<Seat, Choose1fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientReady = MeshedChannelsClient<
    Send<Ready, Choose2fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientDescription = MeshedChannelsClient<
    Recv<Description, Choose3fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientHaircut = MeshedChannelsClient<
    Send<Haircut, Choose4fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientPay = MeshedChannelsClient<
    Recv<Pay, Choose5fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;

/////////////////////////

// Barber
fn recurs_0_barber(s: EndpointBarber0) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromShopBarberToBarber::Done(s) => {
            s.close()
        },
        Branching0fromShopBarberToBarber::Available(s) => {

            let s = s.send(Available {  })?;

            recurs_1_barber(s)
        },
    })
}
fn recurs_1_barber(s: EndpointBarber1) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromShopBarberToBarber::Available(s) => {

            let (_customer, s) = s.recv()?;

            recurs_2_barber(s)
        },
    })
}
fn recurs_2_barber(s: EndpointBarber2) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching2fromShopBarberToBarber::Available(s) => {

            let (_description, s) = s.recv()?;

            recurs_3_barber(s)
        },
    })
}
fn recurs_3_barber(s: EndpointBarber3) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching3fromShopBarberToBarber::Available(s) => {

            let s = s.send(Haircut {  })?;

            recurs_4_barber(s)
        },
    })
}
fn recurs_4_barber(s: EndpointBarber4) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching4fromShopBarberToBarber::Available(s) => {

            let (_pay, s) = s.recv()?;

            recurs_0_barber(s)
        },
    })
}

// Shop
fn endpoint_shop(
    s_shop_barber: EndpointShopBarber0,
    s_shop_client: EndpointShopClient0,
) -> Result<(), Box<dyn Error>> {
    recurs_0_shop(s_shop_barber, s_shop_client, LOOPS, 0)
}

fn recurs_0_shop(
    s_shop_barber: EndpointShopBarber0,
    s_shop_client: EndpointShopClient0,
    loops: usize,
    number_clients: usize,
) -> Result<(), Box<dyn Error>> {
    match loops {
        // If end of the day
        0 => {
            let s_shop_barber: EndpointShopBarberDone = choose_mpst_shopbarber_to_all!(
                s_shop_barber,
                Branching0fromShopBarberToBarber::Done,
            );

            let _ = s_shop_barber.close();

            let s_shop_client: EndpointShopClientDone = choose_mpst_shopclient_to_all!(
                s_shop_client,
                Branching0fromShopClientToClient::Done,
            );

            s_shop_client.close()
        }
        _ => {
            // A client will come with a proba of 10%
            if thread_rng().gen_range(1..=10) == 1 && number_clients < SEATS {
                let s_shop_barber: EndpointShopBarberAvailabe = choose_mpst_shopbarber_to_all!(
                    s_shop_barber,
                    Branching0fromShopBarberToBarber::Available,
                );

                let s_shop_client: EndpointShopClientSeat = choose_mpst_shopclient_to_all!(
                    s_shop_client,
                    Branching0fromShopClientToClient::Available,
                );

                let s_shop_client = s_shop_client.send(Seat {})?;
                let (_available, s_shop_barber) = s_shop_barber.recv()?;

                recurs_1_shop(s_shop_barber, s_shop_client, loops - 1, number_clients + 1)
            } else {
                let s_shop_client: EndpointShopClientFull = choose_mpst_shopclient_to_all!(
                    s_shop_client,
                    Branching0fromShopClientToClient::Full,
                );

                let s_shop_client = s_shop_client.send(Full {})?;

                recurs_0_shop(s_shop_barber, s_shop_client, loops - 1, number_clients)
            }
        }
    }
}

fn recurs_1_shop(
    s_shop_barber: EndpointShopBarberRecurs1,
    s_shop_client: EndpointShopClientRecurs1,
    loops: usize,
    number_clients: usize,
) -> Result<(), Box<dyn Error>> {
    let s_shop_barber: EndpointShopBarberCustomer =
        choose_mpst_shopbarber_to_all!(s_shop_barber, Branching1fromShopBarberToBarber::Available);

    let s_shop_client: EndpointShopClientReady =
        choose_mpst_shopclient_to_all!(s_shop_client, Branching1fromShopClientToClient::Available);

    let s_shop_client = s_shop_client.send(Ready {})?;
    let s_shop_barber = s_shop_barber.send(Customer {})?;

    recurs_2_shop(s_shop_barber, s_shop_client, loops, number_clients)
}

fn recurs_2_shop(
    s_shop_barber: EndpointShopBarberRecurs2,
    s_shop_client: EndpointShopClientRecurs2,
    loops: usize,
    number_clients: usize,
) -> Result<(), Box<dyn Error>> {
    let s_shop_barber: EndpointShopBarberDescription =
        choose_mpst_shopbarber_to_all!(s_shop_barber, Branching2fromShopBarberToBarber::Available);

    let s_shop_client: EndpointShopClientDescription =
        choose_mpst_shopclient_to_all!(s_shop_client, Branching2fromShopClientToClient::Available);

    // Decription
    let (description, s_shop_client) = s_shop_client.recv()?;
    let s_shop_barber = s_shop_barber.send(description)?;

    recurs_3_shop(s_shop_barber, s_shop_client, loops, number_clients)
}

fn recurs_3_shop(
    s_shop_barber: EndpointShopBarberRecurs3,
    s_shop_client: EndpointShopClientRecurs3,
    loops: usize,
    number_clients: usize,
) -> Result<(), Box<dyn Error>> {
    let s_shop_barber: EndpointShopBarberHaircut =
        choose_mpst_shopbarber_to_all!(s_shop_barber, Branching3fromShopBarberToBarber::Available);

    let s_shop_client: EndpointShopClientHaircut =
        choose_mpst_shopclient_to_all!(s_shop_client, Branching3fromShopClientToClient::Available);

    // Haircut
    let (haircut, s_shop_barber) = s_shop_barber.recv()?;
    let s_shop_client = s_shop_client.send(haircut)?;

    recurs_4_shop(s_shop_barber, s_shop_client, loops, number_clients)
}

fn recurs_4_shop(
    s_shop_barber: EndpointShopBarberRecurs4,
    s_shop_client: EndpointShopClientRecurs4,
    loops: usize,
    number_clients: usize,
) -> Result<(), Box<dyn Error>> {
    let s_shop_barber: EndpointShopBarberPay =
        choose_mpst_shopbarber_to_all!(s_shop_barber, Branching4fromShopBarberToBarber::Available);

    let s_shop_client: EndpointShopClientPay =
        choose_mpst_shopclient_to_all!(s_shop_client, Branching4fromShopClientToClient::Available);

    // Pay
    let (pay, s_shop_client) = s_shop_client.recv()?;
    let s_shop_barber = s_shop_barber.send(pay)?;

    recurs_0_shop(s_shop_barber, s_shop_client, loops, number_clients - 1)
}

// Client
fn recurs_0_client(s: EndpointClient0) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromShopClientToClient::Done(s) => {
            s.close()
        },
        Branching0fromShopClientToClient::Full(s) => {
            let (_full, s) = s.recv()?;

            recurs_0_client(s)
        },
        Branching0fromShopClientToClient::Available(s) => {

            let (_seat, s) = s.recv()?;

            recurs_1_client(s)
        },
    })
}
fn recurs_1_client(s: EndpointClient1) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromShopClientToClient::Available(s) => {

            let (_ready, s) = s.recv()?;

            recurs_2_client(s)
        },
    })
}
fn recurs_2_client(s: EndpointClient2) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching2fromShopClientToClient::Available(s) => {

            let s = s.send(Description {  })?;

            recurs_3_client(s)
        },
    })
}
fn recurs_3_client(s: EndpointClient3) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching3fromShopClientToClient::Available(s) => {

            let (_haircut, s) = s.recv()?;

            recurs_4_client(s)
        },
    })
}
fn recurs_4_client(s: EndpointClient4) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching4fromShopClientToClient::Available(s) => {

            let s = s.send(Pay {  })?;

            recurs_0_client(s)
        },
    })
}

/////////////////////////

fn main() {
    let (thread_barber, thread_client, thread_shop) =
        fork_mpst(recurs_0_barber, recurs_0_client, endpoint_shop);

    thread_barber.join().unwrap();
    thread_client.join().unwrap();
    thread_shop.join().unwrap();
}
