use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_atmp!(MeshedChannels, A, B, C);

struct integer;
struct string;
struct payload;
struct Test3 { payload: string }
struct Test1 { payload: payload }
struct Test4 { payload: payload }
struct Test2 { payload: integer }

type Message0FromBToC = SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message1FromBToC>;
type Message1FromBToC = End;
type Message0FromC = End;
type Message0FromAToB = SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message1FromAToB>;
type Message1FromAToC = SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message2FromAToC>;
type Message2FromAToB = SendTimed<Test1, 'a', 0, true, 1, false, ' ', Message3FromAToB>;
type Message3FromAToB = SendTimed<Test1, 'a', 0, false, 1, false, ' ', Message4FromAToB>;
type Message4FromAToB = SendTimed<Test2, 'a', 0, true, 1, true, ' ', Message5FromAToB>;
type Message5FromAToB = SendTimed<Test3, 'a', 0, true, 1, true, 'a', Message6FromAToB>;
type Message6FromAToB = SendTimed<Test4, 'a', 0, true, 1, true, 'a', Message7FromAToB>;
type Message7FromAToB = End;

// Write your functions here.

fn main(){}