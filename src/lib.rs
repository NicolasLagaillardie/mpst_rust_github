#![allow(
    clippy::cognitive_complexity,
    clippy::large_enum_variant,
    clippy::needless_doctest_main
)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(unused_must_use)]
#![deny(semicolon_in_expressions_from_macros)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(doc_cfg, deny(rustdoc::broken_intra_doc_links))]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, unused_attributes))
))]

//! [![github]](https://github.com/NicolasLagaillardie/mpst_rust_github)&ensp;[![crates-io]](https://crates.io/crates/mpstthree)&ensp;[![docs-rs]](https://docs.rs/mpstthree)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64, PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Mpstthree (also called MultiCrusty) is a library to
//! write and check communication protocols based on [Multiparty Session Types](https://mrg.doc.ic.ac.uk/publications/a-very-gentle-introduction-to-multiparty-session-types/).
//!
//! Currently this library is geared toward use with [Scribble](https://www.scribble.org/)
//! and [New Scribble](https://nuscr.github.io/nuscr/) for full checking of protocols.
//!
//! <br>
//!
//! # Example
//!
//! Assume a simple protocol involving 3 participants, **A**, **B** and **C**.
//! **A** sends a payload to **B**, then receives another payload from **C**.
//! Upon receiving the payload from **A**, **B** sends a payload to **C**.
//! This protocol can be written as **A!B.A?C.B!C.0**.
//! To implement this example, first, get the right components from the library.
//!
//! Here is the full working example, detailed afterwards:
//!
//! ```
//! // Used for the functions that will process the protocol
//! use std::boxed::Box;
//! use std::error::Error;
//!
//! // Used for creating the types
//! use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
//! use mpstthree::meshedchannels::MeshedChannels;
//!
//! // Used for creating the stack and the name of each role
//! use mpstthree::role::a::RoleA;
//! use mpstthree::role::b::RoleB;
//! use mpstthree::role::c::RoleC;
//! use mpstthree::role::end::RoleEnd;
//!
//! use mpstthree::name::a::NameA;
//! use mpstthree::name::b::NameB;
//! use mpstthree::name::c::NameC;
//!
//! // Used inside the functions which process the protocol for receiving one payload
//! use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
//! use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
//! use mpstthree::functionmpst::recv::recv_mpst_c_from_b;
//!
//! // Used inside the functions which process the protocol for sending one payload
//! use mpstthree::functionmpst::send::send_mpst_a_to_b;
//! use mpstthree::functionmpst::send::send_mpst_b_to_c;
//! use mpstthree::functionmpst::send::send_mpst_c_to_a;
//!
//! // Used inside the functions which process the protocol for closing the connexion
//! use mpstthree::functionmpst::close::close_mpst;
//!
//! // Used for connecting all the roles, represented as MeshedChannels, together
//! use mpstthree::functionmpst::fork::fork_mpst;
//!
//! // Creating the binary sessions
//! // for A
//! type AtoB<N> = Send<N, End>;
//! type AtoC<N> = Recv<N, End>;
//!
//! // for B
//! type BtoA<N> = Recv<N, End>;
//! type BtoC<N> = Send<N, End>;
//!
//! // for C
//! type CtoA<N> = Send<N, End>;
//! type CtoB<N> = Recv<N, End>;
//!
//! // Stacks
//! // for A
//! type StackA = RoleB<RoleC<RoleEnd>>;
//! // for B
//! type StackB = RoleA<RoleC<RoleEnd>>;
//! // for C
//! type StackC = RoleA<RoleB<RoleEnd>>;
//!
//! // Creating the MP sessions
//! // for A
//! type EndpointA<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, NameA>;
//! // for B
//! type EndpointB<N> = MeshedChannels<BtoA<N>, BtoC<N>, StackB, NameB>;
//! // for C
//! type EndpointC<N> = MeshedChannels<CtoA<N>, CtoB<N>, StackC, NameC>;
//!
//! // Function to process Endpoint of A
//! fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
//!     let s = send_mpst_a_to_b(1, s);
//!     let (_x, s) = recv_mpst_a_from_c(s)?;
//!
//!     close_mpst(s)
//! }

//!
//! // Function to process Endpoint of B
//! fn endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
//!     let (_x, s) = recv_mpst_b_from_a(s)?;
//!     let s = send_mpst_b_to_c(2, s);
//!
//!     close_mpst(s)
//! }

//!
//! // Function to process Endpoint of C
//! fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
//!     let s = send_mpst_c_to_a(3, s);
//!     let (_x, s) = recv_mpst_c_from_b(s)?;
//!
//!     close_mpst(s)
//! }

//!
//! // Fork all endpoints
//! fn main() {
//!     let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);
//!
//!     thread_a.join().unwrap();
//!     thread_b.join().unwrap();
//!     thread_c.join().unwrap();
//! }

