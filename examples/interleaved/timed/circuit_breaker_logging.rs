#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, session::Session};
use mpstthree::binary_timed::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{random, thread_rng, Rng};

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// CB = circuit breaker

generate!(
    "timed_interleaved",
    MeshedChannels,
    Api,
    ControllerCB,
    Storage,
    User,
    2,
    MeshedChannels,
    ControllerLog,
    Logs,
    1,
    fork_mpst
);

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
            RoleControllerCB<
                RoleStorage<
                    RoleStorage<RoleUser<RoleUser<RoleControllerCB<RoleControllerCB<RoleEnd>>>>>,
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
            RoleControllerCB<RoleUser<RoleUser<RoleControllerCB<RoleControllerCB<RoleEnd>>>>>,
            NameApi,
        >,
    ),
    Close(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            End,
            SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            RoleControllerCB<RoleUser<RoleEnd>>,
            NameApi,
        >,
    ),
}
type Recurs0fromCtoA = RecvTimed<Branching0fromCtoA, 'a', 0, true, 10, true, ' ', End>;

// RoleControllerCB
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
            RoleApi<RoleApi<RoleControllerCB<RoleEnd>>>,
            NameStorage,
        >,
    ),
    Down(
        MeshedChannels<
            End,
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoS>,
            End,
            RoleControllerCB<RoleControllerCB<RoleEnd>>,
            NameStorage,
        >,
    ),
    Close(
        MeshedChannels<
            End,
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            End,
            RoleControllerCB<RoleEnd>,
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
            RoleApi<RoleApi<RoleControllerCB<RoleEnd>>>,
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
            RoleApi<RoleApi<RoleControllerCB<RoleEnd>>>,
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

// RoleControllerLog
enum Branching0fromLtoC {
    Success(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoL>,
            RoleLogs<RoleLogs<RoleEnd>>,
            NameControllerLog,
        >,
    ),
    Failure(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose1fromCtoL>,
            RoleLogs<RoleBroadcast>,
            NameControllerLog,
        >,
    ),
}

type Recurs0fromCtoL = RecvTimed<Branching0fromLtoC, 'a', 0, true, 10, true, ' ', End>;

type Choose1fromCtoL = <Recurs1fromLtoC as Session>::Dual;

// RoleLogs
type Choose0fromLtoC = SendTimed<Branching0fromLtoC, 'a', 0, true, 10, true, ' ', End>;

enum Branching1fromCtoL {
    Restart(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromLtoC>,
            RoleControllerLog<RoleBroadcast>,
            NameLogs,
        >,
    ),
    Stop(
        MeshedChannels<
            RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
            RoleControllerLog<RoleEnd>,
            NameLogs,
        >,
    ),
}

type Recurs1fromLtoC = RecvTimed<Branching1fromCtoL, 'a', 0, true, 10, true, ' ', End>;

// Creating the MP sessions
// RoleApi
type EndpointApi0 = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoA>,
    End,
    RecvTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    RoleUser<RoleControllerCB<RoleControllerCB<RoleEnd>>>,
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
    RoleControllerCB<RoleUser<RoleControllerCB<RoleControllerCB<RoleEnd>>>>,
    NameApi,
>;

// RoleControllerCB
type EndpointCBControllerDown = MeshedChannels<
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
    NameControllerCB,
>;
type EndpointCBControllerUp = MeshedChannels<
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
    NameControllerCB,
>;
type EndpointCBControllerClose = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    End,
    RoleApi<RoleStorage<RoleEnd>>,
    NameControllerCB,
>;
type EndpointCBController0 = MeshedChannels<
    RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromCtoA>,
    Choose0fromCtoS,
    Choose0fromCtoU,
    RoleApi<RoleBroadcast>,
    NameControllerCB,
>;
type EndpointCBControllerInit = MeshedChannels<
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
    NameControllerCB,
>;
type EndpointLogController1Stop = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    RoleLogs<RoleEnd>,
    NameControllerLog,
>;
type EndpointLogController1Restart = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoL>,
    RoleLogs<RoleLogs<RoleEnd>>,
    NameControllerLog,
>;
type EndpointLogController0 =
    MeshedChannels<Recurs0fromCtoL, RoleLogs<RoleEnd>, NameControllerLog>;
type EndpointLogController1 = MeshedChannels<Choose1fromCtoL, RoleBroadcast, NameControllerLog>;
type EndpointLogControllerInit = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs0fromCtoL>,
    RoleLogs<RoleLogs<RoleEnd>>,
    NameControllerLog,
>;

// RoleStorage
type EndpointStorage0 =
    MeshedChannels<End, Recurs0fromCtoS, End, RoleControllerCB<RoleEnd>, NameStorage>;
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
    RoleControllerCB<RoleControllerCB<RoleControllerCB<RoleEnd>>>,
    NameStorage,
>;

// RoleUser
type EndpointUserInit = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', End>,
    Recurs0fromCtoU,
    End,
    RoleApi<RoleControllerCB<RoleEnd>>,
    NameUser,
>;

// RoleLogs
type EndpointLogs0Success = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromLtoC>,
    RoleControllerLog<RoleBroadcast>,
    NameLogs,
>;
type EndpointLogs0Failure = MeshedChannels<
    SendTimed<i32, 'a', 0, true, 10, true, ' ', Recurs1fromLtoC>,
    RoleControllerLog<RoleControllerLog<RoleEnd>>,
    NameLogs,
>;
type EndpointLogs0 = MeshedChannels<Choose0fromLtoC, RoleBroadcast, NameLogs>;
type EndpointLogs1 = MeshedChannels<Recurs1fromLtoC, RoleControllerLog<RoleEnd>, NameLogs>;
type EndpointLogsInit = MeshedChannels<
    RecvTimed<i32, 'a', 0, true, 10, true, ' ', Choose0fromLtoC>,
    RoleControllerLog<RoleBroadcast>,
    NameLogs,
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
    s_circuit_breaker: EndpointCBControllerInit,
    all_clocks_circuit_breaker: &mut HashMap<char, Instant>,
    s_logging: EndpointLogControllerInit,
    all_clocks_logging: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks_circuit_breaker.insert('a', Instant::now());
    all_clocks_logging.insert('a', Instant::now());

    let start_circuit_breaker: i32 = thread_rng().gen_range(50..100);
    let s_circuit_breaker =
        s_circuit_breaker.send(start_circuit_breaker, all_clocks_circuit_breaker)?;
    let s_circuit_breaker =
        s_circuit_breaker.send(start_circuit_breaker, all_clocks_circuit_breaker)?;
    let (_hard_ping, s_circuit_breaker) = s_circuit_breaker.recv(all_clocks_circuit_breaker)?;

    let start_logging: i32 = thread_rng().gen_range(50..100);
    let s_logging = s_logging.send(start_logging, all_clocks_logging)?;

    recurs_controller(
        s_circuit_breaker,
        start_circuit_breaker,
        all_clocks_circuit_breaker,
        s_logging,
        start_logging,
        all_clocks_logging,
    )
}

