#![allow(dead_code, non_camel_case_types, unused_variables)]

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
struct Test0;
struct TestInit;

struct Test2 {
    payload: Payload,
}
struct Test1;
struct Test3;
struct Test4 {
    payload: Payload,
}

// Binary sessions in depth 0
// Binary sessions for A
type Message_0_v_0_FromAToB =
    SendTimed<TestInit, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromAToB>;
type Message_0_v_1_FromAToB = SendTimed<Test0, 'a', 0, true, 1, true, ' ', Message_0_v_2_FromAToB>;
type Message_0_v_2_FromAToB = SendTimed<Choice_0_FromAToB, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromAToC = SendTimed<Choice_0_FromAToC, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for B
type Message_0_v_0_FromBToA =
    RecvTimed<TestInit, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromBToA>;
type Message_0_v_1_FromBToA = RecvTimed<Test0, 'a', 0, true, 1, true, ' ', Message_0_v_2_FromBToA>;
type Message_0_v_2_FromBToA = RecvTimed<Choice_0_FromAToB, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromBToC = End;

// Binary sessions for C
type Message_0_v_0_FromCToA = RecvTimed<Choice_0_FromAToC, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromCToB = End;

// Binary sessions in depth 0.1
// Binary sessions for A
type Message_0_1_v_0_FromAToB =
    SendTimed<Test1, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromAToB>;
type Message_0_1_v_1_FromAToB =
    SendTimed<Test2, 'a', 0, true, 1, true, ' ', Message_0_1_v_2_FromAToB>;
type Message_0_1_v_2_FromAToB =
    SendTimed<Test3, 'a', 0, true, 1, true, 'a', Message_0_1_v_3_FromAToB>;
type Message_0_1_v_3_FromAToB =
    SendTimed<Test4, 'a', 0, true, 1, true, 'a', Message_0_1_v_4_FromAToB>;
type Message_0_1_v_4_FromAToB = End;
type Message_0_1_v_0_FromAToC = End;

// Binary sessions for B
type Message_0_1_v_0_FromBToA =
    RecvTimed<Test1, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromBToA>;
type Message_0_1_v_1_FromBToA =
    RecvTimed<Test2, 'a', 0, true, 1, true, ' ', Message_0_1_v_2_FromBToA>;
type Message_0_1_v_2_FromBToA =
    RecvTimed<Test3, 'a', 0, true, 1, true, 'a', Message_0_1_v_3_FromBToA>;
type Message_0_1_v_3_FromBToA =
    RecvTimed<Test4, 'a', 0, true, 1, true, 'a', Message_0_1_v_4_FromBToA>;
type Message_0_1_v_4_FromBToA = End;
type Message_0_1_v_0_FromBToC = End;

// Binary sessions for C
type Message_0_1_v_0_FromCToA = End;
type Message_0_1_v_0_FromCToB = End;

// Binary sessions in depth 0.0
// Binary sessions for C
type Message_0_0_v_0_FromCToA = RecvTimed<Choice_0_FromAToC, ' ', -2, false, -1, false, ' ', End>;
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
type Message_0_0_v_4_FromAToB =
    SendTimed<Test0, 'a', 0, true, 1, true, ' ', Message_0_0_v_5_FromAToB>;
type Message_0_0_v_5_FromAToB = SendTimed<Choice_0_FromAToB, ' ', -2, false, -1, false, ' ', End>;
type Message_0_0_v_0_FromAToC = SendTimed<Choice_0_FromAToC, ' ', -2, false, -1, false, ' ', End>;

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
type Message_0_0_v_4_FromBToA =
    RecvTimed<Test0, 'a', 0, true, 1, true, ' ', Message_0_0_v_5_FromBToA>;
type Message_0_0_v_5_FromBToA = RecvTimed<Choice_0_FromAToB, ' ', -2, false, -1, false, ' ', End>;

// Stacks in depth 0
// Stacks for C
type Ordering_0_v_0_ForC = RoleA<RoleEnd>;

