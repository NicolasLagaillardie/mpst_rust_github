use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_atmp!(MeshedChannels, A, B, C);

// Types of the payloads
struct Payload;

struct Test7;
struct Test5 {
    payload: Payload,
}
struct Test6 {
    payload: Payload,
}
struct Test8 {
    payload: Payload,
}

struct Test4 {
    payload: Payload,
}
struct Test3;
struct Test2 {
    payload: Payload,
}
struct Test1;

// Binary sessions in depth 0
// Binary sessions for A
type Message_0_v_0_FromAToC = SendTimed<Choice_0_FromAToC, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromAToB = SendTimed<Choice_0_FromAToB, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for B
type Message_0_v_0_FromBToA = RecvTimed<Choice_0_FromAToB, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromBToC = End;

// Binary sessions for C
type Message_0_v_0_FromCToA = RecvTimed<Choice_0_FromAToC, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromCToB = End;

// Binary sessions in depth 0.2
// Binary sessions for A
type Message_0_2_v_0_FromAToB =
    SendTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_2_v_1_FromAToB>;
type Message_0_2_v_1_FromAToB =
    SendTimed<Test6, 'a', 0, true, 1, true, ' ', Message_0_2_v_2_FromAToB>;
type Message_0_2_v_2_FromAToB =
    SendTimed<Test7, 'a', 0, true, 1, true, 'a', Message_0_2_v_3_FromAToB>;
type Message_0_2_v_3_FromAToB =
    SendTimed<Test8, 'a', 0, true, 1, true, 'a', Message_0_2_v_4_FromAToB>;
type Message_0_2_v_4_FromAToB = End;
type Message_0_2_v_0_FromAToC =
    SendTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_2_v_1_FromAToC>;
type Message_0_2_v_1_FromAToC = End;

// Binary sessions for B
type Message_0_2_v_0_FromBToC = End;
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
type Message_0_2_v_0_FromCToA =
    RecvTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_2_v_1_FromCToA>;
type Message_0_2_v_1_FromCToA = End;
type Message_0_2_v_0_FromCToB = End;

// Binary sessions in depth 0.1
// Binary sessions for C
type Message_0_1_v_0_FromCToB = End;
type Message_0_1_v_0_FromCToA =
    RecvTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromCToA>;
type Message_0_1_v_1_FromCToA = End;

// Binary sessions for A
type Message_0_1_v_0_FromAToB =
    SendTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromAToB>;
type Message_0_1_v_1_FromAToB =
    SendTimed<Test6, 'a', 0, true, 1, true, ' ', Message_0_1_v_2_FromAToB>;
type Message_0_1_v_2_FromAToB =
    SendTimed<Test7, 'a', 0, true, 1, true, 'a', Message_0_1_v_3_FromAToB>;
type Message_0_1_v_3_FromAToB =
    SendTimed<Test8, 'a', 0, true, 1, true, 'a', Message_0_1_v_4_FromAToB>;
type Message_0_1_v_4_FromAToB = End;
type Message_0_1_v_0_FromAToC =
    SendTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromAToC>;
type Message_0_1_v_1_FromAToC = End;

// Binary sessions for B
type Message_0_1_v_0_FromBToA =
    RecvTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromBToA>;
type Message_0_1_v_1_FromBToA =
    RecvTimed<Test6, 'a', 0, true, 1, true, ' ', Message_0_1_v_2_FromBToA>;
type Message_0_1_v_2_FromBToA =
    RecvTimed<Test7, 'a', 0, true, 1, true, 'a', Message_0_1_v_3_FromBToA>;
type Message_0_1_v_3_FromBToA =
    RecvTimed<Test8, 'a', 0, true, 1, true, 'a', Message_0_1_v_4_FromBToA>;
type Message_0_1_v_4_FromBToA = End;
type Message_0_1_v_0_FromBToC = End;

// Binary sessions in depth 0.0
// Binary sessions for B
type Message_0_0_v_0_FromBToC = End;
type Message_0_0_v_0_FromBToA =
    RecvTimed<Test1, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromBToA>;
type Message_0_0_v_1_FromBToA =
    RecvTimed<Test2, 'a', 0, true, 1, true, ' ', Message_0_0_v_2_FromBToA>;
type Message_0_0_v_2_FromBToA =
    RecvTimed<Test3, 'a', 0, true, 1, true, 'a', Message_0_0_v_3_FromBToA>;
