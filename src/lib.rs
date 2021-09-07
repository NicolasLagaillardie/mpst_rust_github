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
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Mpstthree (also called mp-anon) is a library to
//! write and check communication protocols based on [Multiparty Session Types](https://mrg.doc.ic.ac.uk/publications/a-very-gentle-introduction-to-multiparty-session-types/).
//!
//! Currently this library is geared toward use with [Scribble](https://www.scribble.org/)
//! and [New Scribble](https://nuscr.github.io/nuscr/) for full checking of protocols.
//!
//! <br>

#[cfg(feature = "binary")]
pub mod binary;

#[cfg(feature = "role")]
pub mod role;

#[cfg(feature = "meshedchannels")]
pub mod meshedchannels;

#[cfg(feature = "functionmpst")]
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

#[cfg(feature = "transport")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "transport")))]
pub mod transport;
