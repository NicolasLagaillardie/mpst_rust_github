use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_atmp!(MeshedChannels, A, B, C);

// Types of the payloads
struct Payload;

struct Test3;
struct Test1;
struct Test2 {
    payload: Payload,
}
struct Test4 {
    payload: Payload,
}

struct Test8 {
    payload: Payload,
}
struct Test7;
struct Test5;
struct Test6 {
    payload: Payload,
}

// Binary sessions for B
type Message_0_v_0_FromBToA = RecvTimed<Choice_0_0_FromAToB, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromBToC = End;

// Binary sessions for C
type Message_0_v_0_FromCToA = RecvTimed<Choice_0_0_FromAToC, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromCToB = End;

// Binary sessions for A
type Message_0_v_0_FromAToC = SendTimed<Choice_0_0_FromAToC, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromAToB = SendTimed<Choice_0_0_FromAToB, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for A
type Message_0_0_v_0_FromAToB =
    SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromAToB>;
type Message_0_0_v_1_FromAToB =
    SendTimed<Test2, 'a', 0, true, 1, true, ' ', Message_0_0_v_2_FromAToB>;
type Message_0_0_v_2_FromAToB =
    SendTimed<Test3, 'a', 0, true, 1, true, 'a', Message_0_0_v_3_FromAToB>;
type Message_0_0_v_3_FromAToB =
    SendTimed<Test4, 'a', 0, true, 1, true, 'a', Message_0_0_v_4_FromAToB>;
type Message_0_0_v_4_FromAToB = SendTimed<Choice_0_1_FromAToB, ' ', -2, false, -1, false, ' ', End>;
type Message_0_0_v_0_FromAToC = SendTimed<Choice_0_1_FromAToC, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for C
type Message_0_0_v_0_FromCToB = End;
type Message_0_0_v_0_FromCToA = RecvTimed<Choice_0_1_FromAToC, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for B
type Message_0_0_v_0_FromBToA =
    RecvTimed<Test1, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromBToA>;
type Message_0_0_v_1_FromBToA =
    RecvTimed<Test2, 'a', 0, true, 1, true, ' ', Message_0_0_v_2_FromBToA>;
type Message_0_0_v_2_FromBToA =
    RecvTimed<Test3, 'a', 0, true, 1, true, 'a', Message_0_0_v_3_FromBToA>;
type Message_0_0_v_3_FromBToA =
    RecvTimed<Test4, 'a', 0, true, 1, true, 'a', Message_0_0_v_4_FromBToA>;
type Message_0_0_v_4_FromBToA = RecvTimed<Choice_0_1_FromAToB, ' ', -2, false, -1, false, ' ', End>;
type Message_0_0_v_0_FromBToC = End;

// Binary sessions for B
type Message_0_1_v_1_FromBToC = End;
type Message_0_1_v_0_FromBToA =
    RecvTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromBToA>;
type Message_0_1_v_1_FromBToA =
    RecvTimed<Test6, 'a', 0, true, 1, true, ' ', Message_0_1_v_2_FromBToA>;
type Message_0_1_v_2_FromBToA =
    RecvTimed<Test7, 'a', 0, true, 1, true, 'a', Message_0_1_v_3_FromBToA>;
type Message_0_1_v_3_FromBToA =
    RecvTimed<Test8, 'a', 0, true, 1, true, 'a', Message_0_1_v_4_FromBToA>;
type Message_0_1_v_4_FromBToA = RecvTimed<Choice_0_2_FromAToB, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for A
type Message_0_1_v_0_FromAToB =
    SendTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromAToB>;
type Message_0_1_v_1_FromAToB =
    SendTimed<Test6, 'a', 0, true, 1, true, ' ', Message_0_1_v_2_FromAToB>;
type Message_0_1_v_2_FromAToB =
    SendTimed<Test7, 'a', 0, true, 1, true, 'a', Message_0_1_v_3_FromAToB>;
