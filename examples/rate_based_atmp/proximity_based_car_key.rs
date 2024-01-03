#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_timed;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

// Create the new MeshedChannels for three participants and the close and fork functions
generate_timed!(MeshedChannels, Car, Key);

// Labels
struct Wake {}
struct Present {}
struct Challenge {}
struct Response {}
struct Absent {}
struct Stop {}

// Types
// Car
type Choice0FromKeyToCar = RecvTimed<Branching0FromKeyToCar, 'a', 0, true, 1, true, ' ', End>;

type Loop0FromKeyToCar = SendTimed<Wake, 'a', 0, true, 1, true, ' ', Choice0FromKeyToCar>;

type CarToKeyPresent0 = RecvTimed<Present, 'a', 0, true, 1, true, 'a', Choice1FromKeyToCar>;

type Choice1FromKeyToCar = RecvTimed<Branching1FromKeyToCar, 'a', 0, true, 1, true, ' ', End>;

type CarToKeyPresent1 = SendTimed<Challenge, 'a', 0, true, 1, true, 'a', Choice2FromKeyToCar>;

type Choice2FromKeyToCar = RecvTimed<Branching2FromKeyToCar, 'a', 0, true, 1, true, ' ', End>;

type CarToKeyPresent2 = RecvTimed<Response, 'a', 0, true, 1, true, 'a', Loop0FromKeyToCar>;

type CarToKeyAbsent = RecvTimed<Absent, 'a', 0, true, 1, true, 'a', Loop0FromKeyToCar>;

type CarToKeyStop = RecvTimed<Stop, 'a', 0, true, 1, true, 'a', End>;

enum Branching0FromKeyToCar {
    Present(MeshedChannels<CarToKeyPresent0, RoleKey<RoleKey<RoleEnd>>, NameCar>),
    Absent(MeshedChannels<CarToKeyAbsent, RoleKey<RoleKey<RoleKey<RoleEnd>>>, NameCar>),
    Stop(MeshedChannels<CarToKeyStop, RoleKey<RoleEnd>, NameCar>),
}

enum Branching1FromKeyToCar {
    Present(MeshedChannels<CarToKeyPresent1, RoleKey<RoleKey<RoleEnd>>, NameCar>),
}

enum Branching2FromKeyToCar {
    Present(MeshedChannels<CarToKeyPresent2, RoleKey<RoleKey<RoleKey<RoleEnd>>>, NameCar>),
}

// Key
type ChooseFromKeyToCar = <Choice0FromKeyToCar as Session>::Dual;

type KeyToCarPresent1 = <CarToKeyPresent0 as Session>::Dual;

type KeyToCarPresent2 = <CarToKeyPresent1 as Session>::Dual;

type KeyToCarPresent3 = <CarToKeyPresent2 as Session>::Dual;

type KeyToCarAbsent = <CarToKeyAbsent as Session>::Dual;

type KeyToCarStop = <CarToKeyStop as Session>::Dual;

// Endpoints
// Car
type Endpoint0Car = MeshedChannels<Choice0FromKeyToCar, RoleKey<RoleEnd>, NameCar>;

// Key
type Endpoint0Key = MeshedChannels<ChooseFromKeyToCar, RoleBroadcast, NameKey>;

type Endpoint1KeyData = MeshedChannels<KeyToCarPresent1, RoleCar<RoleBroadcast>, NameKey>;

type Endpoint2KeyData = MeshedChannels<KeyToCarPresent2, RoleCar<RoleBroadcast>, NameKey>;

type Endpoint3KeyData = MeshedChannels<KeyToCarPresent3, RoleCar<RoleCar<RoleBroadcast>>, NameKey>;

type Endpoint1KeyAbsent = MeshedChannels<KeyToCarAbsent, RoleCar<RoleCar<RoleBroadcast>>, NameKey>;

type Endpoint1KeyStop = MeshedChannels<KeyToCarStop, RoleCar<RoleEnd>, NameKey>;

// Functions

/////////////////////////

// Car
fn endpoint_car(
    s: Endpoint0Car,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_car(s, all_clocks)
}

fn recurs_car(
    s: Endpoint0Car,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));

    offer_mpst!(s, all_clocks, {
        Branching0FromKeyToCar::Stop(s) => {

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let (_stop, s) = s.recv(all_clocks)?;

            s.close()
        },
        Branching0FromKeyToCar::Absent(s) => {
            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let (_absent, s) = s.recv(all_clocks)?;

            let s = s.send(Wake {}, all_clocks)?;

            recurs_car(s, all_clocks)
        },
        Branching0FromKeyToCar::Present(s) => {
            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let (_present, s) = s.recv(all_clocks)?;

            offer_mpst!(s, all_clocks, {
                Branching1FromKeyToCar::Present(s) => {
                    sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
                    let s = s.send(Challenge {}, all_clocks)?;

                    offer_mpst!(s, all_clocks, {
                        Branching2FromKeyToCar::Present(s) => {
                            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
                            let (_response, s) = s.recv(all_clocks)?;

                            let s = s.send(Wake {}, all_clocks)?;

                            recurs_car(s, all_clocks)
                        },
                    })
                },
            })
        },
    })
}

/////////////////////////

// Key
fn endpoint_key(
    s: Endpoint0Key,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    recurs_key(s, all_clocks, LOOPS)
}

fn recurs_key(
    s: Endpoint0Key,
    all_clocks: &mut HashMap<char, Instant>,
    loops: i64,
) -> Result<(), Box<dyn Error>> {
    sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));

    match loops {
        0 => {
            let s: Endpoint1KeyStop =
                choose_mpst_key_to_all!(s, all_clocks, Branching0FromKeyToCar::Stop,);

            sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
            let s = s.send(Stop {}, all_clocks)?;

            s.close()
        }
        i => {
            if thread_rng().gen_range(0..=5) == 1 {
                let s: Endpoint1KeyAbsent =
                    choose_mpst_key_to_all!(s, all_clocks, Branching0FromKeyToCar::Absent);

                sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
                let s = s.send(Absent {}, all_clocks)?;

                let (_wake, s) = s.recv(all_clocks)?;

                recurs_key(s, all_clocks, i - 1)
            } else {
                let s: Endpoint1KeyData =
                    choose_mpst_key_to_all!(s, all_clocks, Branching0FromKeyToCar::Present);

                sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
                let s = s.send(Present {}, all_clocks)?;

                let s: Endpoint2KeyData =
                    choose_mpst_key_to_all!(s, all_clocks, Branching1FromKeyToCar::Present);

                sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
                let (_challenge, s) = s.recv(all_clocks)?;

                sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
                let s: Endpoint3KeyData =
                    choose_mpst_key_to_all!(s, all_clocks, Branching2FromKeyToCar::Present);

                sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
                let s = s.send(Response {}, all_clocks)?;

                sleep(Duration::from_millis(thread_rng().gen_range(0..=100)));
                let (_wake, s) = s.recv(all_clocks)?;

                recurs_key(s, all_clocks, i - 1)
            }
        }
    }
}

/////////////////////////

fn main() {
    let (thread_car, thread_key) = fork_mpst(endpoint_car, endpoint_key);

    thread_car.join().unwrap();
    thread_key.join().unwrap();
}

static LOOPS: i64 = 100;
