#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(or_patterns)]
#![feature(rustc_private)]
#![feature(stmt_expr_attributes)]
#![allow(clippy::missing_docs_in_private_items, clippy::must_use_candidate)]
#![recursion_limit = "512"]
#![warn(rust_2018_idioms, trivial_casts, trivial_numeric_casts)]
#![deny(rustc::internal)]
#![cfg_attr(feature = "deny-warnings", deny(warnings))]
#![feature(crate_visibility_modifier)]
#![feature(concat_idents)]

// FIXME: switch to something more ergonomic here, once available.
// (Currently there is no way to opt into sysroot crates without `extern crate`.)
#[allow(unused_extern_crates)]
extern crate fmt_macros;
#[allow(unused_extern_crates)]
extern crate rustc_ast;
#[allow(unused_extern_crates)]
extern crate rustc_ast_pretty;
#[allow(unused_extern_crates)]
extern crate rustc_attr;
#[allow(unused_extern_crates)]
extern crate rustc_data_structures;
#[allow(unused_extern_crates)]
extern crate rustc_driver;
#[allow(unused_extern_crates)]
extern crate rustc_errors;
#[allow(unused_extern_crates)]
extern crate rustc_hir;
#[allow(unused_extern_crates)]
extern crate rustc_hir_pretty;
#[allow(unused_extern_crates)]
extern crate rustc_index;
#[allow(unused_extern_crates)]
extern crate rustc_infer;
#[allow(unused_extern_crates)]
extern crate rustc_lexer;
#[allow(unused_extern_crates)]
extern crate rustc_lint;
#[allow(unused_extern_crates)]
extern crate rustc_middle;
#[allow(unused_extern_crates)]
extern crate rustc_mir;
#[allow(unused_extern_crates)]
extern crate rustc_parse;
#[allow(unused_extern_crates)]
extern crate rustc_session;
#[allow(unused_extern_crates)]
extern crate rustc_span;
#[allow(unused_extern_crates)]
extern crate rustc_target;
#[allow(unused_extern_crates)]
extern crate rustc_trait_selection;
#[allow(unused_extern_crates)]
extern crate rustc_typeck;

use rustc_data_structures::fx::FxHashSet;
use rustc_lint::LintId;
use rustc_session::Session;

pub use crate::utils::conf::Conf;