use criterion::{black_box, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    choose_mpst_multi_to_all, close_mpst, create_meshedchannels, create_multiple_normal_name,
    create_multiple_normal_role, create_recv_mpst_session_bundle, create_send_mpst_cancel_bundle,
    fork_mpst_multi, offer_mpst,
};

use rand::random;

use std::error::Error;
use std::marker;

// use std::time::Duration;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for seven participants
create_meshedchannels!(MeshedChannelsFour, 4);

// Create Roles
create_multiple_normal_role!(
    Api, DualAPI |
    Controller, DualController |
    Storage, DualStorage |
    User, DualUser |
);

// Create Names
create_multiple_normal_name!(NameApi, NameController, NameStorage, NameUser);

// Create send
create_send_mpst_cancel_bundle!(
    send_api_to_controller, Controller, 1 |
    send_api_to_storage, Storage, 2 |
    send_api_to_user, User, 3 | =>
    NameApi, MeshedChannelsFour, 4
);
create_send_mpst_cancel_bundle!(
    send_controller_to_api, Api, 1 |
    send_controller_to_storage, Storage, 2 | =>
    NameController, MeshedChannelsFour, 4
);
create_send_mpst_cancel_bundle!(
    send_storage_to_api, Api, 1 |
    send_storage_to_controller, Controller, 2 | =>
    NameStorage, MeshedChannelsFour, 4
);
create_send_mpst_cancel_bundle!(
    send_user_to_api, Api, 1 | =>
    NameUser, MeshedChannelsFour, 4
);

// Create recv
create_recv_mpst_session_bundle!(
    recv_api_from_controller, Controller, 1 |
    recv_api_from_storage, Storage, 2 |
    recv_api_from_user, User, 3 | =>
    NameApi, MeshedChannelsFour, 4
);
create_recv_mpst_session_bundle!(
    recv_controller_from_api, Api, 1 |
    recv_controller_from_storage, Storage, 2 | =>
    NameController, MeshedChannelsFour, 4
);
create_recv_mpst_session_bundle!(
    recv_storage_from_api, Api, 1 |
    recv_storage_from_controller, Controller, 2 | =>
    NameStorage, MeshedChannelsFour, 4
);
create_recv_mpst_session_bundle!(
    recv_user_from_api, Api, 1 |
    recv_user_from_controller, Controller, 2 | =>
    NameUser, MeshedChannelsFour, 4
);

// Create close function
close_mpst!(close_mpst_multi, MeshedChannelsFour, 4);

// Create fork function
fork_mpst_multi!(fork_mpst, MeshedChannelsFour, 4);

// Api
enum Branching0fromCtoA<N: marker::Send> {
    Up(
        MeshedChannelsFour<
            Recv<N, Send<N, Recurs0fromCtoA<N>>>,
            Send<N, Recv<N, End>>,
            Send<N, Recv<N, End>>,
            Controller<Storage<Storage<User<User<Controller<Controller<RoleEnd>>>>>>>,
            NameApi,
        >,
    ),
    Down(
        MeshedChannelsFour<
            Recv<N, Send<N, Recurs0fromCtoA<N>>>,
            End,
            Send<N, Recv<N, End>>,
            Controller<User<User<Controller<Controller<RoleEnd>>>>>,
            NameApi,
        >,
    ),
    Close(MeshedChannelsFour<Recv<N, End>, End, Send<N, End>, Controller<User<RoleEnd>>, NameApi>),
}
type Recurs0fromCtoA<N> = Recv<Branching0fromCtoA<N>, End>;

// Controller
type Choose0fromCtoA<N> = Send<Branching0fromCtoA<N>, End>;
type Choose0fromCtoS<N> = Send<Branching0fromCtoS<N>, End>;
type Choose0fromCtoU<N> = Send<Branching0fromCtoU<N>, End>;

// Storage
enum Branching0fromCtoS<N: marker::Send> {
    Up(
        MeshedChannelsFour<
            Recv<N, Send<N, End>>,
            Recurs0fromCtoS<N>,
            End,
            Api<Api<Controller<RoleEnd>>>,
            NameStorage,
        >,
    ),
    Down(
        MeshedChannelsFour<
            End,
            Recv<N, Recurs0fromCtoS<N>>,
            End,
            Controller<Controller<RoleEnd>>,
            NameStorage,
        >,
    ),
    Close(MeshedChannelsFour<End, Recv<N, End>, End, Controller<RoleEnd>, NameStorage>),
}
type Recurs0fromCtoS<N> = Recv<Branching0fromCtoS<N>, End>;

// User
enum Branching0fromCtoU<N: marker::Send> {
    Up(
        MeshedChannelsFour<
            Recv<N, Send<N, End>>,
            Recurs0fromCtoU<N>,
            End,
            Api<Api<Controller<RoleEnd>>>,
            NameUser,
        >,
    ),
    Down(
        MeshedChannelsFour<
            Recv<N, Send<N, End>>,
            Recurs0fromCtoU<N>,
            End,
            Api<Api<Controller<RoleEnd>>>,
            NameUser,
        >,
    ),
    Close(MeshedChannelsFour<Recv<N, End>, End, End, Api<RoleEnd>, NameUser>),
}
type Recurs0fromCtoU<N> = Recv<Branching0fromCtoU<N>, End>;

// Creating the MP sessions
// Api
type EndpointApi0<N> = MeshedChannelsFour<
    Send<N, Recurs0fromCtoA<N>>,
    End,
    Recv<N, End>,
    User<Controller<Controller<RoleEnd>>>,
    NameApi,
