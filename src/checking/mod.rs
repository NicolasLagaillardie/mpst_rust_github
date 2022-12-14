#![cfg(feature = "checking")]

//! This module contains the macros and the functions for
//! checking whether a protocol is well written or not,
//! according to a bottom-up method.
//!
//! *This module is available only if MultiCrusty is built with
//! the `"checking"` feature.*

use petgraph::Graph;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, remove_file, File};
use std::io::Write;
use std::process::Command;
use std::str;

#[doc(hidden)]
mod aux_checker;

use aux_checker::*;

type HashGraph = HashMap<String, Graph<String, String>>;

/// The macro that allows to create digraphs from each endpoint,
/// along with `enum` if needed. You can also provide the name of
/// a file for running the [`KMC`] tool and checking the
/// properties of the provided protocol: it will
/// return the minimal `k` according to this tool if it exists,
/// and ```None``` if `k` is bigger than 50 or does not exist.
///
/// The [`KMC`] tool
/// must be installed with `cabal install` and the resulting
/// binary must be added to PATH.
///
/// /!\ The provided types and enum cannot be checked if they contain
/// a parameter, such as <N>, as seen in some examples.
///
/// # Arguments
///
/// * \[Optional\] The name of the new file after running the [`KMC`] tool
/// * Each starting endpoint, separated by a comma
/// * \[Optional\] Each new `MeshedChannels` adopted by each sender of each choice, along with all
///   the different branches sent.
///
/// Currently, we do not support parameters for branches with `enum`
///
/// # Example
///
/// Assume that there are two choices (Branches0BtoA and Branches0CtoA), each one with two branches
/// (Video and End). Assume also that the starting Endpoints of the three roles are EndpointAFull,
/// EndpointCFull and EndpointBFull (the order does not matter). The call for this macro would be as
/// followed
///
/// ```ignore
/// mpstthree::checker_concat!(
///     EndpointAFull,
///     EndpointCFull,
///     EndpointBFull
///     =>
///     [
///         EndpointAVideo,
///         Branches0BtoA, Video,
///         Branches0CtoA, Video
///     ],
///     [
///         EndpointAEnd,
///         Branches0BtoA, End,
///         Branches0CtoA, End
///     ]
/// )
/// ```
///
/// [`KMC`]: https://github.com/julien-lange/kmc
///
/// *This macro is available only if MultiCrusty is built with
/// the `"checking"` feature.*
#[macro_export]
#[cfg_attr(doc_cfg, doc(cfg(feature = "checking")))]
macro_rules! checker_concat {
    (
        $(
            $sessiontype: ty
        ),+ $(,)?
    ) => {
        {
            mpstthree::checker_concat!(
                "",
                $(
                    $sessiontype,
                )+
            )
        }
    };
    (
        $name_file: expr,
        $(
            $sessiontype: ty
        ),+ $(,)?
    ) => {
        {
            let mut sessions = Vec::new();

            $(
                sessions.push(String::from(std::any::type_name::<$sessiontype>()));
            )+

            let state_branching_sessions = std::collections::hash_map::RandomState::new();
            let branching_sessions: std::collections::HashMap<String, String> =
                std::collections::HashMap::with_hasher(state_branching_sessions);

            let state_group_branches = std::collections::hash_map::RandomState::new();
            let group_branches: std::collections::HashMap<String, i32> =
                std::collections::HashMap::with_hasher(state_group_branches);

            let state_branches = std::collections::hash_map::RandomState::new();
            let branches_receivers: std::collections::HashMap<String, std::collections::HashMap<String, String>> =
                std::collections::HashMap::with_hasher(state_branches);

            mpstthree::checking::checker(
                $name_file,
                sessions,
                branches_receivers,
                branching_sessions,
                group_branches
            )
        }
    };
    (
        $(
            $sessiontype: ty
        ),+ $(,)?
        =>
        $(
            [
                $branch_stack: ty,
                $(
                    $choice: ty, $branch: ident
                ),+ $(,)?
            ]
        ),+ $(,)?
    ) => {
        {
            mpstthree::checker_concat!(
                "",
                $(
                    $sessiontype,
                )+
                =>
                $(
                    [
                        $branch_stack,
                        $(
                            $choice, $branch,
                        )+
                    ],
                )+
            )
        }
    };
    (
        $name_file: expr,
        $(
            $sessiontype: ty
        ),+ $(,)?
        =>
        $(
            [
                $branch_stack: ty,
                $(
                    $choice: ty, $branch: ident
                ),+ $(,)?
            ]
        ),+ $(,)?
    ) => {
        {
            // All the starting sessions, stringified
            let mut sessions = Vec::new();

            $(
                sessions.push(String::from(std::any::type_name::<$sessiontype>()));
            )+

            // Each choice and branch:  { choice_1 : { branch_1 : session_1 ; branch_2 : session_2 ; ... } ; ... }
            let state_branching_sessions = std::collections::hash_map::RandomState::new();
            let mut branching_sessions: std::collections::HashMap<String, String> =
                std::collections::HashMap::with_hasher(state_branching_sessions);

            // All branches, grouped by choice:  { choice_1::branch_1 : 0 ; choice_1::branch_2 : 1 ; choice_2::branch_1 : 0 ; choice_2::branch_2 : 1 ; ...  }
            let state_group_branches = std::collections::hash_map::RandomState::new();
            let mut group_branches: std::collections::HashMap<String, i32> =
                std::collections::HashMap::with_hasher(state_group_branches);

            // Start the index for group_branches
            let mut index = 0;

            $(
                let temp_branch_stack = String::from(std::any::type_name::<$branch_stack>());
                $(
                    branching_sessions.insert(
                        format!(
                            "{}::{}",
                            stringify!($choice).to_string(),
                            stringify!($branch).to_string(),
                        ),
                        temp_branch_stack.clone()
                    );

                    group_branches.insert(
                        format!(
                            "{}::{}",
                            stringify!($choice).to_string(),
                            stringify!($branch).to_string(),
                        ),
                        index
                    );
                )+

                index += 1;
            )+

            // Macro to implement Display for the `enum`
            mpst_seq::checking!(
                $(
                    $(
                        {
                            $choice,
                            $branch,
                        }
                    )+
                )+
            );

            // Create the graphs with the previous inputs
            mpstthree::checking::checker(
                $name_file,
                sessions,
                branches_receivers,
                branching_sessions,
                group_branches
            )
        }
    };
}

