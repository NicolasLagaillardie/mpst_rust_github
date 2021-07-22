use petgraph::dot::Dot;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use regex::Regex;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

#[macro_export]
macro_rules! checker_concat {
    (
        $( $sessiontype: ty , )+
        $(
            {
                $choice: ty,
                $(
                    $branches: ident
                ),+ $(,)?
            }
        ),+ $(,)?
    ) => {

        fn type_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        let mut sessions = Vec::new();
        let mut tail_sessions = Vec::new();

        $(
            sessions.push(String::from(std::any::type_name::<$sessiontype>()));
            tail_sessions.push(<$sessiontype as mpstthree::binary::struct_trait::session::Session>::tail_str());
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

        let result = mpstthree::checking::new_test::checker(sessions, tail_sessions, branches_receivers)?;

        println!("result: {:?}", result);
    };
    (
        $(
            $sessiontype: ty
        ),+ $(,)?
    ) => {

        fn type_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        let mut sessions = Vec::new();
        let mut tail_sessions = Vec::new();

        $(
            sessions.push(String::from(std::any::type_name::<$sessiontype>()));
            tail_sessions.push(<$sessiontype as mpstthree::binary::struct_trait::session::Session>::tail_str());
        )+

        let state_branches = std::collections::hash_map::RandomState::new();
        let mut branches_receivers: std::collections::HashMap<String, std::collections::HashMap<String, String>> =
            std::collections::HashMap::with_hasher(state_branches);

        let result = mpstthree::checking::new_test::checker(sessions, tail_sessions, branches_receivers)?;

        println!("result: {:?}", result);
    };
}

#[doc(hidden)]
fn clean_session(session: String) -> Result<Vec<String>, Box<dyn Error>> {
    // The regex expression
    let main_re = Regex::new(r"([^<,>\s]+)::([^<,>\s]+)").unwrap();
    let mut temp = String::from(&session).replace("&", "");

    // Replace with regex expression -> term1::term2::term3 by term3
    for caps in main_re.captures_iter(&session) {
        temp = temp.replace(&caps[0], &caps[caps.len() - 1]);
    }

    // Remove whitespaces
    temp.retain(|c| !c.is_whitespace());

    // Get each field of the MeshedChannels
    let mut full_block = get_blocks(temp)?;

    // Get the name of the role
    let name = &full_block[full_block.len() - 1]
        .split(['<', '>'].as_ref())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<_>>()[0];

    full_block.push(String::from(name));

    Ok(full_block)
}

/// Clean the sessions received and returns a Hashmap of the cleaned sessions and their respective role.
///
/// Remove the unnecessary terms before each :: (such as mpstthree in mpstthree::Session),
/// and link each new String with its respective role.
#[doc(hidden)]
fn clean_sessions(sessions: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    // The hasher of the HashMap
    let state_branches_receivers = RandomState::new();

    // The result
    let mut all_sessions: HashMap<String, Vec<String>> =
        HashMap::with_hasher(state_branches_receivers);

    for session in sessions {
        let full_block = clean_session(session)?;

        let name = String::from(&full_block[full_block.len() - 1]);

        // Insert the vec of fields (minus the name's role) linked to the name of the role
        all_sessions.insert(name, full_block[..(full_block.len() - 2)].to_vec());
    }

    Ok(all_sessions)
}

// Get the roles from the tail_sessions (the ones from the method tail_str for MeshedChannels).
#[doc(hidden)]
fn roles(tail_sessions: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut roles = Vec::new();
    for tail_session in tail_sessions {
        // Split according to '\n'
        let full_vec: Vec<&str> = tail_session
            .split('\n')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        // Split according to both '<' and '>', and append to result
        // roles.append(
        //     &mut full_vec[full_vec.len() - 2]
        //         .split(['<', '>'].as_ref())
        //         .filter(|s| !s.is_empty())
        //         .map(String::from)
        //         .collect::<Vec<_>>(),
        // );

        // Split and push the name of the role of the MeshecChannels
        roles.push(String::from(
            full_vec[full_vec.len() - 1].split('<').collect::<Vec<_>>()[0],
        ));
    }

    // Remove RoleBroadcast and RoleEnd
    // roles = roles
    //     .iter()
    //     .filter(|s| *s != "RoleBroadcast" && *s != "RoleEnd")
    //     .map(String::from)
    //     .collect();

    // Sort
    roles.sort();

    // Remove duplicates
    roles.dedup();

    Ok(roles)
}

/// Separate the different _fields_ of a stringified type.
#[doc(hidden)]
fn get_blocks(full_block: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::new();
    let mut temp = "".to_string();

    // Start at -1 because we want to remove the first `<` and the term before
    let mut index = -1;

    for i in full_block.chars() {
        if i == '&' || i.is_whitespace() {
        } else if i == '>' && index == 0 {
            result.push(temp.to_string());
            temp = "".to_string();
        } else if i == '<' && index >= 0 {
            temp = format!("{}{}", temp, i);
            index += 1;
        } else if i == '>' && index >= 0 {
            temp = format!("{}{}", temp, i);
            index -= 1;
        } else if i == ',' && index == 0 {
            result.push(temp);
            temp = "".to_string();
        } else if index >= 0 {
            temp = format!("{}{}", temp, i);
        } else if i == '<' {
            index += 1;
        } else if i == '>' {
            index -= 1;
        }
    }

    if !temp.is_empty() {
        let mut chars = temp.chars();
        chars.next_back();

        result.push(chars.as_str().to_string());
    }

    Ok(result)
}

/// Get the start of a Recv/Send session, and its payload and continuation.
#[doc(hidden)]
fn get_head_payload_continuation(full_block: String) -> Result<Vec<String>, Box<dyn Error>> {
    println!("full_block: {:?}", &full_block);
    if full_block == *"End" {
        // If the full block is a `End` type
        Ok(vec!["End".to_string()])
    } else if full_block == *"RoleEnd" {
        // If the full block is a `End` type
        Ok(vec!["RoleEnd".to_string()])
    } else {
        let mut result = vec![full_block.split('<').collect::<Vec<_>>()[0].to_string()];
        result.append(&mut get_blocks(full_block)?);
        Ok(result)
    }
}

#[doc(hidden)]
fn extract_index_node(
    index_node: Vec<usize>,
    depth_level: usize,
) -> Result<String, Box<dyn Error>> {
    Ok(format!(
        "{}{}",
        index_node[..depth_level]
            .to_vec()
            .into_iter()
            .map(|i| format!("{}.", i))
            .collect::<String>(),
        index_node[depth_level]
    ))
}

#[doc(hidden)]
fn aux_get_graph(
    current_role: &str,
    mut full_session: Vec<String>,
    roles: &[String],
    mut index_node: Vec<usize>,
    mut previous_node: NodeIndex<u32>,
    compare_end: Vec<String>,
    mut depth_level: usize,
    index_current_role: usize,
    mut g: Graph<String, String>,
) -> Result<Graph<String, String>, Box<dyn Error>> {
    if compare_end == full_session {
        index_node[depth_level] += 1;
        let new_node = g.add_node(extract_index_node(index_node.clone(), depth_level)?);
        g.add_edge(previous_node, new_node, "0".to_string());
        Ok(g)
    } else {
        // Get the size of the full_session
        let size_full_session = full_session.len() - 1;

        // Get the head of the stack
        let stack = &get_head_payload_continuation(full_session[size_full_session].clone())?;

        println!("stack: {:?}", &stack);

        if stack.len() == 3 {
            // If it is a simple choice

            let mut number_of_send = 0;
            let mut number_of_recv = 0;
            let mut pos_recv = 0;
            for (pos, session) in full_session[..(full_session.len() - 1)]
                .to_vec()
                .iter()
                .enumerate()
            {
                match (
                    get_head_payload_continuation(session.to_string())?[0].as_str(),
                    number_of_send,
                    number_of_recv,
                    pos,
                ) {
                    ("Send", n_send, 0, n_pos) if n_send == n_pos => {
                        number_of_send += 1;
                    }
                    ("Recv", 0, 0, new_pos) => {
                        number_of_recv += 1;
                        pos_recv = new_pos;
                    }
                    ("End", 0, _, _) => {}
                    _ => panic!("Wrong session heads"),
                }
            }

            // Increase the index for the nodes
            index_node.push(0);

            // Increase the depth level
            depth_level += 1;

            // Add the new `step`
            let new_node = g.add_node(extract_index_node(index_node.clone(), depth_level)?);

            if number_of_recv == 1 {
                // The offset depending on the relative positions of the roles
                let offset = (index_current_role <= pos_recv) as usize;

                // Add the new edge between the previous and the new node,
                // and label it with the corresponding interaction
                println!("current role for choose: {:?}", &current_role);
                println!("possible role for choose: {:?}", &roles[pos_recv + offset]);
                g.add_edge(
                    previous_node,
                    new_node,
                    format!("& {}", &roles[pos_recv + offset]),
                );
                previous_node = new_node;

                // Should be `Either<MC, MC>`
                let payload_either =
                    &get_head_payload_continuation(full_session[pos_recv].to_string())?[1];
                println!("payload_either: {:?}", &payload_either);

                // Should be `[Either, MC, MC]`
                let choices = get_head_payload_continuation(payload_either.to_string())?;
                println!("choices: {:?}", &choices);

                // The left choice
                let choice_left = clean_session(choices[1].clone())?;
                println!("Choose left: {:?}", &choice_left);
                g = aux_get_graph(
                    current_role,
                    choice_left[..(choice_left.len() - 2)].to_vec(),
                    roles,
                    index_node.clone(),
                    previous_node,
                    compare_end.clone(),
                    depth_level,
                    index_current_role,
                    g,
                )?;

                println!("Current g: {:?}", &g);

                let choice_right = clean_session(choices[2].clone())?;
                println!("Choose right: {:?}", &choice_right);
                aux_get_graph(
                    current_role,
                    choice_right[..(choice_right.len() - 2)].to_vec(),
                    roles,
                    index_node,
                    previous_node,
                    compare_end,
                    depth_level,
                    index_current_role,
                    g,
                )
            } else {
                Ok(g)
            }
        } else if stack.len() == 2 {
            // If it is a simple interaction
            let head_stack = &stack[0];

            // The index of the head_stack among the roles
            let index_head = roles.iter().position(|r| r == head_stack).unwrap();

            // The offset depending on the relative positions of the roles
            let offset = (index_current_role < index_head) as usize;

            // The running session
            let running_session =
                get_head_payload_continuation(full_session[index_head - offset].to_string())?;

            // If Send/Recv, everything is good, else, panic
            if running_session[0] == *"Send" {
                // Increase the index for the nodes
                index_node[depth_level] += 1;

                // Add the new `step`
                let new_node = g.add_node(extract_index_node(index_node.clone(), depth_level)?);

                // Add the new edge between the previous and the new node,
                // and label it with the corresponding interaction
                g.add_edge(
                    previous_node,
                    new_node,
                    format!("{}!{}: {}", &current_role, &head_stack, &running_session[1]),
                );

                // Replace the old binary session with the new one
                full_session[index_head - offset] = running_session[2].to_string();

                // Replace the old stack with the new one
                full_session[size_full_session] = stack[1].to_string();

                // Update the previous node
                previous_node = new_node;
            } else if running_session[0] == *"Recv" {
                index_node[depth_level] += 1;
                let new_node = g.add_node(extract_index_node(index_node.clone(), depth_level)?);
                g.add_edge(
                    previous_node,
                    new_node,
                    format!("{}?{}: {}", &current_role, &head_stack, &running_session[1]),
                );
                full_session[index_head - offset] = running_session[2].to_string();
                full_session[size_full_session] = stack[1].to_string();
                previous_node = new_node;
            } else {
                panic!("Did not found a correct session")
            }

            aux_get_graph(
                current_role,
                full_session,
                roles,
                index_node,
                previous_node,
                compare_end,
                depth_level,
                index_current_role,
                g,
            )
        } else {
            panic!("Did not found a correct stack")
        }
    }
}

/// Build the digraph from the current full_session.
#[doc(hidden)]
fn get_graph_session(
    current_role: &str,
    full_session: Vec<String>,
    roles: &[String],
) -> Result<Graph<String, String>, Box<dyn Error>> {
    // Create the new graph that will be returned in the end
    let mut g = Graph::<String, String>::new();

    // Start the index for the different `steps` of the choreography
    let index_node = vec![0];

    // Add the first node for the graph
    let previous_node = g.add_node(index_node[0].to_string());

    // The `End` vec that we will compare to `full_session`
    let mut compare_end = vec!["End".to_string(); full_session.len() - 1];
    compare_end.push("RoleEnd".to_string());

    // The index of the current_role among the roles
    let index_current_role = roles.iter().position(|r| r == current_role).unwrap();

    // The index of the current_role among the roles
    let start_depth_level = 0;

    aux_get_graph(
        current_role,
        full_session,
        roles,
        index_node,
        previous_node,
        compare_end,
        start_depth_level,
        index_current_role,
        g,
    )
}

pub fn checker(
    sessions: Vec<String>,
    tail_sessions: Vec<String>,
    branches_receivers: HashMap<String, HashMap<String, String>>,
) -> Result<Vec<String>, Box<dyn Error>> {
    println!("sessions: {:?}", &sessions);
    println!();

    let clean_sessions = clean_sessions(sessions)?;
    let roles = roles(tail_sessions)?;

    let state_branches = RandomState::new();
    let mut update_branches_receivers: HashMap<String, HashMap<String, Vec<String>>> =
        HashMap::with_hasher(state_branches);

    println!("branches_receivers: {:?}", &branches_receivers);
    println!();

    for (choice, branches) in branches_receivers {
        let state_branch = RandomState::new();
        let mut temp_branch: HashMap<String, Vec<String>> = HashMap::with_hasher(state_branch);

        for (branch, session) in branches {
            temp_branch.insert(branch, clean_session(session)?);
        }

        update_branches_receivers.insert(choice, temp_branch);
    }

    println!("clean_sessions: {:?}", &clean_sessions);
    println!();

    println!("roles: {:?}", &roles);
    println!();

    println!(
        "update_branches_receivers: {:?}",
        &update_branches_receivers
    );
    println!();

    for (role, full_session) in clean_sessions.clone() {
        let graph = get_graph_session(&role, full_session, &roles)?;
        println!("Role: {:?}", &role);
        println!("Graph: {:?}", Dot::new(&graph));
    }

    for (role, fields) in clean_sessions {
        println!("role: {:?}:", role);
        println!("fields: {:?}:", &fields);
        println!();
        for field in fields {
            println!("field: {:?}:", &field);
            println!(
                "head, payload and continuation: {:?}:",
                get_head_payload_continuation(String::from(&field))?
            );
            println!();
        }
        println!();
    }

    Ok(vec!["".to_string()])
}
