/////// CANNOT BE IMPLEMENTED: OVERFLOW EVALUATION!!!!11!111!!!

#![allow(clippy::type_complexity)]
#![recursion_limit = "256"]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{baker, fork_mpst_multi_interleaved, offer_mpst};

use rand::{thread_rng, Rng};

use std::error::Error;

use std::sync::Mutex;

// Create new MeshedChannels for four participants
baker!("interleaved", MeshedChannelsBarber, Barber, ShopBarber);

// Create new MeshedChannels for two participants
baker!("interleaved", MeshedChannelsClient, Client, ShopClient);

fork_mpst_multi_interleaved!(
    fork_mpst,
    MeshedChannelsBarber,
    2,
    2,
    MeshedChannelsClient,
    2,
    2
);

// Number of seats
static SEATS: usize = 3;

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
enum Branching0fromShopBarberToBarber {
    Done(MeshedChannelsBarber<End, RoleEnd, NameBarber>),
    Available(
        MeshedChannelsBarber<
            Send<
                Available,
                Recv<
                    Customer,
                    Recv<Description, Send<Haircut, Recv<Pay, Recurs0fromShopBarberToBarber>>>,
                >,
            >,
            RoleShopBarber<
                RoleShopBarber<
                    RoleShopBarber<
                        RoleShopBarber<RoleShopBarber<RoleShopBarber<RoleShopBarber<RoleEnd>>>>,
                    >,
                >,
            >,
            NameBarber,
        >,
    ),
}
type Recurs0fromShopBarberToBarber = Recv<Branching0fromShopBarberToBarber, End>;

// ShopBarber
type Choose0fromShopBarberToBarber = <Recurs0fromShopBarberToBarber as Session>::Dual;

// Client
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
            Recv<
                Available,
                Send<
                    Customer,
                    Send<Description, Recv<Haircut, Send<Pay, Recurs0fromShopClientToClient>>>,
                >,
            >,
            RoleShopClient<
                RoleShopClient<
                    RoleShopClient<RoleShopClient<RoleShopClient<RoleShopClient<RoleEnd>>>>,
                >,
            >,
            NameClient,
        >,
    ),
}
type Recurs0fromShopClientToClient = Recv<Branching0fromShopClientToClient, End>;

// ShopClient
type Choose0fromShopClientToClient = <Recurs0fromShopClientToClient as Session>::Dual;

// Creating the MP sessions
// Barber
type EndpointBarber0 = MeshedChannelsBarber<
    Recv<Branching0fromShopBarberToBarber, End>,
    RoleShopBarber<RoleEnd>,
    NameBarber,
>;

// ShopBarber
type EndpointShopBarber0 = MeshedChannelsBarber<
    Send<Branching0fromShopBarberToBarber, End>,
    RoleBroadcast,
    NameShopBarber,
>;
type EndpointShopBarberDone = MeshedChannelsBarber<End, RoleEnd, NameShopBarber>;
type EndpointShopBarberAvailabe = MeshedChannelsBarber<
    Recv<
        Available,
        Send<
            Customer,
            Send<
                Description,
                Recv<Haircut, Send<Pay, Send<Branching0fromShopBarberToBarber, End>>>,
            >,
        >,
    >,
    RoleBarber<RoleBarber<RoleBarber<RoleBarber<RoleBarber<RoleBroadcast>>>>>,
    NameShopBarber,
>;

// Client
type EndpointClient0 = MeshedChannelsClient<
    Recv<Branching0fromShopClientToClient, End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;

// ShopClient
type EndpointShopClient0 = MeshedChannelsClient<
    Send<Branching0fromShopClientToClient, End>,
    RoleBroadcast,
    NameShopClient,
>;
type EndpointShopClientDone = MeshedChannelsClient<End, RoleEnd, NameShopClient>;
type EndpointShopClientAvailable = MeshedChannelsClient<
    Send<
        Seat,
        Send<
            Ready,
            Recv<
                Description,
                Send<Haircut, Recv<Pay, Send<Branching0fromShopClientToClient, End>>>,
            >,
        >,
    >,
    RoleClient<RoleClient<RoleClient<RoleClient<RoleClient<RoleClient<RoleBroadcast>>>>>>,
    NameShopClient,
