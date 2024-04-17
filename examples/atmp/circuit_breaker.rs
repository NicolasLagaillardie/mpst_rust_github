#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::random;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
generate_atmp!(MeshedChannels, Api, Controller, Storage, User);

// RoleApi
enum Branching0fromCtoA {
    Up(
        MeshedChannels<
            RecvTimed<
                i32,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoA>,
            >,
            SendTimed<
                i32,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            >,
            SendTimed<
                i32,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            >,
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
            RecvTimed<
                i32,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoA>,
            >,
            End,
            SendTimed<
                i32,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            >,
            RoleController<RoleUser<RoleUser<RoleController<RoleController<RoleEnd>>>>>,
            NameApi,
        >,
    ),
    Close(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            End,
            SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            RoleController<RoleUser<RoleEnd>>,
            NameApi,
        >,
    ),
}
type Recurs0fromCtoA = RecvTimed<Branching0fromCtoA, 'a', 0, true, 10, true, ' ', End>;

// RoleController
type Choose0fromCtoA = <Recurs0fromCtoA as Session>::Dual;
type Choose0fromCtoS = <Recurs0fromCtoS as Session>::Dual;
type Choose0fromCtoU = <Recurs0fromCtoU as Session>::Dual;

// RoleStorage
enum Branching0fromCtoS {
    Up(
        MeshedChannels<
            RecvTimed<
                i32,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            >,
            Recurs0fromCtoS,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameStorage,
        >,
    ),
    Down(
        MeshedChannels<
            End,
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoS>,
            End,
            RoleController<RoleController<RoleEnd>>,
            NameStorage,
        >,
    ),
    Close(
        MeshedChannels<
            End,
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            End,
            RoleController<RoleEnd>,
            NameStorage,
        >,
    ),
}
type Recurs0fromCtoS = RecvTimed<Branching0fromCtoS, 'a', 0, true, 10, true, ' ', End>;

// RoleUser
enum Branching0fromCtoU {
    Up(
        MeshedChannels<
            RecvTimed<
                i32,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            >,
            Recurs0fromCtoU,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameUser,
        >,
    ),
    Down(
        MeshedChannels<
            RecvTimed<
                i32,
                'a',
                0,
                true,
                10,
                true,
                ' ',
                SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            >,
            Recurs0fromCtoU,
            End,
            RoleApi<RoleApi<RoleController<RoleEnd>>>,
            NameUser,
        >,
    ),
    Close(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            End,
            End,
            RoleApi<RoleEnd>,
            NameUser,
        >,
    ),
}
type Recurs0fromCtoU = RecvTimed<Branching0fromCtoU, 'a', 0, true, 10, true, ' ', End>;

// Creating the MP sessions
// RoleApi
type EndpointApi0 = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoA>,
    End,
    RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    RoleUser<RoleController<RoleController<RoleEnd>>>,
    NameApi,
>;
type EndpointApiInit = MeshedChannels<
    RecvTimed<
        i32,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoA>,
    >,
    End,
    RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    RoleController<RoleUser<RoleController<RoleController<RoleEnd>>>>,
    NameApi,
>;

// RoleController
type EndpointControllerDown = MeshedChannels<
    SendTimed<
        i32,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromCtoA>,
    >,
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromCtoS>,
    Choose0fromCtoU,
    RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>,
    NameController,
>;
type EndpointControllerUp = MeshedChannels<
    SendTimed<
        i32,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromCtoA>,
    >,
    Choose0fromCtoS,
    Choose0fromCtoU,
    RoleApi<RoleApi<RoleBroadcast>>,
    NameController,
>;
type EndpointControllerClose = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    End,
    RoleApi<RoleStorage<RoleEnd>>,
    NameController,
>;
type EndpointController0 = MeshedChannels<
    RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromCtoA>,
    Choose0fromCtoS,
    Choose0fromCtoU,
    RoleApi<RoleBroadcast>,
    NameController,
>;
type EndpointControllerInit = MeshedChannels<
    SendTimed<
        i32,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromCtoA>,
    >,
    SendTimed<
        i32,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromCtoS>,
    >,
    Choose0fromCtoU,
    RoleStorage<RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>>,
    NameController,
>;

// RoleStorage
type EndpointStorage0 =
    MeshedChannels<End, Recurs0fromCtoS, End, RoleController<RoleEnd>, NameStorage>;
