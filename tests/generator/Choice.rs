use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_atmp!(MeshedChannels, A, B, C);

struct payload;
struct Test4 { payload: payload }
struct Test1;
struct Test3;
struct Test2 { payload: payload }

type Message0FromAToB = SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message1FromAToB>;
type Message1FromAToB = SendTimed<Test2, 'a', 0, true, 1, true, ' ', Message2FromAToB>;
type Message2FromAToB = SendTimed<Test3, 'a', 0, true, 1, true, 'a', Message3FromAToB>;
type Message3FromAToB = SendTimed<Test4, 'a', 0, true, 1, true, 'a', Message4FromAToB>;
type Message4FromAToB = SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message5FromAToB>;
type Message5FromAToB = SendTimed<Test2, 'a', 0, true, 1, true, ' ', Message6FromAToB>;
type Message6FromAToB = SendTimed<Test3, 'a', 0, true, 1, true, 'a', Message7FromAToB>;
type Message7FromAToB = SendTimed<Test4, 'a', 0, true, 1, true, 'a', Message8FromAToB>;
type Message8FromAToB = End;
type Message0FromC = End;
type Message0FromB = End;

// Write your functions here.

fn main(){}