type Message_0_0_v_3_FromBToA =
    RecvTimed<Test4, 'a', 0, true, 1, true, 'a', Message_0_0_v_4_FromBToA>;
type Message_0_0_v_4_FromBToA = End;

// Binary sessions for C
type Message_0_0_v_0_FromCToA =
    RecvTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromCToA>;
type Message_0_0_v_1_FromCToA = End;
type Message_0_0_v_0_FromCToB = End;

// Binary sessions for A
type Message_0_0_v_0_FromAToB =
    SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromAToB>;
type Message_0_0_v_1_FromAToB =
    SendTimed<Test2, 'a', 0, true, 1, true, ' ', Message_0_0_v_2_FromAToB>;
type Message_0_0_v_2_FromAToB =
    SendTimed<Test3, 'a', 0, true, 1, true, 'a', Message_0_0_v_3_FromAToB>;
type Message_0_0_v_3_FromAToB =
    SendTimed<Test4, 'a', 0, true, 1, true, 'a', Message_0_0_v_4_FromAToB>;
type Message_0_0_v_4_FromAToB = End;
type Message_0_0_v_0_FromAToC =
    SendTimed<Test5, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromAToC>;
type Message_0_0_v_1_FromAToC = End;

// Stacks in depth 0
// Stacks for A
type Ordering_0_v_0_ForA = RoleBroadcast;

// Stacks for B
type Ordering_0_v_0_ForB = RoleA<RoleEnd>;

// Stacks for C
type Ordering_0_v_0_ForC = RoleA<RoleEnd>;

// Stacks in depth 0.2
// Stacks for C
type Ordering_0_2_v_0_ForC = RoleA<Ordering_0_2_v_1_ForC>;
type Ordering_0_2_v_1_ForC = RoleEnd;

// Stacks for B
type Ordering_0_2_v_0_ForB = RoleA<Ordering_0_2_v_1_ForB>;
type Ordering_0_2_v_1_ForB = RoleA<Ordering_0_2_v_2_ForB>;
type Ordering_0_2_v_2_ForB = RoleA<Ordering_0_2_v_3_ForB>;
type Ordering_0_2_v_3_ForB = RoleA<Ordering_0_2_v_4_ForB>;
type Ordering_0_2_v_4_ForB = RoleEnd;

// Stacks for A
type Ordering_0_2_v_0_ForA = RoleB<Ordering_0_2_v_1_ForA>;
type Ordering_0_2_v_1_ForA = RoleB<Ordering_0_2_v_2_ForA>;
type Ordering_0_2_v_2_ForA = RoleB<Ordering_0_2_v_3_ForA>;
type Ordering_0_2_v_3_ForA = RoleB<Ordering_0_2_v_4_ForA>;
type Ordering_0_2_v_4_ForA = RoleC<Ordering_0_2_v_5_ForA>;
type Ordering_0_2_v_5_ForA = RoleEnd;

// Stacks in depth 0.1
// Stacks for B
type Ordering_0_1_v_0_ForB = RoleA<Ordering_0_1_v_1_ForB>;
type Ordering_0_1_v_1_ForB = RoleA<Ordering_0_1_v_2_ForB>;
type Ordering_0_1_v_2_ForB = RoleA<Ordering_0_1_v_3_ForB>;
type Ordering_0_1_v_3_ForB = RoleA<Ordering_0_1_v_4_ForB>;
type Ordering_0_1_v_4_ForB = RoleEnd;

// Stacks for C
type Ordering_0_1_v_0_ForC = RoleA<Ordering_0_1_v_1_ForC>;
type Ordering_0_1_v_1_ForC = RoleEnd;

// Stacks for A
type Ordering_0_1_v_0_ForA = RoleB<Ordering_0_1_v_1_ForA>;
type Ordering_0_1_v_1_ForA = RoleB<Ordering_0_1_v_2_ForA>;
type Ordering_0_1_v_2_ForA = RoleB<Ordering_0_1_v_3_ForA>;
type Ordering_0_1_v_3_ForA = RoleB<Ordering_0_1_v_4_ForA>;
type Ordering_0_1_v_4_ForA = RoleC<Ordering_0_1_v_5_ForA>;
type Ordering_0_1_v_5_ForA = RoleEnd;