// Stacks for B
type Ordering_0_v_0_ForB = RoleA<Ordering_0_v_1_ForB>;
type Ordering_0_v_1_ForB = RoleA<Ordering_0_v_2_ForB>;
type Ordering_0_v_2_ForB = RoleA<RoleEnd>;

// Stacks for A
type Ordering_0_v_0_ForA = RoleB<Ordering_0_v_1_ForA>;
type Ordering_0_v_1_ForA = RoleB<Ordering_0_v_2_ForA>;
type Ordering_0_v_2_ForA = RoleBroadcast;

// Stacks in depth 0.1
// Stacks for B
type Ordering_0_1_v_0_ForB = RoleA<Ordering_0_1_v_1_ForB>;
type Ordering_0_1_v_1_ForB = RoleA<Ordering_0_1_v_2_ForB>;
type Ordering_0_1_v_2_ForB = RoleA<Ordering_0_1_v_3_ForB>;
type Ordering_0_1_v_3_ForB = RoleA<Ordering_0_1_v_4_ForB>;
type Ordering_0_1_v_4_ForB = RoleEnd;

// Stacks for C
type Ordering_0_1_v_0_ForC = RoleEnd;

// Stacks for A
type Ordering_0_1_v_0_ForA = RoleB<Ordering_0_1_v_1_ForA>;
type Ordering_0_1_v_1_ForA = RoleB<Ordering_0_1_v_2_ForA>;
type Ordering_0_1_v_2_ForA = RoleB<Ordering_0_1_v_3_ForA>;
type Ordering_0_1_v_3_ForA = RoleB<Ordering_0_1_v_4_ForA>;
type Ordering_0_1_v_4_ForA = RoleEnd;

// Stacks in depth 0.0
// Stacks for A
type Ordering_0_0_v_0_ForA = RoleB<Ordering_0_0_v_1_ForA>;
type Ordering_0_0_v_1_ForA = RoleB<Ordering_0_0_v_2_ForA>;
type Ordering_0_0_v_2_ForA = RoleB<Ordering_0_0_v_3_ForA>;
type Ordering_0_0_v_3_ForA = RoleB<Ordering_0_0_v_4_ForA>;
type Ordering_0_0_v_4_ForA = RoleB<Ordering_0_0_v_5_ForA>;
type Ordering_0_0_v_5_ForA = RoleBroadcast;

// Stacks for B
type Ordering_0_0_v_0_ForB = RoleA<Ordering_0_0_v_1_ForB>;
type Ordering_0_0_v_1_ForB = RoleA<Ordering_0_0_v_2_ForB>;
type Ordering_0_0_v_2_ForB = RoleA<Ordering_0_0_v_3_ForB>;
type Ordering_0_0_v_3_ForB = RoleA<Ordering_0_0_v_4_ForB>;
type Ordering_0_0_v_4_ForB = RoleA<Ordering_0_0_v_5_ForB>;
type Ordering_0_0_v_5_ForB = RoleA<RoleEnd>;

// Stacks for C
type Ordering_0_0_v_0_ForC = RoleA<RoleEnd>;

// Enums (Branchings) in depth 0

// Enums (Branchings) for B
enum Choice_0_FromAToB {
    Branching0(Endpoint_0_0_ForB),
    Branching1(Endpoint_0_1_ForB),
}

// Enums (Branchings) for C
enum Choice_0_FromAToC {
    Branching0(Endpoint_0_0_ForC),
    Branching1(Endpoint_0_1_ForC),
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
type Endpoint_0_0_ForA_Loop = MeshedChannels<
    SendTimed<
        Test0,
        'a',
        0,
        true,
        1,
        true,
        ' ',
        SendTimed<Choice_0_FromAToB, ' ', -2, false, -1, false, ' ', End>,
    >,
    SendTimed<Choice_0_FromAToC, ' ', -2, false, -1, false, ' ', End>,
    RoleB<RoleBroadcast>,
    NameA,
>;

// Endpoint for role B
type Endpoint_0_0_ForB_Loop = MeshedChannels<
    RecvTimed<
        Test0,
        'a',
        0,
        true,
        1,
        true,
        ' ',
        RecvTimed<Choice_0_FromAToB, ' ', -2, false, -1, false, ' ', End>,
    >,
    Message_0_0_v_0_FromBToC,
    RoleA<RoleA<RoleEnd>>,
    NameB,
>;

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

type Endpoint_0_0_ForC_Loop = MeshedChannels<
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

