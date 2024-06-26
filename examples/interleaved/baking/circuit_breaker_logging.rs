use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{generate, offer_mpst};

use rand::{random, thread_rng, Rng};

use std::error::Error;
use std::marker;

// CB = circuit breaker

generate!(
    "interleaved",
    MeshedChannelsFour,
    Api,
    ControllerCB,
    Storage,
    User,
    2,
    MeshedChannelsTwo,
    ControllerLog,
    Logs,
    1,
    fork_mpst
);

// RoleApi
enum Branching0fromCtoA<N: marker::Send> {
    Up(
        MeshedChannelsFour<
            Recv<N, Send<N, Recurs0fromCtoA<N>>>,
            Send<N, Recv<N, End>>,
            Send<N, Recv<N, End>>,
            RoleControllerCB<
                RoleStorage<
                    RoleStorage<RoleUser<RoleUser<RoleControllerCB<RoleControllerCB<RoleEnd>>>>>,
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
            RoleControllerCB<RoleUser<RoleUser<RoleControllerCB<RoleControllerCB<RoleEnd>>>>>,
            NameApi,
        >,
    ),
    Close(
        MeshedChannelsFour<
            Recv<N, End>,
            End,
            Send<N, End>,
            RoleControllerCB<RoleUser<RoleEnd>>,
            NameApi,
        >,
    ),
}
type Recurs0fromCtoA<N> = Recv<Branching0fromCtoA<N>, End>;

// RoleControllerCB
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
            RoleApi<RoleApi<RoleControllerCB<RoleEnd>>>,
            NameStorage,
        >,
    ),
    Down(
        MeshedChannelsFour<
            End,
            Recv<N, Recurs0fromCtoS<N>>,
            End,
            RoleControllerCB<RoleControllerCB<RoleEnd>>,
            NameStorage,
        >,
    ),
    Close(MeshedChannelsFour<End, Recv<N, End>, End, RoleControllerCB<RoleEnd>, NameStorage>),
}
type Recurs0fromCtoS<N> = Recv<Branching0fromCtoS<N>, End>;

// RoleUser
enum Branching0fromCtoU<N: marker::Send> {
    Up(
        MeshedChannelsFour<
            Recv<N, Send<N, End>>,
            Recurs0fromCtoU<N>,
            End,
            RoleApi<RoleApi<RoleControllerCB<RoleEnd>>>,
            NameUser,
        >,
    ),
    Down(
        MeshedChannelsFour<
            Recv<N, Send<N, End>>,
            Recurs0fromCtoU<N>,
            End,
            RoleApi<RoleApi<RoleControllerCB<RoleEnd>>>,
            NameUser,
        >,
    ),
    Close(MeshedChannelsFour<Recv<N, End>, End, End, RoleApi<RoleEnd>, NameUser>),
}
type Recurs0fromCtoU<N> = Recv<Branching0fromCtoU<N>, End>;

// RoleControllerLog
enum Branching0fromLtoC<N: marker::Send> {
    Success(
        MeshedChannelsTwo<
            Recv<N, Recurs0fromCtoL<N>>,
            RoleLogs<RoleLogs<RoleEnd>>,
            NameControllerLog,
        >,
    ),
    Failure(
        MeshedChannelsTwo<Recv<N, Choose1fromCtoL<N>>, RoleLogs<RoleBroadcast>, NameControllerLog>,
    ),
}

type Recurs0fromCtoL<N> = Recv<Branching0fromLtoC<N>, End>;

type Choose1fromCtoL<N> = Send<Branching1fromCtoL<N>, End>;

// RoleLogs
type Choose0fromLtoC<N> = Send<Branching0fromLtoC<N>, End>;

enum Branching1fromCtoL<N: marker::Send> {
    Restart(
        MeshedChannelsTwo<Recv<N, Choose0fromLtoC<N>>, RoleControllerLog<RoleBroadcast>, NameLogs>,
    ),
    Stop(MeshedChannelsTwo<Recv<N, End>, RoleControllerLog<RoleEnd>, NameLogs>),
}

type Recurs1fromLtoC<N> = Recv<Branching1fromCtoL<N>, End>;

