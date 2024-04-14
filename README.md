# Multiparty session types for Rust

![Ubuntu](https://github.com/NicolasLagaillardie/mpst_rust_github/actions/workflows/ubuntu.yml/badge.svg)
![Windows](https://github.com/NicolasLagaillardie/mpst_rust_github/actions/workflows/windows.yml/badge.svg)
![Mac](https://github.com/NicolasLagaillardie/mpst_rust_github/actions/workflows/mac.yml/badge.svg)
[![Crate](https://img.shields.io/crates/v/mpstthree.svg)](https://crates.io/crates/mpstthree)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.77+-brightgreen.svg)](https://github.com/NicolasLagaillardie/mpst_rust_github)
[![Documentation](https://docs.rs/mpstthree/badge.svg)](https://docs.rs/mpstthree/)
[![codecov](https://codecov.io/gh/NicolasLagaillardie/mpst_rust_github/branch/master/graph/badge.svg?token=VEUNVJJAOY)](https://codecov.io/gh/NicolasLagaillardie/mpst_rust_github)
[![Dependency status](https://deps.rs/repo/github/NicolasLagaillardie/mpst_rust_github/status.svg)](https://deps.rs/repo/github/NicolasLagaillardie/mpst_rust_github)

This library implements [multiparty session types](http://mrg.doc.ic.ac.uk/publications/a-gentle-introduction-to-multiparty-asynchronous-session-types/) in Rust for at least two participants.
It relies on [sesh](https://github.com/wenkokke/sesh).

A short video presentation of the library can be found here: [https://youtu.be/ej1FetN31HE](https://youtu.be/ej1FetN31HE).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mpstthree = "0.1.17"
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

// Used for creating the stack of each role
use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

// Importing the names of the participants
use mpstthree::name::a::NameA;
use mpstthree::name::b::NameB;
use mpstthree::name::c::NameC;

// Used for connecting all the roles, represented as MeshedChannels, together
use mpstthree::functionmpst::fork::fork_mpst;
```

Then, you have to create the **binary session types** defining the interactions for each pair of participants.
Note that each created type can be reused as many times as needed.
For our example, we create several times the same binary session type for clarity,
but we could use only two of those types for the whole protocol instead.

```rust
// Creating the binary sessions
// for A
type AtoB = Send<i32, End>;
type AtoC = Recv<i32, End>;

// for B
type BtoA = Recv<i32, End>;
type BtoC = Send<i32, End>;

// for C
type CtoA = Send<i32, End>;
type CtoB = Recv<i32, End>;
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
type EndpointA = MeshedChannels<AtoB, AtoC, StackA, NameA>;
// for B
type EndpointB = MeshedChannels<BtoA, BtoC, StackB, NameB>;
// for C
type EndpointC = MeshedChannels<CtoA, CtoB, StackC, NameC>;
```

To run the protocol,
we need to detail the behaviour of the participants with functions that input the **Endpoints** defined above.

```rust
// Function to process Endpoint of A
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let s = s.send(1);
    let (_x, s) = s.recv()?;

    s.close()
}

// Function to process Endpoint of B
fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    let (_x, s) = s.recv()?;
    let s = s.send(2);

    s.close()
}

// Function to process Endpoint of C
fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = s.send(3);
    let (_x, s) = s.recv()?;

    s.close()
}
```

In the end, you have to link/fork the threads,
related to the functions above, together with **fork_mpst()**.
Do not forget to **unwrap()** the returned threads.

```rust
// Fork all endpoints
fn main() {
    let (thread_a, thread_b, thread_c) = fork_mpst(endpoint_a, endpoint_b, endpoint_c);

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}
```

### Running this example

For running this example, assuming it is in the **examples/** folder, use:

```sh
cargo run --example [name of your example] --features="mpst"
```

where `--features="mpst"` is used for building the `mpst` feature of this library, which includes the `MeshedChannels` and `role`s types, among other things.

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
cargo test
```

### Running

For running an example [XXX] of the library, run this code.

```sh
cargo run --example [XXX]
```

Depending on the example you would like to run,
you may have to modify the previous command line into:

```sh
cargo run --example [XXX] --features="[YYY]"
```

where [YYY] is one or more of the features
provided in [Available features](#Available-features) hereafter.

## Going further

With this library, one can write any protocol with at least two participants and using methods to shorten the writing and checking.
You can check the tests and examples to have a larger overview of the different possibilities provided by this library.

## Available features

The different features available are:

0. `default`: default features, for implementing the basic example above.
1. `message`: feature for using the _message_ structure provided by the library.
2. `macros_simple`: feature for implementing protocols with three participants, whatever are their name.
3. `macros_multiple`: feature for implementing protocols with any number of participants. Contains `macros_simple`.
4. `baking`: feature for implementing protocols with any number of participants and using associated functions instead of functions. Contains `macros_multiple`.
5. `baking_timed`: feature for implementing _asynchronous_ _timed_ protocols with any number of participants and using associated functions instead of functions.
6. `transport_tcp`: feature containing primitives for communicating with TCP. **Requires `openssl`, `pkg-config` and `libssl-dev` installed on your machine**.
7. `transport_udp`: feature containing primitives for communicating with UDP. **Requires `openssl`, `pkg-config` and `libssl-dev` installed on your machine**.
8. `transport_http`: feature containing primitives for communicating with HTTP/HTTPS. **Requires `openssl`, `pkg-config` and `libssl-dev` installed on your machine**.
9. `transport`: feature containing `transport_tcp`, `transport_udp` and `transport_http`.
10. `checking`: feature for the top-down approach. Needs the [`KMC`] tool.
11. `full`: feature containing `checking`, `baking` and `transport`.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning.

## Authors

* **Nicolas Lagaillardie** - *Initial work* - [Nicolas Lagaillardie](https://github.com/NicolasLagaillardie)
* **Rumyana Neykova** - *Initial work* - [Rumyana Neykova](https://github.com/rumineykova)
* **Nobuko Yoshida** - *Initial work* - [Nobuko Yoshida](https://github.com/NobukoYoshida)

See also the list of [contributors](https://github.com/NicolasLagaillardie/mpst_rust_github/graphs/contributors) who participated in this project.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.

## Acknowledgment

This project is part of my current PhD under the supervision of [Nobuko Yoshida](https://www.imperial.ac.uk/people/n.yoshida), that I would like to thank.
I was also helped by my [colleagues](http://mrg.doc.ic.ac.uk/people/) from [Imperial College London](https://www.imperial.ac.uk/).