// Run the KMC command line
pub(crate) fn kmc_cli(name_file: &str, kmc_number: i32) -> Result<(bool, String), Box<dyn Error>> {
    // Delete previous files
    remove_file(format!(
        "../mpst_rust_github/outputs/{name_file}_{kmc_number}_kmc.txt"
    ))
    .unwrap_or(());
    remove_file(format!(
        "../mpst_rust_github/outputs/{name_file}-sync-0norm-system.dot"
    ))
    .unwrap_or(());
    remove_file(format!(
        "../mpst_rust_github/outputs/{name_file}-sync-0norm-system.png"
    ))
    .unwrap_or(());
    remove_file(format!(
        "../mpst_rust_github/outputs/{name_file}-ts-{kmc_number}.fsm"
    ))
    .unwrap_or(());

    // Run KMC tool, the outputs files of the tool are in the "outputs" folder
    let kmc = Command::new("KMC")
        .arg(format!("cfsm/{name_file}.txt"))
        .arg(format!("{kmc_number:?}"))
        .arg("--fsm")
        .output()?;

    let stdout = String::from(str::from_utf8(&kmc.stdout)?);

    if stdout.contains("False") {
        Ok((false, stdout))
    } else {
        // Write down the stdout of the previous command into
        // a corresponding file in the "outputs" folder
        let mut kmc_file = File::create(format!("outputs/{name_file}_{kmc_number}_kmc.txt"))?;
        writeln!(kmc_file, "{stdout}")?;
        Ok((true, stdout))
    }
}