>;

/////////////////////////

// Barber
fn recurs_barber(s: EndpointBarber0) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromShopBarberToBarber::Done(s) => {
            s.close()
        },
        Branching0fromShopBarberToBarber::Available(s) => {

            let s = s.send(Available {  })?;

            let (_customer, s) = s.recv()?;

            let (_description, s) = s.recv()?;

            let s = s.send(Haircut {  })?;

            let (_pay, s) = s.recv()?;

            recurs_barber(s)
        },
    })
}

// Shop
fn endpoint_shop(
    s_shop_barber: EndpointShopBarber0,
    s_shop_client: EndpointShopClient0,
) -> Result<(), Box<dyn Error>> {
    let loops: i32 = thread_rng().gen_range(50..100);

    recurs_shop(s_shop_barber, s_shop_client, loops, Mutex::new(0))
}

fn recurs_shop(
    s_shop_barber: EndpointShopBarber0,
    s_shop_client: EndpointShopClient0,
    loops: i32,
    number_clients: Mutex<i32>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        // If end of the day
        i if i < 0 => {
            let s_shop_barber: EndpointShopBarberDone = choose_mpst_shopbarber_to_all!(
                s_shop_barber,
                Branching0fromShopBarberToBarber::Done,
            );

            s_shop_barber.close();

            let s_shop_client: EndpointShopClientDone = choose_mpst_shopclient_to_all!(
                s_shop_client,
                Branching0fromShopClientToClient::Done,
            );

            s_shop_client.close()
        }
        _ => {
            // A client will come with a proba of 10%
            if thread_rng().gen_range(1..=10) == 1 && number_clients.lock().unwrap() < SEATS {
                let s_shop_barber: EndpointShopBarberAvailabe = choose_mpst_shopbarber_to_all!(
                    s_shop_barber,
                    Branching0fromShopBarberToBarber::Available,
                );

                let s_shop_client: EndpointShopClientAvailable = choose_mpst_shopclient_to_all!(
                    s_shop_client,
                    Branching0fromShopClientToClient::Available,
                );

                let s_shop_client = s_shop_client.send(Seat {})?;
                let (_available, s) = s_shop_barber.recv()?;
                let s_shop_client = s_shop_client.send(Ready {})?;
                let s_shop_barber = s_shop_barber.send(Customer {})?;

                // Decription
                let (description, s_shop_client) = s_shop_client.recv()?;
                let s_shop_barber = s_shop_barber.send(description)?;

                // Haircut
                let (haircut, s_shop_barber) = s_shop_barber.recv()?;
                let s_shop_client = s_shop_client.send(haircut)?;

                // Pay
                let (pay, s_shop_client) = s_shop_client.recv()?;
                let s_shop_barber = s_shop_barber.send(pay)?;

                recurs_shop(s_shop_barber, s_shop_client, loops - 1, number_clients + 1)
            } else {
                recurs_shop(s_shop_barber, s_shop_client, loops - 1, number_clients)
            }
        }
    }
}

// Client
fn recurs_client(s: EndpointClient0) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromShopClientToClient::Done(s) => {
            s.close()
        },
        Branching0fromShopClientToClient::Full(s) => {
            let (_full, s) = s.recv()?;

            recurs_client(s)
        },
        Branching0fromShopClientToClient::Available(s) => {

            let (_seat, s) = s.recv()?;

            let (_ready, s) = s.recv()?;

            let s = s.send(Description {  })?;

            let (_haircut, s) = s.recv()?;

            let s = s.send(Pay {  })?;

            recurs_client(s)
        },
    })
}

/////////////////////////

fn main() {
    let (thread_barber, thread_client, thread_shop) =
        fork_mpst(recurs_barber, recurs_client, endpoint_shop);

    thread_barber.join().unwrap();
    thread_client.join().unwrap();
    thread_shop.join().unwrap();
}
