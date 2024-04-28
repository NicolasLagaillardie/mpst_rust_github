#![allow(dead_code, non_camel_case_types, unused_variables)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::end::RoleEnd;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_atmp!(MeshedChannels, A, B, C);

// Types of the payloads
struct Payload;
struct Stri;
struct Integer;
struct Test1;
struct Test4 { payload: Payload }
struct Test3 { payload: Stri }
struct Test2 { payload: Integer }
struct Test5 { payload: Payload }

// Binary sessions in depth 0
// Binary sessions for B
type Message_0_v_0_FromBToA = RecvTimed<Test1, 'a', 0, true, 2, true, ' ', Message_0_v_1_FromBToA>;
type Message_0_v_1_FromBToA = RecvTimed<Test1, 'a', 0, true, 2, true, ' ', Message_0_v_2_FromBToA>;
type Message_0_v_2_FromBToA = RecvTimed<Test1, 'a', 0, true, 2, true, ' ', Message_0_v_3_FromBToA>;
type Message_0_v_3_FromBToA = RecvTimed<Test5, 'a', 0, true, 2, true, ' ', Message_0_v_4_FromBToA>;
type Message_0_v_4_FromBToA = RecvTimed<Test2, 'a', 0, true, 2, true, ' ', Message_0_v_5_FromBToA>;
type Message_0_v_5_FromBToA = RecvTimed<Test3, 'a', 0, true, 2, true, ' ', Message_0_v_6_FromBToA>;
type Message_0_v_6_FromBToA = RecvTimed<Test4, 'a', 0, true, 2, true, ' ', Message_0_v_7_FromBToA>;
type Message_0_v_7_FromBToA = End;
type Message_0_v_0_FromBToC = SendTimed<Test1, 'a', 0, true, 2, true, ' ', Message_0_v_1_FromBToC>;
type Message_0_v_1_FromBToC = End;

// Binary sessions for C
type Message_0_v_0_FromCToA = End;
type Message_0_v_0_FromCToB = RecvTimed<Test1, 'a', 0, true, 2, true, ' ', Message_0_v_1_FromCToB>;
type Message_0_v_1_FromCToB = End;

// Binary sessions for A
type Message_0_v_0_FromAToC = End;
type Message_0_v_0_FromAToB = SendTimed<Test1, 'a', 0, true, 2, true, ' ', Message_0_v_1_FromAToB>;
type Message_0_v_1_FromAToB = SendTimed<Test1, 'a', 0, true, 2, true, ' ', Message_0_v_2_FromAToB>;
type Message_0_v_2_FromAToB = SendTimed<Test1, 'a', 0, true, 2, true, ' ', Message_0_v_3_FromAToB>;
type Message_0_v_3_FromAToB = SendTimed<Test5, 'a', 0, true, 2, true, ' ', Message_0_v_4_FromAToB>;
type Message_0_v_4_FromAToB = SendTimed<Test2, 'a', 0, true, 2, true, ' ', Message_0_v_5_FromAToB>;
type Message_0_v_5_FromAToB = SendTimed<Test3, 'a', 0, true, 2, true, ' ', Message_0_v_6_FromAToB>;
type Message_0_v_6_FromAToB = SendTimed<Test4, 'a', 0, true, 2, true, ' ', Message_0_v_7_FromAToB>;
type Message_0_v_7_FromAToB = End;

// Stacks in depth 0
// Stacks for C
type Ordering_0_v_0_ForC = RoleB<Ordering_0_v_1_ForC>;
type Ordering_0_v_1_ForC = RoleEnd;

// Stacks for B
type Ordering_0_v_0_ForB = RoleA<Ordering_0_v_1_ForB>;
type Ordering_0_v_1_ForB = RoleC<Ordering_0_v_2_ForB>;
type Ordering_0_v_2_ForB = RoleA<Ordering_0_v_3_ForB>;
type Ordering_0_v_3_ForB = RoleA<Ordering_0_v_4_ForB>;
type Ordering_0_v_4_ForB = RoleA<Ordering_0_v_5_ForB>;
type Ordering_0_v_5_ForB = RoleA<Ordering_0_v_6_ForB>;
type Ordering_0_v_6_ForB = RoleA<Ordering_0_v_7_ForB>;
type Ordering_0_v_7_ForB = RoleA<Ordering_0_v_8_ForB>;
type Ordering_0_v_8_ForB = RoleEnd;

// Stacks for A
type Ordering_0_v_0_ForA = RoleB<Ordering_0_v_1_ForA>;
type Ordering_0_v_1_ForA = RoleB<Ordering_0_v_2_ForA>;
type Ordering_0_v_2_ForA = RoleB<Ordering_0_v_3_ForA>;
type Ordering_0_v_3_ForA = RoleB<Ordering_0_v_4_ForA>;
type Ordering_0_v_4_ForA = RoleB<Ordering_0_v_5_ForA>;
type Ordering_0_v_5_ForA = RoleB<Ordering_0_v_6_ForA>;
type Ordering_0_v_6_ForA = RoleB<Ordering_0_v_7_ForA>;
type Ordering_0_v_7_ForA = RoleEnd;

// Endpoints in depth 0
// Endpoint for role A
type Endpoint_0_ForA = MeshedChannels<Message_0_v_0_FromAToB, Message_0_v_0_FromAToC, Ordering_0_v_0_ForA, NameA>;

// Endpoint for role B
type Endpoint_0_ForB = MeshedChannels<Message_0_v_0_FromBToA, Message_0_v_0_FromBToC, Ordering_0_v_0_ForB, NameB>;

// Endpoint for role C
type Endpoint_0_ForC = MeshedChannels<Message_0_v_0_FromCToA, Message_0_v_0_FromCToB, Ordering_0_v_0_ForC, NameC>;

// Fill in the functions here.
fn endpoint_a_0_v_0(s: Endpoint_0_ForA, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn endpoint_b_0_v_0(s: Endpoint_0_ForB, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn endpoint_c_0_v_0(s: Endpoint_0_ForC, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {
	all_clocks.insert('a', Instant::now());

	Ok(())
}

fn main() {
	let (thread_a, thread_b, thread_c, ) = fork_mpst(endpoint_a_0_v_0, endpoint_b_0_v_0, endpoint_c_0_v_0, );

	println!("Thread A: {:?}", thread_a.join());
	println!("Thread B: {:?}", thread_b.join());
	println!("Thread C: {:?}", thread_c.join());
}