fn recurs_controller(
    s_circuit_breaker: EndpointCBController0,
    loops_circuit_breaker: i32,
    all_clocks_circuit_breaker: &mut HashMap<char, Instant>,
    s_logging: EndpointLogController0,
    loops_logging: i32,
    all_clocks_logging: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    let (_get_mode, s_circuit_breaker) = s_circuit_breaker.recv(all_clocks_circuit_breaker)?;

    match loops_circuit_breaker {
        i if i < 0 => {
            let s_circuit_breaker: EndpointCBControllerClose = choose_mpst_controllercb_to_all!(
                s_circuit_breaker,
                all_clocks_circuit_breaker,
                Branching0fromCtoA::Close,
                Branching0fromCtoS::Close,
                Branching0fromCtoU::Close
            );

            recurs_1_controller_end(
                s_circuit_breaker,
                loops_circuit_breaker - 1,
                all_clocks_circuit_breaker,
                s_logging,
                loops_logging,
                all_clocks_logging,
            )
        }
        i if i % 2 == 0 => {
            let s_circuit_breaker: EndpointCBControllerUp = choose_mpst_controllercb_to_all!(
                s_circuit_breaker,
                all_clocks_circuit_breaker,
                Branching0fromCtoA::Up,
                Branching0fromCtoS::Up,
                Branching0fromCtoU::Up
            );

            let s_circuit_breaker = s_circuit_breaker.send(random(), all_clocks_circuit_breaker)?;

            recurs_1_controller(
                s_circuit_breaker,
                loops_circuit_breaker - 1,
                all_clocks_circuit_breaker,
                s_logging,
                loops_logging,
                all_clocks_logging,
            )
        }
        _ => {
            let s_circuit_breaker: EndpointCBControllerDown = choose_mpst_controllercb_to_all!(
                s_circuit_breaker,
                all_clocks_circuit_breaker,
                Branching0fromCtoA::Down,
                Branching0fromCtoS::Down,
                Branching0fromCtoU::Down
            );

            let s_circuit_breaker = s_circuit_breaker.send(random(), all_clocks_circuit_breaker)?;
            let s_circuit_breaker = s_circuit_breaker.send(random(), all_clocks_circuit_breaker)?;

            recurs_1_controller(
                s_circuit_breaker,
                loops_circuit_breaker - 1,
                all_clocks_circuit_breaker,
                s_logging,
                loops_logging,
                all_clocks_logging,
            )
        }
    }
}

