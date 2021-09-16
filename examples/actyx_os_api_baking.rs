use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum_and_cancel, offer_mpst};

use rand::random;

use std::error::Error;
use std::marker;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
bundle_impl_with_enum_and_cancel!(
    MeshedChannelsFour =>
    Api,
    Controller,
    Storage,
    User =>
    fork_mpst
);

// Names
type NameRoleApi = RoleApi<RoleEnd>;
type NameRoleController = RoleController<RoleEnd>;
type NameRoleStorage = RoleStorage<RoleEnd>;
type NameRoleUser = RoleUser<RoleEnd>;

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
            NameRoleApi,
        >,
    ),
    Down(
        MeshedChannelsFour<
            Recv<N, Send<N, Recurs0fromCtoA<N>>>,
            End,
            Send<N, Recv<N, End>>,
            RoleController<RoleUser<RoleUser<RoleController<RoleController<RoleEnd>>>>>,
            NameRoleApi,
        >,
    ),
    Close(
        MeshedChannelsFour<
            Recv<N, End>,
            End,
            Send<N, End>,
            RoleController<RoleUser<RoleEnd>>,
            NameRoleApi,
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
            NameRoleStorage,
        >,
    ),
    Down(
        MeshedChannelsFour<
            End,
            Recv<N, Recurs0fromCtoS<N>>,
            End,
            RoleController<RoleController<RoleEnd>>,
            NameRoleStorage,
        >,
    ),
    Close(MeshedChannelsFour<End, Recv<N, End>, End, RoleController<RoleEnd>, NameRoleStorage>),
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
            NameRoleUser,
        >,
    ),
    Down(
        MeshedChannelsFour<
            Recv<N, Send<N, End>>,
            Recurs0fromCtoU<N>,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameRoleUser,
        >,
    ),
    Close(MeshedChannelsFour<Recv<N, End>, End, End, RoleApi<RoleEnd>, NameRoleUser>),
}
type Recurs0fromCtoU<N> = Recv<Branching0fromCtoU<N>, End>;

// Creating the MP sessions
// RoleApi
type EndpointApi0<N> = MeshedChannelsFour<
    Send<N, Recurs0fromCtoA<N>>,
    End,
    Recv<N, End>,
    RoleUser<RoleController<RoleController<RoleEnd>>>,
    NameRoleApi,
>;
type EndpointApiInit<N> = MeshedChannelsFour<
    Recv<N, Send<N, Recurs0fromCtoA<N>>>,
    End,
    Recv<N, End>,
    RoleController<RoleUser<RoleController<RoleController<RoleEnd>>>>,
    NameRoleApi,
>;
// RoleController
type EndpointControllerDown<N> = MeshedChannelsFour<
    Send<N, Recv<N, Choose0fromCtoA<N>>>,
    Send<N, Choose0fromCtoS<N>>,
    Choose0fromCtoU<N>,
    RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>,
    NameRoleController,
>;
type EndpointControllerUp<N> = MeshedChannelsFour<
    Send<N, Recv<N, Choose0fromCtoA<N>>>,
    Choose0fromCtoS<N>,
    Choose0fromCtoU<N>,
    RoleApi<RoleApi<RoleBroadcast>>,
    NameRoleController,
>;
type EndpointControllerClose<N> = MeshedChannelsFour<
    Send<N, End>,
    Send<N, End>,
    End,
    RoleApi<RoleStorage<RoleEnd>>,
    NameRoleController,
>;
type EndpointController0<N> = MeshedChannelsFour<
    Recv<N, Choose0fromCtoA<N>>,
    Choose0fromCtoS<N>,
    Choose0fromCtoU<N>,
    RoleApi<RoleBroadcast>,
    NameRoleController,
>;
type EndpointControllerInit<N> = MeshedChannelsFour<
    Send<N, Recv<N, Choose0fromCtoA<N>>>,
    Send<N, Recv<N, Choose0fromCtoS<N>>>,
    Choose0fromCtoU<N>,
    RoleStorage<RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>>,
    NameRoleController,
>;
// RoleStorage
type EndpointStorage0<N> =
    MeshedChannelsFour<End, Recurs0fromCtoS<N>, End, RoleController<RoleEnd>, NameRoleStorage>;
type EndpointStorageInit<N> = MeshedChannelsFour<
    End,
    Recv<N, Send<N, Recurs0fromCtoS<N>>>,
    End,
    RoleController<RoleController<RoleController<RoleEnd>>>,
    NameRoleStorage,
>;
// RoleUser
type EndpointUserInit<N> = MeshedChannelsFour<
    Send<N, End>,
    Recurs0fromCtoU<N>,
    End,
    RoleApi<RoleController<RoleEnd>>,
    NameRoleUser,
>;

/////////////////////////

fn endpoint_api(s: EndpointApiInit<i32>) -> Result<(), Box<dyn Error>> {
    let (_start, s) = s.recv()?;

    recurs_api(s)
}

fn recurs_api(s: EndpointApi0<i32>) -> Result<(), Box<dyn Error>> {
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

fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
    let s = s.send(100)?;
    let s = s.send(100)?;
    let (_hard_ping, s) = s.recv()?;

    recurs_controller(s, 100)
}

fn recurs_controller(s: EndpointController0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    let (_get_mode, s) = s.recv()?;

    match loops {
        i if i < 0 => {
            let s: EndpointControllerClose<i32> = choose_mpst_controller_to_all!(
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
            let s: EndpointControllerUp<i32> = choose_mpst_controller_to_all!(
                s,
                Branching0fromCtoA::Up,
                Branching0fromCtoS::Up,
                Branching0fromCtoU::Up
            );

            let s = s.send(random::<i32>())?;

            recurs_controller(s, loops - 1)
        }
        _ => {
            let s: EndpointControllerDown<i32> = choose_mpst_controller_to_all!(
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

fn endpoint_storage(s: EndpointStorageInit<i32>) -> Result<(), Box<dyn Error>> {
    let (_start, s) = s.recv()?;

    let s = s.send(random::<i32>())?;

    recurs_storage(s)
}

fn recurs_storage(s: EndpointStorage0<i32>) -> Result<(), Box<dyn Error>> {
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

fn endpoint_user(s: EndpointUserInit<i32>) -> Result<(), Box<dyn Error>> {
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

fn main() {
    let (thread_api, thread_controller, thread_storage, thread_user) = fork_mpst(
        endpoint_api,
        endpoint_controller,
        endpoint_storage,
        endpoint_user,
    );

    thread_api.join().unwrap();
    thread_controller.join().unwrap();
    thread_storage.join().unwrap();
    thread_user.join().unwrap();
}