    let s = s.send(TestInit, all_clocks)?;
    let s = s.send(Test0, all_clocks)?;

    let s: Endpoint_0_0_ForA = choose_mpst_a_to_all!(
        s,
        all_clocks,
        Choice_0_FromAToB::Branching0,
        Choice_0_FromAToC::Branching0
    );
    let s = s.send(Test1, all_clocks)?;
    let s = s.send(Test2 { payload: Payload }, all_clocks)?;
    let s = s.send(Test3, all_clocks)?;
    let s = s.send(Test4 { payload: Payload }, all_clocks)?;
    rec_endpoint_a_0_v_0(s, all_clocks, 5)
}

fn rec_endpoint_a_0_v_0(
    s: Endpoint_0_0_ForA_Loop,
    all_clocks: &mut HashMap<char, Instant>,
    loops: i32,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());
    let s = s.send(Test0, all_clocks)?;

    if loops > 0 {
        let s: Endpoint_0_0_ForA = choose_mpst_a_to_all!(
            s,
            all_clocks,
            Choice_0_FromAToB::Branching0,
            Choice_0_FromAToC::Branching0
        );
        let s = s.send(Test1, all_clocks)?;
        let s = s.send(Test2 { payload: Payload }, all_clocks)?;
        let s = s.send(Test3, all_clocks)?;
        let s = s.send(Test4 { payload: Payload }, all_clocks)?;
        rec_endpoint_a_0_v_0(s, all_clocks, loops - 1)
    } else {
        let s: Endpoint_0_1_ForA = choose_mpst_a_to_all!(
            s,
            all_clocks,
            Choice_0_FromAToB::Branching1,
            Choice_0_FromAToC::Branching1
        );
        let s = s.send(Test1, all_clocks)?;
        let s = s.send(Test2 { payload: Payload }, all_clocks)?;
        let s = s.send(Test3, all_clocks)?;
        let s = s.send(Test4 { payload: Payload }, all_clocks)?;
        s.close()
    }
}

fn endpoint_b_0_v_0(
    s: Endpoint_0_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv(all_clocks)?;
    let (_, s) = s.recv(all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Choice_0_FromAToB::Branching0(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            aux_endpoint_b_0_v_0(s, all_clocks)
        },
        Choice_0_FromAToB::Branching1(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            s.close()
        },
    })
}

fn aux_endpoint_b_0_v_0(
    s: Endpoint_0_0_ForB_Loop,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv(all_clocks)?;

    offer_mpst!(s, all_clocks, {
        Choice_0_FromAToB::Branching0(s) => {
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;
            let (_, s) = s.recv(all_clocks)?;

            aux_endpoint_b_0_v_0(s, all_clocks)
        },
        Choice_0_FromAToB::Branching1(s) => {
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
    offer_mpst!(s, all_clocks, {
        Choice_0_FromAToC::Branching0(s) => {

            aux_endpoint_c_0_v_0(s, all_clocks)
        },
        Choice_0_FromAToC::Branching1(s) => {

            s.close()
        },
    })
}

fn aux_endpoint_c_0_v_0(
    s: Endpoint_0_0_ForC_Loop,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, all_clocks, {
        Choice_0_FromAToC::Branching0(s) => {

            aux_endpoint_c_0_v_0(s, all_clocks)
        },
        Choice_0_FromAToC::Branching1(s) => {

            s.close()
        },
    })
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
    Ok(())
}

fn endpoint_c_0_1_v_1(
    s: Endpoint_0_1_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
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
    Ok(())
}

fn main() {
    let (thread_a, thread_b, thread_c) =
        fork_mpst(endpoint_a_0_v_0, endpoint_b_0_v_0, endpoint_c_0_v_0);

    println!("Thread A: {:?}", thread_a.join());
    println!("Thread B: {:?}", thread_b.join());
    println!("Thread C: {:?}", thread_c.join());
}
