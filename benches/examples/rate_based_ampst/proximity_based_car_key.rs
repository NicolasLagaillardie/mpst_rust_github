#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::session::Session;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::error::Error;
// Create the new MeshedChannels for three participants and the close and fork functions
generate!("rec_and_cancel", MeshedChannels, Car, Key);
// Labels
struct Wake {}
struct Present {}
struct Challenge {}
struct Response {}
struct Absent {}
struct Stop {}
// Types
// Car
type Choice0FromKeyToCar = Recv<Branching0FromKeyToCar, End>;

type Loop0FromKeyToCar = Send<Wake, Choice0FromKeyToCar>;

type CarToKeyPresent0 = Recv<Present, Choice1FromKeyToCar>;

type Choice1FromKeyToCar = Recv<Branching1FromKeyToCar, End>;

type CarToKeyPresent1 = Send<Challenge, Choice2FromKeyToCar>;

type Choice2FromKeyToCar = Recv<Branching2FromKeyToCar, End>;

type CarToKeyPresent2 = Recv<Response, Loop0FromKeyToCar>;

type CarToKeyAbsent = Recv<Absent, Loop0FromKeyToCar>;

type CarToKeyStop = Recv<Stop, End>;

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
fn endpoint_car(s: Endpoint0Car) -> Result<(), Box<dyn Error>> {
    recurs_car(s)
}

fn recurs_car(s: Endpoint0Car) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0FromKeyToCar::Stop(s) => {

            let (_stop, s) = s.recv()?;
            s.close()
        },
        Branching0FromKeyToCar::Absent(s) => {
            let (_absent, s) = s.recv()?;
            let s = s.send(Wake {}, )?;
            recurs_car(s, )
        },
        Branching0FromKeyToCar::Present(s) => {
            let (_present, s) = s.recv()?;
            offer_mpst!(s, {
                Branching1FromKeyToCar::Present(s) => {
                    let s = s.send(Challenge {}, )?;
                    offer_mpst!(s, {
                        Branching2FromKeyToCar::Present(s) => {
                            let (_response, s) = s.recv()?;
                            let s = s.send(Wake {}, )?;
                            recurs_car(s, )
                        },
                    })
                },
            })
        },
    })
}
/////////////////////////
// Key
fn endpoint_key(s: Endpoint0Key) -> Result<(), Box<dyn Error>> {
    recurs_key(s, LOOPS)
}

fn recurs_key(s: Endpoint0Key, loops: i64) -> Result<(), Box<dyn Error>> {
    match loops {
        0 => {
            let s: Endpoint1KeyStop = choose_mpst_key_to_all!(s, Branching0FromKeyToCar::Stop,);
            let s = s.send(Stop {})?;
            s.close()
        }
        i => {
            if thread_rng().gen_range(0..=5) == 1 {
                let s: Endpoint1KeyAbsent =
                    choose_mpst_key_to_all!(s, Branching0FromKeyToCar::Absent);
                let s = s.send(Absent {})?;
                let (_wake, s) = s.recv()?;
                recurs_key(s, i - 1)
            } else {
                let s: Endpoint1KeyData =
                    choose_mpst_key_to_all!(s, Branching0FromKeyToCar::Present);

                let s = s.send(Present {})?;

                let s: Endpoint2KeyData =
                    choose_mpst_key_to_all!(s, Branching1FromKeyToCar::Present);

                let (_challenge, s) = s.recv()?;

                let s: Endpoint3KeyData =
                    choose_mpst_key_to_all!(s, Branching2FromKeyToCar::Present);
                let s = s.send(Response {})?;

                let (_wake, s) = s.recv()?;

                recurs_key(s, i - 1)
            }
        }
    }
}

/////////////////////////

fn aux() {
    let (thread_car, thread_key) = fork_mpst(black_box(endpoint_car), black_box(endpoint_key));

    thread_car.join().unwrap();
    thread_key.join().unwrap();
}

static LOOPS: i64 = 100;

/////////////////////////

pub fn proximity_based_car_key(c: &mut Criterion) {
    c.bench_function("Car-Key AMPST", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(100000);
    targets = proximity_based_car_key,
}

criterion_main! {
    bench
}
