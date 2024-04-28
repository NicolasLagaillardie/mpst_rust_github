#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// Create the new MeshedChannels for three participants and the close and fork functions
generate_atmp!(MeshedChannels, Calculator1, Calculator2, Server);

// Labels
struct Data1 {}
struct Data2 {}
struct ProcesData2 {}
struct ProcesData3 {}
struct Complete {}
struct Stop {}

// Types
// Calculator1
type OfferFromServertoCalculator1 =
    RecvTimed<BranchingFromServertoCalculator1, 'a', 0, true, 1, true, ' ', End>;

type Calculator1toCalculator2Data = RecvTimed<
    ProcesData2,
    'b',
    0,
    true,
    1,
    true,
    ' ',
    SendTimed<Complete, 'b', 0, true, 1, true, 'b', End>,
>;
type Calculator1toServerData = RecvTimed<
    Data1,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    SendTimed<ProcesData3, 'a', 0, true, 1, true, 'a', OfferFromServertoCalculator1>,
>;

type Calculator1toCalculator2Stop = End;
type Calculator1toServerStop = RecvTimed<Stop, 'a', 0, true, 1, true, 'a', End>;

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
type OfferFromServertoCalculator2 =
    RecvTimed<BranchingFromServertoCalculator2, 'c', 0, true, 1, true, ' ', End>;

type Calculator2toCalculator1Data = <Calculator1toCalculator2Data as Session>::Dual;
type Calculator2toServerData = RecvTimed<
    Data2,
    'c',
    0,
    true,
    1,
    true,
    ' ',
    RecvTimed<Complete, 'c', 0, true, 1, true, 'c', OfferFromServertoCalculator2>,
>;

type Calculator2toCalculator1Stop = <Calculator1toCalculator2Stop as Session>::Dual;
type Calculator2toServerStop = RecvTimed<Stop, 'c', 0, true, 1, true, ' ', End>;

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
fn endpoint_calculator_1(
    s: Endpoint0Calculator1,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    all_clocks.insert('b', Instant::now());

    recurs_calculator_1(s, all_clocks)
}

fn recurs_calculator_1(
    s: Endpoint0Calculator1,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        BranchingFromServertoCalculator1::Stop(s) => {

            let (_stop, s) = s.recv(all_clocks)?;

            s.close()
        },
        BranchingFromServertoCalculator1::Data(s) => {

            let (_data_1, s) = s.recv(all_clocks)?;

            let (_processed_data_2, s) = s.recv(all_clocks)?;

            let s = s.send(ProcesData3 {}, all_clocks)?;

            let s = s.send(Complete {}, all_clocks)?;

            recurs_calculator_1(s, all_clocks)
        },
    })
}

/////////////////////////

// Calculator2
fn endpoint_calculator_2(
    s: Endpoint0Calculator2,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('b', Instant::now());
    all_clocks.insert('c', Instant::now());

    recurs_calculator_2(s, all_clocks)
}

fn recurs_calculator_2(
    s: Endpoint0Calculator2,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        BranchingFromServertoCalculator2::Stop(s) => {

            let (_stop, s) = s.recv(all_clocks)?;

            s.close()
        },
        BranchingFromServertoCalculator2::Data(s) => {

            let (_data_2, s) = s.recv(all_clocks)?;

            let s = s.send(ProcesData2 {}, all_clocks)?;

            let (_complete, s) = s.recv(all_clocks)?;

            let (_complete, s) = s.recv(all_clocks)?;

            recurs_calculator_2(s, all_clocks)
        },
    })
}

/////////////////////////

// Server
fn endpoint_server(
    s: Endpoint0Server,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    all_clocks.insert('c', Instant::now());

    recurs_server(s, all_clocks, LOOPS)
}

fn recurs_server(
    s: Endpoint0Server,
    all_clocks: &mut HashMap<char, Instant>,
    loops: i64,
) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s: Endpoint1ServerStop = choose_mpst_server_to_all!(
                s,
                all_clocks,
                BranchingFromServertoCalculator1::Stop,
                BranchingFromServertoCalculator2::Stop,
            );

            let s = s.send(Stop {}, all_clocks)?;

            let s = s.send(Stop {}, all_clocks)?;

            s.close()
        }
        i => {
            let s: Endpoint1ServerData = choose_mpst_server_to_all!(
                s,
                all_clocks,
                BranchingFromServertoCalculator1::Data,
                BranchingFromServertoCalculator2::Data,
            );

            let s = s.send(Data1 {}, all_clocks)?;

            let s = s.send(Data2 {}, all_clocks)?;

            let (_processed_data_3, s) = s.recv(all_clocks)?;

            let s = s.send(Complete {}, all_clocks)?;

            recurs_server(s, all_clocks, i - 1)
        }
    }
}

/////////////////////////

fn aux() {
    let (thread_calculator_1, thread_calculator_2, thread_server) = fork_mpst(
        black_box(endpoint_calculator_1),
        black_box(endpoint_calculator_2),
        black_box(endpoint_server),
    );

    thread_calculator_1.join().unwrap();
    thread_calculator_2.join().unwrap();
    thread_server.join().unwrap();
}

/////////////////////////

static LOOPS: i64 = 100;

pub fn distributed_calculators(c: &mut Criterion) {
    c.bench_function("ATMP Distributed calculators", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = distributed_calculators,
}

criterion_main! {
    bench
}
