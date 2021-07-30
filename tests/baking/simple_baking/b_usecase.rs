use rand::{thread_rng, Rng};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::functionmpst::fork::fork_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

use std::boxed::Box;
use std::error::Error;

use mpstthree::role::a::RoleA;
use mpstthree::role::a_dual::RoleADual;
use mpstthree::role::all_to_b::RoleAlltoB;
use mpstthree::role::b::RoleB;
use mpstthree::role::b_to_all::RoleBtoAll;
use mpstthree::role::c::RoleC;
use mpstthree::role::c_dual::RoleCDual;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::ChooseMpst;
use mpstthree::functionmpst::OfferMpst;

use petgraph::dot::Dot;

/// Test our usecase
/// Simple types
/// Authenticator = C
/// Server = A
/// Client = B

type CtoBClose = End;
type CtoAClose = End;
type CtoBVideo<N> = Recv<N, Send<N, End>>;
type CtoAVideo<N> = Send<N, Recv<N, End>>;

type AtoCClose = <CtoAClose as Session>::Dual;
type AtoBClose = End;
type AtoCVideo<N> = <CtoAVideo<N> as Session>::Dual;

type BtoAClose = <AtoBClose as Session>::Dual;
type BtoCClose = <CtoBClose as Session>::Dual;
type BtoCVideo<N> = <CtoBVideo<N> as Session>::Dual;

/// Stacks
type StackCEnd = RoleEnd;
type StackCVideo = RoleB<RoleA<RoleA<RoleB<RoleEnd>>>>;
type StackCVideoDual = <StackCVideo as Role>::Dual;
type StackCFull = RoleB<RoleB<RoleAlltoB<RoleEnd, RoleEnd>>>;

type StackAEnd = RoleEnd;
type StackAVideo = RoleC<RoleC<RoleEnd>>;
type StackAVideoDual = <StackAVideo as Role>::Dual;
type StackAFull = RoleAlltoB<RoleEnd, RoleEnd>;

type StackBEnd = RoleEnd;
type StackBVideo = RoleC<RoleC<RoleEnd>>;
type StackBChoice = RoleBtoAll<StackBVideo, StackBEnd>;
type StackBFull = RoleC<RoleC<StackBChoice>>;

/// Creating the MP sessions
/// For C
type ChooseBtoC<N> = ChooseMpst<
    AtoCVideo<N>,
    BtoCVideo<N>,
    AtoCClose,
    BtoCClose,
    StackCVideoDual,
    StackCEnd,
    RoleCDual<RoleEnd>,
>;
type ChooseBtoA<N> = ChooseMpst<
    BtoAClose,
    CtoAVideo<N>,
    BtoAClose,
    CtoAClose,
    StackAVideoDual,
    StackAEnd,
    RoleADual<RoleEnd>,
>;
type InitB<N> = Send<N, Recv<N, ChooseBtoC<N>>>;
type EndpointBFull<N> = MeshedChannels<ChooseBtoA<N>, InitB<N>, StackBFull, RoleB<RoleEnd>>;

/// For A
type EndpointCVideo<N> = MeshedChannels<CtoAVideo<N>, CtoBVideo<N>, StackCVideo, RoleC<RoleEnd>>;
type EndpointCEnd = MeshedChannels<CtoAClose, CtoBClose, StackCEnd, RoleC<RoleEnd>>;

type OfferC<N> = OfferMpst<
    CtoAVideo<N>,
    CtoBVideo<N>,
    CtoAClose,
    CtoBClose,
    StackCVideo,
    StackCEnd,
    RoleC<RoleEnd>,
>;
type InitC<N> = Recv<N, Send<N, OfferC<N>>>;
type EndpointCFull<N> = MeshedChannels<End, InitC<N>, StackCFull, RoleC<RoleEnd>>;

/// For B
type EndpointAVideo<N> = MeshedChannels<AtoBClose, AtoCVideo<N>, StackAVideo, RoleA<RoleEnd>>;
type EndpointAEnd = MeshedChannels<AtoBClose, AtoCClose, StackAEnd, RoleA<RoleEnd>>;

type OfferA<N> = OfferMpst<
    AtoBClose,
    AtoCVideo<N>,
    AtoBClose,
    AtoCClose,
    StackAVideo,
    StackAEnd,
    RoleA<RoleEnd>,
>;
type EndpointAFull<N> = MeshedChannels<OfferA<N>, End, StackAFull, RoleA<RoleEnd>>;

