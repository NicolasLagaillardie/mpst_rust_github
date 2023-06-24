#![allow(clippy::type_complexity)]
#![recursion_limit = "256"]

use mpstthree::baker;
use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

baker!(
    "timed_interleaved",
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
type Recurs0fromShopBarberToBarber =
    RecvTimed<Branching0fromShopBarberToBarber, 'a', 0, true, 10, true, ' ', End>;

enum Branching0fromShopBarberToBarber {
    Done(MeshedChannelsBarber<End, RoleEnd, NameBarber>),
    Available(
        MeshedChannelsBarber<
            SendTimed<Available, 'a', 0, true, 10, true, ' ', Recurs1fromShopBarberToBarber>,
            RoleShopBarber<RoleShopBarber<RoleEnd>>,
            NameBarber,
        >,
    ),
}
type Recurs1fromShopBarberToBarber =
    RecvTimed<Branching1fromShopBarberToBarber, 'a', 0, true, 10, true, ' ', End>;

enum Branching1fromShopBarberToBarber {
    Available(
        MeshedChannelsBarber<
            RecvTimed<Customer, 'a', 0, true, 10, true, ' ', Recurs2fromShopBarberToBarber>,
            RoleShopBarber<RoleShopBarber<RoleEnd>>,
            NameBarber,
        >,
    ),
}
type Recurs2fromShopBarberToBarber =
    RecvTimed<Branching2fromShopBarberToBarber, 'a', 0, true, 10, true, ' ', End>;

enum Branching2fromShopBarberToBarber {
    Available(
        MeshedChannelsBarber<
            RecvTimed<Description, 'a', 0, true, 10, true, ' ', Recurs3fromShopBarberToBarber>,
            RoleShopBarber<RoleShopBarber<RoleEnd>>,
            NameBarber,
        >,
    ),
}
type Recurs3fromShopBarberToBarber =
    RecvTimed<Branching3fromShopBarberToBarber, 'a', 0, true, 10, true, ' ', End>;

enum Branching3fromShopBarberToBarber {
    Available(
        MeshedChannelsBarber<
            SendTimed<Haircut, 'a', 0, true, 10, true, ' ', Recurs4fromShopBarberToBarber>,
            RoleShopBarber<RoleShopBarber<RoleEnd>>,
            NameBarber,
        >,
    ),
}
type Recurs4fromShopBarberToBarber =
    RecvTimed<Branching4fromShopBarberToBarber, 'a', 0, true, 10, true, ' ', End>;

enum Branching4fromShopBarberToBarber {
    Available(
        MeshedChannelsBarber<
            RecvTimed<Pay, 'a', 0, true, 10, true, ' ', Recurs5fromShopBarberToBarber>,
            RoleShopBarber<RoleShopBarber<RoleEnd>>,
            NameBarber,
        >,
    ),
}
type Recurs5fromShopBarberToBarber =
    RecvTimed<Branching0fromShopBarberToBarber, 'a', 0, true, 10, true, ' ', End>;

// ShopBarber
type Choose0fromShopBarberToBarber = <Recurs0fromShopBarberToBarber as Session>::Dual;
type Choose1fromShopBarberToBarber = <Recurs1fromShopBarberToBarber as Session>::Dual;
type Choose2fromShopBarberToBarber = <Recurs2fromShopBarberToBarber as Session>::Dual;
type Choose3fromShopBarberToBarber = <Recurs3fromShopBarberToBarber as Session>::Dual;
type Choose4fromShopBarberToBarber = <Recurs4fromShopBarberToBarber as Session>::Dual;
type Choose5fromShopBarberToBarber = <Recurs5fromShopBarberToBarber as Session>::Dual;

// Client
type Recurs0fromShopClientToClient =
    RecvTimed<Branching0fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>;

enum Branching0fromShopClientToClient {
    Done(MeshedChannelsClient<End, RoleEnd, NameClient>),
    Full(
        MeshedChannelsClient<
            RecvTimed<
                Full,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                RecvTimed<Branching0fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
            >,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
    Available(
        MeshedChannelsClient<
            RecvTimed<Seat, 'a', 0, true, 10, true, ' ', Recurs1fromShopClientToClient>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
}
type Recurs1fromShopClientToClient =
    RecvTimed<Branching1fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>;

enum Branching1fromShopClientToClient {
    Available(
        MeshedChannelsClient<
            RecvTimed<Ready, 'a', 0, true, 10, true, ' ', Recurs2fromShopClientToClient>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
}
type Recurs2fromShopClientToClient =
    RecvTimed<Branching2fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>;

enum Branching2fromShopClientToClient {
    Available(
        MeshedChannelsClient<
            SendTimed<Description, 'a', 0, true, 10, true, ' ', Recurs3fromShopClientToClient>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
}
type Recurs3fromShopClientToClient =
    RecvTimed<Branching3fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>;

enum Branching3fromShopClientToClient {
    Available(
        MeshedChannelsClient<
            RecvTimed<Haircut, 'a', 0, true, 10, true, ' ', Recurs4fromShopClientToClient>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
}
type Recurs4fromShopClientToClient =
    RecvTimed<Branching4fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>;

enum Branching4fromShopClientToClient {
    Available(
        MeshedChannelsClient<
            SendTimed<Pay, 'a', 0, true, 10, true, ' ', Recurs5fromShopClientToClient>,
            RoleShopClient<RoleShopClient<RoleEnd>>,
            NameClient,
        >,
    ),
}
type Recurs5fromShopClientToClient =
    RecvTimed<Branching0fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>;

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
    RecvTimed<Available, 'a', 0, true, 10, true, ' ', Choose1fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberCustomer = MeshedChannelsBarber<
    SendTimed<Customer, 'a', 0, true, 10, true, ' ', Choose2fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberDescription = MeshedChannelsBarber<
    SendTimed<Description, 'a', 0, true, 10, true, ' ', Choose3fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberHaircut = MeshedChannelsBarber<
    RecvTimed<Haircut, 'a', 0, true, 10, true, ' ', Choose4fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberPay = MeshedChannelsBarber<
    SendTimed<Pay, 'a', 0, true, 10, true, ' ', Choose5fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;

// Client
type EndpointClient0 = MeshedChannelsClient<
    RecvTimed<Branching0fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient1 = MeshedChannelsClient<
    RecvTimed<Branching1fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient2 = MeshedChannelsClient<
    RecvTimed<Branching2fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient3 = MeshedChannelsClient<
    RecvTimed<Branching3fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient4 = MeshedChannelsClient<
    RecvTimed<Branching4fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;

// ShopClient
type EndpointShopClient0 =
    MeshedChannelsClient<Choose0fromShopClientToClient, RoleBroadcast, NameShopClient>;
type EndpointShopClientDone = MeshedChannelsClient<End, RoleEnd, NameShopClient>;
type EndpointShopClientFull = MeshedChannelsClient<
    SendTimed<Full, 'a', 0, true, 10, true, ' ', Choose5fromShopClientToClient>,
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
    SendTimed<Seat, 'a', 0, true, 10, true, ' ', Choose1fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientReady = MeshedChannelsClient<
    SendTimed<Ready, 'a', 0, true, 10, true, ' ', Choose2fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientDescription = MeshedChannelsClient<
    RecvTimed<Description, 'a', 0, true, 10, true, ' ', Choose3fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientHaircut = MeshedChannelsClient<
    SendTimed<Haircut, 'a', 0, true, 10, true, ' ', Choose4fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientPay = MeshedChannelsClient<
    RecvTimed<Pay, 'a', 0, true, 10, true, ' ', Choose5fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;

/////////////////////////

// Barber
fn recurs_0_barber(
    s: EndpointBarber0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_1_barber(s, all_clocks)
}
fn recurs_1_barber(
    s: EndpointBarber0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branching0fromShopBarberToBarber::Done(s) => {
                s.close()
            },
            Branching0fromShopBarberToBarber::Available(s) => {

                let s = s.send(Available {  }, all_clocks)?;

                recurs_2_barber(s, all_clocks)
            },
        }
    )
}
fn recurs_2_barber(
    s: EndpointBarber1,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branching1fromShopBarberToBarber::Available(s) => {

                let (_customer, s) = s.recv(all_clocks)?;

                recurs_3_barber(s, all_clocks)
            },
        }
    )
}
fn recurs_3_barber(
    s: EndpointBarber2,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branching2fromShopBarberToBarber::Available(s) => {

                let (_description, s) = s.recv(all_clocks)?;

                recurs_4_barber(s, all_clocks)
            },
        }
    )
}
fn recurs_4_barber(
    s: EndpointBarber3,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branching3fromShopBarberToBarber::Available(s) => {

                let s = s.send(Haircut {  }, all_clocks)?;

                recurs_5_barber(s, all_clocks)
            },
        }
    )
}
fn recurs_5_barber(
    s: EndpointBarber4,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branching4fromShopBarberToBarber::Available(s) => {

                let (_pay, s) = s.recv(all_clocks)?;

                recurs_1_barber(s, all_clocks)
            },
        }
    )
}

