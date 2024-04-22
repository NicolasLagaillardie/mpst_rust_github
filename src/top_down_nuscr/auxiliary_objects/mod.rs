#![cfg(feature = "top_down_nuscr")]

//! This module is used for the
//! auxiliary functions for the generator
//! main function.

use std::collections::{HashMap, HashSet};
use std::fs::File;

pub(crate) struct GlobalElements {
    pub(crate) output_path: String,
    // Init the output variable, should contain the output file in the end
    pub(crate) output: Option<File>,
    // Basic elements
    pub(crate) roles: Vec<String>,
    pub(crate) payloads: HashSet<String>,
    pub(crate) clocks: HashMap<String, HashSet<String>>,
    // Count loops
    pub(crate) loops: Vec<String>,
    // Brackets
    pub(crate) opening_brackets: usize,
    pub(crate) closing_brackets: usize,
}

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

pub(crate) struct Tree {
    pub(crate) index: Vec<i32>,
    pub(crate) message_with_payloads: HashMap<String, String>,
    pub(crate) messages: HashMap<String, HashMap<String, Vec<String>>>,
    pub(crate) first_message: HashMap<String, HashMap<String, String>>,
    pub(crate) previous_message_wrt_clocks:
        HashMap<String, HashMap<String, (String, String, String, String)>>,
    pub(crate) last_message: HashMap<String, HashMap<String, String>>,
    pub(crate) stacks: HashMap<String, Vec<String>>,
    pub(crate) first_stack: HashMap<String, String>,
    pub(crate) last_stack: HashMap<String, String>,
    pub(crate) enums: HashMap<String, (String, i32)>,
    pub(crate) endpoints: HashMap<String, Vec<String>>,
    pub(crate) sub_trees: Vec<Tree>,
}

pub(crate) mod code_generate;
pub(crate) mod line_is_message;
pub(crate) mod messages_and_stacks_update;
pub(crate) mod process_line;
pub(crate) mod regex;
