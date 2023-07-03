#![allow(
    clippy::large_enum_variant,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::baker;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::end::RoleEnd;

use std::error::Error;

baker!(
    "rec_and_cancel",
    MeshedChannels,
    Constellation,
    Layout,
    Script
);

// Payload types

struct GetCurrentState;
struct DocumentLoading;
struct WebFontLoaded;
struct GetWebPageLoadState;
struct OutstandingWebFonts;

// Binary types

// Constellation thread

type CtoL = Send<GetWebPageLoadState, Recv<OutstandingWebFonts, End>>;
type CtoS = Send<GetCurrentState, Recv<DocumentLoading, End>>;

//Layout thread

type LtoC = Recv<GetWebPageLoadState, Send<OutstandingWebFonts, End>>;
type LtoS = Send<WebFontLoaded, End>;

// Script thread

type StoC = Recv<GetCurrentState, Send<DocumentLoading, End>>;
type StoL = Recv<WebFontLoaded, End>;

// Orderings

type OrderingC = RoleScript<RoleScript<RoleLayout<RoleLayout<RoleEnd>>>>;

type OrderingL = RoleScript<RoleConstellation<RoleConstellation<RoleEnd>>>;

type OrderingS = RoleConstellation<RoleConstellation<RoleLayout<RoleEnd>>>;

// MeshedChannels

type EndpointC = MeshedChannels<CtoL, CtoS, OrderingC, NameConstellation>;

type EndpointL = MeshedChannels<LtoC, LtoS, OrderingL, NameLayout>;

type EndpointS = MeshedChannels<StoC, StoL, OrderingS, NameScript>;

// Functions

/////////////////////////

// Functions related to endpoints
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = s.send(GetCurrentState {})?;

    let (_, s) = s.recv()?;

    // To "process" the information
    let s = s.send(GetWebPageLoadState {})?;

    let (_, s) = s.recv()?;

    s.close()
}

/////////////////////////

fn endpoint_l(s: EndpointL) -> Result<(), Box<dyn Error>> {
    let s = s.send(WebFontLoaded {})?;

    let (_, s) = s.recv()?;

    let s = s.send(OutstandingWebFonts {})?;

    s.close()
}

/////////////////////////

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    let s = s.send(DocumentLoading {})?;

    let (_, s) = s.recv()?;

    s.close()
}

////////////////////////////////////////

fn aux() {
    let (thread_c, thread_l, thread_s) = fork_mpst(
        black_box(endpoint_c),
        black_box(endpoint_l),
        black_box(endpoint_s),
    );

    thread_c.join().unwrap();
    thread_l.join().unwrap();
    thread_s.join().unwrap();
}

/////////////////////////

pub fn servo(c: &mut Criterion) {
    c.bench_function("Servo baking", |b| b.iter(aux));
}

/////////////////////////

criterion_group! {
    name = bench;
    config = Criterion::default().significance_level(0.1).sample_size(10000);
    targets = servo,
}

criterion_main! {
    bench
}
