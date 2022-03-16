# Multiparty session types for Rust

<!-- [![Build Status](https://travis-ci.com/NicolasLagaillardie/mpst_rust_github.svg?token=svBAgWJGqmCpdC4i1kLT&branch=master)](https://travis-ci.com/NicolasLagaillardie/mpst_rust_github) -->
![Ubuntu](https://github.com/NicolasLagaillardie/mpst_rust_github/actions/workflows/ubuntu.yml/badge.svg)
![Windows](https://github.com/NicolasLagaillardie/mpst_rust_github/actions/workflows/windows.yml/badge.svg)
![Mac](https://github.com/NicolasLagaillardie/mpst_rust_github/actions/workflows/mac.yml/badge.svg)
[![Crate](https://img.shields.io/crates/v/mpstthree.svg)](https://crates.io/crates/mpstthree)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.58+-brightgreen.svg)](https://github.com/NicolasLagaillardie/mpst_rust_github)
[![Documentation](https://docs.rs/mpstthree/badge.svg)](https://docs.rs/mpstthree/)
[![codecov](https://codecov.io/gh/NicolasLagaillardie/mpst_rust_github/branch/master/graph/badge.svg?token=VEUNVJJAOY)](https://codecov.io/gh/NicolasLagaillardie/mpst_rust_github)
[![dependency status](https://deps.rs/repo/github/NicolasLagaillardie/mpst_rust_github/status.svg)](https://deps.rs/repo/github/NicolasLagaillardie/mpst_rust_github)
<!-- [![License: "MIT OR Apache-2.0"](https://img.shields.io/crates/l/mpstthree.svg)](#license) -->

This library implements [multiparty session types](http://mrg.doc.ic.ac.uk/publications/a-gentle-introduction-to-multiparty-asynchronous-session-types/) in Rust for at least two participants.
It relies on [sesh](https://github.com/wenkokke/sesh).

A short video presentation of the library can be found here: [https://youtu.be/ej1FetN31HE](https://youtu.be/ej1FetN31HE).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mpstthree = "0.1.16"
```

## Example

Assume a simple protocol involving 3 participants, **A**, **B** and **C**.
**A** sends a payload to **B**, then receives another payload from **C**.
Upon receiving the payload from **A**, **B** sends a payload to **C**.
This protocol can be written as **A!B.A?C.B!C.0**.
To implement this example, first, get the right components from the library.

```rust
// Used for the functions that will process the protocol
use std::boxed::Box;
use std::error::Error;

// Used for creating the types
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::meshedchannels::MeshedChannels;

// Used for creating the stack and the name of each role
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

// Used inside the functions which process the protocol for receiving one payload
use mpstthree::functionmpst::recv::recv_mpst_a_from_c;
use mpstthree::functionmpst::recv::recv_mpst_b_from_a;
use mpstthree::functionmpst::recv::recv_mpst_c_from_b;

// Used inside the functions which process the protocol for sending one payload
use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_a;

// Used inside the functions which process the protocol for closing the connexion
use mpstthree::functionmpst::close::close_mpst;

// Used for connecting all the roles, represented as MeshedChannels, together
use mpstthree::functionmpst::fork::fork_mpst;
```

Then, you have to create the **binary session types** defining the interactions for each pair of participants.
Note that each created type can be reused as many time as needed.
For our example, we create several times the same binary session type for clarity,
but we could use only two of those types for the whole protocol instead.

```rust
// Creating the binary sessions
// for A
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

// for B
type BtoA<N> = Recv<N, End>;
type BtoC<N> = Send<N, End>;

// for C
type CtoA<N> = Send<N, End>;
type CtoB<N> = Recv<N, End>;
```

Add the **stacks** which give the correct order of the operations for each participant.

```rust
// Stacks
// for A
type StackA = RoleB<RoleC<RoleEnd>>;
// for B
type StackB = RoleA<RoleC<RoleEnd>>;
// for C
type StackC = RoleA<RoleB<RoleEnd>>;
```

You can now encapsulate those **binary session types** and **stacks** into **MeshedChannels** for each participant.
We also add the names of the related roles.

```rust
// Creating the MP sessions
// for A
type EndpointA<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, RoleA<RoleEnd>>;
// for B
type EndpointB<N> = MeshedChannels<BtoA<N>, BtoC<N>, StackB, RoleB<RoleEnd>>;
// for C
type EndpointC<N> = MeshedChannels<CtoA<N>, CtoB<N>, StackC, RoleC<RoleEnd>>;
```

To run the protocol,
we need to detail the behaviour of the participants with functions that input the **Endpoints** defined above.

```rust
// Function to process Endpoint of A
fn endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_b(1, s);
    let (x, s) = recv_mpst_a_from_c(s)?;

    close_mpst(s)
}

// Function to process Endpoint of B
fn endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    let (x, s) = recv_mpst_b_from_a(s)?;
    let s = send_mpst_b_to_c(2, s);

    close_mpst(s)
}

// Function to process Endpoint of C
fn endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_a(3, s);
    let (x, s) = recv_mpst_c_from_b(s)?;

    close_mpst(s)
}
```

In the end, you have to link/fork the threads,
related to the functions above, together with **fork_mpst()**.
Do not forget to **unwrap()** the returned threads.

```rust
// Fork all endpoints
fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        endpoint_a,
        endpoint_b,
        endpoint_c,
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}
```

## Getting started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You need to have [Rust](https://www.rust-lang.org/).
You will get `cargo` installed.

### Building

For building the library, run this code.

```sh
cargo build
```

### Run test

For running the tests, run this code.

```sh
cargo test --all-features
```

### Running

For running an example XXX of the library, run this code.

```sh
cargo run --example XXX --all-features
```

## Going further

With this library, one can write any protocol with at least two participants and using methods to shorten the writing and checking.
You can check the tests and examples to have a larger overview of the different possibilities provided by this library.

## Available features

The different features available are:

1. `default`: default features, for implementing the basic example above.
2. `macros_simple`: feature for implementing protocols with three participants, whatever are their name.
3. `macros_multiple`: feature for implementing protocols with any number of participants. Contains `macros_simple`.
4. `baking`: feature for implementing protocols with any number of participants and using associated functions instead of functions. Contains `macros_multiple`.
5. `transport_tcp`: feature containing primitives for communicating with TCP. **Requires `openssl`, `pkg-config` and `libssl-dev` installed on your machine**.
6. `transport_udp`: feature containing primitives for communicating with UDP. **Requires `openssl`, `pkg-config` and `libssl-dev` installed on your machine**.
7. `transport_http`: feature containing primitives for communicating with HTTP/HTTPS. **Requires `openssl`, `pkg-config` and `libssl-dev` installed on your machine**.
8. `transport`: feature containing `transport_tcp`, `transport_udp` and `transport_http`.
9. `checking`: feature for the top-down approach. Needs the [`KMC`] tool.
10. `full`: feature containing `checking`, `baking` and `transport`.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning.

## Authors

* **Nicolas Lagaillardie** - *Initial work* - [NicolasLagaillardie](https://github.com/NicolasLagaillardie)
* **Rumyana Neykova** - *Initial work* - [RumyanaNeykova](https://github.com/rumineykova)
* **Nobuko Yoshida** - *Initial work* - [NobukoYoshida](https://github.com/NobukoYoshida)

See also the list of [contributors](https://github.com/NicolasLagaillardie/mpst_rust_github/graphs/contributors) who participated in this project.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.

## Acknowledgment

This project is part of my current PhD under the supervision of [Nobuko Yoshida](https://www.imperial.ac.uk/people/n.yoshida), that I would like to thank.
I was also helped by my [colleagues](http://mrg.doc.ic.ac.uk/people/) from [Imperial College London](https://www.imperial.ac.uk/).