fn recurs_1_controller(
    s_circuit_breaker: EndpointCBController0,
    loops_circuit_breaker: i32,
    all_clocks_circuit_breaker: &mut HashMap<char, Instant>,
    s_logging: EndpointLogController0,
    loops_logging: i32,
    all_clocks_logging: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s_logging, all_clocks_logging, {
        Branching0fromLtoC::Success(s_logging) => {
            let (_, s_logging) = s_logging.recv(all_clocks_logging)?;
            recurs_controller(s_circuit_breaker, loops_circuit_breaker - 1, all_clocks_circuit_breaker, s_logging, loops_logging, all_clocks_logging)
        },
        Branching0fromLtoC::Failure(s_logging) => {
            let (_, s_logging) = s_logging.recv(all_clocks_logging)?;
            recurs_2_controller(s_circuit_breaker, loops_circuit_breaker, all_clocks_circuit_breaker, s_logging, loops_logging, all_clocks_logging)
        },
    })
}

fn recurs_2_controller(
    s_circuit_breaker: EndpointCBController0,
    loops_circuit_breaker: i32,
    all_clocks_circuit_breaker: &mut HashMap<char, Instant>,
    s_logging: EndpointLogController1,
    loops_logging: i32,
    all_clocks_logging: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops_logging {
        i if i <= 0 => {
            // Stop
            let s_logging: EndpointLogController1Stop = choose_mpst_controllerlog_to_all!(
                s_logging,
                all_clocks_logging,
                Branching1fromCtoL::Stop
            );

            let s_logging = s_logging.send(loops_logging - 1, all_clocks_logging)?;

            let (_get_mode, s_circuit_breaker) =
                s_circuit_breaker.recv(all_clocks_circuit_breaker)?;

            let s_circuit_breaker: EndpointCBControllerClose = choose_mpst_controllercb_to_all!(
                s_circuit_breaker,
                all_clocks_circuit_breaker,
                Branching0fromCtoA::Close,
                Branching0fromCtoS::Close,
                Branching0fromCtoU::Close
            );

            let s_circuit_breaker = s_circuit_breaker.send(random(), all_clocks_circuit_breaker)?;
            let s_circuit_breaker = s_circuit_breaker.send(random(), all_clocks_circuit_breaker)?;

            s_circuit_breaker.close()?;
            s_logging.close()
        }
        _ => {
            // Restart
            let s_logging: EndpointLogController1Restart = choose_mpst_controllerlog_to_all!(
                s_logging,
                all_clocks_logging,
                Branching1fromCtoL::Restart
            );

            let s_logging = s_logging.send(loops_logging - 1, all_clocks_logging)?;

            recurs_controller(
                s_circuit_breaker,
                loops_circuit_breaker - 1,
                all_clocks_circuit_breaker,
                s_logging,
                loops_logging - 1,
                all_clocks_logging,
            )
        }
    }
}

