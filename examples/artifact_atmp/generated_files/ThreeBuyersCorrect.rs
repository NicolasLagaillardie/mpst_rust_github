#![allow(dead_code, non_camel_case_types, unused_variables)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_atmp!(MeshedChannels, A, C, S);

// Types of the payloads
struct Int;
struct Empty4 {
    payload: Int,
}
struct Empty2 {
    payload: Int,
}
struct Empty1 {
    payload: Int,
}
struct Empty3 {
    payload: Int,
}

struct Quit;

struct Empty5 {
    payload: Int,
}
struct Ok {
    payload: Int,
}

// Binary sessions in depth 0
// Binary sessions for A
type Message_0_v_0_FromAToS = SendTimed<Empty1, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromAToS>;
type Message_0_v_1_FromAToS = RecvTimed<Empty2, 'a', 0, true, 1, true, ' ', Message_0_v_2_FromAToS>;
type Message_0_v_2_FromAToS = End;
type Message_0_v_0_FromAToC = SendTimed<Empty4, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromAToC>;
type Message_0_v_1_FromAToC = RecvTimed<Choice_0_FromCToA, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for C
type Message_0_v_0_FromCToA = RecvTimed<Empty4, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromCToA>;
type Message_0_v_1_FromCToA = SendTimed<Choice_0_FromCToA, ' ', -2, false, -1, false, ' ', End>;
type Message_0_v_0_FromCToS = RecvTimed<Empty3, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromCToS>;
type Message_0_v_1_FromCToS = SendTimed<Choice_0_FromCToS, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions for S
type Message_0_v_0_FromSToA = RecvTimed<Empty1, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromSToA>;
type Message_0_v_1_FromSToA = SendTimed<Empty2, 'a', 0, true, 1, true, ' ', Message_0_v_2_FromSToA>;
type Message_0_v_2_FromSToA = End;
type Message_0_v_0_FromSToC = SendTimed<Empty3, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromSToC>;
type Message_0_v_1_FromSToC = RecvTimed<Choice_0_FromCToS, ' ', -2, false, -1, false, ' ', End>;

// Binary sessions in depth 0.1
// Binary sessions for S
type Message_0_1_v_0_FromSToA = End;
type Message_0_1_v_0_FromSToC =
    RecvTimed<Quit, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromSToC>;
type Message_0_1_v_1_FromSToC = End;

// Binary sessions for A
type Message_0_1_v_0_FromAToC =
    RecvTimed<Quit, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromAToC>;
type Message_0_1_v_1_FromAToC = End;
type Message_0_1_v_0_FromAToS = End;

// Binary sessions for C
type Message_0_1_v_0_FromCToS =
    SendTimed<Quit, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromCToS>;
type Message_0_1_v_1_FromCToS = End;
type Message_0_1_v_0_FromCToA =
    SendTimed<Quit, 'a', 0, true, 1, true, ' ', Message_0_1_v_1_FromCToA>;
type Message_0_1_v_1_FromCToA = End;

// Binary sessions in depth 0.0
// Binary sessions for A
type Message_0_0_v_0_FromAToC = RecvTimed<Ok, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromAToC>;
type Message_0_0_v_1_FromAToC = End;
type Message_0_0_v_0_FromAToS = End;

// Binary sessions for C
type Message_0_0_v_0_FromCToS = SendTimed<Ok, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromCToS>;
type Message_0_0_v_1_FromCToS =
    RecvTimed<Empty5, 'a', 0, true, 1, true, ' ', Message_0_0_v_2_FromCToS>;
type Message_0_0_v_2_FromCToS = End;
type Message_0_0_v_0_FromCToA = SendTimed<Ok, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromCToA>;
type Message_0_0_v_1_FromCToA = End;

// Binary sessions for S
type Message_0_0_v_0_FromSToA = End;
type Message_0_0_v_0_FromSToC = RecvTimed<Ok, 'a', 0, true, 1, true, ' ', Message_0_0_v_1_FromSToC>;
type Message_0_0_v_1_FromSToC =
    SendTimed<Empty5, 'a', 0, true, 1, true, ' ', Message_0_0_v_2_FromSToC>;
type Message_0_0_v_2_FromSToC = End;