// Stacks in depth 0.0
// Stacks for B
type Ordering_0_0_v_0_ForB = RoleA<Ordering_0_0_v_1_ForB>;
type Ordering_0_0_v_1_ForB = RoleA<Ordering_0_0_v_2_ForB>;
type Ordering_0_0_v_2_ForB = RoleA<Ordering_0_0_v_3_ForB>;
type Ordering_0_0_v_3_ForB = RoleA<Ordering_0_0_v_4_ForB>;
type Ordering_0_0_v_4_ForB = RoleEnd;

// Stacks for A
type Ordering_0_0_v_0_ForA = RoleB<Ordering_0_0_v_1_ForA>;
type Ordering_0_0_v_1_ForA = RoleB<Ordering_0_0_v_2_ForA>;
type Ordering_0_0_v_2_ForA = RoleB<Ordering_0_0_v_3_ForA>;
type Ordering_0_0_v_3_ForA = RoleB<Ordering_0_0_v_4_ForA>;
type Ordering_0_0_v_4_ForA = RoleC<Ordering_0_0_v_5_ForA>;
type Ordering_0_0_v_5_ForA = RoleEnd;

// Stacks for C
type Ordering_0_0_v_0_ForC = RoleA<Ordering_0_0_v_1_ForC>;
type Ordering_0_0_v_1_ForC = RoleEnd;

// Enums (Branchings) in depth 0

// Enums (Branchings) for B
enum Choice_0_FromAToB {
    Branching0(Endpoint_0_0_ForB),
    Branching1(Endpoint_0_1_ForB),
    Branching2(Endpoint_0_2_ForB),
}

// Enums (Branchings) for C
enum Choice_0_FromAToC {
    Branching0(Endpoint_0_0_ForC),
    Branching1(Endpoint_0_1_ForC),
    Branching2(Endpoint_0_2_ForC),
}

// Endpoints in depth 0
// Endpoint for role A
type Endpoint_0_ForA =
    MeshedChannels<Message_0_v_0_FromAToB, Message_0_v_0_FromAToC, Ordering_0_v_0_ForA, NameA>;

// Endpoint for role B
type Endpoint_0_ForB =
    MeshedChannels<Message_0_v_0_FromBToA, Message_0_v_0_FromBToC, Ordering_0_v_0_ForB, NameB>;

// Endpoint for role C
type Endpoint_0_ForC =
    MeshedChannels<Message_0_v_0_FromCToA, Message_0_v_0_FromCToB, Ordering_0_v_0_ForC, NameC>;

// Endpoints in depth 0.2
// Endpoint for role A
type Endpoint_0_2_ForA = MeshedChannels<
    Message_0_2_v_0_FromAToB,
    Message_0_2_v_0_FromAToC,
    Ordering_0_2_v_0_ForA,
    NameA,
>;

// Endpoint for role B
type Endpoint_0_2_ForB = MeshedChannels<
    Message_0_2_v_0_FromBToA,
    Message_0_2_v_0_FromBToC,
    Ordering_0_2_v_0_ForB,
    NameB,
>;

// Endpoint for role C
type Endpoint_0_2_ForC = MeshedChannels<
    Message_0_2_v_0_FromCToA,
    Message_0_2_v_0_FromCToB,
    Ordering_0_2_v_0_ForC,
    NameC,
>;

// Endpoints in depth 0.1
// Endpoint for role A
type Endpoint_0_1_ForA = MeshedChannels<
    Message_0_1_v_0_FromAToB,
    Message_0_1_v_0_FromAToC,
    Ordering_0_1_v_0_ForA,
    NameA,
>;

// Endpoint for role B
type Endpoint_0_1_ForB = MeshedChannels<
    Message_0_1_v_0_FromBToA,
    Message_0_1_v_0_FromBToC,
    Ordering_0_1_v_0_ForB,
    NameB,
>;

// Endpoint for role C
type Endpoint_0_1_ForC = MeshedChannels<
    Message_0_1_v_0_FromCToA,
    Message_0_1_v_0_FromCToB,
    Ordering_0_1_v_0_ForC,
    NameC,
>;

// Endpoints in depth 0.0
// Endpoint for role A
type Endpoint_0_0_ForA = MeshedChannels<
    Message_0_0_v_0_FromAToB,
    Message_0_0_v_0_FromAToC,
    Ordering_0_0_v_0_ForA,
    NameA,
>;

// Endpoint for role B
type Endpoint_0_0_ForB = MeshedChannels<
    Message_0_0_v_0_FromBToA,
    Message_0_0_v_0_FromBToC,
    Ordering_0_0_v_0_ForB,
    NameB,
