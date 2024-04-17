#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::upper_case_acronyms
)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary_atmp::struct_trait::recv::RecvTimed;
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

// Create the new MeshedChannels for three participants and the close and fork functions
generate_atmp!(MeshedChannels, Scl, Sda);

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
type Choice0FromSclToSda = RecvTimed<Branching0FromSclToSda, 'a', 0, true, 1, true, ' ', End>;

type SdaToSclLoop = RecvTimed<
    BUF,
    'a',
    0,
    true,
    1,
    true,
    ' ',
    RecvTimed<
        HDSTA,
        'a',
        0,
        true,
        1,
        true,
        ' ',
        RecvTimed<
            SUSTA,
            'a',
            0,
            true,
            1,
            true,
            ' ',
            RecvTimed<
                SUSTO,
                'a',
                0,
                true,
                1,
                true,
                ' ',
                RecvTimed<
                    HDDAT,
                    'a',
                    0,
                    true,
                    1,
                    true,
                    ' ',
                    RecvTimed<SUDAT, 'a', 0, true, 1, true, 'a', Choice0FromSclToSda>,
                >,
            >,
        >,
    >,
>;

type SdaToSclStop = RecvTimed<Stop, 'a', 0, true, 1, true, 'a', End>;

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
fn endpoint_scl(
    s: Endpoint0Car,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_scl(s, all_clocks)
}

fn recurs_scl(
    s: Endpoint0Car,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));

    offer_mpst!(s, all_clocks, {
        Branching0FromSclToSda::Stop(s) => {

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let (_stop, s) = s.recv(all_clocks)?;

            s.close()
        },
        Branching0FromSclToSda::Loop(s) => {
            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let (_buf, s) = s.recv(all_clocks)?;

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let (_hdsta, s) = s.recv(all_clocks)?;

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let (_susta, s) = s.recv(all_clocks)?;

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let (_susto, s) = s.recv(all_clocks)?;

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let (_hddat, s) = s.recv(all_clocks)?;

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let (_sudat, s) = s.recv(all_clocks)?;

            recurs_scl(s, all_clocks)
        },
    })
}

/////////////////////////

// Key
fn endpoint_sda(
    s: Endpoint0Key,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_sda(s, all_clocks, LOOPS)
}

fn recurs_sda(
    s: Endpoint0Key,
    all_clocks: &mut HashMap<char, Instant>,
    loops: i64,
) -> Result<(), Box<dyn Error>> {
    sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));

    match loops {
        0 => {
            let s: Endpoint1KeyStop =
                choose_mpst_sda_to_all!(s, all_clocks, Branching0FromSclToSda::Stop,);

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let s = s.send(Stop {}, all_clocks)?;

            s.close()
        }
        i => {
            let s: Endpoint1KeyLoop =
                choose_mpst_sda_to_all!(s, all_clocks, Branching0FromSclToSda::Loop);

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let s = s.send(BUF {}, all_clocks)?;

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let s = s.send(HDSTA {}, all_clocks)?;

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let s = s.send(SUSTA {}, all_clocks)?;

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let s = s.send(SUSTO {}, all_clocks)?;

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let s = s.send(HDDAT {}, all_clocks)?;

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let s = s.send(SUDAT {}, all_clocks)?;

            recurs_sda(s, all_clocks, i - 1)
        }
    }
}

/////////////////////////

fn main() {
    let (thread_scl, thread_sda) = fork_mpst(endpoint_scl, endpoint_sda);

    thread_scl.join().unwrap();
    thread_sda.join().unwrap();
}

static LOOPS: i64 = 100;