fn recurs_1_controller_end(
    s_circuit_breaker: EndpointCBControllerClose,
    loops_circuit_breaker: i32,
    all_clocks_circuit_breaker: &mut HashMap<char, Instant>,
    s_logging: EndpointLogController0,
    loops_logging: i32,
    all_clocks_logging: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s_logging, all_clocks_logging, {
        Branching0fromLtoC::Success(s_logging) => {
            let (_, s_logging) = s_logging.recv(all_clocks_logging)?;
            recurs_1_controller_end(s_circuit_breaker, loops_circuit_breaker - 1, all_clocks_circuit_breaker, s_logging, loops_logging, all_clocks_logging)
        },
        Branching0fromLtoC::Failure(s_logging) => {
            let (_, s_logging) = s_logging.recv(all_clocks_logging)?;
            recurs_2_controller_end(s_circuit_breaker, loops_circuit_breaker, all_clocks_circuit_breaker, s_logging, loops_logging, all_clocks_logging)
        },
    })
}

fn recurs_2_controller_end(
    s_circuit_breaker: EndpointCBControllerClose,
    loops_circuit_breaker: i32,
    all_clocks_circuit_breaker: &mut HashMap<char, Instant>,
    s_logging: EndpointLogController1,
    loops_logging: i32,
    all_clocks_logging: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops_logging {
        i if i <= 0 => {
            // Stop
            let s_logging: EndpointLogController1Stop = choose_mpst_controllerlog_to_all!(
                s_logging,
                all_clocks_logging,
                Branching1fromCtoL::Stop
            );

            let s_logging = s_logging.send(loops_logging - 1, all_clocks_logging)?;

            let s_circuit_breaker = s_circuit_breaker.send(random(), all_clocks_circuit_breaker)?;
            let s_circuit_breaker = s_circuit_breaker.send(random(), all_clocks_circuit_breaker)?;

            s_circuit_breaker.close()?;
            s_logging.close()
        }
        _ => {
            // Restart
            let s_logging: EndpointLogController1Restart = choose_mpst_controllerlog_to_all!(
                s_logging,
                all_clocks_logging,
                Branching1fromCtoL::Restart
            );

            let s_logging = s_logging.send(loops_logging - 1, all_clocks_logging)?;

            recurs_1_controller_end(
                s_circuit_breaker,
                loops_circuit_breaker - 1,
                all_clocks_circuit_breaker,
                s_logging,
                loops_logging - 1,
                all_clocks_logging,
            )
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

fn endpoint_logs(
    s: EndpointLogsInit,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (status, s) = s.recv(all_clocks)?;
    recurs_0_logs(s, status, all_clocks)
}

fn recurs_0_logs(
    s: EndpointLogs0,
    loops: i32,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i % 2 == 0 && i > 0 => {
            // Success
            let s: EndpointLogs0Success =
                choose_mpst_logs_to_all!(s, all_clocks, Branching0fromLtoC::Success);

            let s = s.send(loops - 1, all_clocks)?;

            recurs_0_logs(s, loops - 1, all_clocks)
        }
        _ => {
            // Failure
            let s: EndpointLogs0Failure =
                choose_mpst_logs_to_all!(s, all_clocks, Branching0fromLtoC::Failure);

            let s = s.send(loops - 1, all_clocks)?;

            recurs_1_logs(s, all_clocks)
        }
    }
}

fn recurs_1_logs(
    s: EndpointLogs1,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Branching1fromCtoL::Restart(s) => {

            let (loops, s) = s.recv(all_clocks)?;

            recurs_0_logs(s, loops - 1, all_clocks)
        },
        Branching1fromCtoL::Stop(s) => {

            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
    })
}

/////////////////////////

fn main() {
    let (thread_api, thread_storage, thread_user, thread_logs, thread_controller) = fork_mpst(
        endpoint_api,
        endpoint_storage,
        endpoint_user,
        endpoint_logs,
        endpoint_controller,
    );

    thread_api.join().unwrap();
    thread_controller.join().unwrap();
    thread_storage.join().unwrap();
    thread_user.join().unwrap();
    thread_logs.join().unwrap();
}
