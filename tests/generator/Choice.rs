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
struct Test5;
struct Test8 { payload: payload }
struct Test6 { payload: payload }
struct Test7;
struct Test3;
struct Test1;
struct Test4 { payload: payload }
struct Test2 { payload: payload }

struct payload;

// Binary sessions for B
type Message_0_v_0_FromBToC = End;
type Message_0_v_0_FromBToA = RecvTimed<Choice_0_FromAToB, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_1_FromBToA = RecvTimed<Test1, 'a', 0, true, 1, true, ' ', Message_0_v_2_FromBToA>;
type Message_0_v_2_FromBToA = RecvTimed<Test2, 'a', 0, true, 1, true, ' ', Message_0_v_3_FromBToA>;
type Message_0_v_3_FromBToA = RecvTimed<Test3, 'a', 0, true, 1, true, 'a', Message_0_v_4_FromBToA>;
type Message_0_v_4_FromBToA = RecvTimed<Test4, 'a', 0, true, 1, true, 'a', Message_0_v_5_FromBToA>;
type Message_1_v_5_FromBToA = RecvTimed<Test5, 'a', 0, true, 1, true, ' ', Message_1_v_6_FromBToA>;
type Message_1_v_6_FromBToA = RecvTimed<Test6, 'a', 0, true, 1, true, ' ', Message_1_v_7_FromBToA>;
type Message_1_v_7_FromBToA = RecvTimed<Test7, 'a', 0, true, 1, true, 'a', Message_1_v_8_FromBToA>;
type Message_1_v_8_FromBToA = RecvTimed<Test8, 'a', 0, true, 1, true, 'a', Message_1_v_9_FromBToA>;
type Message_1_v_9_FromBToA = End;

// Binary sessions for C
type Message_0_v_0_FromCToA = RecvTimed<Choice_0_FromAToC, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromCToA = End;
type Message_0_v_0_FromCToB = End;

// Binary sessions for A
type Message_0_v_0_FromAToC = SendTimed<Choice_0_FromAToC, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromAToC = End;
type Message_0_v_0_FromAToB = SendTimed<Choice_0_FromAToB, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_1_FromAToB = SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message_0_v_2_FromAToB>;
type Message_0_v_2_FromAToB = SendTimed<Test2, 'a', 0, true, 1, true, ' ', Message_0_v_3_FromAToB>;
type Message_0_v_3_FromAToB = SendTimed<Test3, 'a', 0, true, 1, true, 'a', Message_0_v_4_FromAToB>;
type Message_0_v_4_FromAToB = SendTimed<Test4, 'a', 0, true, 1, true, 'a', Message_0_v_5_FromAToB>;
type Message_1_v_5_FromAToB = SendTimed<Test5, 'a', 0, true, 1, true, ' ', Message_1_v_6_FromAToB>;
type Message_1_v_6_FromAToB = SendTimed<Test6, 'a', 0, true, 1, true, ' ', Message_1_v_7_FromAToB>;
type Message_1_v_7_FromAToB = SendTimed<Test7, 'a', 0, true, 1, true, 'a', Message_1_v_8_FromAToB>;
type Message_1_v_8_FromAToB = SendTimed<Test8, 'a', 0, true, 1, true, 'a', Message_1_v_9_FromAToB>;
type Message_1_v_9_FromAToB = End;

// Binary sessions for A
type Message_0_v_0_FromAToC = End;
type Message_0_v_0_FromAToB = End;

// Binary sessions for C
type Message_0_v_0_FromCToB = End;
type Message_0_v_0_FromCToA = End;

// Binary sessions for B
type Message_0_v_0_FromBToA = End;
type Message_0_v_0_FromBToC = End;

// Stacks for A
type Ordering_0_v_0_ForA = RoleBroadcast;
type Ordering_0_v_1_ForA = RoleB<Ordering_0_v_2_ForA>;
type Ordering_0_v_2_ForA = RoleB<Ordering_0_v_3_ForA>;
type Ordering_0_v_3_ForA = RoleB<Ordering_0_v_4_ForA>;
type Ordering_0_v_4_ForA = RoleB<Ordering_0_v_5_ForA>;
type Ordering_1_v_5_ForA = RoleB<Ordering_1_v_6_ForA>;
type Ordering_1_v_6_ForA = RoleB<Ordering_1_v_7_ForA>;
type Ordering_1_v_7_ForA = RoleB<Ordering_1_v_8_ForA>;
type Ordering_1_v_8_ForA = RoleB<Ordering_1_v_9_ForA>;
type Ordering_1_v_9_ForA = RoleEnd;

// Stacks for B
type Ordering_0_v_0_ForB = RoleA<RoleEnd>;
type Ordering_0_v_1_ForB = RoleA<Ordering_0_v_2_ForB>;
type Ordering_0_v_2_ForB = RoleA<Ordering_0_v_3_ForB>;
type Ordering_0_v_3_ForB = RoleA<Ordering_0_v_4_ForB>;
type Ordering_0_v_4_ForB = RoleA<Ordering_0_v_5_ForB>;
type Ordering_1_v_5_ForB = RoleA<Ordering_1_v_6_ForB>;
type Ordering_1_v_6_ForB = RoleA<Ordering_1_v_7_ForB>;
type Ordering_1_v_7_ForB = RoleA<Ordering_1_v_8_ForB>;
type Ordering_1_v_8_ForB = RoleA<Ordering_1_v_9_ForB>;
type Ordering_1_v_9_ForB = RoleEnd;

// Stacks for C
type Ordering_0_v_0_ForC = RoleA<RoleEnd>;
type Ordering_0_v_1_ForC = RoleEnd;

// Stacks for A
type Ordering_0_v_0_ForA = RoleEnd;

// Stacks for B
type Ordering_0_v_0_ForB = RoleEnd;

// Stacks for C
type Ordering_0_v_0_ForC = RoleEnd;


// Enums (Branching) for B
enum Choice_0_FromAToB {
	Branching_0(Endpoint_0_v_0_ForB),
