# Multiparty session types for Rust

[![Build Status](https://travis-ci.org/NicolasLagaillardie/mpst_rust_github.svg?branch=master)](https://travis-ci.com/NicolasLagaillardie/mpst_rust_github)

This library implements [multiparty session types](http://mrg.doc.ic.ac.uk/publications/a-gentle-introduction-to-multiparty-asynchronous-session-types/) in Rust for three participants. It relies on [sesh](https://github.com/wenkokke/sesh). An other library is coming soon to extend to any number of participants.

## Getting started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

You need to have [Rust](https://www.rust-lang.org/). You should get `cargo` installed. The project has been coded using version `cargo 1.41.0`, and may not work with other versions.

### Building

For building the library, run this code.

```sh
cargo build
```

### Running

For running the library, run this code.

```sh
cargo run
```

### Run test

For running the tests, run this code.

```sh
cargo test
```

Tests are divided in 4 files:

* [simple](tests/simple.rs) checks that a basic global protocol works  and is well implemented. In this test, a role A sends a payoad to a role B, then receives an other from a role C. Upon receiving the payload from A, B sends a payload to C. This protocol can be written as **A!B.A?C.B!C**.
* [choose](tests/choose.rs) checks that a protocol where a role B spreads a choice to the two other roles. For simplifying the test, role C is doing nothing. The protocol can be written as **B→A:{B!A, B!A}**.
* [usecase](test/usecase.rs) is implementing the protocol given in [1](.github/pdf/GPS.pdf), where **Client → C**, **Authenticator → A** and **Server → B**.
* [usecase-recursive](test/usecase-recursive.rs) is implementing the protocol given in [2](.github/pdf/GPR.pdf), where **Client → C**, **Authenticator → A** and **Server → B**.

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning.

## Authors

* **Nicolas Lagaillardie** - *Initial work* - [NicolasLagaillardie](https://github.com/NicolasLagaillardie)
* **Rumyana Neykova** - *Initial work* - [rumineykova](https://github.com/rumineykova)

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details