// Shop
fn endpoint_shop(
    s_shop_barber: EndpointShopBarber0,
    all_clocks_one: &mut HashMap<char, Instant>,
    s_shop_client: EndpointShopClient0,
    all_clocks_two: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks_one.insert('a', Instant::now());
    all_clocks_two.insert('a', Instant::now());

    recurs_0_shop(
        s_shop_barber,
        all_clocks_one,
        s_shop_client,
        all_clocks_two,
        LOOPS,
        0,
    )
}

fn recurs_0_shop(
    s_shop_barber: EndpointShopBarber0,
    all_clocks_one: &mut HashMap<char, Instant>,
    s_shop_client: EndpointShopClient0,
    all_clocks_two: &mut HashMap<char, Instant>,
    loops: usize,
    number_clients: usize,
) -> Result<(), Box<dyn Error>> {
    match loops {
        // If end of the day
        i if i == 0 => {
            let s_shop_barber: EndpointShopBarberDone = choose_mpst_shopbarber_to_all!(
                s_shop_barber,
                all_clocks_one,
                Branching0fromShopBarberToBarber::Done,
            );

            let _ = s_shop_barber.close();

            let s_shop_client: EndpointShopClientDone = choose_mpst_shopclient_to_all!(
                s_shop_client,
                all_clocks_two,
                Branching0fromShopClientToClient::Done,
            );

            s_shop_client.close()
        }
        _ => {
            // A client will come with a proba of 10%
            if thread_rng().gen_range(1..=10) == 1 && number_clients < SEATS {
                let s_shop_barber: EndpointShopBarberAvailabe = choose_mpst_shopbarber_to_all!(
                    s_shop_barber,
                    all_clocks_one,
                    Branching0fromShopBarberToBarber::Available,
                );

                let s_shop_client: EndpointShopClientSeat = choose_mpst_shopclient_to_all!(
                    s_shop_client,
                    all_clocks_two,
                    Branching0fromShopClientToClient::Available,
                );

                let s_shop_client = s_shop_client.send(Seat {}, all_clocks_two)?;
                let (_available, s_shop_barber) = s_shop_barber.recv(all_clocks_one)?;

                recurs_1_shop(
                    s_shop_barber,
                    all_clocks_one,
                    s_shop_client,
                    all_clocks_two,
                    loops - 1,
                    number_clients + 1,
                )
            } else {
                let s_shop_client: EndpointShopClientFull = choose_mpst_shopclient_to_all!(
                    s_shop_client,
                    all_clocks_two,
                    Branching0fromShopClientToClient::Full,
                );

                let s_shop_client = s_shop_client.send(Full {}, all_clocks_two)?;

                recurs_0_shop(
                    s_shop_barber,
                    all_clocks_one,
                    s_shop_client,
                    all_clocks_two,
                    loops - 1,
                    number_clients,
                )
            }
        }
    }
}

