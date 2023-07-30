#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::random;

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
generate!(
    "rec_and_cancel",
    MeshedChannels,
    Api,
    Controller,
    Storage,
    User
);

// RoleApi
enum Branching0fromCtoA {
    Up(
        MeshedChannels<
            Recv<i32, Send<i32, Recurs0fromCtoA>>,
            Send<i32, Recv<i32, End>>,
            Send<i32, Recv<i32, End>>,
            RoleController<
                RoleStorage<
                    RoleStorage<RoleUser<RoleUser<RoleController<RoleController<RoleEnd>>>>>,
                >,
            >,
            NameApi,
        >,
    ),
    Down(
        MeshedChannels<
            Recv<i32, Send<i32, Recurs0fromCtoA>>,
            End,
            Send<i32, Recv<i32, End>>,
            RoleController<RoleUser<RoleUser<RoleController<RoleController<RoleEnd>>>>>,
            NameApi,
        >,
    ),
    Close(
        MeshedChannels<
            Recv<i32, End>,
            End,
            Send<i32, End>,
            RoleController<RoleUser<RoleEnd>>,
            NameApi,
        >,
    ),
}
type Recurs0fromCtoA = Recv<Branching0fromCtoA, End>;

// RoleController
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoS = Send<Branching0fromCtoS, End>;
type Choose0fromCtoU = Send<Branching0fromCtoU, End>;

// RoleStorage
enum Branching0fromCtoS {
    Up(
        MeshedChannels<
            Recv<i32, Send<i32, End>>,
            Recurs0fromCtoS,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameStorage,
        >,
    ),
    Down(
        MeshedChannels<
            End,
            Recv<i32, Recurs0fromCtoS>,
            End,
            RoleController<RoleController<RoleEnd>>,
            NameStorage,
        >,
    ),
    Close(MeshedChannels<End, Recv<i32, End>, End, RoleController<RoleEnd>, NameStorage>),
}
type Recurs0fromCtoS = Recv<Branching0fromCtoS, End>;

// RoleUser
enum Branching0fromCtoU {
    Up(
        MeshedChannels<
            Recv<i32, Send<i32, End>>,
            Recurs0fromCtoU,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameUser,
        >,
    ),
    Down(
        MeshedChannels<
            Recv<i32, Send<i32, End>>,
            Recurs0fromCtoU,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameUser,
        >,
    ),
    Close(MeshedChannels<Recv<i32, End>, End, End, RoleApi<RoleEnd>, NameUser>),
}
type Recurs0fromCtoU = Recv<Branching0fromCtoU, End>;

// Creating the MP sessions
// RoleApi
type EndpointApi0 = MeshedChannels<
    Send<i32, Recurs0fromCtoA>,
    End,
    Recv<i32, End>,
    RoleUser<RoleController<RoleController<RoleEnd>>>,
    NameApi,
>;
type EndpointApiInit = MeshedChannels<
    Recv<i32, Send<i32, Recurs0fromCtoA>>,
    End,
    Recv<i32, End>,
    RoleController<RoleUser<RoleController<RoleController<RoleEnd>>>>,
    NameApi,
>;

// RoleController
type EndpointControllerDown = MeshedChannels<
    Send<i32, Recv<i32, Choose0fromCtoA>>,
    Send<i32, Choose0fromCtoS>,
    Choose0fromCtoU,
    RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>,
    NameController,
>;
type EndpointControllerUp = MeshedChannels<
    Send<i32, Recv<i32, Choose0fromCtoA>>,
    Choose0fromCtoS,
    Choose0fromCtoU,
    RoleApi<RoleApi<RoleBroadcast>>,
    NameController,
>;
type EndpointControllerClose = MeshedChannels<
    Send<i32, End>,
    Send<i32, End>,
    End,
    RoleApi<RoleStorage<RoleEnd>>,
    NameController,
>;
type EndpointController0 = MeshedChannels<
    Recv<i32, Choose0fromCtoA>,
    Choose0fromCtoS,
    Choose0fromCtoU,
    RoleApi<RoleBroadcast>,
    NameController,
>;
type EndpointControllerInit = MeshedChannels<
    Send<i32, Recv<i32, Choose0fromCtoA>>,
    Send<i32, Recv<i32, Choose0fromCtoS>>,
    Choose0fromCtoU,
    RoleStorage<RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>>,
    NameController,
>;

// RoleStorage
type EndpointStorage0 =
    MeshedChannels<End, Recurs0fromCtoS, End, RoleController<RoleEnd>, NameStorage>;
type EndpointStorageInit = MeshedChannels<
    End,
    Recv<i32, Send<i32, Recurs0fromCtoS>>,
    End,
    RoleController<RoleController<RoleController<RoleEnd>>>,
    NameStorage,
