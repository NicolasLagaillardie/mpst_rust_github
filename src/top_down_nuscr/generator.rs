//! This module is focused on the generation
//! of Rust protocols from *nuscr* protocols.
//! It is **NOT** a parser,
//! it needs to have a correctly formatted
//! *nuscr* protocol.

use std::fs::File;

use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

use super::auxiliary_objects::{
    code_generate::*, process_line::process_line, GlobalElements, Tree,
};
use super::generate_regular_protocol::check_fsm;

/// Generate endpoints from a nuscr file
/// with timed global protocol
pub fn generator(
    filepath: &str,
    output_path: &str,
    nuscr_check: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Global elements
    let mut global_elements = GlobalElements {
        output_path: output_path.to_string(),
        output: None,
        roles: vec![],
        payloads: HashSet::new(),
        clocks: HashMap::new(),
        loops: HashMap::new(),
        opening_brackets: 0,
        closing_brackets: 0,
    };

    // Running elements
    let mut main_tree = Tree {
        index: vec![0],
        messages_with_payloads: HashMap::new(),
        messages: HashMap::new(),
        first_message: HashMap::new(),
        previous_message_wrt_clocks: HashMap::new(),
        last_message: HashMap::new(),
        stacks: HashMap::new(),
        first_stack: HashMap::new(),
        last_stack: HashMap::new(),
        enums: HashMap::new(),
        endpoints: HashMap::new(),
        sub_trees: vec![],
    };

    if nuscr_check {
        check_fsm(filepath)?;
    }

    // Generate the Rust file
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut lines_iter = reader
        .lines()
        .enumerate()
        .map(|(line_number, line)| (line_number, line.unwrap()));

    process_line(
        &mut lines_iter,
        &mut global_elements,
        &mut Tree {
            index: vec![],
            messages_with_payloads: HashMap::new(),
            messages: HashMap::new(),
            first_message: HashMap::new(),
            last_message: HashMap::new(),
            stacks: HashMap::new(),
            first_stack: HashMap::new(),
            previous_message_wrt_clocks: HashMap::new(),
            last_stack: HashMap::new(),
            enums: HashMap::new(),
            endpoints: HashMap::new(),
            sub_trees: vec![],
        },
        &mut main_tree,
        &mut 0,
    )?;

    // Check if number of opening and closing brackets are the same
    if global_elements.opening_brackets != global_elements.closing_brackets {
        return Err(
            "The number of opening and closing brackets is not the same in the end.".into(),
        );
    }

    // Generate everything
    imports::generate_imports(&mut global_elements, &main_tree)?;
    structs::generate_structs(&mut global_elements, &main_tree, &mut vec![], true)?;
    sessions::generate_sessions(&mut global_elements, &main_tree)?;
    stacks::generate_stacks(&mut global_elements, &main_tree)?;
    enums::generate_enums(&mut global_elements, &main_tree)?;
    endpoints::generate_endpoints(&mut global_elements, &main_tree)?;
    fn_endpoints::generate_fn_endpoints(&mut global_elements, &main_tree, true)?;
    fn_main::generate_fn_main(&mut global_elements)?;

    Ok(())
}
