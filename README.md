# Multiparty session types for Rust

[![Build Status](https://travis-ci.com/NicolasLagaillardie/mpst_rust_github.svg?token=svBAgWJGqmCpdC4i1kLT&branch=master)](https://travis-ci.com/NicolasLagaillardie/mpst_rust_github)
[![Crate](https://img.shields.io/crates/v/mpstthree.svg)](https://crates.io/crates/mpstthree)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.41+-brightgreen.svg)](https://github.com/NicolasLagaillardie/mpst_rust_github)
[![License: MIT](https://img.shields.io/crates/l/mpstthree.svg)](#license)


This library implements [multiparty session types](http://mrg.doc.ic.ac.uk/publications/a-gentle-introduction-to-multiparty-asynchronous-session-types/) in Rust for three participants. It relies on [sesh](https://github.com/wenkokke/sesh).
An other library is coming soon to extend to any number of participants.

A short video presentation of the library can be found here: [https://youtu.be/ej1FetN31HE](https://youtu.be/ej1FetN31HE).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mpstthree = "0.0.1"
```

## Example

Here a way to create a simple protocol involving 3 participants, **A**, **B** and **C**. **A** sends a payoad to **B**, then receives an other from **C**. Upon receiving the payload from **A**, **B** sends a payload to **C**. This protocol can be written as **A!B.A?C.B!C**. 
To implement this example, first, get the right components from the library.

```rust
extern crate mpstthree;

use std::boxed::Box;
use std::error::Error;

use mpstthree::binary::{End, Recv, Send};
use mpstthree::run_processes;
use mpstthree::sessionmpst::SessionMpst;

use mpstthree::functionmpst::close::close_mpst;

use mpstthree::role::a_to_b::RoleAtoB;
use mpstthree::role::a_to_c::RoleAtoC;
use mpstthree::role::b_to_a::RoleBtoA;
use mpstthree::role::b_to_c::RoleBtoC;
use mpstthree::role::c_to_a::RoleCtoA;
use mpstthree::role::c_to_b::RoleCtoB;
use mpstthree::role::end::RoleEnd;

use mpstthree::functionmpst::recv::recv_mpst_a_to_c;
use mpstthree::functionmpst::recv::recv_mpst_b_to_a;
use mpstthree::functionmpst::recv::recv_mpst_c_to_b;

use mpstthree::functionmpst::send::send_mpst_a_to_b;
use mpstthree::functionmpst::send::send_mpst_b_to_c;
use mpstthree::functionmpst::send::send_mpst_c_to_a;
```

Then, you have to create the **binary session types** defining the interactions for each pair of participants.

```rust
/// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

type BtoA<N> = Recv<N, End>;
type BtoC<N> = Send<N, End>;

type CtoA<N> = Send<N, End>;
type CtoB<N> = Recv<N, End>;
```

Add the **queues**, which give the correct order of the operations for each participants.

```rust
/// Queues
type QueueA = RoleAtoB<RoleAtoC<RoleEnd>>;
type QueueB = RoleBtoA<RoleBtoC<RoleEnd>>;
type QueueC = RoleCtoA<RoleCtoB<RoleEnd>>;
```

You can now encapsulate those **binary session types** and **queues** into **SessionMpst** for each participant.

```rust
/// Creating the MP sessions
type EndpointA<N> = SessionMpst<AtoB<N>, AtoC<N>, QueueA>;
type EndpointB<N> = SessionMpst<BtoA<N>, BtoC<N>, QueueB>;
type EndpointC<N> = SessionMpst<CtoA<N>, CtoB<N>, QueueC>;
```

To check to the protocol is *correct*, it is  mandatory to detail the behaviour of the participants with functions which input the **Endpoints** defined above.

```rust
/// Endpoint for A
fn simple_triple_endpoint_a(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_b(1, s);
    let (x, s) = recv_mpst_a_to_c(s)?;

    close_mpst(s)?;

    Ok(())
}

/// Endpoint for B
fn simple_triple_endpoint_b(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    let (x, s) = recv_mpst_b_to_a(s)?;
    let s = send_mpst_b_to_c(2, s);

    close_mpst(s)?;

    Ok(())
}

/// Endpoint for C
fn simple_triple_endpoint_c(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_c_to_a(3, s);
    let (x, s) = recv_mpst_c_to_b(s)?;

    close_mpst(s)?;

    Ok(())
}
```

In the end, you have to link the threads, related to the functions above, together with **run_processes()**. Do not forget to **unwrap()** the returned threads.

```rust
/// Fork all endpoints
fn simple_triple_endpoints() {
    let (thread_a, thread_b, thread_c) = run_processes(
        simple_triple_endpoint_a,
        simple_triple_endpoint_b,
        simple_triple_endpoint_c,
    );

    thread_a.unwrap();
    thread_b.unwrap();
    thread_c.unwrap();
}
```

## Getting started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You need to have [Rust](https://www.rust-lang.org/). You should get `cargo` installed.

### Building

For building the library, run this code.

```sh
$ cargo build
```

### Running

For running the library, run this code.

```sh
$ cargo run
```

### Run test

For running the tests, run this code.

```sh
$ cargo test
```

Tests are divided in 4 files:

* [simple](tests/simple.rs) is the basic global protocol shown in [Examples](#Example).
* [choose](tests/choose.rs) checks that a protocol where a role **B** spreads a choice to the two other roles. For simplifying the test, role **C** is doing nothing. The protocol can be written as **B→A:{B!A, B!A}**.
* [usecase](test/usecase.rs) is implementing the protocol given in [1](.github/pdf/GPS.pdf), where **Client → C**, **Authenticator → A** and **Server → B**.
* [usecase-recursive](test/usecase-recursive.rs) is implementing the protocol given in [2](.github/pdf/GPR.pdf), where **Client → C**, **Authenticator → A** and **Server → B**.

### Going further

This subsection explains more complex and diverse features of the library.

#### Parametrisation on the names of the roles

This part details how to create new roles and how to use them.

##### Creation of new roles

Instead of being limited by roles `RoleAtoB`, `RoleBtoC` and so on, you can now create your own roles. To achieve this, you need to use the macros  `create_normal_role` and `create_broadcast_role`, respectively for binary types and broadcasted ones. Example of use can be found in the [macro-basic](tests/macro-basics.rs). Those macros take, as parameters and in the order, the name of the role, the name of the `next`function to go through the stack, the name of the *dual* of this role and the name of the `next` function for this dual. For instance, let's create the role `RoleAtoD`. The expected code will be:

```rust
create_normal_role!(RoleAtoD, next_a_to_d, RoleDtoA, next_d_to_a);
```

##### Sending and receiving with those new roles

To create the role `RoleAtoD`, you need the related `next` function, that can be named `next_a_to_d`, to go through a stack which head is `RoleAtoD`, such as `RoleAtoD<RoleEnd>`. The *dual* of `RoleAtoD`is `RoleDtoA`and the related `next` function, that can be named `next_d_to_a`.

To *send* and *receive* with those new roles, it is mandatory to define new `send` and `recv` functions. This can easily be done with the macros `create_send_mpst_session_1`, `create_send_mpst_session_2`, `create_recv_mpst_session_1` and `create_recv_mpst_session_2`.
As you may notice, there is a difference made between `session_1` and `session_2`. This is due to the current limitation of the library: this is for making the difference between the binary channels used during the communication. If `A` sends to `B`, it will send on the first channel, and by convention (alphanumerical order), it will be the first binary channel, hence `create_send_mpst_session_1` will be used. If `A` send to `C` and `B` is among the participants, then `create_send_mpst_session_2` will be used.
The macros `create_send_mpst_session_1`, `create_send_mpst_session_2`, `create_recv_mpst_session_1` and `create_recv_mpst_session_2` expect the same inputs: the name of the new function created and the names of the role and the related `next`function. To create the `send` function from `A` to `B`, here is the expected line of code: 

```rust
create_send_mpst_session_1!(send_mpst_a_to_b, RoleAtoB, next_a_to_b);
```

##### Making choice and offer with those new roles

To add a layer of features, one may expect to implement `choice` and `offer`. There are two different kind of branching: *binary* and *multiple*. The former refers to a branching with only two choices, whereas the latter refers to branching with as many choices as wanted.
For the *binary branching*, the macros `create_offer_mpst_session_1` and `create_offer_mpst_session_2` for offer, and `create_choose_left_from_X_to_Y_and_Z` (where X, Y and Z are numbers linked to the roles) are used. The inputs are the name of the new `offer`( respectively `choose`) functions and the names of the role and the related `next` function. For instance, to create an *offer* function for role `B` to receive from role `C`, here is an example of code: 

```rust
create_recv_mpst_all_session_2!(recv_mpst_b_all_to_c, RoleAlltoC, next_all_to_c);
```

On the opposite side, to create a *choice* from role `C` to the other roles, where `C` will choose the left choice, here is the expected code.

```rust
create_choose_left_from_3_to_1_and_2!(choose_left_mpst_session_c_to_all, RoleCtoAll, next_c_to_all);
```

To compare the traditional and the more complex methods, you can check the [usecse](tests/usecase.rs) and [macro-choice](tests/macro-choice.rs) files

For the *multipke branching*, instead of creating new functions, the macro `offer_mpst` and `choose_mpst_to_all` can be used directly. The `offer_mpst` macro expects a session, the name of the `recv` function used and the branches for the matching. The `choose_mpst_to_all` macro expects the path to the different choices, the session and the `send` functions used for sending the choice. A comparison can be made between the files [usecase-recursive](tests/usecase-recursive.rs) and [macro-recursive](test/macro-recursive.rs), which are respectively the traditional methode and the more complex method.

#### Parametrisation on the  number of roles

This part details how to create create protocols many multiple roles. This is still a work in progress.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning.

## Authors

* **Nicolas Lagaillardie** - *Initial work* - [NicolasLagaillardie](https://github.com/NicolasLagaillardie)
* **Rumyana Neykova** - *Initial work* - [rumineykova](https://github.com/rumineykova)
* **Nobuko Yoshida** - *Initial work* - [NobukoYoshida](https://github.com/NobukoYoshida)

See also the list of [contributors](https://github.com/NicolasLagaillardie/mpst_rust_github/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgment

This project is part of my current PhD under the supervision of [Nobuko Yoshida](https://www.imperial.ac.uk/people/n.yoshida), that I would like to thank.
I was also helped by my [colleagues](http://mrg.doc.ic.ac.uk/people/) from [Imperial College London](https://www.imperial.ac.uk/).