type Message_0_1_v_3_FromAToB =
    SendTimed<Test8, 'a', 0, true, 1, true, 'a', Message_0_1_v_4_FromAToB>;
type Message_0_1_v_4_FromAToB = SendTimed<Choice_0_2_FromAToB, ' ', -2, false, -1, false, ' ', End>;
type Message_0_1_v_1_FromAToC = SendTimed<Choice_0_2_FromAToC, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for C
type Message_0_1_v_1_FromCToA = RecvTimed<Choice_0_2_FromAToC, ' ', -2, false, -1, false, ' ', End>;
type Message_0_1_v_1_FromCToB = End;

// Binary sessions for B
type Message_0_2_v_2_FromBToC = End;
type Message_0_2_v_0_FromBToA =
    RecvTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_2_v_1_FromBToA>;
type Message_0_2_v_1_FromBToA =
    RecvTimed<Test6, 'a', 0, true, 1, true, ' ', Message_0_2_v_2_FromBToA>;
type Message_0_2_v_2_FromBToA =
    RecvTimed<Test7, 'a', 0, true, 1, true, 'a', Message_0_2_v_3_FromBToA>;
type Message_0_2_v_3_FromBToA =
    RecvTimed<Test8, 'a', 0, true, 1, true, 'a', Message_0_2_v_4_FromBToA>;
type Message_0_2_v_4_FromBToA = End;

// Binary sessions for C
type Message_0_2_v_2_FromCToB = End;
type Message_0_2_v_2_FromCToA = End;

// Binary sessions for A
type Message_0_2_v_2_FromAToC = End;
type Message_0_2_v_0_FromAToB =
    SendTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_2_v_1_FromAToB>;
type Message_0_2_v_1_FromAToB =
    SendTimed<Test6, 'a', 0, true, 1, true, ' ', Message_0_2_v_2_FromAToB>;
type Message_0_2_v_2_FromAToB =
    SendTimed<Test7, 'a', 0, true, 1, true, 'a', Message_0_2_v_3_FromAToB>;
type Message_0_2_v_3_FromAToB =
    SendTimed<Test8, 'a', 0, true, 1, true, 'a', Message_0_2_v_4_FromAToB>;
type Message_0_2_v_4_FromAToB = End;

// Stacks for A
type Ordering_0_v_0_ForA = RoleBroadcast;

// Stacks for B
type Ordering_0_v_0_ForB = RoleA<RoleEnd>;

// Stacks for C
type Ordering_0_v_0_ForC = RoleA<RoleEnd>;

// Stacks for A
type Ordering_0_0_v_0_ForA = RoleB<Ordering_0_0_v_1_ForA>;
type Ordering_0_0_v_1_ForA = RoleB<Ordering_0_0_v_2_ForA>;
type Ordering_0_0_v_2_ForA = RoleB<Ordering_0_0_v_3_ForA>;
type Ordering_0_0_v_3_ForA = RoleB<Ordering_0_0_v_4_ForA>;
type Ordering_0_0_v_4_ForA = RoleBroadcast;

// Stacks for B
type Ordering_0_0_v_0_ForB = RoleA<Ordering_0_0_v_1_ForB>;
type Ordering_0_0_v_1_ForB = RoleA<Ordering_0_0_v_2_ForB>;
type Ordering_0_0_v_2_ForB = RoleA<Ordering_0_0_v_3_ForB>;
type Ordering_0_0_v_3_ForB = RoleA<Ordering_0_0_v_4_ForB>;
type Ordering_0_0_v_4_ForB = RoleA<RoleEnd>;

// Stacks for C
type Ordering_0_0_v_0_ForC = RoleA<RoleEnd>;

// Stacks for C
type Ordering_0_1_v_1_ForC = RoleA<RoleEnd>;

