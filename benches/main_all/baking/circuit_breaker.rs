#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum_and_cancel, offer_mpst};

use rand::random;

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
bundle_impl_with_enum_and_cancel!(MeshedChannelsFour, Api, Controller, Storage, User);

// Names
type NameRoleApi = RoleApi<RoleEnd>;
type NameRoleController = RoleController<RoleEnd>;
type NameRoleStorage = RoleStorage<RoleEnd>;
type NameRoleUser = RoleUser<RoleEnd>;

// RoleApi
enum Branching0fromCtoA {
    Up(
        MeshedChannelsFour<
            Recv<i32, Send<i32, Recurs0fromCtoA>>,
            Send<i32, Recv<i32, End>>,
            Send<i32, Recv<i32, End>>,
            RoleController<
                RoleStorage<
                    RoleStorage<RoleUser<RoleUser<RoleController<RoleController<RoleEnd>>>>>,
                >,
            >,
            NameRoleApi,
        >,
    ),
    Down(
        MeshedChannelsFour<
            Recv<i32, Send<i32, Recurs0fromCtoA>>,
            End,
            Send<i32, Recv<i32, End>>,
            RoleController<RoleUser<RoleUser<RoleController<RoleController<RoleEnd>>>>>,
            NameRoleApi,
        >,
    ),
    Close(
        MeshedChannelsFour<
            Recv<i32, End>,
            End,
            Send<i32, End>,
            RoleController<RoleUser<RoleEnd>>,
            NameRoleApi,
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
        MeshedChannelsFour<
            Recv<i32, Send<i32, End>>,
            Recurs0fromCtoS,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameRoleStorage,
        >,
    ),
    Down(
        MeshedChannelsFour<
            End,
            Recv<i32, Recurs0fromCtoS>,
            End,
            RoleController<RoleController<RoleEnd>>,
            NameRoleStorage,
        >,
    ),
    Close(MeshedChannelsFour<End, Recv<i32, End>, End, RoleController<RoleEnd>, NameRoleStorage>),
}
type Recurs0fromCtoS = Recv<Branching0fromCtoS, End>;
// RoleUser
enum Branching0fromCtoU {
    Up(
        MeshedChannelsFour<
            Recv<i32, Send<i32, End>>,
            Recurs0fromCtoU,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameRoleUser,
        >,
    ),
    Down(
        MeshedChannelsFour<
            Recv<i32, Send<i32, End>>,
            Recurs0fromCtoU,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameRoleUser,
        >,
    ),
    Close(MeshedChannelsFour<Recv<i32, End>, End, End, RoleApi<RoleEnd>, NameRoleUser>),
}
type Recurs0fromCtoU = Recv<Branching0fromCtoU, End>;

// Creating the MP sessions
// RoleApi
type EndpointApi0 = MeshedChannelsFour<
    Send<i32, Recurs0fromCtoA>,
    End,
    Recv<i32, End>,
    RoleUser<RoleController<RoleController<RoleEnd>>>,
    NameRoleApi,
>;
type EndpointApiInit = MeshedChannelsFour<
    Recv<i32, Send<i32, Recurs0fromCtoA>>,
    End,
    Recv<i32, End>,
    RoleController<RoleUser<RoleController<RoleController<RoleEnd>>>>,
    NameRoleApi,
>;
// RoleController
type EndpointControllerDown = MeshedChannelsFour<
    Send<i32, Recv<i32, Choose0fromCtoA>>,
    Send<i32, Choose0fromCtoS>,
    Choose0fromCtoU,
    RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>,
    NameRoleController,
>;
type EndpointControllerUp = MeshedChannelsFour<
    Send<i32, Recv<i32, Choose0fromCtoA>>,
    Choose0fromCtoS,
    Choose0fromCtoU,
    RoleApi<RoleApi<RoleBroadcast>>,
    NameRoleController,
>;
type EndpointControllerClose = MeshedChannelsFour<
    Send<i32, End>,
    Send<i32, End>,
    End,
    RoleApi<RoleStorage<RoleEnd>>,
    NameRoleController,
>;
type EndpointController0 = MeshedChannelsFour<
    Recv<i32, Choose0fromCtoA>,
    Choose0fromCtoS,
    Choose0fromCtoU,
    RoleApi<RoleBroadcast>,
    NameRoleController,
>;
type EndpointControllerInit = MeshedChannelsFour<
    Send<i32, Recv<i32, Choose0fromCtoA>>,
    Send<i32, Recv<i32, Choose0fromCtoS>>,
    Choose0fromCtoU,
    RoleStorage<RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>>,
    NameRoleController,
>;
// RoleStorage
type EndpointStorage0 =
    MeshedChannelsFour<End, Recurs0fromCtoS, End, RoleController<RoleEnd>, NameRoleStorage>;
type EndpointStorageInit = MeshedChannelsFour<
    End,
    Recv<i32, Send<i32, Recurs0fromCtoS>>,
    End,
    RoleController<RoleController<RoleController<RoleEnd>>>,
    NameRoleStorage,
>;
// RoleUser
type EndpointUserInit = MeshedChannelsFour<
    Send<i32, End>,
    Recurs0fromCtoU,
    End,
    RoleApi<RoleController<RoleEnd>>,
    NameRoleUser,
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
    let s = s.send(100)?;
    let s = s.send(100)?;
    let (_hard_ping, s) = s.recv()?;

    recurs_controller(s, 100)
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

            let s = s.send(-request)?;

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

fn all_mpst() {
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

fn circuit_breaker_main(c: &mut Criterion) {
    c.bench_function(&"Circuit breaker baking".to_string(), |b| b.iter(all_mpst));
}

criterion_group! {
    name = circuit_breaker;
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = circuit_breaker_main
}

criterion_main!(circuit_breaker);