/// Functions related to endpoints
fn server(s: EndpointAFull<i32>) -> Result<(), Box<dyn Error>> {
    s.offer(
        |s: EndpointAVideo<i32>| {
            let (request, s) = s.recv()?;
            s.send(request + 1).close()
        },
        |s: EndpointAEnd| s.close(),
    )
}

fn authenticator(s: EndpointCFull<i32>) -> Result<(), Box<dyn Error>> {
    let (id, s) = s.recv()?;
    s.send(id + 1).offer(
        |s: EndpointCVideo<i32>| {
            let (request, s) = s.recv()?;
            let (video, s) = s.send(request + 1).recv()?;

            assert_eq!(request, id + 1);
            assert_eq!(video, id + 3);

            s.send(video + 1).close()
        },
        |s: EndpointCEnd| s.close(),
    )
}

fn client_video(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let id: i32 = thread_rng().gen();

    let (accept, s) = s.send(id).recv()?;
    let (result, s) = s.choose_left().send(accept).recv()?;

    assert_eq!(accept, id + 1);
    assert_eq!(result, accept + 3);

    s.close()
}

fn client_close(s: EndpointBFull<i32>) -> Result<(), Box<dyn Error>> {
    let id: i32 = thread_rng().gen();

    let (accept, s) = s.send(id).recv()?;

    assert_eq!(accept, id + 1);

    s.choose_right().close()
}

/////////////////////////////////////////

pub fn run_b_usecase_left() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test video branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(server, client_video, authenticator);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn run_b_usecase_right() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        // Test end branch.
        {
            let (thread_a, thread_b, thread_c) = fork_mpst(server, client_close, authenticator);

            assert!(thread_a.join().is_ok());
            assert!(thread_b.join().is_ok());
            assert!(thread_c.join().is_ok());
        }

        Ok(())
    }()
    .is_ok());
}

pub fn run_b_usecase_checker() {
    let graphs =
        mpstthree::checker_concat!(EndpointAFull<i32>, EndpointBFull<i32>, EndpointCFull<i32>)
            .unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_a)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"0.0\\\"\" ]\n    \
            2 [ label = \"\\\"0.1\\\"\" ]\n    \
            3 [ label = \"\\\"0.2\\\"\" ]\n    \
            4 [ label = \"\\\"0.3\\\"\" ]\n    \
            5 [ label = \"\\\"0.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"& RoleB\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleA?RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"RoleA!RoleC: i32\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"0\\\"\" ]\n    \
            1 -> 5 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_b)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"1\\\"\" ]\n    \
            2 [ label = \"\\\"2\\\"\" ]\n    \
            3 [ label = \"\\\"2.0\\\"\" ]\n    \
            4 [ label = \"\\\"2.1\\\"\" ]\n    \
            5 [ label = \"\\\"2.2\\\"\" ]\n    \
            6 [ label = \"\\\"2.3\\\"\" ]\n    \
            7 [ label = \"\\\"2.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleB!RoleC: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"+ RoleB\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleB!RoleC: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleB?RoleC: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"0\\\"\" ]\n    \
            3 -> 7 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(
        format!("{:?}", Dot::new(&graph_c)),
        "digraph {\n    \
            0 [ label = \"\\\"0\\\"\" ]\n    \
            1 [ label = \"\\\"1\\\"\" ]\n    \
            2 [ label = \"\\\"2\\\"\" ]\n    \
            3 [ label = \"\\\"2.0\\\"\" ]\n    \
            4 [ label = \"\\\"2.1\\\"\" ]\n    \
            5 [ label = \"\\\"2.2\\\"\" ]\n    \
            6 [ label = \"\\\"2.3\\\"\" ]\n    \
            7 [ label = \"\\\"2.4\\\"\" ]\n    \
            8 [ label = \"\\\"2.5\\\"\" ]\n    \
            9 [ label = \"\\\"2.1\\\"\" ]\n    \
            0 -> 1 [ label = \"\\\"RoleC?RoleB: i32\\\"\" ]\n    \
            1 -> 2 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            2 -> 3 [ label = \"\\\"& RoleB\\\"\" ]\n    \
            3 -> 4 [ label = \"\\\"RoleC?RoleB: i32\\\"\" ]\n    \
            4 -> 5 [ label = \"\\\"RoleC!RoleA: i32\\\"\" ]\n    \
            5 -> 6 [ label = \"\\\"RoleC?RoleA: i32\\\"\" ]\n    \
            6 -> 7 [ label = \"\\\"RoleC!RoleB: i32\\\"\" ]\n    \
            7 -> 8 [ label = \"\\\"0\\\"\" ]\n    \
            3 -> 9 [ label = \"\\\"0\\\"\" ]\n\
        }\n"
    );
}
