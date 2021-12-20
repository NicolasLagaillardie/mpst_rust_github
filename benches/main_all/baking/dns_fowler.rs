use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::bundle_impl_with_enum_and_cancel;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_impl_with_enum_and_cancel!(MeshedChannelsThree, Data, Handler, Regional);

// Payload types
struct FindNearestZone;
struct ZoneResponse;
struct ZoneDataRequest;
struct ZoneDataResponse;
struct InvalidZone;
struct Received;

// Names
type NameData = RoleData<RoleEnd>;
type NameHandler = RoleHandler<RoleEnd>;
type NameRegional = RoleRegional<RoleEnd>;

// Types
// REGIONAL
type Choose0fromRegionalToData = Send<Branching0fromRegionalToData, End>;
type Choose0fromRegionalToHandler = Send<Branching0fromRegionalToHandler, End>;
// DATA
enum Branching0fromRegionalToData {
    Loops(
        MeshedChannelsThree<
            Recv<ZoneDataRequest, Send<ZoneDataResponse, End>>,
            Offer0fromRegionalToData,
            RoleHandler<RoleHandler<RoleRegional<RoleEnd>>>,
            NameData,
        >,
    ),
    Invalid(
        MeshedChannelsThree<
            Recv<InvalidZone, Send<Received, End>>,
            End,
            RoleHandler<RoleHandler<RoleEnd>>,
            NameData,
        >,
    ),
}
type Offer0fromRegionalToData = Recv<Branching0fromRegionalToData, End>;
// HANDLER
enum Branching0fromRegionalToHandler {
    Loops(
        MeshedChannelsThree<
            Send<ZoneDataRequest, Recv<ZoneDataResponse, End>>,
            Recv<ZoneResponse, Send<FindNearestZone, Offer0fromRegionalToHandler>>,
            RoleRegional<RoleData<RoleData<RoleRegional<RoleRegional<RoleEnd>>>>>,
            NameHandler,
        >,
    ),
    Invalid(
        MeshedChannelsThree<
            Send<InvalidZone, Recv<Received, End>>,
            Recv<InvalidZone, End>,
            RoleRegional<RoleData<RoleData<RoleEnd>>>,
            NameHandler,
        >,
    ),
}
type Offer0fromRegionalToHandler = Recv<Branching0fromRegionalToHandler, End>;

// Creating the MP sessions
// DATA
type EndpointData =
    MeshedChannelsThree<End, Offer0fromRegionalToData, RoleRegional<RoleEnd>, NameData>;
// HANDLER
type EndpointHandler = MeshedChannelsThree<
    End,
    Send<FindNearestZone, Offer0fromRegionalToHandler>,
    RoleRegional<RoleRegional<RoleEnd>>,
    NameHandler,
>;
// REGIONAL
type EndpointRegionalInvalid =
    MeshedChannelsThree<End, Send<InvalidZone, End>, RoleHandler<RoleEnd>, NameRegional>;
type EndpointRegionalLoops = MeshedChannelsThree<
    Choose0fromRegionalToData,
    Send<ZoneResponse, Recv<FindNearestZone, Choose0fromRegionalToHandler>>,
    RoleHandler<RoleHandler<RoleBroadcast>>,
    NameRegional,
>;
type EndpointRegional = MeshedChannelsThree<
    Choose0fromRegionalToData,
    Recv<FindNearestZone, Choose0fromRegionalToHandler>,
    RoleHandler<RoleBroadcast>,
    NameRegional,
>;

// Functions
fn endpoint_data(s: EndpointData) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromRegionalToData::Loops(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(ZoneDataResponse {})?;
            endpoint_data(s)
        },
        Branching0fromRegionalToData::Invalid(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Received {})?;
            s.close()
        },
    })
}

fn endpoint_handler(s: EndpointHandler) -> Result<(), Box<dyn Error>> {
    let s = s.send(FindNearestZone {})?;

    offer_mpst!(s, {
        Branching0fromRegionalToHandler::Loops(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(ZoneDataRequest {} )?;
            let (_, s) = s.recv()?;
            endpoint_handler(s)
        },
        Branching0fromRegionalToHandler::Invalid(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(InvalidZone {} )?;
            let (_, s) = s.recv()?;
            s.close()
        },
    })
}

fn endpoint_regional(s: EndpointRegional) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..3);

    let (_, s) = s.recv()?;

    if choice == 1 {
        let s: EndpointRegionalLoops = choose_mpst_regional_to_all!(
            s,
            Branching0fromRegionalToData::Loops,
            Branching0fromRegionalToHandler::Loops
        );
        let s = s.send(ZoneResponse {})?;
        endpoint_regional(s)
    } else {
        let s: EndpointRegionalInvalid = choose_mpst_regional_to_all!(
            s,
            Branching0fromRegionalToData::Invalid,
            Branching0fromRegionalToHandler::Invalid
        );
        let s = s.send(InvalidZone {})?;
        s.close()
    }
}

fn all_mpst() {
    let (thread_handler, thread_regional, thread_data) = fork_mpst(
        black_box(endpoint_data),
        black_box(endpoint_handler),
        black_box(endpoint_regional),
    );

    thread_data.join().unwrap();
    thread_regional.join().unwrap();
    thread_handler.join().unwrap();
}

/////////////////////////

fn dns_fowler_main(c: &mut Criterion) {
    c.bench_function("DNS Fowler baking", |b| b.iter(all_mpst));
}

criterion_group! {
    name = dns_fowler;
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = dns_fowler_main
}

criterion_main!(dns_fowler);
