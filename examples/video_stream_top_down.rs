use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::bundle_impl_with_enum_and_cancel;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::error::Error;
bundle_impl_with_enum_and_cancel!(MeshedChannels, AUTH, CLIENT, SERVER,);

type RequestVideoSERVERtoAUTH = Recv<i32, Send<i32, End>>;

type OrderingServer10 = RoleClient<RoleEnd>;
type OrderingServer11Full = RoleAuth<RoleAuth<OrderingServer10>>;
type EndpointServer12 = MeshedChannels<
    RequestVideoSERVERtoAUTH,
    Recv<Branches0ServertoClient, End>,
    OrderingServer11Full,
    RoleServer<RoleEnd>,
>;

type CloseSERVERtoAUTH = Recv<(), End>;

type OrderingServer13Full = RoleAuth<RoleEnd>;
type EndpointServer14 =
    MeshedChannels<CloseSERVERtoAUTH, End, OrderingServer13Full, RoleServer<RoleEnd>>;

enum Branches0ServertoClient {
    RequestVideo(EndpointServer12),
    Close(EndpointServer14),
}
type Choose0forServertoClient = Send<Branches0ServertoClient, End>;

type OrderingServer35 = RoleClient<RoleEnd>;
type OrderingServer36Full = OrderingServer35;
type EndpointServer37 = MeshedChannels<
    End,
    Recv<Branches0ServertoClient, End>,
    OrderingServer36Full,
    RoleServer<RoleEnd>,
>;

type RequestVideoAUTHtoCLIENT = Recv<i32, Send<i32, Recv<Branches0AuthtoClient, End>>>;
type RequestVideoAUTHtoSERVER = Send<i32, Recv<i32, End>>;

type OrderingAuth10 = RoleClient<RoleEnd>;
type OrderingAuth11Full = RoleClient<RoleServer<RoleServer<RoleClient<OrderingAuth10>>>>;
type EndpointAuth12 = MeshedChannels<
    RequestVideoAUTHtoCLIENT,
    RequestVideoAUTHtoSERVER,
    OrderingAuth11Full,
    RoleAuth<RoleEnd>,
>;

type CloseAUTHtoCLIENT = Recv<(), End>;
type CloseAUTHtoSERVER = Send<(), End>;

type OrderingAuth13Full = RoleClient<RoleServer<RoleEnd>>;
type EndpointAuth14 =
    MeshedChannels<CloseAUTHtoCLIENT, CloseAUTHtoSERVER, OrderingAuth13Full, RoleAuth<RoleEnd>>;

enum Branches0AuthtoClient {
    RequestVideo(EndpointAuth12),
    Close(EndpointAuth14),
}
type Choose0forAuthtoClient = Send<Branches0AuthtoClient, End>;

type DeclareAUTHtoCLIENT = Recv<i32, Send<i32, Recv<Branches0AuthtoClient, End>>>;

type OrderingAuth35 = RoleClient<RoleEnd>;
type OrderingAuth36Full = RoleClient<RoleClient<OrderingAuth35>>;
type EndpointAuth37 =
    MeshedChannels<DeclareAUTHtoCLIENT, End, OrderingAuth36Full, RoleAuth<RoleEnd>>;

type DeclareCLIENTtoAUTH = Send<i32, Recv<i32, Choose0forAuthtoClient>>;

type CloseCLIENTtoAUTH = Send<(), End>;

type OrderingClient6Full = RoleAuth<RoleBroadcast>;
type EndpointClient7 = MeshedChannels<
    CloseCLIENTtoAUTH,
    Choose0forServertoClient,
    OrderingClient6Full,
    RoleClient<RoleEnd>,
>;

type RequestVideoCLIENTtoAUTH = Send<i32, End>;

type OrderingClient4Full = RoleAuth<RoleBroadcast>;
type EndpointClient5 = MeshedChannels<
    RequestVideoCLIENTtoAUTH,
    Choose0forServertoClient,
    OrderingClient4Full,
    RoleClient<RoleEnd>,
>;

type EndpointClient8 = MeshedChannels<
    Choose0forAuthtoClient,
    Choose0forServertoClient,
    RoleBroadcast,
    RoleClient<RoleEnd>,
>;

type OrderingClient10Full = RoleAuth<RoleAuth<RoleEnd>>;
type EndpointClient11 = MeshedChannels<
    DeclareCLIENTtoAUTH,
    Choose0forServertoClient,
    OrderingClient10Full,
    RoleClient<RoleEnd>,
>;