// Stacks for A
type Ordering_0_1_v_0_ForA = RoleB<Ordering_0_1_v_1_ForA>;
type Ordering_0_1_v_1_ForA = RoleB<Ordering_0_1_v_2_ForA>;
type Ordering_0_1_v_2_ForA = RoleB<Ordering_0_1_v_3_ForA>;
type Ordering_0_1_v_3_ForA = RoleB<Ordering_0_1_v_4_ForA>;
type Ordering_0_1_v_4_ForA = RoleBroadcast;

// Stacks for B
type Ordering_0_1_v_0_ForB = RoleA<Ordering_0_1_v_1_ForB>;
type Ordering_0_1_v_1_ForB = RoleA<Ordering_0_1_v_2_ForB>;
type Ordering_0_1_v_2_ForB = RoleA<Ordering_0_1_v_3_ForB>;
type Ordering_0_1_v_3_ForB = RoleA<Ordering_0_1_v_4_ForB>;
type Ordering_0_1_v_4_ForB = RoleA<RoleEnd>;

// Stacks for B
type Ordering_0_2_v_0_ForB = RoleA<Ordering_0_2_v_1_ForB>;
type Ordering_0_2_v_1_ForB = RoleA<Ordering_0_2_v_2_ForB>;
type Ordering_0_2_v_2_ForB = RoleA<Ordering_0_2_v_3_ForB>;
type Ordering_0_2_v_3_ForB = RoleA<Ordering_0_2_v_4_ForB>;
type Ordering_0_2_v_4_ForB = RoleEnd;

// Stacks for C
type Ordering_0_2_v_2_ForC = RoleEnd;

// Stacks for A
type Ordering_0_2_v_0_ForA = RoleB<Ordering_0_2_v_1_ForA>;
type Ordering_0_2_v_1_ForA = RoleB<Ordering_0_2_v_2_ForA>;
type Ordering_0_2_v_2_ForA = RoleB<Ordering_0_2_v_3_ForA>;
type Ordering_0_2_v_3_ForA = RoleB<Ordering_0_2_v_4_ForA>;
type Ordering_0_2_v_4_ForA = RoleEnd;

// Enums (Branching) for B
enum Choice_0_0_FromAToB {
    Branching0(Endpoint_0_0_v_0_ForB),
}

// Enums (Branching) for C
enum Choice_0_0_FromAToC {
    Branching0(Endpoint_0_0_v_0_ForC),
}

// Enums (Branching) for B
enum Choice_0_1_FromAToB {
    Branching0(Endpoint_0_1_v_1_ForB),
}

// Enums (Branching) for C
enum Choice_0_1_FromAToC {
    Branching0(Endpoint_0_1_v_1_ForC),
}

// Enums (Branching) for B
enum Choice_0_2_FromAToB {
    Branching0(Endpoint_0_2_v_2_ForB),
}

// Enums (Branching) for C
enum Choice_0_2_FromAToC {
    Branching0(Endpoint_0_2_v_2_ForC),
}

// Endpoint for role A
type Endpoint_0_v_0_ForA =
    MeshedChannels<Message_0_v_0_FromAToB, Message_0_v_0_FromAToC, Ordering_0_v_0_ForA, NameA>;

// Endpoint for role B
type Endpoint_0_v_0_ForB =
    MeshedChannels<Message_0_v_0_FromBToA, Message_0_v_0_FromBToC, Ordering_0_v_0_ForB, NameB>;

// Endpoint for role C
type Endpoint_0_v_0_ForC =
    MeshedChannels<Message_0_v_0_FromCToA, Message_0_v_0_FromCToB, Ordering_0_v_0_ForC, NameC>;

// Endpoint for role A
type Endpoint_0_0_v_0_ForA = MeshedChannels<
    Message_0_0_v_0_FromAToB,
    Message_0_0_v_0_FromAToC,
    Ordering_0_0_v_0_ForA,
    NameA,
>;

// Endpoint for role B
type Endpoint_0_0_v_0_ForB = MeshedChannels<
    Message_0_0_v_0_FromBToA,
    Message_0_0_v_0_FromBToC,
    Ordering_0_0_v_0_ForB,
    NameB,
