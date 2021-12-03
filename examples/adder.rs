// use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
// use mpstthree::bundle_impl_with_enum_and_cancel;
// use mpstthree::role::broadcast::RoleBroadcast;
// use mpstthree::role::end::RoleEnd;
// use std::marker;

// bundle_impl_with_enum_and_cancel!(MeshedChannels, B, C, A,);
// type ADDAtoB<N> = Recv<N, End>;

// type OrderingA6Full = RoleB<RoleEnd>;
// type EndpointA7<N> = MeshedChannels<ADDAtoB<N>, End, OrderingA6Full, RoleA<RoleEnd>>;
// type BYEAtoB = Recv<(), End>;

// type OrderingA8Full = RoleB<RoleEnd>;
// type EndpointA9 = MeshedChannels<BYEAtoB, End, OrderingA8Full, RoleA<RoleEnd>>;

// enum Branches0AtoC<N: marker::Send> {
//     ADD(EndpointA7<N>),
//     BYE(EndpointA9),
// }
// type Choose0forAtoC<N> = Send<Branches0AtoC<N>, End>;

// type TestAtoC<N> = Recv<N, Recv<Branches0AtoC<N>, End>>;

// type OrderingA10 = RoleC<RoleEnd>;
// type OrderingA11Full = RoleC<OrderingA10>;
// type EndpointA12<N> = MeshedChannels<End, TestAtoC<N>, OrderingA11Full, RoleA<RoleEnd>>;

// type ADDBtoA<N> = Send<N, End>;
// type ADDBtoC<N> = Recv<N, End>;

// type OrderingB8Full = RoleC<RoleA<RoleEnd>>;
// type EndpointB9<N> = MeshedChannels<ADDBtoA<N>, ADDBtoC<N>, OrderingB8Full, RoleB<RoleEnd>>;
// type BYEBtoA = Send<(), End>;
// type BYEBtoC = Recv<(), End>;

// type OrderingB10Full = RoleC<RoleA<RoleEnd>>;
// type EndpointB11 = MeshedChannels<BYEBtoA, BYEBtoC, OrderingB10Full, RoleB<RoleEnd>>;

// enum Branches0BtoC<N: marker::Send> {
//     ADD(EndpointB9<N>),
//     BYE(EndpointB11),
// }
// type Choose0forBtoC<N> = Send<Branches0BtoC<N>, End>;

// type OrderingB12 = RoleC<RoleEnd>;
// type OrderingB13Full = OrderingB12;
// type EndpointB14<N> =
//     MeshedChannels<End, Recv<Branches0BtoC<N>, End>, OrderingB13Full, RoleB<RoleEnd>>;

// type TestCtoA<N> = Send<N, Choose0forAtoC<N>>;

// type OrderingC2Full = RoleA<RoleBroadcast>;
// type EndpointC3<N> = MeshedChannels<TestCtoA<N>, Choose0forBtoC<N>, OrderingC2Full, RoleC<RoleEnd>>;

// fn endpoint_a(s: EndpointA7<i32>) -> Result<(), Box<dyn Error>> {
//     let (_, s) = s.recv()?;
//     recurs_a(s)
// }

// fn recurs_a(s: EndpointA9<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst!(s,
//         Branches0AtoC::ADD(s) => {
//             let (_,s) = s.recv()?;
//             recurs_a(s)
//         },
//         Branches0AtoC::BYE(s) => {
//             let (_,s) = s.recv()?;
//             s.close()
//         },
//     )
// }

// fn endpoint_b(s: EndpointB9<i32>) -> Result<(), Box<dyn Error>> {
//     recurs_b(s)
// }

// fn recurs_b(s: EndpointB11<i32>) -> Result<(), Box<dyn Error>> {
//     offer_mpst!(s,
//         Branches0BtoC::ADD(s) => {
//             let (_,s) = s.recv()?;
//             let s = s.send(0)?;
//             recurs_b(s)
//         },
//         Branches0BtoC::BYE(s) => {
//             let (_,s) = s.recv()?;
//             let s = s.send(0)?;
//             s.close()
//         },
//     )
// }

// fn endpoint_c(s: EndpointC3) -> Result<(), Box<dyn Error>>  {
//     let s = s.send(0)?;
//     recurs_c(s)
// }

// fn recurs_c(s: End)

fn main() {}
