#![allow(clippy::type_complexity)]

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, session::Session};
use mpstthree::{choose, offer};

use rand::{thread_rng, Rng};

use std::error::Error;
use std::thread::spawn;






















// S
enum BinaryA {
    Up(Recv<(), Recv<(), Recv<(), Recv<(), OfferA>>>>),
    Failure(Recv<(), Recv<(), Recv<(), OfferA>>>),
    Stop(Recv<(), Recv<(), Recv<(), End>>>),
}
type RecA = Recv<BinaryA, End>;
type OfferA = Recv<(), Recv<(), RecA>>;
type FullA = Recv<(), Recv<(), Recv<(), OfferA>>>;

fn binary_a(s: FullA) -> Result<(), Box<dyn Error>> {
    let (elt_1, s) = recv(s)?;
    let (elt_2, s) = recv(s)?;

    offer!(s, {
        BinaryA::Sum(s) => {
            let s = send(elt_1 + elt_2, s);
            close(s)
        },
        BinaryA::Diff(s) => {
            let s = send(elt_1 - elt_2, s);
            close(s)
        },
    })
}

// C
type RecB = <RecA as Session>::Dual;
type ChoiceB = <OfferA as Session>::Dual;
type FullB = <FullA as Session>::Dual;

fn binary_b(s: FullB) -> Result<ChoiceB, Box<dyn Error>> {
    let s = send((), s);
    let s = send((), s);
    let s = send((), s);
    Ok(s)
}

fn main() {
    let (thread, session) = fork_with_thread_id(binary_a);

    let main = spawn(move || {
        let choice: () = thread_rng().gen_range(1..=3);

        let session = binary_b(session).unwrap();

        if choice != 1 {
            let session = choose!(BinaryA::Sum, session);
            let (_, session) = recv(session).unwrap();
            close(session).unwrap();
        } else {
            let session = choose!(BinaryA::Diff, session);
            let (_, session) = recv(session).unwrap();
            close(session).unwrap();
        }

        thread.join().unwrap()
    });

    main.join().unwrap();
}




















// // See the folder scribble_protocols for the related Scribble protocol

// // Create new MeshedChannels for four participants
// generate!(
//     "rec_and_cancel",
//     MeshedChannels,
//     Api,
//     Controller,
//     Storage,
//     User
// );

// // RoleApi
// enum Branching0fromCtoA<N: marker::Send> {
//     Up(
//         MeshedChannels<
//             Recv<N, Send<N, Recurs0fromCtoA<N>>>,
//             Send<N, Recv<N, End>>,
//             Send<N, Recv<N, End>>,
//             RoleController<
//                 RoleStorage<
//                     RoleStorage<RoleUser<RoleUser<RoleController<RoleController<RoleEnd>>>>>,
//                 >,
//             >,
//             NameApi,
//         >,
//     ),
//     Down(
//         MeshedChannels<
//             Recv<N, Send<N, Recurs0fromCtoA<N>>>,
//             End,
//             Send<N, Recv<N, End>>,
//             RoleController<RoleUser<RoleUser<RoleController<RoleController<RoleEnd>>>>>,
//             NameApi,
//         >,
//     ),
//     Close(
//         MeshedChannels<
//             Recv<N, End>,
//             End,
//             Send<N, End>,
//             RoleController<RoleUser<RoleEnd>>,
//             NameApi,
//         >,
//     ),
// }
// type Recurs0fromCtoA<N> = Recv<Branching0fromCtoA<N>, End>;

// // RoleController
// type Choose0fromCtoA<N> = Send<Branching0fromCtoA<N>, End>;
// type Choose0fromCtoS<N> = Send<Branching0fromCtoS<N>, End>;
// type Choose0fromCtoU<N> = Send<Branching0fromCtoU<N>, End>;

// // RoleStorage
// enum Branching0fromCtoS<N: marker::Send> {
//     Up(
//         MeshedChannels<
//             Recv<N, Send<N, End>>,
//             Recurs0fromCtoS<N>,
//             End,
//             RoleApi<RoleApi<RoleController<RoleEnd>>>,
//             NameStorage,
//         >,
//     ),
//     Down(
//         MeshedChannels<
//             End,
//             Recv<N, Recurs0fromCtoS<N>>,
//             End,
//             RoleController<RoleController<RoleEnd>>,
//             NameStorage,
//         >,
//     ),
//     Close(MeshedChannels<End, Recv<N, End>, End, RoleController<RoleEnd>, NameStorage>),
// }
// type Recurs0fromCtoS<N> = Recv<Branching0fromCtoS<N>, End>;