// Creating the MP sessions
// RoleApi
type EndpointApi0<N> = MeshedChannelsFour<
    Send<N, Recurs0fromCtoA<N>>,
    End,
    Recv<N, End>,
    RoleUser<RoleControllerCB<RoleControllerCB<RoleEnd>>>,
    NameApi,
>;
type EndpointApiInit<N> = MeshedChannelsFour<
    Recv<N, Send<N, Recurs0fromCtoA<N>>>,
    End,
    Recv<N, End>,
    RoleControllerCB<RoleUser<RoleControllerCB<RoleControllerCB<RoleEnd>>>>,
    NameApi,
>;

// RoleControllerCB
type EndpointCBControllerDown<N> = MeshedChannelsFour<
    Send<N, Recv<N, Choose0fromCtoA<N>>>,
    Send<N, Choose0fromCtoS<N>>,
    Choose0fromCtoU<N>,
    RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>,
    NameControllerCB,
>;
type EndpointCBControllerUp<N> = MeshedChannelsFour<
    Send<N, Recv<N, Choose0fromCtoA<N>>>,
    Choose0fromCtoS<N>,
    Choose0fromCtoU<N>,
    RoleApi<RoleApi<RoleBroadcast>>,
    NameControllerCB,
>;
type EndpointCBControllerClose<N> = MeshedChannelsFour<
    Send<N, End>,
    Send<N, End>,
    End,
    RoleApi<RoleStorage<RoleEnd>>,
    NameControllerCB,
>;
type EndpointCBController0<N> = MeshedChannelsFour<
    Recv<N, Choose0fromCtoA<N>>,
    Choose0fromCtoS<N>,
    Choose0fromCtoU<N>,
    RoleApi<RoleBroadcast>,
    NameControllerCB,
>;
type EndpointCBControllerInit<N> = MeshedChannelsFour<
    Send<N, Recv<N, Choose0fromCtoA<N>>>,
    Send<N, Recv<N, Choose0fromCtoS<N>>>,
    Choose0fromCtoU<N>,
    RoleStorage<RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>>,
    NameControllerCB,
>;
type EndpointLogController1Stop<N> =
    MeshedChannelsTwo<Send<N, End>, RoleLogs<RoleEnd>, NameControllerLog>;
type EndpointLogController1Restart<N> =
    MeshedChannelsTwo<Send<N, Recurs0fromCtoL<N>>, RoleLogs<RoleLogs<RoleEnd>>, NameControllerLog>;
type EndpointLogController0<N> =
    MeshedChannelsTwo<Recurs0fromCtoL<N>, RoleLogs<RoleEnd>, NameControllerLog>;
type EndpointLogController1<N> =
    MeshedChannelsTwo<Choose1fromCtoL<N>, RoleBroadcast, NameControllerLog>;
type EndpointLogControllerInit<N> =
    MeshedChannelsTwo<Send<N, Recurs0fromCtoL<N>>, RoleLogs<RoleLogs<RoleEnd>>, NameControllerLog>;

// RoleStorage
type EndpointStorage0<N> =
    MeshedChannelsFour<End, Recurs0fromCtoS<N>, End, RoleControllerCB<RoleEnd>, NameStorage>;
type EndpointStorageInit<N> = MeshedChannelsFour<
    End,
    Recv<N, Send<N, Recurs0fromCtoS<N>>>,
    End,
    RoleControllerCB<RoleControllerCB<RoleControllerCB<RoleEnd>>>,
    NameStorage,
>;

// RoleUser
type EndpointUserInit<N> = MeshedChannelsFour<
    Send<N, End>,
    Recurs0fromCtoU<N>,
    End,
    RoleApi<RoleControllerCB<RoleEnd>>,
    NameUser,
>;

// RoleLogs
type EndpointLogs0Success<N> =
    MeshedChannelsTwo<Send<N, Choose0fromLtoC<N>>, RoleControllerLog<RoleBroadcast>, NameLogs>;
type EndpointLogs0Failure<N> = MeshedChannelsTwo<
    Send<N, Recurs1fromLtoC<N>>,
    RoleControllerLog<RoleControllerLog<RoleEnd>>,
    NameLogs,