// The starting function for extracting the graphs
#[doc(hidden)]
pub fn checker(
    name_file: &str,
    sessions: Vec<String>,
    branches_receivers: HashMap<String, HashMap<String, String>>,
    branching_sessions: HashMap<String, String>,
    group_branches: HashMap<String, i32>,
) -> Result<(HashGraph, Option<i32>), Box<dyn Error>> {
    // Clean the input sessions and extract the roles
    let (clean_sessions, roles) = clean_sessions(sessions.to_vec())?;

    // The cleaned branches_receivers
    let state_branches = RandomState::new();
    let mut update_branches_receivers: HashMap<String, HashMap<String, Vec<String>>> =
        HashMap::with_hasher(state_branches);

    for (choice, branches) in branches_receivers {
        let state_branch = RandomState::new();
        let mut temp_branch: HashMap<String, Vec<String>> = HashMap::with_hasher(state_branch);

        for (branch, session) in branches {
            temp_branch.insert(branch, clean_session(&session)?);
        }

        update_branches_receivers.insert(choice, temp_branch);
    }

    // The cleaned branching_sessions
    let state_branching_sessions = RandomState::new();
    let mut update_branching_sessions: HashMap<String, Vec<String>> =
        HashMap::with_hasher(state_branching_sessions);

    for (branch, session) in branching_sessions {
        let current_clean_session = clean_session(&session)?;
        update_branching_sessions.insert(
            branch.to_string(),
            current_clean_session[..(current_clean_session.len() - 1)].to_vec(),
        );
    }

    // The final result Hashmap
    let state_result = RandomState::new();
    let mut result: HashGraph = HashMap::with_hasher(state_result);

    if !name_file.is_empty() {
        // If a name file has been provided

        // Create cfsm folder if missing
        create_dir_all("cfsm")?;

        // Create the file
        let mut cfsm_file = File::create(format!("cfsm/{name_file}.txt"))?;

        let mut cfsm_sort = vec![vec!["".to_string()]; roles.len()];

        // Get all the graphs and add them to the result Hashmap
        for (role, full_session) in clean_sessions {
            // Get the graph and the cfsm for the current role
            let (graph, cfsm) = get_graph_session(
                &role,
                full_session,
                &roles,
                update_branches_receivers.clone(),
                update_branching_sessions.clone(),
                group_branches.clone(),
            )?;

            // Insert the graph to the returned result
            result.insert(role.to_string(), graph);

            let index_role = roles.iter().position(|r| r == &role).unwrap();

            cfsm_sort[index_role] = cfsm;
        }

        // Write the cfsm into the file
        for elt_cfsm in cfsm_sort.iter() {
            for elt in elt_cfsm.iter() {
                writeln!(cfsm_file, "{elt}")?;
            }

            // Add a blank line
            writeln!(cfsm_file)?;
        }

        let mut kmc_number = 1;
        let mut kmc_result = kmc_cli(name_file, kmc_number)?;

        while !kmc_result.0 && kmc_number < 50 {
            kmc_number += 1;
            kmc_result = kmc_cli(name_file, kmc_number)?;
        }

        if kmc_number == 50 {
            println!(
                "The protocol does not seem correct. Here is the last output: {:?}",
                kmc_result.1
            );
            Ok((result, None))
        } else {
            Ok((result, Some(kmc_number)))
        }
    } else {
        // Get all the graphs and add them to the result Hashmap
        for (role, full_session) in clean_sessions {
            // Get the graph and the cfsm for the current role
            let (graph, _) = get_graph_session(
                &role,
                full_session,
                &roles,
                update_branches_receivers.clone(),
                update_branching_sessions.clone(),
                group_branches.clone(),
            )?;

            // Insert the graph to the returned result
            result.insert(role.to_string(), graph);
        }
        Ok((result, None))
    }
}
