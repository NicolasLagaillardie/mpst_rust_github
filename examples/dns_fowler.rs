use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose_mpst_create_multi_to_all,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer_mpst,
};

use mpstthree::role::broadcast::RoleBroadcast;
use rand::{random, thread_rng, Rng};
use std::error::Error;

// global protocol HandleDNSRequest(role Handler, role Regional) {
//     rec QueryResolution {
//         FindNearestZone(DomainName) from Handler to Regional;
//         choice at Regional {
//             ZoneResponse(ZonePID) from Regional to Handler;
//             Handler initiates GetZoneData(Handler, new Data);
//             continue QueryResolution;
//         } or {
//             InvalidZone() from Regional to Handler;
//         }
//     }
// }
//
// global protocol GetZoneData(role Handler, role Data) {
//     ZoneDataRequest() from Handler to Data;
//     ZoneDataResponse(RRTree) from Data to Handler;
// }

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new Roles
// normal
create_multiple_normal_role_short!(Data, Handler, Regional);

// Create new send functions
// DATA
create_send_mpst_session_bundle!(
    send_mpst_data_to_handler, RoleHandler, 1 |
    send_mpst_data_to_regional, RoleRegional, 2 | =>
    RoleData, MeshedChannelsThree, 3
);
// HANDLER
create_send_mpst_session_bundle!(
    send_mpst_handler_to_data, RoleData, 1 |
    send_mpst_handler_to_regional, RoleRegional, 2 | =>
    RoleHandler, MeshedChannelsThree, 3
);
// REGIONAL
create_send_mpst_session_bundle!(
    send_mpst_regional_to_data, RoleData, 1 |
    send_mpst_regional_to_handler, RoleHandler, 2 | =>
    RoleRegional, MeshedChannelsThree, 3
);

// Create new recv functions and related types
// DATA
create_recv_mpst_session_bundle!(
    recv_mpst_data_from_handler, RoleHandler, 1 |
    recv_mpst_data_from_regional, RoleRegional, 2 | =>
    RoleData, MeshedChannelsThree, 3
);
// HANDLER
create_recv_mpst_session_bundle!(
    recv_mpst_handler_from_data, RoleData, 1 |
    recv_mpst_handler_from_regional, RoleRegional, 2 | =>
    RoleHandler, MeshedChannelsThree, 3
);
// REGIONAL
create_recv_mpst_session_bundle!(
    recv_mpst_regional_from_data, RoleData, 1 |
    recv_mpst_regional_from_handler, RoleHandler, 2 | =>
    RoleRegional, MeshedChannelsThree, 3
);

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
    Loop(
        MeshedChannelsThree<
            Recv<(), Send<i32, End>>,
            Offer0fromRegionalToData,
            RoleHandler<RoleHandler<RoleRegional<RoleEnd>>>,
            NameData,
        >,
    ),
    Invalid(MeshedChannelsThree<End, End, RoleEnd, NameData>),
}
type Offer0fromRegionalToData = Recv<Branching0fromRegionalToData, End>;
// HANDLER
enum Branching0fromRegionalToHandler {
    Loop(
        MeshedChannelsThree<
            Send<(), Recv<i32, End>>,
            Recv<i32, Send<i32, Offer0fromRegionalToHandler>>,
            RoleRegional<RoleData<RoleData<RoleRegional<RoleRegional<RoleEnd>>>>>,
            NameHandler,
        >,
    ),
    Invalid(MeshedChannelsThree<End, Recv<(), End>, RoleRegional<RoleEnd>, NameHandler>),
}
type Offer0fromRegionalToHandler = Recv<Branching0fromRegionalToHandler, End>;

// Creating the MP sessions
// DATA
type EndpointData = MeshedChannelsThree<
    End,
    Recv<Branching0fromRegionalToData, End>,
    RoleRegional<RoleEnd>,
    NameData,
>;
// HANDLER
type EndpointHandler = MeshedChannelsThree<
    End,
    Send<i32, Recv<Branching0fromRegionalToHandler, End>>,
    RoleRegional<RoleRegional<RoleEnd>>,
    NameHandler,
>;
// REGIONAL
type EndpointRegional = MeshedChannelsThree<
    Choose0fromRegionalToData,
    Recv<i32, Choose0fromRegionalToHandler>,
    RoleHandler<RoleBroadcast>,
    NameRegional,
>;

choose_mpst_create_multi_to_all!(
    choose_mpst_regional_to_all,
    RoleData,
    RoleHandler, =>
    RoleRegional,
    MeshedChannelsThree,
    3
);

// Functions
fn endpoint_data(s: EndpointData) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_data_from_regional, {
        Branching0fromRegionalToData::Loop(s) => {

            let (_, s) = recv_mpst_data_from_handler(s)?;

            let r_r_tree = random::<i32>();

            let s = send_mpst_data_to_handler(r_r_tree, s);

            endpoint_data(s)
        },
        Branching0fromRegionalToData::Invalid(s) => {

            close_mpst_multi(s)
        },
    })
}

fn endpoint_handler(s: EndpointHandler) -> Result<(), Box<dyn Error>> {
    let domain_name = random::<i32>();

    let s = send_mpst_handler_to_regional(domain_name, s);

    offer_mpst!(s, recv_mpst_handler_from_regional, {
        Branching0fromRegionalToHandler::Loop(s) => {

            let (_, s) = recv_mpst_handler_from_regional(s)?;

            let s = send_mpst_handler_to_data((), s);

            let (_r_r_tree, s) = recv_mpst_handler_from_data(s)?;

            endpoint_handler(s)
        },
        Branching0fromRegionalToHandler::Invalid(s) => {
            let ((), s) = recv_mpst_handler_from_regional(s)?;

            close_mpst_multi(s)
        },
    })
}

fn endpoint_regional(s: EndpointRegional) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..3);

    let (_domain_name, s) = recv_mpst_regional_from_handler(s)?;

    if choice == 1 {
        let s = choose_mpst_regional_to_all!(
            s,
            Branching0fromRegionalToData::Loop,
            Branching0fromRegionalToHandler::Loop
        );

        let zone_pid = random::<i32>();

        let s = send_mpst_regional_to_handler(zone_pid, s);

        endpoint_regional(s)
    } else {
        let s = choose_mpst_regional_to_all!(
            s,
            Branching0fromRegionalToData::Invalid,
            Branching0fromRegionalToHandler::Invalid
        );

        let s = send_mpst_regional_to_handler((), s);

        close_mpst_multi(s)
    }
}

fn main() {
    let (thread_handler, thread_regional, thread_data) =
        fork_mpst(endpoint_data, endpoint_handler, endpoint_regional);

    thread_data.join().unwrap();
    thread_regional.join().unwrap();
    thread_handler.join().unwrap();
}