>;

// Endpoint for role C
type Endpoint_0_0_v_0_ForC = MeshedChannels<
    Message_0_0_v_0_FromCToA,
    Message_0_0_v_0_FromCToB,
    Ordering_0_0_v_0_ForC,
    NameC,
>;

// Endpoint for role A
type Endpoint_0_1_v_1_ForA = MeshedChannels<
    Message_0_1_v_1_FromAToB,
    Message_0_1_v_1_FromAToC,
    Ordering_0_1_v_1_ForA,
    NameA,
>;

// Endpoint for role B
type Endpoint_0_1_v_1_ForB = MeshedChannels<
    Message_0_1_v_1_FromBToA,
    Message_0_1_v_1_FromBToC,
    Ordering_0_1_v_1_ForB,
    NameB,
>;

// Endpoint for role C
type Endpoint_0_1_v_1_ForC = MeshedChannels<
    Message_0_1_v_1_FromCToA,
    Message_0_1_v_1_FromCToB,
    Ordering_0_1_v_1_ForC,
    NameC,
>;

// Endpoint for role A
type Endpoint_0_2_v_2_ForA = MeshedChannels<
    Message_0_2_v_2_FromAToB,
    Message_0_2_v_2_FromAToC,
    Ordering_0_2_v_2_ForA,
    NameA,
>;

// Endpoint for role B
type Endpoint_0_2_v_2_ForB = MeshedChannels<
    Message_0_2_v_2_FromBToA,
    Message_0_2_v_2_FromBToC,
    Ordering_0_2_v_2_ForB,
    NameB,
>;

// Endpoint for role C
type Endpoint_0_2_v_2_ForC = MeshedChannels<
    Message_0_2_v_2_FromCToA,
    Message_0_2_v_2_FromCToB,
    Ordering_0_2_v_2_ForC,
    NameC,
>;

// Fill in the functions here.
fn endpoint_a_0_v_0(
    s: Endpoint_0_v_0_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_v_0(
    s: Endpoint_0_v_0_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_c_0_v_0(
    s: Endpoint_0_v_0_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn endpoint_a_0_0_v_0(
    s: Endpoint_0_0_v_0_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_0_v_0(
    s: Endpoint_0_0_v_0_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_c_0_0_v_0(
    s: Endpoint_0_0_v_0_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn endpoint_a_0_1_v_0(
    s: Endpoint_0_1_v_1_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_a_0_1_v_1(
    s: Endpoint_0_1_v_1_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_1_v_0(
    s: Endpoint_0_1_v_1_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_1_v_1(
    s: Endpoint_0_1_v_1_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_c_0_1_v_0(
    s: Endpoint_0_1_v_1_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn endpoint_c_0_1_v_1(
    s: Endpoint_0_1_v_1_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn endpoint_a_0_2_v_0(
    s: Endpoint_0_2_v_2_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_a_0_2_v_1(
    s: Endpoint_0_2_v_2_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_a_0_2_v_2(
    s: Endpoint_0_2_v_2_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_2_v_0(
    s: Endpoint_0_2_v_2_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_2_v_1(
    s: Endpoint_0_2_v_2_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_2_v_2(
    s: Endpoint_0_2_v_2_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_c_0_2_v_0(
    s: Endpoint_0_2_v_2_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn endpoint_c_0_2_v_1(
    s: Endpoint_0_2_v_2_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn endpoint_c_0_2_v_2(
    s: Endpoint_0_2_v_2_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn main() {
    let (thread_a, thread_b, thread_c) =
        fork_mpst(endpoint_a_0_v_0, endpoint_b_0_v_0, endpoint_c_0_v_0);

    println!("Thread A: {:?}", thread_a.join());
    println!("Thread B: {:?}", thread_b.join());
    println!("Thread C: {:?}", thread_c.join());
}