// Stacks in depth 0
// Stacks for S
type Ordering_0_v_0_ForS = RoleA<Ordering_0_v_1_ForS>;
type Ordering_0_v_1_ForS = RoleA<Ordering_0_v_2_ForS>;
type Ordering_0_v_2_ForS = RoleC<Ordering_0_v_3_ForS>;
type Ordering_0_v_3_ForS = RoleC<RoleEnd>;

// Stacks for C
type Ordering_0_v_0_ForC = RoleS<Ordering_0_v_1_ForC>;
type Ordering_0_v_1_ForC = RoleA<Ordering_0_v_2_ForC>;
type Ordering_0_v_2_ForC = RoleBroadcast;

// Stacks for A
type Ordering_0_v_0_ForA = RoleS<Ordering_0_v_1_ForA>;
type Ordering_0_v_1_ForA = RoleS<Ordering_0_v_2_ForA>;
type Ordering_0_v_2_ForA = RoleC<Ordering_0_v_3_ForA>;
type Ordering_0_v_3_ForA = RoleC<RoleEnd>;

// Stacks in depth 0.1
// Stacks for A
type Ordering_0_1_v_0_ForA = RoleC<Ordering_0_1_v_1_ForA>;
type Ordering_0_1_v_1_ForA = RoleEnd;

// Stacks for C
type Ordering_0_1_v_0_ForC = RoleA<Ordering_0_1_v_1_ForC>;
type Ordering_0_1_v_1_ForC = RoleS<Ordering_0_1_v_2_ForC>;
type Ordering_0_1_v_2_ForC = RoleEnd;

// Stacks for S
type Ordering_0_1_v_0_ForS = RoleC<Ordering_0_1_v_1_ForS>;
type Ordering_0_1_v_1_ForS = RoleEnd;

// Stacks in depth 0.0
// Stacks for A
type Ordering_0_0_v_0_ForA = RoleC<Ordering_0_0_v_1_ForA>;
type Ordering_0_0_v_1_ForA = RoleEnd;

// Stacks for C
type Ordering_0_0_v_0_ForC = RoleA<Ordering_0_0_v_1_ForC>;
type Ordering_0_0_v_1_ForC = RoleS<Ordering_0_0_v_2_ForC>;
type Ordering_0_0_v_2_ForC = RoleS<Ordering_0_0_v_3_ForC>;
type Ordering_0_0_v_3_ForC = RoleEnd;

// Stacks for S
type Ordering_0_0_v_0_ForS = RoleC<Ordering_0_0_v_1_ForS>;
type Ordering_0_0_v_1_ForS = RoleC<Ordering_0_0_v_2_ForS>;
type Ordering_0_0_v_2_ForS = RoleEnd;

// Enums (Branchings) in depth 0
// Enums (Branchings) for A
enum Choice_0_FromCToA {
    Branching0(Endpoint_0_0_ForA),
    Branching1(Endpoint_0_1_ForA),
}

// Enums (Branchings) for S
enum Choice_0_FromCToS {
    Branching0(Endpoint_0_0_ForS),
    Branching1(Endpoint_0_1_ForS),
}

// Endpoints in depth 0
// Endpoint for role A
type Endpoint_0_ForA =
    MeshedChannels<Message_0_v_0_FromAToC, Message_0_v_0_FromAToS, Ordering_0_v_0_ForA, NameA>;

// Endpoint for role C
type Endpoint_0_ForC =
    MeshedChannels<Message_0_v_0_FromCToA, Message_0_v_0_FromCToS, Ordering_0_v_0_ForC, NameC>;

// Endpoint for role S
type Endpoint_0_ForS =
    MeshedChannels<Message_0_v_0_FromSToA, Message_0_v_0_FromSToC, Ordering_0_v_0_ForS, NameS>;

// Endpoints in depth 0.1
// Endpoint for role A
type Endpoint_0_1_ForA = MeshedChannels<
    Message_0_1_v_0_FromAToC,
    Message_0_1_v_0_FromAToS,
    Ordering_0_1_v_0_ForA,
    NameA,
>;

// Endpoint for role C
type Endpoint_0_1_ForC = MeshedChannels<
    Message_0_1_v_0_FromCToA,
    Message_0_1_v_0_FromCToS,
    Ordering_0_1_v_0_ForC,
    NameC,
