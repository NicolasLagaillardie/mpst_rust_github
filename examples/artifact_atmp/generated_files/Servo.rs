#![allow(dead_code, non_camel_case_types, unused_variables)]

use mpstthree::binary::struct_trait::end::End;
use mpstthree::binary_atmp::struct_trait::{recv::RecvTimed, send::SendTimed};
use mpstthree::generate_atmp;
use mpstthree::role::end::RoleEnd;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

generate_atmp!(MeshedChannels, C, L, S);

// Types of the payloads
struct WebFontLoaded;
struct OutstandingWebFonts;
struct GetWebPageLoadState;
struct GetCurrentState;
struct DocumentLoading;

// Binary sessions in depth 0
// Binary sessions for L
type Message_0_v_0_FromLToS =
    SendTimed<WebFontLoaded, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromLToS>;
type Message_0_v_1_FromLToS = End;
type Message_0_v_0_FromLToC =
    RecvTimed<GetWebPageLoadState, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromLToC>;
type Message_0_v_1_FromLToC =
    SendTimed<OutstandingWebFonts, 'a', 0, true, 1, true, ' ', Message_0_v_2_FromLToC>;
type Message_0_v_2_FromLToC = End;

// Binary sessions for C
type Message_0_v_0_FromCToS =
    SendTimed<GetCurrentState, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromCToS>;
type Message_0_v_1_FromCToS =
    RecvTimed<DocumentLoading, 'a', 0, true, 1, true, ' ', Message_0_v_2_FromCToS>;
type Message_0_v_2_FromCToS = End;
type Message_0_v_0_FromCToL =
    SendTimed<GetWebPageLoadState, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromCToL>;
type Message_0_v_1_FromCToL =
    RecvTimed<OutstandingWebFonts, 'a', 0, true, 1, true, ' ', Message_0_v_2_FromCToL>;
type Message_0_v_2_FromCToL = End;

// Binary sessions for S
type Message_0_v_0_FromSToC =
    RecvTimed<GetCurrentState, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromSToC>;
type Message_0_v_1_FromSToC =
    SendTimed<DocumentLoading, 'a', 0, true, 1, true, ' ', Message_0_v_2_FromSToC>;
type Message_0_v_2_FromSToC = End;
type Message_0_v_0_FromSToL =
    RecvTimed<WebFontLoaded, 'a', 0, true, 1, true, ' ', Message_0_v_1_FromSToL>;
type Message_0_v_1_FromSToL = End;

// Stacks in depth 0
// Stacks for L
type Ordering_0_v_0_ForL = RoleS<Ordering_0_v_1_ForL>;
type Ordering_0_v_1_ForL = RoleC<Ordering_0_v_2_ForL>;
type Ordering_0_v_2_ForL = RoleC<Ordering_0_v_3_ForL>;
type Ordering_0_v_3_ForL = RoleEnd;

// Stacks for C
type Ordering_0_v_0_ForC = RoleS<Ordering_0_v_1_ForC>;
type Ordering_0_v_1_ForC = RoleS<Ordering_0_v_2_ForC>;
type Ordering_0_v_2_ForC = RoleL<Ordering_0_v_3_ForC>;
type Ordering_0_v_3_ForC = RoleL<Ordering_0_v_4_ForC>;
type Ordering_0_v_4_ForC = RoleEnd;

// Stacks for S
type Ordering_0_v_0_ForS = RoleC<Ordering_0_v_1_ForS>;
type Ordering_0_v_1_ForS = RoleC<Ordering_0_v_2_ForS>;
type Ordering_0_v_2_ForS = RoleL<Ordering_0_v_3_ForS>;
type Ordering_0_v_3_ForS = RoleEnd;

// Endpoints in depth 0
// Endpoint for role C
type Endpoint_0_ForC =
    MeshedChannels<Message_0_v_0_FromCToL, Message_0_v_0_FromCToS, Ordering_0_v_0_ForC, NameC>;

// Endpoint for role L
type Endpoint_0_ForL =
    MeshedChannels<Message_0_v_0_FromLToC, Message_0_v_0_FromLToS, Ordering_0_v_0_ForL, NameL>;

// Endpoint for role S
type Endpoint_0_ForS =
    MeshedChannels<Message_0_v_0_FromSToC, Message_0_v_0_FromSToL, Ordering_0_v_0_ForS, NameS>;

// Fill in the functions here.
fn endpoint_c_0_v_0(
    s: Endpoint_0_ForC,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(GetCurrentState {}, all_clocks)?;
    let (_, s) = s.recv(all_clocks)?;
    let s = s.send(GetWebPageLoadState {}, all_clocks)?;
    let (_, s) = s.recv(all_clocks)?;

    s.close()
}

fn endpoint_l_0_v_0(
    s: Endpoint_0_ForL,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let s = s.send(WebFontLoaded {}, all_clocks)?;
    let (_, s) = s.recv(all_clocks)?;
    let s = s.send(OutstandingWebFonts {}, all_clocks)?;

    s.close()
}

fn endpoint_s_0_v_0(
    s: Endpoint_0_ForS,
    all_clocks: &mut HashMap<char, Instant>,
) -> Result<(), Box<dyn Error>> {
    all_clocks.insert('a', Instant::now());

    let (_, s) = s.recv(all_clocks)?;
    let s = s.send(DocumentLoading {}, all_clocks)?;
    let (_, s) = s.recv(all_clocks)?;

    s.close()
}

fn main() {
    let (thread_c, thread_l, thread_s) =
        fork_mpst(endpoint_c_0_v_0, endpoint_l_0_v_0, endpoint_s_0_v_0);

    println!("Thread C: {:?}", thread_c.join());
    println!("Thread L: {:?}", thread_l.join());
    println!("Thread S: {:?}", thread_s.join());
}
