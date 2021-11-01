// This test is similar to the previous where want to ensure that the macro
// correctly generates an error when the input enum is out of order, but this
// time it is using an enum that also has data associated with each variant.

use mpst_seq_proc::branching;

use std::env::VarError;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::str::Utf8Error;

#[branching]
pub enum Error {
    Dyn(Box<dyn StdError>),
    Fmt(fmt::Error),
    Io(io::Error),
    Utf8(Utf8Error),
    Var(VarError),
}

fn main() {}