>;
type EndpointLogs0<N> = MeshedChannelsTwo<Choose0fromLtoC<N>, RoleBroadcast, NameLogs>;
type EndpointLogs1<N> = MeshedChannelsTwo<Recurs1fromLtoC<N>, RoleControllerLog<RoleEnd>, NameLogs>;
type EndpointLogsInit<N> =
    MeshedChannelsTwo<Recv<N, Choose0fromLtoC<N>>, RoleControllerLog<RoleBroadcast>, NameLogs>;

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

fn endpoint_controller(
    s_circuit_breaker: EndpointCBControllerInit<i32>,
    s_logging: EndpointLogControllerInit<i32>,
) -> Result<(), Box<dyn Error>> {
    let start_circuit_breaker: i32 = thread_rng().gen_range(50..100);
    let s_circuit_breaker = s_circuit_breaker.send(start_circuit_breaker)?;
    let s_circuit_breaker = s_circuit_breaker.send(start_circuit_breaker)?;
    let (_hard_ping, s_circuit_breaker) = s_circuit_breaker.recv()?;

    let start_logging: i32 = thread_rng().gen_range(50..100);
    let s_logging = s_logging.send(start_logging)?;

    recurs_controller(
        s_circuit_breaker,
        start_circuit_breaker,
        s_logging,
        start_logging,
    )
}

fn recurs_controller(
    s_circuit_breaker: EndpointCBController0<i32>,
    loops_circuit_breaker: i32,
    s_logging: EndpointLogController0<i32>,
    loops_logging: i32,
) -> Result<(), Box<dyn Error>> {
    let (_get_mode, s_circuit_breaker) = s_circuit_breaker.recv()?;

    match loops_circuit_breaker {
        i if i < 0 => {
            let s_circuit_breaker: EndpointCBControllerClose<i32> = choose_mpst_controllercb_to_all!(
                s_circuit_breaker,
                Branching0fromCtoA::Close,
                Branching0fromCtoS::Close,
                Branching0fromCtoU::Close
            );

            recurs_1_controller_end(
                s_circuit_breaker,
                loops_circuit_breaker - 1,
                s_logging,
                loops_logging,
            )
        }
        i if i % 2 == 0 => {
            let s_circuit_breaker: EndpointCBControllerUp<i32> = choose_mpst_controllercb_to_all!(
                s_circuit_breaker,
                Branching0fromCtoA::Up,
                Branching0fromCtoS::Up,
                Branching0fromCtoU::Up
            );

            let s_circuit_breaker = s_circuit_breaker.send(random::<i32>())?;

            recurs_1_controller(
                s_circuit_breaker,
                loops_circuit_breaker - 1,
                s_logging,
                loops_logging,
            )
        }
        _ => {
            let s_circuit_breaker: EndpointCBControllerDown<i32> = choose_mpst_controllercb_to_all!(
                s_circuit_breaker,
                Branching0fromCtoA::Down,
                Branching0fromCtoS::Down,
                Branching0fromCtoU::Down
            );

            let s_circuit_breaker = s_circuit_breaker.send(random::<i32>())?;
            let s_circuit_breaker = s_circuit_breaker.send(random::<i32>())?;

            recurs_1_controller(
                s_circuit_breaker,
                loops_circuit_breaker - 1,
                s_logging,
                loops_logging,
            )
        }
    }
}

fn recurs_1_controller(
    s_circuit_breaker: EndpointCBController0<i32>,
    loops_circuit_breaker: i32,
    s_logging: EndpointLogController0<i32>,
    loops_logging: i32,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s_logging, {
        Branching0fromLtoC::Success(s_logging) => {
            let (_, s_logging) = s_logging.recv()?;
            recurs_controller(s_circuit_breaker, loops_circuit_breaker - 1, s_logging, loops_logging)
        },
        Branching0fromLtoC::Failure(s_logging) => {
            let (_, s_logging) = s_logging.recv()?;
            recurs_2_controller(s_circuit_breaker, loops_circuit_breaker, s_logging, loops_logging)
        },
    })
}

