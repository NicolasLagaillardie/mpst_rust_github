#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate!(
    "timed_interleaved",
    MeshedChannels,
    Barber,
    ShopBarber,
    2,
    MeshedChannels,
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
    Done(MeshedChannels<End, RoleEnd, NameBarber>),
    Available(
        MeshedChannels<
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
        MeshedChannels<
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
        MeshedChannels<
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
        MeshedChannels<
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
        MeshedChannels<
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
    Done(MeshedChannels<End, RoleEnd, NameClient>),
    Full(
        MeshedChannels<
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
        MeshedChannels<
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
        MeshedChannels<
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
        MeshedChannels<
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
        MeshedChannels<
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
        MeshedChannels<
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
    MeshedChannels<Recurs0fromShopBarberToBarber, RoleShopBarber<RoleEnd>, NameBarber>;
type EndpointBarber1 =
    MeshedChannels<Recurs1fromShopBarberToBarber, RoleShopBarber<RoleEnd>, NameBarber>;
type EndpointBarber2 =
    MeshedChannels<Recurs2fromShopBarberToBarber, RoleShopBarber<RoleEnd>, NameBarber>;
type EndpointBarber3 =
    MeshedChannels<Recurs3fromShopBarberToBarber, RoleShopBarber<RoleEnd>, NameBarber>;
type EndpointBarber4 =
    MeshedChannels<Recurs4fromShopBarberToBarber, RoleShopBarber<RoleEnd>, NameBarber>;

// ShopBarber
type EndpointShopBarber0 =
    MeshedChannels<Choose0fromShopBarberToBarber, RoleBroadcast, NameShopBarber>;
type EndpointShopBarberDone = MeshedChannels<End, RoleEnd, NameShopBarber>;
// Recurs
type EndpointShopBarberRecurs1 =
    MeshedChannels<Choose1fromShopBarberToBarber, RoleBroadcast, NameShopBarber>;
type EndpointShopBarberRecurs2 =
    MeshedChannels<Choose2fromShopBarberToBarber, RoleBroadcast, NameShopBarber>;
type EndpointShopBarberRecurs3 =
    MeshedChannels<Choose3fromShopBarberToBarber, RoleBroadcast, NameShopBarber>;
type EndpointShopBarberRecurs4 =
    MeshedChannels<Choose4fromShopBarberToBarber, RoleBroadcast, NameShopBarber>;
// Full branches
type EndpointShopBarberAvailabe = MeshedChannels<
    RecvTimed<Available, 'a', 0, true, 10, true, ' ', Choose1fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberCustomer = MeshedChannels<
    SendTimed<Customer, 'a', 0, true, 10, true, ' ', Choose2fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberDescription = MeshedChannels<
    SendTimed<Description, 'a', 0, true, 10, true, ' ', Choose3fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberHaircut = MeshedChannels<
    RecvTimed<Haircut, 'a', 0, true, 10, true, ' ', Choose4fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;
type EndpointShopBarberPay = MeshedChannels<
    SendTimed<Pay, 'a', 0, true, 10, true, ' ', Choose5fromShopBarberToBarber>,
    RoleBarber<RoleBroadcast>,
    NameShopBarber,
>;

// Client
type EndpointClient0 = MeshedChannels<
    RecvTimed<Branching0fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient1 = MeshedChannels<
    RecvTimed<Branching1fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient2 = MeshedChannels<
    RecvTimed<Branching2fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient3 = MeshedChannels<
    RecvTimed<Branching3fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;
type EndpointClient4 = MeshedChannels<
    RecvTimed<Branching4fromShopClientToClient, 'a', 0, true, 10, true, ' ', End>,
    RoleShopClient<RoleEnd>,
    NameClient,
>;

// ShopClient
type EndpointShopClient0 =
    MeshedChannels<Choose0fromShopClientToClient, RoleBroadcast, NameShopClient>;
type EndpointShopClientDone = MeshedChannels<End, RoleEnd, NameShopClient>;
type EndpointShopClientFull = MeshedChannels<
    SendTimed<Full, 'a', 0, true, 10, true, ' ', Choose5fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
// Recurs
type EndpointShopClientRecurs1 =
    MeshedChannels<Choose1fromShopClientToClient, RoleBroadcast, NameShopClient>;
type EndpointShopClientRecurs2 =
    MeshedChannels<Choose2fromShopClientToClient, RoleBroadcast, NameShopClient>;
type EndpointShopClientRecurs3 =
    MeshedChannels<Choose3fromShopClientToClient, RoleBroadcast, NameShopClient>;
type EndpointShopClientRecurs4 =
    MeshedChannels<Choose4fromShopClientToClient, RoleBroadcast, NameShopClient>;
// Full branches
type EndpointShopClientSeat = MeshedChannels<
    SendTimed<Seat, 'a', 0, true, 10, true, ' ', Choose1fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientReady = MeshedChannels<
    SendTimed<Ready, 'a', 0, true, 10, true, ' ', Choose2fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientDescription = MeshedChannels<
    RecvTimed<Description, 'a', 0, true, 10, true, ' ', Choose3fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientHaircut = MeshedChannels<
    SendTimed<Haircut, 'a', 0, true, 10, true, ' ', Choose4fromShopClientToClient>,
    RoleClient<RoleBroadcast>,
    NameShopClient,
>;
type EndpointShopClientPay = MeshedChannels<
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