>;
type EndpointApiInit<N> = MeshedChannelsFour<
    Recv<N, Send<N, Recurs0fromCtoA<N>>>,
    End,
    Recv<N, End>,
    Controller<User<Controller<Controller<RoleEnd>>>>,
    NameApi,
>;

// Controller
type EndpointController0<N> = MeshedChannelsFour<
    Recv<N, Choose0fromCtoA<N>>,
    Choose0fromCtoS<N>,
    Choose0fromCtoU<N>,
    Api<RoleBroadcast>,
    NameController,
>;
type EndpointControllerInit<N> = MeshedChannelsFour<
    Send<N, Recv<N, Choose0fromCtoA<N>>>,
    Send<N, Recv<N, Choose0fromCtoS<N>>>,
    Choose0fromCtoU<N>,
    Storage<Api<Storage<Api<RoleBroadcast>>>>,
    NameController,
>;

// Storage
type EndpointStorage0<N> =
    MeshedChannelsFour<End, Recurs0fromCtoS<N>, End, Controller<RoleEnd>, NameStorage>;
type EndpointStorageInit<N> = MeshedChannelsFour<
    End,
    Recv<N, Send<N, Recurs0fromCtoS<N>>>,
    End,
    Controller<Controller<Controller<RoleEnd>>>,
    NameStorage,
>;

// User
type EndpointUserInit<N> =
    MeshedChannelsFour<Send<N, End>, Recurs0fromCtoU<N>, End, Api<Controller<RoleEnd>>, NameUser>;

/////////////////////////

fn endpoint_api(s: EndpointApiInit<i32>) -> Result<(), Box<dyn Error>> {
    let (_start, s) = recv_api_from_controller(s)?;

    recurs_api(s)
}

fn recurs_api(s: EndpointApi0<i32>) -> Result<(), Box<dyn Error>> {
    let (request, s) = recv_api_from_user(s)?;

    let s = send_api_to_controller(random::<i32>(), s)?;

    offer_mpst!(s, recv_api_from_controller, {
        Branching0fromCtoA::Up(s) => {

            let (_up, s) = recv_api_from_controller(s)?;

            let s = send_api_to_storage(request, s)?;

            let (response, s) = recv_api_from_storage(s)?;

            let s = send_api_to_user(response, s)?;

            recurs_api(s)
        },
        Branching0fromCtoA::Down(s) => {

            let (failure, s) = recv_api_from_controller(s)?;

            let s = send_api_to_user(failure, s)?;

            recurs_api(s)
        },
        Branching0fromCtoA::Close(s) => {

            let (close, s) = recv_api_from_controller(s)?;

            let s = send_api_to_user(close, s)?;

            close_mpst_multi(s)
        },
    })
}

fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_controller_to_storage(100, s)?;
    let s = send_controller_to_api(100, s)?;
    let (_hard_ping, s) = recv_controller_from_storage(s)?;

    recurs_controller(s, 100)
}

fn recurs_controller(s: EndpointController0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    let (_get_mode, s) = recv_controller_from_api(s)?;

    match loops {
        i if i < 0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromCtoA::Close,
                Branching0fromCtoS::Close,
                Branching0fromCtoU::Close, =>
                NameController,
                MeshedChannelsFour,
                2
            );

            let s = send_controller_to_api(0, s)?;

            let s = send_controller_to_storage(0, s)?;

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromCtoA::Up,
                Branching0fromCtoS::Up,
                Branching0fromCtoU::Up, =>
                NameController,
                MeshedChannelsFour,
                2
            );

            let s = send_controller_to_api(random::<i32>(), s)?;

            recurs_controller(s, loops - 1)
        }
        _ => {
            let s = choose_mpst_multi_to_all!(
                s,
                Branching0fromCtoA::Down,
                Branching0fromCtoS::Down,
                Branching0fromCtoU::Down, =>
                NameController,
                MeshedChannelsFour,
                2
            );

            let s = send_controller_to_api(random::<i32>(), s)?;

            let s = send_controller_to_storage(random::<i32>(), s)?;

            recurs_controller(s, loops - 1)
        }
    }
}

fn endpoint_storage(s: EndpointStorageInit<i32>) -> Result<(), Box<dyn Error>> {
    let (_start, s) = recv_storage_from_controller(s)?;

    let s = send_storage_to_controller(random::<i32>(), s)?;

    recurs_storage(s)
}

fn recurs_storage(s: EndpointStorage0<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_storage_from_controller, {
        Branching0fromCtoS::Up(s) => {

            let (request, s) = recv_storage_from_api(s)?;

            let s = send_storage_to_api(request, s)?;

            recurs_storage(s)
        },
        Branching0fromCtoS::Down(s) => {

            let (_failure, s) = recv_storage_from_controller(s)?;

            recurs_storage(s)
        },
        Branching0fromCtoS::Close(s) => {

            let (_close, s) = recv_storage_from_controller(s)?;

            close_mpst_multi(s)
        },
    })
}

fn endpoint_user(s: EndpointUserInit<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_user_to_api(random::<i32>(), s)?;

    offer_mpst!(s, recv_user_from_controller, {
        Branching0fromCtoU::Up(s) => {

            let (_response, s) = recv_user_from_api(s)?;

            endpoint_user(s)
        },
        Branching0fromCtoU::Down(s) => {

            let (_failure, s) = recv_user_from_api(s)?;

            endpoint_user(s)
        },
        Branching0fromCtoU::Close(s) => {

            let (_close, s) = recv_user_from_api(s)?;

            close_mpst_multi(s)
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

pub fn circuit_breaker(c: &mut Criterion) {
    c.bench_function("Circuit breaker", |b| b.iter(all_mpst));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets = circuit_breaker,
}

criterion_main! {
    bench
}