fn recurs_2_controller(
    s_circuit_breaker: EndpointCBController0<i32>,
    loops_circuit_breaker: i32,
    s_logging: EndpointLogController1<i32>,
    loops_logging: i32,
) -> Result<(), Box<dyn Error>> {
    match loops_logging {
        i if i <= 0 => {
            // Stop
            let s_logging: EndpointLogController1Stop<i32> =
                choose_mpst_controllerlog_to_all!(s_logging, Branching1fromCtoL::Stop);

            let s_logging = s_logging.send(loops_logging - 1)?;

            let (_get_mode, s_circuit_breaker) = s_circuit_breaker.recv()?;

            let s_circuit_breaker: EndpointCBControllerClose<i32> = choose_mpst_controllercb_to_all!(
                s_circuit_breaker,
                Branching0fromCtoA::Close,
                Branching0fromCtoS::Close,
                Branching0fromCtoU::Close
            );

            let s_circuit_breaker = s_circuit_breaker.send(random::<i32>())?;
            let s_circuit_breaker = s_circuit_breaker.send(random::<i32>())?;

            s_circuit_breaker.close()?;
            s_logging.close()
        }
        _ => {
            // Restart
            let s_logging: EndpointLogController1Restart<i32> =
                choose_mpst_controllerlog_to_all!(s_logging, Branching1fromCtoL::Restart);

            let s_logging = s_logging.send(loops_logging - 1)?;

            recurs_controller(
                s_circuit_breaker,
                loops_circuit_breaker - 1,
                s_logging,
                loops_logging - 1,
            )
        }
    }
}

fn recurs_1_controller_end(
    s_circuit_breaker: EndpointCBControllerClose<i32>,
    loops_circuit_breaker: i32,
    s_logging: EndpointLogController0<i32>,
    loops_logging: i32,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s_logging, {
        Branching0fromLtoC::Success(s_logging) => {
            let (_, s_logging) = s_logging.recv()?;
            recurs_1_controller_end(s_circuit_breaker, loops_circuit_breaker - 1, s_logging, loops_logging)
        },
        Branching0fromLtoC::Failure(s_logging) => {
            let (_, s_logging) = s_logging.recv()?;
            recurs_2_controller_end(s_circuit_breaker, loops_circuit_breaker, s_logging, loops_logging)
        },
    })
}

fn recurs_2_controller_end(
    s_circuit_breaker: EndpointCBControllerClose<i32>,
    loops_circuit_breaker: i32,
    s_logging: EndpointLogController1<i32>,
    loops_logging: i32,
) -> Result<(), Box<dyn Error>> {
    match loops_logging {
        i if i <= 0 => {
            // Stop
            let s_logging: EndpointLogController1Stop<i32> =
                choose_mpst_controllerlog_to_all!(s_logging, Branching1fromCtoL::Stop);

            let s_logging = s_logging.send(loops_logging - 1)?;

            let s_circuit_breaker = s_circuit_breaker.send(random::<i32>())?;
            let s_circuit_breaker = s_circuit_breaker.send(random::<i32>())?;

            s_circuit_breaker.close()?;
            s_logging.close()
        }
        _ => {
            // Restart
            let s_logging: EndpointLogController1Restart<i32> =
                choose_mpst_controllerlog_to_all!(s_logging, Branching1fromCtoL::Restart);

            let s_logging = s_logging.send(loops_logging - 1)?;

            recurs_1_controller_end(
                s_circuit_breaker,
                loops_circuit_breaker - 1,
                s_logging,
                loops_logging - 1,
            )
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

fn endpoint_logs(s: EndpointLogsInit<i32>) -> Result<(), Box<dyn Error>> {
    let (status, s) = s.recv()?;
    recurs_0_logs(s, status)
}

fn recurs_0_logs(s: EndpointLogs0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
    match loops {
        i if i % 2 == 0 && i > 0 => {
            // Success
            let s: EndpointLogs0Success<i32> =
                choose_mpst_logs_to_all!(s, Branching0fromLtoC::Success);

            let s = s.send(loops - 1)?;

            recurs_0_logs(s, loops - 1)
        }
        _ => {
            // Failure
            let s: EndpointLogs0Failure<i32> =
                choose_mpst_logs_to_all!(s, Branching0fromLtoC::Failure);

            let s = s.send(loops - 1)?;

            recurs_1_logs(s)
        }
    }
}

fn recurs_1_logs(s: EndpointLogs1<i32>) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching1fromCtoL::Restart(s) => {

            let (loops, s) = s.recv()?;

            recurs_0_logs(s, loops - 1)
        },
        Branching1fromCtoL::Stop(s) => {

            let (_, s) = s.recv()?;

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
