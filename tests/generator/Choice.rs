use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::end::RoleEnd;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_atmp!(MeshedChannels, A, B, C);

struct payload;
struct Test1;
struct Test2 { payload: payload }
struct Test4 { payload: payload }
struct Test3;

// Binary sessions for A
type Message0FromAToC = End;
type Message0FromAToB = SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message1FromAToB>;
type Message1FromAToB = SendTimed<Test2, 'a', 0, true, 1, true, ' ', Message2FromAToB>;
type Message2FromAToB = SendTimed<Test3, 'a', 0, true, 1, true, 'a', Message3FromAToB>;
type Message3FromAToB = SendTimed<Test4, 'a', 0, true, 1, true, 'a', Message4FromAToB>;
type Message4FromAToB = SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message5FromAToB>;
type Message5FromAToB = SendTimed<Test2, 'a', 0, true, 1, true, ' ', Message6FromAToB>;
type Message6FromAToB = SendTimed<Test3, 'a', 0, true, 1, true, 'a', Message7FromAToB>;
type Message7FromAToB = SendTimed<Test4, 'a', 0, true, 1, true, 'a', Message8FromAToB>;
type Message8FromAToB = End;

// Binary sessions for B
type Message0FromBToC = End;
type Message0FromBToA = RecvTimed<Test1, 'a', 0, true, 1, true, ' ', Message1FromBToA>;
type Message1FromBToA = RecvTimed<Test2, 'a', 0, true, 1, true, ' ', Message2FromBToA>;
type Message2FromBToA = RecvTimed<Test3, 'a', 0, true, 1, true, 'a', Message3FromBToA>;
type Message3FromBToA = RecvTimed<Test4, 'a', 0, true, 1, true, 'a', Message4FromBToA>;
type Message4FromBToA = RecvTimed<Test1, 'a', 0, true, 1, true, ' ', Message5FromBToA>;
type Message5FromBToA = RecvTimed<Test2, 'a', 0, true, 1, true, ' ', Message6FromBToA>;
type Message6FromBToA = RecvTimed<Test3, 'a', 0, true, 1, true, 'a', Message7FromBToA>;
type Message7FromBToA = RecvTimed<Test4, 'a', 0, true, 1, true, 'a', Message8FromBToA>;
type Message8FromBToA = End;

// Binary sessions for C
type Message0FromCToA = End;
type Message0FromCToB = End;

// Stacks for B
type Ordering0ForB = RoleA<Ordering1ForB>;
type Ordering1ForB = RoleA<Ordering2ForB>;
type Ordering2ForB = RoleA<Ordering3ForB>;
type Ordering3ForB = RoleA<Ordering4ForB>;
type Ordering4ForB = RoleA<Ordering5ForB>;
type Ordering5ForB = RoleA<Ordering6ForB>;
type Ordering6ForB = RoleA<Ordering7ForB>;
type Ordering7ForB = RoleA<Ordering8ForB>;
type Ordering8ForB = RoleEnd;

// Stacks for C
type Ordering0ForC = RoleEnd;

// Stacks for A
type Ordering0ForA = RoleB<Ordering1ForA>;
type Ordering1ForA = RoleB<Ordering2ForA>;
type Ordering2ForA = RoleB<Ordering3ForA>;
type Ordering3ForA = RoleB<Ordering4ForA>;
type Ordering4ForA = RoleB<Ordering5ForA>;
type Ordering5ForA = RoleB<Ordering6ForA>;
type Ordering6ForA = RoleB<Ordering7ForA>;
type Ordering7ForA = RoleB<Ordering8ForA>;
type Ordering8ForA = RoleEnd;

// Endpoint(s) for role A
type Endpoint0ByA = MeshedChannels<Message0FromAToB, Message0FromAToC, Ordering0ByA>;

// Endpoint(s) for role B
type Endpoint0ByB = MeshedChannels<Message0FromBToA, Message0FromBToC, Ordering0ByB>;

// Endpoint(s) for role C
type Endpoint0ByC = MeshedChannels<Message0FromCToA, Message0FromCToB, Ordering0ByC>;

// Write your functions here.

fn main(){}
