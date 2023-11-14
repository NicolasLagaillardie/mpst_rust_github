#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::upper_case_acronyms
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary::struct_trait::{end::End, recv::Recv};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// Create the new MeshedChannels for three participants and the close and fork functions
generate!("rec_and_cancel", MeshedChannels, Scl, Sda);
// Labels
// Bus free time between start and stop condition
struct BUF {}
// Hold time(repeated) START condition
// After this period, the first clock pulse is generated
struct HDSTA {}
// Set-up time for a repeated START condition
struct SUSTA {}
// Set-up time for STOP condition
struct SUSTO {}
// Data hold time
struct HDDAT {}
// Data setup time
struct SUDAT {}
// Stop loop
struct Stop {}
// Types
// Scl
type Choice0FromSclToSda = Recv<Branching0FromSclToSda, End>;

type SdaToSclLoop =
    Recv<BUF, Recv<HDSTA, Recv<SUSTA, Recv<SUSTO, Recv<HDDAT, Recv<SUDAT, Choice0FromSclToSda>>>>>>;

type SdaToSclStop = Recv<Stop, End>;

enum Branching0FromSclToSda {
    Loop(
        MeshedChannels<
            SdaToSclLoop,
            RoleSda<RoleSda<RoleSda<RoleSda<RoleSda<RoleSda<RoleSda<RoleEnd>>>>>>>,
            NameScl,
        >,
    ),
    Stop(MeshedChannels<SdaToSclStop, RoleSda<RoleEnd>, NameScl>),
}
// Sda
type ChooseFromSclToSda = <Choice0FromSclToSda as Session>::Dual;

type SclToSdaLoop = <SdaToSclLoop as Session>::Dual;

type SclToSdaStop = <SdaToSclStop as Session>::Dual;
// Endpoints
// Scl
type Endpoint0Car = MeshedChannels<Choice0FromSclToSda, RoleSda<RoleEnd>, NameScl>;
// Sda
type Endpoint0Key = MeshedChannels<ChooseFromSclToSda, RoleBroadcast, NameSda>;

type Endpoint1KeyLoop = MeshedChannels<
    SclToSdaLoop,
    RoleScl<RoleScl<RoleScl<RoleScl<RoleScl<RoleScl<RoleBroadcast>>>>>>,
    NameSda,
>;

type Endpoint1KeyStop = MeshedChannels<SclToSdaStop, RoleScl<RoleEnd>, NameSda>;
// Functions

/////////////////////////
// Car
fn endpoint_scl(s: Endpoint0Car) -> Result<(), Box<dyn Error>> {
    recurs_scl(s)
}

fn recurs_scl(s: Endpoint0Car) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0FromSclToSda::Stop(s) => {

            let (_stop, s) = s.recv()?;

            s.close()
        },
        Branching0FromSclToSda::Loop(s) => {
            let (_buf, s) = s.recv()?;

            let (_hdsta, s) = s.recv()?;

            let (_susta, s) = s.recv()?;

            let (_susto, s) = s.recv()?;

            let (_hddat, s) = s.recv()?;

            let (_sudat, s) = s.recv()?;
            recurs_scl(s)
        },
    })
}
/////////////////////////
// Key
fn endpoint_sda(s: Endpoint0Key) -> Result<(), Box<dyn Error>> {
    recurs_sda(s, LOOPS)
}

fn recurs_sda(s: Endpoint0Key, loops: i64) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s: Endpoint1KeyStop = choose_mpst_sda_to_all!(s, Branching0FromSclToSda::Stop);

            let s = s.send(Stop {})?;

            s.close()
        }
        i => {
            let s: Endpoint1KeyLoop = choose_mpst_sda_to_all!(s, Branching0FromSclToSda::Loop);

            let s = s.send(BUF {})?;

            let s = s.send(HDSTA {})?;

            let s = s.send(SUSTA {})?;

            let s = s.send(SUSTO {})?;

            let s = s.send(HDDAT {})?;

            let s = s.send(SUDAT {})?;
            recurs_sda(s, i - 1)
        }
    }
}

/////////////////////////

fn aux() {
    let (thread_scl, thread_sda) = fork_mpst(black_box(endpoint_scl), black_box(endpoint_sda));

    thread_scl.join().unwrap();
    thread_sda.join().unwrap();
}

static LOOPS: i64 = 100;

/////////////////////////

pub fn pinetime_heart_rate(c: &mut Criterion) {
    c.bench_function("Heart Rate AMPST", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = pinetime_heart_rate,
}

criterion_main! {
    bench
}