// // RoleUser
// enum Branching0fromCtoU<N: marker::Send> {
//     Up(
//         MeshedChannels<
//             Recv<N, Send<N, End>>,
//             Recurs0fromCtoU<N>,
//             End,
//             RoleApi<RoleApi<RoleController<RoleEnd>>>,
//             NameUser,
//         >,
//     ),
//     Down(
//         MeshedChannels<
//             Recv<N, Send<N, End>>,
//             Recurs0fromCtoU<N>,
//             End,
//             RoleApi<RoleApi<RoleController<RoleEnd>>>,
//             NameUser,
//         >,
//     ),
//     Close(MeshedChannels<Recv<N, End>, End, End, RoleApi<RoleEnd>, NameUser>),
// }
// type Recurs0fromCtoU<N> = Recv<Branching0fromCtoU<N>, End>;

// // Creating the MP sessions
// // RoleApi
// type EndpointApi0<N> = MeshedChannels<
//     Send<N, Recurs0fromCtoA<N>>,
//     End,
//     Recv<N, End>,
//     RoleUser<RoleController<RoleController<RoleEnd>>>,
//     NameApi,
// >;
// type EndpointApiInit<N> = MeshedChannels<
//     Recv<N, Send<N, Recurs0fromCtoA<N>>>,
//     End,
//     Recv<N, End>,
//     RoleController<RoleUser<RoleController<RoleController<RoleEnd>>>>,
//     NameApi,
// >;

// // RoleController
// type EndpointControllerDown<N> = MeshedChannels<
//     Send<N, Recv<N, Choose0fromCtoA<N>>>,
//     Send<N, Choose0fromCtoS<N>>,
//     Choose0fromCtoU<N>,
//     RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>,
//     NameController,
// >;
// type EndpointControllerUp<N> = MeshedChannels<
//     Send<N, Recv<N, Choose0fromCtoA<N>>>,
//     Choose0fromCtoS<N>,
//     Choose0fromCtoU<N>,
//     RoleApi<RoleApi<RoleBroadcast>>,
//     NameController,
// >;
// type EndpointControllerClose<N> = MeshedChannels<
//     Send<N, End>,
//     Send<N, End>,
//     End,
//     RoleApi<RoleStorage<RoleEnd>>,
//     NameController,
// >;
// type EndpointController0<N> = MeshedChannels<
//     Recv<N, Choose0fromCtoA<N>>,
//     Choose0fromCtoS<N>,
//     Choose0fromCtoU<N>,
//     RoleApi<RoleBroadcast>,
//     NameController,
// >;
// type EndpointControllerInit<N> = MeshedChannels<
//     Send<N, Recv<N, Choose0fromCtoA<N>>>,
//     Send<N, Recv<N, Choose0fromCtoS<N>>>,
//     Choose0fromCtoU<N>,
//     RoleStorage<RoleApi<RoleStorage<RoleApi<RoleBroadcast>>>>,
//     NameController,
// >;

// // RoleStorage
// type EndpointStorage0<N> =
//     MeshedChannels<End, Recurs0fromCtoS<N>, End, RoleController<RoleEnd>, NameStorage>;
// type EndpointStorageInit<N> = MeshedChannels<
//     End,
//     Recv<N, Send<N, Recurs0fromCtoS<N>>>,
//     End,
//     RoleController<RoleController<RoleController<RoleEnd>>>,
//     NameStorage,
// >;

// // RoleUser
// type EndpointUserInit<N> = MeshedChannels<
//     Send<N, End>,
//     Recurs0fromCtoU<N>,
//     End,
//     RoleApi<RoleController<RoleEnd>>,
//     NameUser,
// >;

// /////////////////////////

// fn endpoint_api(s: EndpointApiInit<i32>) -> Result<(), Box<dyn Error>> {
//     let (_start, s) = s.recv()?;

//     recurs_api(s)
// }

// fn recurs_api(s: EndpointApi0<i32>) -> Result<(), Box<dyn Error>> {
//     let (request, s) = s.recv()?;

