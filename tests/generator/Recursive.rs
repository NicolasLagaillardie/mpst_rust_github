use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::end::RoleEnd;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_atmp!(MeshedChannels, A, B, C);

// Binary sessions for B
type Message_0_v_0_FromBToA = End;
type Message_0_v_0_FromBToC = End;

// Binary sessions for C
type Message_0_v_0_FromCToB = End;
type Message_0_v_0_FromCToA = End;

// Binary sessions for A
type Message_0_v_0_FromAToB = End;
type Message_0_v_0_FromAToC = End;

// Stacks for A
type Ordering_0_v_0_ForA = RoleEnd;

// Stacks for C
type Ordering_0_v_0_ForC = RoleEnd;

// Stacks for B
type Ordering_0_v_0_ForB = RoleEnd;

// Endpoint for role A
type Endpoint_0_v_0_ForA =
    MeshedChannels<Message_0_v_0_FromAToB, Message_0_v_0_FromAToC, Ordering_0_v_0_ForA, NameA>;

// Endpoint for role B
type Endpoint_0_v_0_ForB =
    MeshedChannels<Message_0_v_0_FromBToA, Message_0_v_0_FromBToC, Ordering_0_v_0_ForB, NameB>;

// Endpoint for role C
type Endpoint_0_v_0_ForC =
    MeshedChannels<Message_0_v_0_FromCToA, Message_0_v_0_FromCToB, Ordering_0_v_0_ForC, NameC>;

// Fill in the functions here.

fn endpoint_a(
    s: Endpoint_0_v_0_ForA,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn endpoint_b(
    s: Endpoint_0_v_0_ForB,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn endpoint_c(
    s: Endpoint_0_v_0_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    println!("Thread A: {:?}", thread_a.join());
    println!("Thread B: {:?}", thread_b.join());
    println!("Thread C: {:?}", thread_c.join());
}