type EndpointStorageInit = MeshedChannels<
    End,
    RecvTimed<
        i32,
        'a',
        0,
        true,
        10,
        true,
        ' ',
        SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoS>,
    >,
    End,
    RoleController<RoleController<RoleController<RoleEnd>>>,
    NameStorage,
>;

// RoleUser
type EndpointUserInit = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    Recurs0fromCtoU,
    End,
    RoleApi<RoleController<RoleEnd>>,
    NameUser,
>;

/////////////////////////

fn endpoint_api(
    s: EndpointApiInit,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_start, s) = s.recv(all_clocks)?;

    recurs_api(s, all_clocks)
}

fn recurs_api(
    s: EndpointApi0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    let (request, s) = s.recv(all_clocks)?;

    let s = s.send(random(), all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Branching0fromCtoA::Up(s) => {

            let (_up, s) = s.recv(all_clocks)?;

            let s = s.send(request, all_clocks)?;

            let (response, s) = s.recv(all_clocks)?;

            let s = s.send(response, all_clocks)?;

            recurs_api(s, all_clocks)
        },
        Branching0fromCtoA::Down(s) => {

            let (failure, s) = s.recv(all_clocks)?;

            let s = s.send(failure, all_clocks)?;

            recurs_api(s, all_clocks)
        },
        Branching0fromCtoA::Close(s) => {

            let (close, s) = s.recv(all_clocks)?;

            let s = s.send(close, all_clocks)?;

            s.close()
        },
    })
}

fn endpoint_controller(
    s: EndpointControllerInit,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(LOOPS, all_clocks)?;
    let s = s.send(LOOPS, all_clocks)?;
    let (_hard_ping, s) = s.recv(all_clocks)?;

    recurs_controller(s, LOOPS, all_clocks)
}

fn recurs_controller(
    s: EndpointController0,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    let (_get_mode, s) = s.recv(all_clocks)?;

    match loops {
        i if i < 0 => {
            let s: EndpointControllerClose = choose_mpst_controller_to_all!(
                s,
                all_clocks,
                Branching0fromCtoA::Close,
                Branching0fromCtoS::Close,
                Branching0fromCtoU::Close
            );

            let s = s.send(0, all_clocks)?;

            let s = s.send(0, all_clocks)?;

            s.close()
        }
        i if i % 2 == 0 => {
            let s: EndpointControllerUp = choose_mpst_controller_to_all!(
                s,
                all_clocks,
                Branching0fromCtoA::Up,
                Branching0fromCtoS::Up,
                Branching0fromCtoU::Up
            );

            let s = s.send(random(), all_clocks)?;

            recurs_controller(s, loops - 1, all_clocks)
        }
        _ => {
            let s: EndpointControllerDown = choose_mpst_controller_to_all!(
                s,
                all_clocks,
                Branching0fromCtoA::Down,
                Branching0fromCtoS::Down,
                Branching0fromCtoU::Down
            );

            let s = s.send(random(), all_clocks)?;

            let s = s.send(random(), all_clocks)?;

            recurs_controller(s, loops - 1, all_clocks)
        }
    }
}

fn endpoint_storage(
    s: EndpointStorageInit,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_start, s) = s.recv(all_clocks)?;

    let s = s.send(random(), all_clocks)?;

    recurs_storage(s, all_clocks)
}

fn recurs_storage(
    s: EndpointStorage0,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching0fromCtoS::Up(s) => {

            let (request, s) = s.recv(all_clocks)?;

            let s = s.send(request, all_clocks)?;

            recurs_storage(s, all_clocks)
        },
        Branching0fromCtoS::Down(s) => {

            let (_failure, s) = s.recv(all_clocks)?;

            recurs_storage(s, all_clocks)
        },
        Branching0fromCtoS::Close(s) => {

            let (_close, s) = s.recv(all_clocks)?;

            s.close()
        },
    })
}

fn endpoint_user(
    s: EndpointUserInit,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(random(), all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Branching0fromCtoU::Up(s) => {

            let (_response, s) = s.recv(all_clocks)?;

            endpoint_user(s, all_clocks)
        },
        Branching0fromCtoU::Down(s) => {

            let (_failure, s) = s.recv(all_clocks)?;

            endpoint_user(s, all_clocks)
        },
        Branching0fromCtoU::Close(s) => {

            let (_close, s) = s.recv(all_clocks)?;

            s.close()
        },
    })
}

/////////////////////////

static LOOPS: i32 = 100;

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