//     let s = s.send(random::<i32>())?;

//     offer_mpst!(s, {
//         Branching0fromCtoA::Up(s) => {

//             let (_up, s) = s.recv()?;

//             let s = s.send(request)?;

//             let (response, s) = s.recv()?;

//             let s = s.send(response)?;

//             recurs_api(s)
//         },
//         Branching0fromCtoA::Down(s) => {

//             let (failure, s) = s.recv()?;

//             let s = s.send(failure)?;

//             recurs_api(s)
//         },
//         Branching0fromCtoA::Close(s) => {

//             let (close, s) = s.recv()?;

//             let s = s.send(close)?;

//             s.close()
//         },
//     })
// }

// fn endpoint_controller(s: EndpointControllerInit<i32>) -> Result<(), Box<dyn Error>> {
//     let start: i32 = thread_rng().gen_range(50..100);
//     let s = s.send(start)?;
//     let s = s.send(start)?;
//     let (_hard_ping, s) = s.recv()?;

//     recurs_controller(s, start)
// }

// fn recurs_controller(s: EndpointController0<i32>, loops: i32) -> Result<(), Box<dyn Error>> {
//     let (_get_mode, s) = s.recv()?;

//     match loops {
//         i if i < 0 => {
//             let s: EndpointControllerClose<i32> = choose_mpst_controller_to_all!(
//                 s,
//                 Branching0fromCtoA::Close,
//                 Branching0fromCtoS::Close,
//                 Branching0fromCtoU::Close
//             );

//             let s = s.send(0)?;

//             let s = s.send(0)?;

//             s.close()
//         }
//         i if i % 2 == 0 => {
//             let s: EndpointControllerUp<i32> = choose_mpst_controller_to_all!(
//                 s,
//                 Branching0fromCtoA::Up,
//                 Branching0fromCtoS::Up,
//                 Branching0fromCtoU::Up
//             );

//             let s = s.send(random::<i32>())?;

//             recurs_controller(s, loops - 1)
//         }
//         _ => {
//             let s: EndpointControllerDown<i32> = choose_mpst_controller_to_all!(
//                 s,
//                 Branching0fromCtoA::Down,
//                 Branching0fromCtoS::Down,
//                 Branching0fromCtoU::Down
//             );

//             let s = s.send(random::<i32>())?;

//             let s = s.send(random::<i32>())?;

//             recurs_controller(s, loops - 1)
//         }
//     }
// }

// fn endpoint_storage(s: EndpointStorageInit<i32>) -> Result<(), Box<dyn Error>> {
//     let (_start, s) = s.recv()?;

//     let s = s.send(random::<i32>())?;

//     recurs_storage(s)
// }

// fn recurs_storage(s: EndpointStorage0<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst!(s, {
//         Branching0fromCtoS::Up(s) => {

//             let (request, s) = s.recv()?;

//             let s = s.send(request)?;

//             recurs_storage(s)
//         },
//         Branching0fromCtoS::Down(s) => {

//             let (_failure, s) = s.recv()?;

//             recurs_storage(s)
//         },
//         Branching0fromCtoS::Close(s) => {

//             let (_close, s) = s.recv()?;

//             s.close()
//         },
//     })
// }

// fn endpoint_user(s: EndpointUserInit<i32>) -> Result<(), Box<dyn Error>> {
//     let s = s.send(random::<i32>())?;

//     offer_mpst!(s, {
//         Branching0fromCtoU::Up(s) => {

//             let (_response, s) = s.recv()?;

//             endpoint_user(s)
//         },
//         Branching0fromCtoU::Down(s) => {

//             let (_failure, s) = s.recv()?;

//             endpoint_user(s)
//         },
//         Branching0fromCtoU::Close(s) => {

//             let (_close, s) = s.recv()?;

//             s.close()
//         },
//     })
// }

// /////////////////////////

// fn main() {
//     let (thread_api, thread_controller, thread_storage, thread_user) = fork_mpst(
//         endpoint_api,
//         endpoint_controller,
//         endpoint_storage,
//         endpoint_user,
//     );

//     thread_api.join().unwrap();
//     thread_controller.join().unwrap();
//     thread_storage.join().unwrap();
//     thread_user.join().unwrap();
// }