>;

// RoleUser
type EndpointUserInit = MeshedChannels<
    Send<i32, End>,
    Recurs0fromCtoU,
    End,
    RoleApi<RoleController<RoleEnd>>,
    NameUser,
>;

/////////////////////////

fn endpoint_api(s: EndpointApiInit) -> Result<(), Box<dyn Error>> {
    let (_start, s) = s.recv()?;

    recurs_api(s)
}

fn recurs_api(s: EndpointApi0) -> Result<(), Box<dyn Error>> {
    let (request, s) = s.recv()?;

    let s = s.send(random::<i32>())?;

    offer_mpst!(s, {
        Branching0fromCtoA::Up(s) => {

            let (_up, s) = s.recv()?;

            let s = s.send(request)?;

            let (response, s) = s.recv()?;

            let s = s.send(response)?;

            recurs_api(s)
        },
        Branching0fromCtoA::Down(s) => {

            let (failure, s) = s.recv()?;

            let s = s.send(failure)?;

            recurs_api(s)
        },
        Branching0fromCtoA::Close(s) => {

            let (close, s) = s.recv()?;

            let s = s.send(close)?;

            s.close()
        },
    })
}

fn endpoint_controller(s: EndpointControllerInit) -> Result<(), Box<dyn Error>> {
    let s = s.send(LOOPS)?;
    let s = s.send(LOOPS)?;
    let (_hard_ping, s) = s.recv()?;

    recurs_controller(s, LOOPS)
}

fn recurs_controller(s: EndpointController0, loops: i32) -> Result<(), Box<dyn Error>> {
    let (_get_mode, s) = s.recv()?;

    match loops {
        i if i < 0 => {
            let s: EndpointControllerClose = choose_mpst_controller_to_all!(
                s,
                Branching0fromCtoA::Close,
                Branching0fromCtoS::Close,
                Branching0fromCtoU::Close
            );

            let s = s.send(0)?;

            let s = s.send(0)?;

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointControllerUp = choose_mpst_controller_to_all!(
                s,
                Branching0fromCtoA::Up,
                Branching0fromCtoS::Up,
                Branching0fromCtoU::Up
            );

            let s = s.send(random::<i32>())?;

            recurs_controller(s, loops - 1)
        }
        _ => {
            let s: EndpointControllerDown = choose_mpst_controller_to_all!(
                s,
                Branching0fromCtoA::Down,
                Branching0fromCtoS::Down,
                Branching0fromCtoU::Down
            );

            let s = s.send(random::<i32>())?;

            let s = s.send(random::<i32>())?;

            recurs_controller(s, loops - 1)
        }
    }
}

fn endpoint_storage(s: EndpointStorageInit) -> Result<(), Box<dyn Error>> {
    let (_start, s) = s.recv()?;

    let s = s.send(random::<i32>())?;

    recurs_storage(s)
}

fn recurs_storage(s: EndpointStorage0) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromCtoS::Up(s) => {

            let (request, s) = s.recv()?;

            let s = s.send(request)?;

            recurs_storage(s)
        },
        Branching0fromCtoS::Down(s) => {

            let (_failure, s) = s.recv()?;

            recurs_storage(s)
        },
        Branching0fromCtoS::Close(s) => {

            let (_close, s) = s.recv()?;

            s.close()
        },
    })
}

fn endpoint_user(s: EndpointUserInit) -> Result<(), Box<dyn Error>> {
    let s = s.send(random::<i32>())?;

    offer_mpst!(s, {
        Branching0fromCtoU::Up(s) => {

            let (_response, s) = s.recv()?;

            endpoint_user(s)
        },
        Branching0fromCtoU::Down(s) => {

            let (_failure, s) = s.recv()?;

            endpoint_user(s)
        },
        Branching0fromCtoU::Close(s) => {

            let (_close, s) = s.recv()?;

            s.close()
        },
    })
}

/////////////////////////

fn aux() {
    let (thread_api, thread_controller, thread_storage, thread_user) = fork_mpst(
        black_box(endpoint_api),
        black_box(endpoint_controller),
        black_box(endpoint_storage),
        black_box(endpoint_user),
    );

    thread_api.join().unwrap();
    thread_controller.join().unwrap();
    thread_storage.join().unwrap();
    thread_user.join().unwrap();
}

/////////////////////////

static LOOPS: i32 = 100;

pub fn circuit_breaker(c: &mut Criterion) {
    c.bench_function("Circuit breaker AMPST", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.05).without_plots().sample_size(20000);
    targets = circuit_breaker,
}

criterion_main! {
    bench
}
