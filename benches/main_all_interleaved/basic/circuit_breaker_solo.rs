use criterion::{black_box, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    baker, close_mpst_interleaved, fork_mpst_multi_solo,
    offer_mpst_interleaved,
};

use rand::random;

use std::error::Error;
use std::marker;
// use std::time::Duration;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
baker!("rec_and_cancel", MeshedChannelsFour, Api, Controller, Storage, User);

close_mpst_interleaved!(close_mpst, MeshedChannelsFour, 4);

fork_mpst_multi_solo!(fork_mpst_solo, MeshedChannelsFour, 4);

// RoleApi
enum Branching0fromCtoA<N: marker::Send> {
    Up(
        MeshedChannelsFour<
            Recv<N, Send<N, Recurs0fromCtoA<N>>>,
            Send<N, Recv<N, End>>,
            Send<N, Recv<N, End>>,
            RoleController<
                RoleStorage<
                    RoleStorage<RoleUser<RoleUser<RoleController<RoleController<RoleEnd>>>>>,
                >,
            >,
            NameApi,
        >,
    ),
    Down(
        MeshedChannelsFour<
            Recv<N, Send<N, Recurs0fromCtoA<N>>>,
            End,
            Send<N, Recv<N, End>>,
            RoleController<RoleUser<RoleUser<RoleController<RoleController<RoleEnd>>>>>,
            NameApi,
        >,
    ),
    Close(
        MeshedChannelsFour<
            Recv<N, End>,
            End,
            Send<N, End>,
            RoleController<RoleUser<RoleEnd>>,
            NameApi,
        >,
    ),
}
type Recurs0fromCtoA<N> = Recv<Branching0fromCtoA<N>, End>;
// RoleController
type Choose0fromCtoA<N> = Send<Branching0fromCtoA<N>, End>;
type Choose0fromCtoS<N> = Send<Branching0fromCtoS<N>, End>;
type Choose0fromCtoU<N> = Send<Branching0fromCtoU<N>, End>;
// RoleStorage
enum Branching0fromCtoS<N: marker::Send> {
    Up(
        MeshedChannelsFour<
            Recv<N, Send<N, End>>,
            Recurs0fromCtoS<N>,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameStorage,
        >,
    ),
    Down(
        MeshedChannelsFour<
            End,
            Recv<N, Recurs0fromCtoS<N>>,
            End,
            RoleController<RoleController<RoleEnd>>,
            NameStorage,
        >,
    ),
    Close(MeshedChannelsFour<End, Recv<N, End>, End, RoleController<RoleEnd>, NameStorage>),
}
type Recurs0fromCtoS<N> = Recv<Branching0fromCtoS<N>, End>;
// RoleUser
enum Branching0fromCtoU<N: marker::Send> {
    Up(
        MeshedChannelsFour<
            Recv<N, Send<N, End>>,
            Recurs0fromCtoU<N>,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameUser,
        >,
    ),
    Down(
        MeshedChannelsFour<
            Recv<N, Send<N, End>>,
            Recurs0fromCtoU<N>,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameUser,
        >,
    ),
    Close(MeshedChannelsFour<Recv<N, End>, End, End, RoleApi<RoleEnd>, NameUser>),
}
type Recurs0fromCtoU<N> = Recv<Branching0fromCtoU<N>, End>;

// Creating the MP sessions
// RoleApi
type EndpointApi0<N> = MeshedChannelsFour<
    Send<N, Recurs0fromCtoA<N>>,
    End,
    Recv<N, End>,
    RoleUser<RoleController<RoleController<RoleEnd>>>,
    NameApi,
>;
type EndpointApiInit<N> = MeshedChannelsFour<
    Recv<N, Send<N, Recurs0fromCtoA<N>>>,
    End,
    Recv<N, End>,
    RoleController<RoleUser<RoleController<RoleController<RoleEnd>>>>,
    NameApi,
>;
// RoleController
type EndpointControllerDown<N> = MeshedChannelsFour<
    Send<N, Recv<N, Choose0fromCtoA<N>>>,
    Send<N, Choose0fromCtoS<N>>,
    Choose0fromCtoU<N>,
    RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>,
    NameController,
>;
type EndpointControllerUp<N> = MeshedChannelsFour<
    Send<N, Recv<N, Choose0fromCtoA<N>>>,
    Choose0fromCtoS<N>,
    Choose0fromCtoU<N>,
    RoleApi<RoleApi<RoleBroadcast>>,
    NameController,
>;
type EndpointControllerClose<N> = MeshedChannelsFour<
    Send<N, End>,
    Send<N, End>,
    End,
    RoleApi<RoleStorage<RoleEnd>>,
    NameController,
>;
type EndpointController0<N> = MeshedChannelsFour<
    Recv<N, Choose0fromCtoA<N>>,
    Choose0fromCtoS<N>,
    Choose0fromCtoU<N>,
    RoleApi<RoleBroadcast>,
    NameController,
>;
type EndpointControllerInit<N> = MeshedChannelsFour<
    Send<N, Recv<N, Choose0fromCtoA<N>>>,
    Send<N, Recv<N, Choose0fromCtoS<N>>>,
    Choose0fromCtoU<N>,
    RoleStorage<RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>>,
    NameController,