>;

// Endpoint for role S
type Endpoint_0_1_ForS = MeshedChannels<
    Message_0_1_v_0_FromSToA,
    Message_0_1_v_0_FromSToC,
    Ordering_0_1_v_0_ForS,
    NameS,
>;

// Endpoints in depth 0.0
// Endpoint for role A
type Endpoint_0_0_ForA = MeshedChannels<
    Message_0_0_v_0_FromAToC,
    Message_0_0_v_0_FromAToS,
    Ordering_0_0_v_0_ForA,
    NameA,
>;

// Endpoint for role C
type Endpoint_0_0_ForC = MeshedChannels<
    Message_0_0_v_0_FromCToA,
    Message_0_0_v_0_FromCToS,
    Ordering_0_0_v_0_ForC,
    NameC,
>;

// Endpoint for role S
type Endpoint_0_0_ForS = MeshedChannels<
    Message_0_0_v_0_FromSToA,
    Message_0_0_v_0_FromSToC,
    Ordering_0_0_v_0_ForS,
    NameS,
>;

// Fill in the functions here.
fn endpoint_a_0_v_0(
    s: Endpoint_0_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(Empty1 { payload: Int }, all_clocks)?;
    let (_empty2, s) = s.recv(all_clocks)?;
    let s = s.send(Empty4 { payload: Int }, all_clocks)?;
    offer_mpst!(s, all_clocks, {
        Choice_0_FromCToA::Branching0(s) => {
            let (_ok, s) = s.recv(all_clocks)?;
            s.close()
        },
        Choice_0_FromCToA::Branching1(s) => {
            let (_ok, s) = s.recv(all_clocks)?;
            s.close()
        },
    })
}

fn endpoint_c_0_v_0(
    s: Endpoint_0_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_empty3, s) = s.recv(all_clocks)?;
    let (_empty4, s) = s.recv(all_clocks)?;

    let choice: i32 = thread_rng().gen_range(1..=3);

    if choice != 1 {
        let s: Endpoint_0_0_ForC = choose_mpst_c_to_all!(
            s,
            all_clocks,
            Choice_0_FromCToA::Branching0,
            Choice_0_FromCToS::Branching0,
        );

        let s = s.send(Ok { payload: Int }, all_clocks)?;
        let s = s.send(Ok { payload: Int }, all_clocks)?;
        let (_empty5, s) = s.recv(all_clocks)?;

        s.close()
    } else {
        let s: Endpoint_0_1_ForC = choose_mpst_c_to_all!(
            s,
            all_clocks,
            Choice_0_FromCToA::Branching1,
            Choice_0_FromCToS::Branching1,
        );
        let s = s.send(Quit, all_clocks)?;
        let s = s.send(Quit, all_clocks)?;
        s.close()
    }
}

fn endpoint_s_0_v_0(
    s: Endpoint_0_ForS,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_empty1, s) = s.recv(all_clocks)?;
    let s = s.send(Empty2 { payload: Int }, all_clocks)?;
    let s = s.send(Empty3 { payload: Int }, all_clocks)?;
    offer_mpst!(s, all_clocks, {
        Choice_0_FromCToS::Branching0(s) => {
            let (_ok, s) = s.recv(all_clocks)?;
            let s = s.send(Empty5 { payload: Int }, all_clocks)?;
            s.close()
        },
        Choice_0_FromCToS::Branching1(s) => {
            let (_ok, s) = s.recv(all_clocks)?;
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

fn endpoint_s_0_1_v_0(
    s: Endpoint_0_1_ForS,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_s_0_1_v_1(
    s: Endpoint_0_1_ForS,
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

fn endpoint_c_0_0_v_0(
    s: Endpoint_0_0_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn endpoint_s_0_0_v_0(
    s: Endpoint_0_0_ForS,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    Ok(())
}

fn main() {
    let (thread_a, thread_c, thread_s) =
        fork_mpst(endpoint_a_0_v_0, endpoint_c_0_v_0, endpoint_s_0_v_0);

    println!("Thread A: {:?}", thread_a.join());
    println!("Thread C: {:?}", thread_c.join());
    println!("Thread S: {:?}", thread_s.join());
}
