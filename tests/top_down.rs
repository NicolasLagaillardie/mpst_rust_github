// extern crate mpstthree;

// use std::boxed::Box;
// use std::error::Error;

// use mpstthree::binary::{End, Recv, Send, Session};
// use mpstthree::fork_mpst;
// use mpstthree::sessionmpst::SessionMpst;

// use mpstthree::functionmpst::close::close_mpst;
// use mpstthree::role::a::RoleA;
// use mpstthree::role::b::RoleB;
// use mpstthree::role::c::RoleC;
// use mpstthree::role::end::RoleEnd;

// type ADDAtoB<N> = Recv<N, End>;

// type OrderingA0 = RoleB<RoleEnd>;
// type EndpointA1 = SessionMpst<ADDAtoB<N>, End, OrderingA0, RoleA<RoleEnd>>;
// type BYEAtoB<N> = Recv<N, End>;

// type OrderingA2 = RoleB<RoleEnd>;
// type EndpointA3 = SessionMpst<BYEAtoB<N>, End, OrderingA2, RoleA<RoleEnd>>;

// enum CBranchesAtoC<N: marker::Send> {
//     ADD(SessionMpst<ADDAtoB<N>, End, OrderingA0, RoleA<RoleEnd>>),
//     BYE(SessionMpst<BYEAtoB<N>, End, OrderingA2, RoleA<RoleEnd>>),
// }
// type CBranchesAtoC<N> = Send<CBranchesAtoC<N>, End>;

// type TestAtoC<N> = Recv<N, End>;

// type OrderingA4 = RoleC<RoleEnd>;
// type OrderingA5Full = RoleB<RoleEnd>;
// type EndpointA6 = SessionMpst<CBranchesAtoC<N>, TestAtoC<N>, OrderingA5Full, RoleA<RoleEnd>>;

// type ADDBtoA<N> = Send<N, End>;
// type ADDBtoC<N> = Recv<N, End>;

// type OrderingB0 = RoleC<RoleA<RoleEnd>>;
// type EndpointB1 = SessionMpst<ADDBtoA<N>, ADDBtoC<N>, OrderingB0, RoleB<RoleEnd>>;
// type BYEBtoA<N> = Send<N, End>;
// type BYEBtoC<N> = Recv<N, End>;

// type OrderingB2 = RoleC<RoleA<RoleEnd>>;
// type EndpointB3 = SessionMpst<BYEBtoA<N>, BYEBtoC<N>, OrderingB2, RoleB<RoleEnd>>;

// enum CBranchesBtoC<N: marker::Send> {
//     ADD(SessionMpst<ADDBtoA<N>, ADDBtoC<N>, OrderingB0, RoleB<RoleEnd>>),
//     BYE(SessionMpst<BYEBtoA<N>, BYEBtoC<N>, OrderingB2, RoleB<RoleEnd>>),
// }
// type CBranchesBtoC<N> = Send<CBranchesBtoC<N>, End>;

// type OrderingB4 = RoleEnd;
// type OrderingB5Full = RoleC<RoleEnd>;
// type EndpointB6 = SessionMpst<End, CBranchesBtoC<N>, OrderingB5Full, RoleB<RoleEnd>>;

// type TestCtoA<N> = Send<N, ChooseCforAtoC<N>>;

// type OrderingC0 = RoleCtoA<RoleEnd>;
// type EndpointC1 = SessionMpst<TestCtoA<N>, ChooseCforBtoC<N>, OrderingC0, RoleC<RoleEnd>>;
