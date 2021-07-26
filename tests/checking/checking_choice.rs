use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::Role;

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

/////////////////////////////////////////

pub fn main() {
    mpstthree::checker_concat!(EndpointAFull<i32>, EndpointCFull<i32>, EndpointBFull<i32>).unwrap();
}
