// use petgraph::dot::Dot;
use petgraph::Graph;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

mod aux_checker;

use aux_checker::*;

/// The macro that allows to create digraphs from each endpoint, along with `enum` if needed.
///
/// # Arguments
///
/// * Each starting endpoint, separated by a comma
/// * \[Optional\] Each new `MeshedChannels` adopted by each sender of each choice
/// * \[Optional\] Each `enum`, along with their respective branch/variant, separated by a comma.
/// Those `enum` must not have any parameter, such as `<i32>`.
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
                    $corresponding_branch: path
                ),+ $(,)?
            ]
        ),+ $(,)?
        =>
        $(
            {
                $choice: ty,
                $(
                    $branches: ident
                ),+ $(,)?
            }
        ),+ $(,)?
    ) => {
        {
            let mut sessions = Vec::new();
            let mut tail_sessions = Vec::new();
            let state_branching_sessions = std::collections::hash_map::RandomState::new();
            let mut branching_sessions: std::collections::HashMap<String, String> =
                std::collections::HashMap::with_hasher(state_branching_sessions);

            let state_group_branches = std::collections::hash_map::RandomState::new();
            let mut group_branches: std::collections::HashMap<String, i32> =
                std::collections::HashMap::with_hasher(state_group_branches);

            $(
                sessions.push(String::from(std::any::type_name::<$sessiontype>()));
                tail_sessions.push(<$sessiontype as mpstthree::binary::struct_trait::session::Session>::tail_str());
            )+

            let mut index = 0;

            $(
                let temp_branch_stack = String::from(std::any::type_name::<$branch_stack>());
                $(
                    branching_sessions.insert(
                        stringify!($corresponding_branch).to_string(),
                        temp_branch_stack.clone()
                    );
                    group_branches.insert(
                        stringify!($corresponding_branch).to_string(),
                        index
                    );
                )+
                index += 1;
            )+

            mpst_seq::checking!(
                $(
                    {
                        $choice: ty,
                        $(
                            $branches: ident,
                        )+
                    }
                )+
            );

            mpstthree::checking::checker(
                sessions,
                tail_sessions,
                branches_receivers,
                branching_sessions,
                group_branches
            )

            /* println!("result: {:?}", result); */
        }
    };
    (
        $(
            $sessiontype: ty
        ),+ $(,)?
    ) => {
        {
            let mut sessions = Vec::new();
            let mut tail_sessions = Vec::new();

            $(
                sessions.push(String::from(std::any::type_name::<$sessiontype>()));
                tail_sessions.push(<$sessiontype as mpstthree::binary::struct_trait::session::Session>::tail_str());
            )+

            let state_branches = std::collections::hash_map::RandomState::new();
            let mut branches_receivers: std::collections::HashMap<String, std::collections::HashMap<String, String>> =
                std::collections::HashMap::with_hasher(state_branches);

            let state_branching_sessions = std::collections::hash_map::RandomState::new();
            let mut branching_sessions: std::collections::HashMap<String, String> =
                std::collections::HashMap::with_hasher(state_branching_sessions);

            let state_group_branches = std::collections::hash_map::RandomState::new();
            let mut group_branches: std::collections::HashMap<String, i32> =
                std::collections::HashMap::with_hasher(state_group_branches);

            mpstthree::checking::checker(
                sessions,
                tail_sessions,
                branches_receivers,
                branching_sessions,
                group_branches
            )

            /* println!("result: {:?}", result); */
        }
    };
}

#[doc(hidden)]
pub fn checker(
    sessions: Vec<String>,
    tail_sessions: Vec<String>,
    branches_receivers: HashMap<String, HashMap<String, String>>,
    branching_sessions: HashMap<String, String>,
    group_branches: HashMap<String, i32>,
) -> Result<HashMap<String, Graph<String, String>>, Box<dyn Error>> {
    /* println!("sessions: {:?}", &sessions); */
    /* println!(); */

    let clean_sessions = clean_sessions(sessions.to_vec())?;

    /* println!("clean_sessions: {:?}", &clean_sessions); */

    /* println!("tail_sessions: {:?}", &tail_sessions); */
    /* println!(); */

    let roles = roles(tail_sessions)?;

    if roles.len() != sessions.len() {
        panic!("The numbers of roles and sessions are not equal")
    }

    /* println!("roles: {:?}", &roles); */

    let state_branches = RandomState::new();
    let mut update_branches_receivers: HashMap<String, HashMap<String, Vec<String>>> =
        HashMap::with_hasher(state_branches);

    /* println!("branches_receivers: {:?}", &branches_receivers); */
    /* println!(); */

    for (choice, branches) in branches_receivers {
        let state_branch = RandomState::new();
        let mut temp_branch: HashMap<String, Vec<String>> = HashMap::with_hasher(state_branch);

        for (branch, session) in branches {
            /* println!("Dirty session: {:?}", &session); */
            /* println!("Clean session: {:?}", clean_session(session.to_string())?); */
            temp_branch.insert(branch, clean_session(session)?);
        }

        update_branches_receivers.insert(choice, temp_branch);
    }

    /* println!("clean_sessions: {:?}", &clean_sessions); */
    /* println!(); */

    /* println!("roles: {:?}", &roles); */
    /* println!(); */

    /* println!(
        "update_branches_receivers: {:?}",
        &update_branches_receivers
    ); */
    /* println!(); */

    /* println!("branching_sessions: {:?}", &branching_sessions); */
    /* println!(); */

    /* println!("group_branches: {:?}", &group_branches); */
    /* println!(); */

    let state_branching_sessions = RandomState::new();
    let mut update_branching_sessions: HashMap<String, Vec<String>> =
        HashMap::with_hasher(state_branching_sessions);

    for (branch, session) in branching_sessions {
        let current_clean_session = clean_session(session.to_string())?;
        update_branching_sessions.insert(
            branch.to_string(),
            current_clean_session[..(current_clean_session.len() - 1)].to_vec(),
        );
    }

    /* println!(
        "update_branching_sessions: {:?}",
        &update_branching_sessions
    ); */
    /* println!(); */

    let state_result = RandomState::new();
    let mut result: HashMap<String, Graph<String, String>> = HashMap::with_hasher(state_result);

    for (role, full_session) in clean_sessions.clone() {
        let graph = get_graph_session(
            &role,
            full_session,
            &roles,
            update_branches_receivers.clone(),
            update_branching_sessions.clone(),
            group_branches.clone(),
        )?;
        /* println!("Role: {:?}", &role); */
        /* println!("Final graph: {:?}", Dot::new(&graph)); */
        result.insert(role.to_string(), graph);
    }

    // for (role, fields) in clean_sessions {
    //     /* println!("role: {:?}:", role); */
    //     /* println!("fields: {:?}:", &fields); */
    //     /* println!(); */
    //     for field in fields {
    //         /* println!("field: {:?}:", &field); */
    //         /* println!(
    //             "head, payload and continuation: {:?}:",
    //             get_head_payload_continuation(String::from(&field))?
    //         ); */
    //         /* println!(); */
    //     }
    //     /* println!(); */
    // }

    Ok(result)
}