fn recurs_1_shop(
    s_shop_barber: EndpointShopBarberRecurs1,
    all_clocks_one: &mut HashMap<char, Instant>,
    s_shop_client: EndpointShopClientRecurs1,
    all_clocks_two: &mut HashMap<char, Instant>,
    loops: usize,
    number_clients: usize,
) -> Result<(), Box<dyn Error>> {
    let s_shop_barber: EndpointShopBarberCustomer = choose_mpst_shopbarber_to_all!(
        s_shop_barber,
        all_clocks_one,
        Branching1fromShopBarberToBarber::Available
    );

    let s_shop_client: EndpointShopClientReady = choose_mpst_shopclient_to_all!(
        s_shop_client,
        all_clocks_two,
        Branching1fromShopClientToClient::Available
    );

    let s_shop_client = s_shop_client.send(Ready {}, all_clocks_two)?;
    let s_shop_barber = s_shop_barber.send(Customer {}, all_clocks_one)?;

    recurs_2_shop(
        s_shop_barber,
        all_clocks_one,
        s_shop_client,
        all_clocks_two,
        loops,
        number_clients,
    )
}

fn recurs_2_shop(
    s_shop_barber: EndpointShopBarberRecurs2,
    all_clocks_one: &mut HashMap<char, Instant>,
    s_shop_client: EndpointShopClientRecurs2,
    all_clocks_two: &mut HashMap<char, Instant>,
    loops: usize,
    number_clients: usize,
) -> Result<(), Box<dyn Error>> {
    let s_shop_barber: EndpointShopBarberDescription = choose_mpst_shopbarber_to_all!(
        s_shop_barber,
        all_clocks_one,
        Branching2fromShopBarberToBarber::Available
    );

    let s_shop_client: EndpointShopClientDescription = choose_mpst_shopclient_to_all!(
        s_shop_client,
        all_clocks_two,
        Branching2fromShopClientToClient::Available
    );

    // Decription
    let (description, s_shop_client) = s_shop_client.recv(all_clocks_two)?;
    let s_shop_barber = s_shop_barber.send(description, all_clocks_one)?;

    recurs_3_shop(
        s_shop_barber,
        all_clocks_one,
        s_shop_client,
        all_clocks_two,
        loops,
        number_clients,
    )
}

