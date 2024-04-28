#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// Create the new MeshedChannels for three participants and the close and fork functions
generate!(
    "recursive",
    MeshedChannels,
    Calculator1,
    Calculator2,
    Server
);

// Labels
struct Data1 {}
struct Data2 {}
struct ProcesData2 {}
struct ProcesData3 {}
struct Complete {}
struct Stop {}

// Types
// Calculator1
type OfferFromServertoCalculator1 = Recv<BranchingFromServertoCalculator1, End>;

type Calculator1toCalculator2Data = Recv<ProcesData2, Send<Complete, End>>;
type Calculator1toServerData = Recv<Data1, Send<ProcesData3, OfferFromServertoCalculator1>>;

type Calculator1toCalculator2Stop = End;
type Calculator1toServerStop = Recv<Stop, End>;

enum BranchingFromServertoCalculator1 {
    Data(
        MeshedChannels<
            Calculator1toCalculator2Data,
            Calculator1toServerData,
            RoleServer<RoleCalculator2<RoleServer<RoleCalculator2<RoleServer<RoleEnd>>>>>,
            NameCalculator1,
        >,
    ),
    Stop(
        MeshedChannels<
            Calculator1toCalculator2Stop,
            Calculator1toServerStop,
            RoleServer<RoleEnd>,
            NameCalculator1,
        >,
    ),
}

// Calculator2
type OfferFromServertoCalculator2 = Recv<BranchingFromServertoCalculator2, End>;

type Calculator2toCalculator1Data = Send<ProcesData2, Recv<Complete, End>>;
type Calculator2toServerData = Recv<Data2, Recv<Complete, OfferFromServertoCalculator2>>;

type Calculator2toCalculator1Stop = End;
type Calculator2toServerStop = Recv<Stop, End>;

enum BranchingFromServertoCalculator2 {
    Data(
        MeshedChannels<
            Calculator2toCalculator1Data,
            Calculator2toServerData,
            RoleServer<RoleCalculator1<RoleServer<RoleCalculator1<RoleServer<RoleEnd>>>>>,
            NameCalculator2,
        >,
    ),
    Stop(
        MeshedChannels<
            Calculator2toCalculator1Stop,
            Calculator2toServerStop,
            RoleServer<RoleEnd>,
            NameCalculator2,
        >,
    ),
}

// Server
type ChooseFromServertoCalculator1 = <OfferFromServertoCalculator1 as Session>::Dual;
type ChooseFromServertoCalculator2 = <OfferFromServertoCalculator2 as Session>::Dual;

type ServertoCalculator1Data = <Calculator1toServerData as Session>::Dual;
type ServertoCalculator2Data = <Calculator2toServerData as Session>::Dual;

type ServertoCalculator1Stop = <Calculator1toServerStop as Session>::Dual;
type ServertoCalculator2Stop = <Calculator2toServerStop as Session>::Dual;

// Endpoints
// Calculator1
type Endpoint0Calculator1 =
    MeshedChannels<End, OfferFromServertoCalculator1, RoleServer<RoleEnd>, NameCalculator1>;

// Calculator2
type Endpoint0Calculator2 =
    MeshedChannels<End, OfferFromServertoCalculator2, RoleServer<RoleEnd>, NameCalculator2>;

// Server
type Endpoint0Server = MeshedChannels<
    ChooseFromServertoCalculator1,
    ChooseFromServertoCalculator2,
    RoleBroadcast,
    NameServer,
>;
type Endpoint1ServerData = MeshedChannels<
    ServertoCalculator1Data,
    ServertoCalculator2Data,
    RoleCalculator1<RoleCalculator2<RoleCalculator1<RoleCalculator2<RoleBroadcast>>>>,
    NameServer,
>;
type Endpoint1ServerStop = MeshedChannels<
    ServertoCalculator1Stop,
    ServertoCalculator2Stop,
    RoleCalculator1<RoleCalculator2<RoleEnd>>,
    NameServer,
>;

// Functions

/////////////////////////

// Calculator1
fn endpoint_calculator_1(s: Endpoint0Calculator1) -> Result<(), Box<dyn Error>> {
    recurs_calculator_1(s)
}

fn recurs_calculator_1(s: Endpoint0Calculator1) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        BranchingFromServertoCalculator1::Stop(s) => {
            let (_stop, s) = s.recv();

            s.close()
        },
        BranchingFromServertoCalculator1::Data(s) => {
            let (_data_1, s) = s.recv();
            let (_processed_data_2, s) = s.recv();

            let s = s.send(ProcesData3 {});
            let s = s.send(Complete {});

            recurs_calculator_1(s)
        },
    })
}

/////////////////////////

// Calculator2
fn endpoint_calculator_2(s: Endpoint0Calculator2) -> Result<(), Box<dyn Error>> {
    recurs_calculator_2(s)
}

fn recurs_calculator_2(s: Endpoint0Calculator2) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        BranchingFromServertoCalculator2::Stop(s) => {
            let (_stop, s) = s.recv();

            s.close()
        },
        BranchingFromServertoCalculator2::Data(s) => {
            let (_data_2, s) = s.recv();
            let s = s.send(ProcesData2 {});

            let (_complete, s) = s.recv();
            let (_complete, s) = s.recv();

            recurs_calculator_2(s)
        },
    })
}

/////////////////////////

// Server
fn endpoint_server(s: Endpoint0Server) -> Result<(), Box<dyn Error>> {
    recurs_server(s, LOOPS)
}

fn recurs_server(s: Endpoint0Server, loops: i64) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s: Endpoint1ServerStop = choose_mpst_server_to_all!(
                s,
                BranchingFromServertoCalculator1::Stop,
                BranchingFromServertoCalculator2::Stop,
            );

            let s = s.send(Stop {});
            let s = s.send(Stop {});

            s.close()
        }
        i => {
            let s: Endpoint1ServerData = choose_mpst_server_to_all!(
                s,
                BranchingFromServertoCalculator1::Data,
                BranchingFromServertoCalculator2::Data,
            );

            let s = s.send(Data1 {});
            let s = s.send(Data2 {});

            let (_data, s) = s.recv();

            let s = s.send(Complete {});

            recurs_server(s, i - 1)
        }
    }
}

/////////////////////////

fn main() {
    let (thread_calculator_1, thread_calculator_2, thread_server) = fork_mpst(
        endpoint_calculator_1,
        endpoint_calculator_2,
        endpoint_server,
    );

    thread_calculator_1.join().unwrap();
    thread_calculator_2.join().unwrap();
    thread_server.join().unwrap();
}

static LOOPS: i64 = 100;
