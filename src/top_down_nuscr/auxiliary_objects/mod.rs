#![cfg(feature = "top_down_nuscr")]

//! This module is used for the
//! auxiliary functions for the generator
//! main function.

pub(crate) struct MessageParameters {
    pub(crate) sender: String,
    pub(crate) receiver: String,
    pub(crate) message: String,
    pub(crate) clock: String,
    pub(crate) left_bound: String,
    pub(crate) left_bracket: String,
    pub(crate) right_bound: String,
    pub(crate) right_bracket: String,
    pub(crate) reset: String,
}

pub(crate) mod check_brackets;
pub(crate) mod code_generate;
pub(crate) mod messages_and_stacks_update;
pub(crate) mod regex;