>;
// RoleStorage
type EndpointStorage0<N> =
    MeshedChannelsFour<End, Recurs0fromCtoS<N>, End, RoleController<RoleEnd>, NameStorage>;
type EndpointStorageInit<N> = MeshedChannelsFour<
    End,
    Recv<N, Send<N, Recurs0fromCtoS<N>>>,
    End,
    RoleController<RoleController<RoleController<RoleEnd>>>,
    NameStorage,
>;
// RoleUser
type EndpointUserInit<N> = MeshedChannelsFour<
    Send<N, End>,
    Recurs0fromCtoU<N>,
    End,
    RoleApi<RoleController<RoleEnd>>,
    NameUser,
>;

/////////////////////////

fn start(
    s_api: EndpointApiInit<i32>,
    s_controller: EndpointControllerInit<i32>,
    s_storage: EndpointStorageInit<i32>,
    s_user: EndpointUserInit<i32>,
) -> Result<(), Box<dyn Error>> {
    let s_controller = s_controller.send(random::<i32>())?;
    let (_, s_storage) = s_storage.recv()?;
    let s_controller = s_controller.send(random::<i32>())?;
    let (_, s_api) = s_api.recv()?;
    let s_storage = s_storage.send(random::<i32>())?;
    let (_, s_controller) = s_controller.recv()?;

    rec_loop(s_api, s_controller, s_storage, s_user, 10)
}

fn rec_loop(
    s_api: EndpointApi0<i32>,
    s_controller: EndpointController0<i32>,
    s_storage: EndpointStorage0<i32>,
    s_user: EndpointUserInit<i32>,
    loops: i32,
) -> Result<(), Box<dyn Error>> {
    let s_user = s_user.send(random::<i32>())?;
    let (_, s_api) = s_api.recv()?;
    let s_api = s_api.send(random::<i32>())?;
    let (_, s_controller) = s_controller.recv()?;

    match loops {
        i if i <= 0 => {
            let s_controller: EndpointControllerClose<i32> = choose_mpst_controller_to_all!(
                s_controller,
                Branching0fromCtoA::Close,
                Branching0fromCtoS::Close,
                Branching0fromCtoU::Close
            );

            let (s_api, s_storage, s_user) = offer_mpst_interleaved!(
                s_api,
                Branching0fromCtoA::Close,
                s_storage,
                Branching0fromCtoS::Close,
                s_user,
                Branching0fromCtoU::Close
            );

            let s_controller = s_controller.send(random::<i32>())?;
            let (_, s_api) = s_api.recv()?;
            let s_controller = s_controller.send(random::<i32>())?;
            let (_, s_storage) = s_storage.recv()?;
            let s_api = s_api.send(random::<i32>())?;
            let (_, s_user) = s_user.recv()?;

            close_mpst(s_api, s_controller, s_storage, s_user)
        }
        i if i % 2 == 0 => {
            let s_controller: EndpointControllerUp<i32> = choose_mpst_controller_to_all!(
                s_controller,
                Branching0fromCtoA::Up,
                Branching0fromCtoS::Up,
                Branching0fromCtoU::Up
            );

            let (s_api, s_storage, s_user) = offer_mpst_interleaved!(
                s_api,
                Branching0fromCtoA::Up,
                s_storage,
                Branching0fromCtoS::Up,
                s_user,
                Branching0fromCtoU::Up
            );

            let s_controller = s_controller.send(random::<i32>())?;
            let (_, s_api) = s_api.recv()?;
            let s_api = s_api.send(random::<i32>())?;
            let (_, s_storage) = s_storage.recv()?;
            let s_storage = s_storage.send(random::<i32>())?;
            let (_, s_api) = s_api.recv()?;
            let s_api = s_api.send(random::<i32>())?;
            let (_, s_user) = s_user.recv()?;

            rec_loop(s_api, s_controller, s_storage, s_user, loops - 1)
        }
        _ => {
            let s_controller: EndpointControllerDown<i32> = choose_mpst_controller_to_all!(
                s_controller,
                Branching0fromCtoA::Down,
                Branching0fromCtoS::Down,
                Branching0fromCtoU::Down
            );

            let (s_api, s_storage, s_user) = offer_mpst_interleaved!(
                s_api,
                Branching0fromCtoA::Down,
                s_storage,
                Branching0fromCtoS::Down,
                s_user,
                Branching0fromCtoU::Down
            );

            let s_controller = s_controller.send(random::<i32>())?;
            let (_, s_api) = s_api.recv()?;
            let s_controller = s_controller.send(random::<i32>())?;
            let (_, s_storage) = s_storage.recv()?;
            let s_api = s_api.send(random::<i32>())?;
            let (_, s_user) = s_user.recv()?;

            rec_loop(s_api, s_controller, s_storage, s_user, loops - 1)
        }
    }
}

/////////////////////////

fn all_mpst() {
    fork_mpst_solo(black_box(start)).unwrap();
}

/////////////////////////

pub fn circuit_breaker_solo_main(c: &mut Criterion) {
    c.bench_function("Circuit breaker solo", |b| b.iter(all_mpst));
}
