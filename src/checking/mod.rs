#![cfg(feature = "checking")]

use petgraph::Graph;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str;

mod aux_checker;

use aux_checker::*;

/// The macro that allows to create digraphs from each endpoint, along with `enum` if needed.
///
/// # Arguments
///
/// * Each starting endpoint, separated by a comma
/// * \[Optional\] Each new `MeshedChannels` adopted by each sender of each choice, along with all the different branches sent.
/// Currently, we do not support parameters for branches with `enum`
///
/// ```ignore
/// // Assume that there are two choices (Branches0BtoA and Branches0CtoA), each one with two branches (Video and End).
/// // Assume also that the starting Endpoints of the three roles are EndpointAFull, EndpointCFull and EndpointBFull
/// // (the order does not matter). The call for this macro would be as followed
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
#[macro_export]
macro_rules! checker_concat {
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
                            $choice: ty,
                            $branch: ident,
                        }
                    )+
                )+
            );

            // Create the graphs with the previous inputs
            mpstthree::checking::checker(
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
    ) => {
        {
            let mut sessions = Vec::new();

            $(
                sessions.push(String::from(std::any::type_name::<$sessiontype>()));
            )+

            let state_branches = std::collections::hash_map::RandomState::new();
            let branches_receivers: std::collections::HashMap<String, std::collections::HashMap<String, String>> =
                std::collections::HashMap::with_hasher(state_branches);

            let state_branching_sessions = std::collections::hash_map::RandomState::new();
            let branching_sessions: std::collections::HashMap<String, String> =
                std::collections::HashMap::with_hasher(state_branching_sessions);

            let state_group_branches = std::collections::hash_map::RandomState::new();
            let group_branches: std::collections::HashMap<String, i32> =
                std::collections::HashMap::with_hasher(state_group_branches);

            mpstthree::checking::checker(
                sessions,
                branches_receivers,
                branching_sessions,
                group_branches
            )
        }
    };
}

// The starting function for extracting the graphs
#[doc(hidden)]
pub fn checker(
    sessions: Vec<String>,
    branches_receivers: HashMap<String, HashMap<String, String>>,
    branching_sessions: HashMap<String, String>,
    group_branches: HashMap<String, i32>,
) -> Result<HashMap<String, Graph<String, String>>, Box<dyn Error>> {
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
    let mut result: HashMap<String, Graph<String, String>> = HashMap::with_hasher(state_result);

    // Create a new non existing file
    let mut index_cfsm = 0;

    while Path::new(&format!("cfsm/{}.txt", index_cfsm)).exists() {
        index_cfsm += 1;
    }

    let mut cfsm_file = File::create(format!("cfsm/{}.txt", index_cfsm))?;

    // Get all the graphs and add them to the result Hashmap
    for (role, full_session) in clean_sessions.clone() {
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

        // Write the cfsm into the file
        for s in cfsm.iter() {
            writeln!(&mut cfsm_file, "{}", s)?;
        }
        writeln!(&mut cfsm_file)?;
    }

    // Run KMC tool, the outputs files of the tool are in the "outputs" folder
    let kmc = Command::new("./../kmc/KMC")
        .arg(format!("../mpst_rust_github/cfsm/{}.txt", index_cfsm))
        .arg("2")
        .arg("--fsm")
        .output()?;

    // Write down the stdout of the previous command into a corresponding
    // file in the "outputs" folder
    let mut kmc_file = File::create(format!("outputs/{}_kmc.txt", index_cfsm))?;
    writeln!(&mut kmc_file, "{}", str::from_utf8(&kmc.stdout)?)?;

    Ok(result)
}