fn recurs_3_shop(
    s_shop_barber: EndpointShopBarberRecurs3,
    all_clocks_one: &mut HashMap<char, Instant>,
    s_shop_client: EndpointShopClientRecurs3,
    all_clocks_two: &mut HashMap<char, Instant>,
    loops: usize,
    number_clients: usize,
) -> Result<(), Box<dyn Error>> {
    let s_shop_barber: EndpointShopBarberHaircut = choose_mpst_shopbarber_to_all!(
        s_shop_barber,
        all_clocks_one,
        Branching3fromShopBarberToBarber::Available
    );

    let s_shop_client: EndpointShopClientHaircut = choose_mpst_shopclient_to_all!(
        s_shop_client,
        all_clocks_two,
        Branching3fromShopClientToClient::Available
    );

    // Haircut
    let (haircut, s_shop_barber) = s_shop_barber.recv(all_clocks_one)?;
    let s_shop_client = s_shop_client.send(haircut, all_clocks_two)?;

    recurs_4_shop(
        s_shop_barber,
        all_clocks_one,
        s_shop_client,
        all_clocks_two,
        loops,
        number_clients,
    )
}

fn recurs_4_shop(
    s_shop_barber: EndpointShopBarberRecurs4,
    all_clocks_one: &mut HashMap<char, Instant>,
    s_shop_client: EndpointShopClientRecurs4,
    all_clocks_two: &mut HashMap<char, Instant>,
    loops: usize,
    number_clients: usize,
) -> Result<(), Box<dyn Error>> {
    let s_shop_barber: EndpointShopBarberPay = choose_mpst_shopbarber_to_all!(
        s_shop_barber,
        all_clocks_one,
        Branching4fromShopBarberToBarber::Available
    );

    let s_shop_client: EndpointShopClientPay = choose_mpst_shopclient_to_all!(
        s_shop_client,
        all_clocks_two,
        Branching4fromShopClientToClient::Available
    );

    // Pay
    let (pay, s_shop_client) = s_shop_client.recv(all_clocks_two)?;
    let s_shop_barber = s_shop_barber.send(pay, all_clocks_one)?;

    recurs_0_shop(
        s_shop_barber,
        all_clocks_one,
        s_shop_client,
        all_clocks_two,
        loops,
        number_clients - 1,
    )
}

// Client
fn recurs_0_client(
    s: EndpointClient0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_1_client(s, all_clocks)
}
fn recurs_1_client(
    s: EndpointClient0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branching0fromShopClientToClient::Done(s) => {
                s.close()
            },
            Branching0fromShopClientToClient::Full(s) => {
                let (_full, s) = s.recv(all_clocks)?;

                recurs_1_client(s, all_clocks)
            },
            Branching0fromShopClientToClient::Available(s) => {

                let (_seat, s) = s.recv(all_clocks)?;

                recurs_2_client(s, all_clocks)
            },
        }
    )
}
fn recurs_2_client(
    s: EndpointClient1,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branching1fromShopClientToClient::Available(s) => {

                let (_ready, s) = s.recv(all_clocks)?;

                recurs_3_client(s, all_clocks)
            },
        }
    )
}
fn recurs_3_client(
    s: EndpointClient2,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branching2fromShopClientToClient::Available(s) => {

                let s = s.send(Description {  }, all_clocks)?;

                recurs_4_client(s, all_clocks)
            },
        }
    )
}
fn recurs_4_client(
    s: EndpointClient3,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branching3fromShopClientToClient::Available(s) => {

                let (_haircut, s) = s.recv(all_clocks)?;

                recurs_5_client(s, all_clocks)
            },
        }
    )
}
fn recurs_5_client(
    s: EndpointClient4,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(
        s,
        all_clocks,
        {
            Branching4fromShopClientToClient::Available(s) => {

                let s = s.send(Pay {  }, all_clocks)?;

                recurs_1_client(s, all_clocks)
            },
        }
    )
}

/////////////////////////

fn main() {
    let (thread_barber, thread_client, thread_shop) =
        fork_mpst(recurs_0_barber, recurs_0_client, endpoint_shop);

    thread_barber.join().unwrap();
    thread_client.join().unwrap();
    thread_shop.join().unwrap();
}
