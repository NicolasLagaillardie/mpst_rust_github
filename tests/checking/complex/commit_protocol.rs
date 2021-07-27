// Test for Macro, exact same as usecase-recursive
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use mpstthree::bundle_impl;

// Create new roles
bundle_impl!(MeshedChannels => A, B, C => fork_mpst);

// Types
type AtoBVideo = Send<(), Recv<(), End>>;
type AtoCVideo = Recv<(), Send<(), RecursAtoC>>;

type InitA = Recv<(), Send<(), RecursAtoC>>;

type BtoAVideo = <AtoBVideo as Session>::Dual;

type RecursAtoC = Recv<Branches0AtoC, End>;
type RecursBtoC = Recv<Branches0BtoC, End>;

enum Branches0AtoC {
    End(MeshedChannels<End, End, RoleEnd, RoleA<RoleEnd>>),
    Video(MeshedChannels<AtoBVideo, AtoCVideo, StackAVideo, RoleA<RoleEnd>>),
}
enum Branches0BtoC {
    End(MeshedChannels<End, End, RoleEnd, RoleB<RoleEnd>>),
    Video(MeshedChannels<BtoAVideo, RecursBtoC, StackBVideo, RoleB<RoleEnd>>),
}
type Choose0fromCtoA = Send<Branches0AtoC, End>;
type Choose0fromCtoB = Send<Branches0BtoC, End>;

type InitC = Send<(), Recv<(), Choose0fromCtoA>>;

/// Stacks
type StackAVideo = RoleC<RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>>;
type StackAInit = RoleC<RoleC<RoleC<RoleEnd>>>;

type StackBVideo = RoleA<RoleA<RoleC<RoleEnd>>>;

type StackCRecurs = RoleBroadcast;
type StackCFull = RoleA<RoleA<StackCRecurs>>;

/// Creating the MP sessions
/// For C
type EndpointCVideo = MeshedChannels<
    <AtoCVideo as Session>::Dual,
    <RecursBtoC as Session>::Dual,
    RoleA<RoleA<RoleBroadcast>>,
    RoleC<RoleEnd>,
>;
type EndpointCRecurs =
    MeshedChannels<Choose0fromCtoA, Choose0fromCtoB, StackCRecurs, RoleC<RoleEnd>>;
type EndpointCFull = MeshedChannels<InitC, Choose0fromCtoB, StackCFull, RoleC<RoleEnd>>;

/// For A
type EndpointARecurs = MeshedChannels<End, RecursAtoC, RoleC<RoleEnd>, RoleA<RoleEnd>>;
type EndpointAFull = MeshedChannels<End, InitA, StackAInit, RoleA<RoleEnd>>;

/// For B
type EndpointBFull = MeshedChannels<End, RecursBtoC, RoleC<RoleEnd>, RoleB<RoleEnd>>;