>;

// Endpoint for role C
type Endpoint_0_0_ForC = MeshedChannels<
    Message_0_0_v_0_FromCToA,
    Message_0_0_v_0_FromCToB,
    Ordering_0_0_v_0_ForC,
    NameC,
>;

// Fill in the functions here.
fn endpoint_a_0_v_0(
    s: Endpoint_0_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let choice: i32 = thread_rng().gen_range(1..=3);

    if choice == 1 {
        let s: Endpoint_0_0_ForA = choose_mpst_a_to_all!(
            s,
            all_clocks,
            Choice_0_FromAToB::Branching0,
            Choice_0_FromAToC::Branching0
        );
        let s = s.send(Test1 {}, all_clocks)?;
        let s = s.send(Test2 { payload: Payload }, all_clocks)?;
        let s = s.send(Test3 {}, all_clocks)?;
        let s = s.send(Test4 { payload: Payload }, all_clocks)?;
        let s = s.send(Test5 { payload: Payload }, all_clocks)?;
        s.close()
    } else if choice == 2 {
        let s: Endpoint_0_1_ForA = choose_mpst_a_to_all!(
            s,
            all_clocks,
            Choice_0_FromAToB::Branching1,
            Choice_0_FromAToC::Branching1
        );
        let s = s.send(Test5 { payload: Payload }, all_clocks)?;
        let s = s.send(Test6 { payload: Payload }, all_clocks)?;
        let s = s.send(Test7 {}, all_clocks)?;
        let s = s.send(Test8 { payload: Payload }, all_clocks)?;
        let s = s.send(Test5 { payload: Payload }, all_clocks)?;
        s.close()
    } else {
        let s: Endpoint_0_2_ForA = choose_mpst_a_to_all!(
            s,
            all_clocks,
            Choice_0_FromAToB::Branching2,
            Choice_0_FromAToC::Branching2
        );
        let s = s.send(Test5 { payload: Payload }, all_clocks)?;
        let s = s.send(Test6 { payload: Payload }, all_clocks)?;
        let s = s.send(Test7 {}, all_clocks)?;
        let s = s.send(Test8 { payload: Payload }, all_clocks)?;
        let s = s.send(Test5 { payload: Payload }, all_clocks)?;
        s.close()
    }
}

fn endpoint_b_0_v_0(
    s: Endpoint_0_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    offer_mpst!(s, all_clocks, {
        Choice_0_FromAToB::Branching0(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
        Choice_0_FromAToB::Branching1(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
        Choice_0_FromAToB::Branching2(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
    })
}

fn endpoint_c_0_v_0(
    s: Endpoint_0_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    all_clocks.insert('a', Instant::now());
    offer_mpst!(s, all_clocks, {
        Choice_0_FromAToC::Branching0(s) => {
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
        Choice_0_FromAToC::Branching1(s) => {
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
        Choice_0_FromAToC::Branching2(s) => {
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
    })
}

fn endpoint_a_0_2_v_0(
    s: Endpoint_0_2_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_a_0_2_v_1(
    s: Endpoint_0_2_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_a_0_2_v_2(
    s: Endpoint_0_2_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_2_v_0(
    s: Endpoint_0_2_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_2_v_1(
    s: Endpoint_0_2_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_2_v_2(
    s: Endpoint_0_2_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_c_0_2_v_0(
    s: Endpoint_0_2_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_c_0_2_v_1(
    s: Endpoint_0_2_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_c_0_2_v_2(
    s: Endpoint_0_2_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_a_0_1_v_0(
    s: Endpoint_0_1_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_a_0_1_v_1(
    s: Endpoint_0_1_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_1_v_0(
    s: Endpoint_0_1_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_1_v_1(
    s: Endpoint_0_1_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_c_0_1_v_0(
    s: Endpoint_0_1_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_c_0_1_v_1(
    s: Endpoint_0_1_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_a_0_0_v_0(
    s: Endpoint_0_0_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_b_0_0_v_0(
    s: Endpoint_0_0_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_c_0_0_v_0(
    s: Endpoint_0_0_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn main() {
    let (thread_a, thread_b, thread_c) =
        fork_mpst(endpoint_a_0_v_0, endpoint_b_0_v_0, endpoint_c_0_v_0);

    println!("Thread A: {:?}", thread_a.join());
    println!("Thread B: {:?}", thread_b.join());
    println!("Thread C: {:?}", thread_c.join());
}