//! ```
//!
//! Here is the full description. First, you import the correct functions and types.
//! ```ignore
//! // Used for the functions that will process the protocol
//! use std::boxed::Box;
//! use std::error::Error;
//!
//! // Used for creating the types
//! use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
//! use mpstthree::meshedchannels::MeshedChannels;
//!
//! // Used for creating the stack and the name of each role
//! use mpstthree::role::a::RoleA;
//! use mpstthree::role::b::RoleB;
//! use mpstthree::role::c::RoleC;
//! use mpstthree::role::end::RoleEnd;
//!
//! // Used inside the functions which process the protocol for receiving one payload
//! use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
//! use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
//! use mpstthree::functionmpst::recv::recv_mpst_c_from_b;
//!
//! // Used inside the functions which process the protocol for sending one payload
//! use mpstthree::functionmpst::send::send_mpst_a_to_b;
//! use mpstthree::functionmpst::send::send_mpst_b_to_c;
//! use mpstthree::functionmpst::send::send_mpst_c_to_a;
//!
//! // Used inside the functions which process the protocol for closing the connexion
//! use mpstthree::functionmpst::close::close_mpst;
//!
//! // Used for connecting all the roles, represented as MeshedChannels, together
//! use mpstthree::functionmpst::fork::fork_mpst;
//! ```
//!
//! Then, you have to create the **binary session types** defining the interactions for each pair of
//! participants. Note that each created type can be reused as many time as needed.
//! For our example, we create several times the same binary session type for clarity,
//! but we could use only two of those types for the whole protocol instead.
//! ```ignore
//! // Creating the binary sessions
//! // for A
//! type AtoB<N> = Send<N, End>;
//! type AtoC<N> = Recv<N, End>;
//!
//! // for B
//! type BtoA<N> = Recv<N, End>;
//! type BtoC<N> = Send<N, End>;
//!
//! // for C
//! type CtoA<N> = Send<N, End>;
//! type CtoB<N> = Recv<N, End>;
//! ```
//!
//! Add the **stacks** which give the correct order of the operations for each participant.
//! ```ignore
//! // Stacks
//! // for A
//! type StackA = RoleB<RoleC<RoleEnd>>;
//! // for B
//! type StackB = RoleA<RoleC<RoleEnd>>;
//! // for C
//! type StackC = RoleA<RoleB<RoleEnd>>;
//! ```
//!
//! You can now encapsulate those **binary session types** and **stacks** into **MeshedChannels**
//! for each participant. We also add the names of the related roles.
//! ```ignore
//! // Creating the MP sessions
//! // for A
//! type EndpointA<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, NameA>;
//! // for B
//! type EndpointB<N> = MeshedChannels<BtoA<N>, BtoC<N>, StackB, NameB>;
//! // for C
//! type EndpointC<N> = MeshedChannels<CtoA<N>, CtoB<N>, StackC, NameC>;
//! ```
//!
//! To run the protocol,
//! we need to detail the behaviour of the participants with functions that input the **Endpoints**
//! defined above.
//! ```ignore
//! // Function to process Endpoint of A
//! fn simple_triple_endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
//!     let s = send_mpst_a_to_b(1, s);
//!     let (x, s) = recv_mpst_a_from_c(s)?;
//!
//!     close_mpst(s)
//! }

//!
//! // Function to process Endpoint of B
//! fn simple_triple_endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
//!     let (x, s) = recv_mpst_b_from_a(s)?;
//!     let s = send_mpst_b_to_c(2, s);
//!
//!     close_mpst(s)
//! }

//!
//! // Function to process Endpoint of C
//! fn simple_triple_endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
//!     let s = send_mpst_c_to_a(3, s);
//!     let (x, s) = recv_mpst_c_from_b(s)?;
//!
//!     close_mpst(s)
//! }

//! ```
//!
//! In the end, you have to link/fork the threads,
//! related to the functions above, together with **fork_mpst()**.
//! Do not forget to **unwrap()** the returned threads.
//! ```ignore
//! // Fork all endpoints
//! fn main() {
//!     let (thread_a, thread_b, thread_c) = fork_mpst(
//!         endpoint_a,
//!         endpoint_b,
//!         endpoint_c,
//!     );
//!
//!     thread_a.join().unwrap();
//!     thread_b.join().unwrap();
//!     thread_c.join().unwrap();
//! }

//! ```
//!
//! The different features available are:
//!
//! 1. `default`: default features, for implementing the basic example above.
//! 2. `macros_simple`: feature for implementing protocols with three participants, whatever are
//! their name. 3. `macros_multiple`: feature for implementing protocols with any number of
//! participants. Contains `macros_simple`. 4. `baking`: feature for implementing protocols with any
//! number of participants and using associated functions instead of functions. Contains
//! `macros_multiple`. 5. `transport_tcp`: feature containing primitives for communicating with TCP.
//! **Requires `openssl`, `pkg-config` and `libssl-dev` installed on your machine**.
//! 6. `transport_udp`: feature containing primitives for communicating with UDP. **Requires
//! `openssl`, `pkg-config` and `libssl-dev` installed on your machine**. 7. `transport_http`:
//! feature containing primitives for communicating with HTTP/HTTPS. **Requires `openssl`,
//! `pkg-config` and `libssl-dev` installed on your machine**. 8. `transport`: feature containing
//! `transport_tcp`, `transport_udp` and `transport_http`. 9. `checking`: feature for the top-down
//! approach. Needs the [`KMC`] tool. 10. `full`: feature containing `checking`, `baking` and
//! `transport`.
//!
//! [`KMC`]: https://github.com/julien-lange/kmc

#[cfg(feature = "default")]
pub mod binary;

#[cfg(feature = "default")]
pub mod message;

#[cfg(feature = "mpst")]
pub mod role;

#[cfg(feature = "mpst")]
pub mod name;

#[cfg(feature = "mpst")]
pub mod meshedchannels;

#[cfg(feature = "mpst")]
pub mod functionmpst;

#[cfg(feature = "checking")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "checking")))]
pub mod checking;

#[cfg(feature = "macros_simple")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_simple")))]
pub mod macros_simple;

#[cfg(feature = "macros_multiple")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros_multiple")))]
pub mod macros_multiple;

#[cfg(feature = "baking")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "baking")))]
pub mod baking;

#[cfg(feature = "interleaved")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "interleaved")))]
pub mod interleaved;

#[cfg(feature = "binary_timed")]
pub mod binary_timed;

#[cfg(feature = "baking_timed")]
pub mod baking_timed;

#[cfg(feature = "transport")]
pub mod transport;

#[cfg(feature = "protobuf")]
pub mod protobuf;

pub mod attempt;
