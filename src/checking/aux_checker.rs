use petgraph::graph::NodeIndex;
use petgraph::Graph;

use regex::Regex;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

#[doc(hidden)]
pub(crate) fn clean_session(session: String) -> Result<Vec<String>, Box<dyn Error>> {
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
    /* println!("temp: {:?}", &temp); */
    let mut full_block = get_blocks(temp)?;
    /* println!("full_block: {:?}", &full_block); */

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
pub(crate) fn clean_sessions(
    sessions: Vec<String>,
) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
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
pub(crate) fn roles(tail_sessions: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut roles = Vec::new();
    for tail_session in tail_sessions {
        // Split according to '\n'
        let full_vec: Vec<&str> = tail_session
            .split('\n')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        // Split and push the name of the role of the MeshecChannels
        roles.push(String::from(
            full_vec[full_vec.len() - 1].split('<').collect::<Vec<_>>()[0],
        ));
    }

    // Sort
    roles.sort();

    // Remove duplicates
    roles.dedup();

    Ok(roles)
}

/// Separate the different _fields_ of a stringified type.
#[doc(hidden)]
pub(crate) fn get_blocks(full_block: String) -> Result<Vec<String>, Box<dyn Error>> {
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
pub(crate) fn get_head_payload_continuation(
    full_block: String,
) -> Result<Vec<String>, Box<dyn Error>> {
    /* println!("full_block: {:?}", &full_block); */
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
pub(crate) fn extract_index_node(
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
pub(crate) fn build_dual(session: String) -> Result<String, Box<dyn Error>> {
    if session == *"End" {
        Ok(session)
    } else {
        let all_fields = get_head_payload_continuation(session)?;
        match all_fields[0].as_str() {
            "Recv" => Ok(format!(
                "Send<{},{}>",
                all_fields[1],
                build_dual(all_fields[2].to_string())?
            )),
            "Send" => Ok(format!(
                "Recv<{},{}>",
                all_fields[1],
                build_dual(all_fields[2].to_string())?
            )),
            _ => panic!("Wrond head"),
        }
    }
}

#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn aux_get_graph(
    current_role: &str,
    mut full_session: Vec<String>,
    roles: &[String],
    mut index_node: Vec<usize>,
    mut previous_node: NodeIndex<u32>,
    compare_end: Vec<String>,
    mut depth_level: usize,
    index_current_role: usize,
    mut g: Graph<String, String>,
    branches_receivers: HashMap<String, HashMap<String, Vec<String>>>,
    mut branches_aready_seen: HashMap<String, NodeIndex<u32>>,
    branching_sessions: HashMap<String, Vec<String>>,
    group_branches: HashMap<String, i32>,
) -> Result<Graph<String, String>, Box<dyn Error>> {
    if compare_end == full_session {
        index_node[depth_level] += 1;
        /* println!(
            "Adding node: {:?}",
            &extract_index_node(index_node.clone(), depth_level)?
        ); */
        let new_node = g.add_node(extract_index_node(index_node, depth_level)?);
        g.add_edge(previous_node, new_node, "0".to_string());
        Ok(g)
    } else {
        // Get the size of the full_session
        let size_full_session = full_session.len() - 1;

        // Get the head of the stack
        let stack = &get_head_payload_continuation(full_session[size_full_session].clone())?;

        /* println!("stack: {:?}", &stack); */

        if stack.len() == 3 {
            // If it is a simple choice

            let mut number_of_send = 0;
            let mut number_of_recv = 0;
            let mut pos_recv = 0;

            let mut choice_left = Vec::new();
            let mut choice_right = Vec::new();

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

                        // Should be `Either<MC, MC>`
                        let payload_either =
                            &get_head_payload_continuation(session.to_string())?[1];
                        /* println!("payload_either: {:?}", &payload_either); */

                        // Should be `[Either, MC, MC]`
                        let choices = get_head_payload_continuation(payload_either.to_string())?;
                        /* println!("choices: {:?}", &choices); */

                        // Split the new session
                        let blocks_left = get_blocks(choices[1].to_string())?;
                        let blocks_right = get_blocks(choices[2].to_string())?;
                        /* println!("blocks_left: {:?}", &blocks_left); */
                        /* println!("blocks_right: {:?}", &blocks_right); */

                        // Get the index of the receiver
                        let receiver = &get_head_payload_continuation(
                            blocks_left[blocks_left.len() - 1].to_string(),
                        )?[0];
                        let index_receiver = roles.iter().position(|r| r == receiver).unwrap();

                        // The offset depending on the relative positions of the roles
                        let offset = (index_current_role > index_receiver) as usize;

                        // Push the choice
                        choice_left.push(
                            build_dual(blocks_left[index_current_role - offset].to_string())?
                                .to_string(),
                        );
                        choice_right.push(
                            build_dual(blocks_right[index_current_role - offset].to_string())?
                                .to_string(),
                        );
                        /* println!("choice_left: {:?}", &choice_left); */
                        /* println!("choice_right: {:?}", &choice_right); */
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
            /* println!(
                "Adding node: {:?}",
                &extract_index_node(index_node.clone(), depth_level)?
            ); */

            if number_of_recv == 1 {
                // The offset depending on the relative positions of the roles
                let offset = (index_current_role <= pos_recv) as usize;

                // Add the new edge between the previous and the new node,
                // and label it with the corresponding interaction
                /* println!("current role for offer: {:?}", &current_role); */
                /* println!("possible role for offer: {:?}", &roles[pos_recv + offset]); */
                g.add_edge(
                    previous_node,
                    new_node,
                    format!("& {}", &roles[pos_recv + offset]),
                );
                previous_node = new_node;

                // Should be `Either<MC, MC>`
                let payload_either =
                    &get_head_payload_continuation(full_session[pos_recv].to_string())?[1];
                /* println!("payload_either: {:?}", &payload_either); */

                // Should be `[Either, MC, MC]`
                let offers = get_head_payload_continuation(payload_either.to_string())?;
                /* println!("offers: {:?}", &offers); */

                // The left offer
                let offer_left = clean_session(offers[1].clone())?;
                /* println!("Offer left: {:?}", &offer_left); */
                g = aux_get_graph(
                    current_role,
                    offer_left[..(offer_left.len() - 2)].to_vec(),
                    roles,
                    index_node.clone(),
                    previous_node,
                    compare_end.clone(),
                    depth_level,
                    index_current_role,
                    g,
                    branches_receivers.clone(),
                    branches_aready_seen.clone(),
                    branching_sessions.clone(),
                    group_branches.clone(),
                )?;

                /* println!("Current g: {:?}", &g); */

                let offer_right = clean_session(offers[2].clone())?;
                /* println!("Choose right: {:?}", &offer_right); */
                aux_get_graph(
                    current_role,
                    offer_right[..(offer_right.len() - 2)].to_vec(),
                    roles,
                    index_node,
                    previous_node,
                    compare_end,
                    depth_level,
                    index_current_role,
                    g,
                    branches_receivers,
                    branches_aready_seen,
                    branching_sessions,
                    group_branches,
                )
            } else {
                // Add the new edge between the previous and the new node,
                // and label it with the corresponding interaction
                /* println!("current role for choose: {:?}", &current_role); */
                g.add_edge(previous_node, new_node, format!("+ {}", &current_role));
                previous_node = new_node;

                // Add the corresponding stacks
                choice_left.push(stack[1].to_string());
                choice_right.push(stack[2].to_string());
                /* println!("sent choice_left: {:?}", &choice_left); */
                /* println!("sent choice_right: {:?}", &choice_right); */

                g = aux_get_graph(
                    current_role,
                    choice_left,
                    roles,
                    index_node.clone(),
                    previous_node,
                    compare_end.clone(),
                    depth_level,
                    index_current_role,
                    g,
                    branches_receivers.clone(),
                    branches_aready_seen.clone(),
                    branching_sessions.clone(),
                    group_branches.clone(),
                )?;

                aux_get_graph(
                    current_role,
                    choice_right,
                    roles,
                    index_node,
                    previous_node,
                    compare_end,
                    depth_level,
                    index_current_role,
                    g,
                    branches_receivers,
                    branches_aready_seen,
                    branching_sessions,
                    group_branches,
                )
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
                /* println!(
                    "Adding node: {:?}",
                    &extract_index_node(index_node.clone(), depth_level)?
                ); */

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
                if let Some(choice) = branches_receivers.get(&running_session[1]) {
                    let mut all_branches = Vec::new();
                    let mut all_branches_vec = Vec::new();

                    for (branch, session) in choice {
                        all_branches.push((
                            format!(
                                "{}::{}",
                                &running_session[1].to_string(),
                                &branch.to_string()
                            ),
                            session.to_vec(),
                        ));

                        all_branches_vec.push(format!(
                            "{}::{}",
                            &running_session[1].to_string(),
                            &branch.to_string()
                        ));
                    }

                    all_branches_vec.sort();
                    all_branches.sort();

                    /* println!("all_branches: {:?}", &all_branches); */
                    /* println!("all_branches_vec: {:?}", &all_branches_vec); */
                    /* println!("current_role: {:?}", &current_role); */

                    let mut node_added = false;

                    for (current_branch, session) in all_branches.clone() {
                        /* println!("current branch: {:?}", &current_branch); */
                        /* println!("branches_aready_seen: {:?}", &branches_aready_seen); */
                        /* println!("group_branches: {:?}", &group_branches); */
                        /* println!(); */
                        if let Some(new_node) = branches_aready_seen.get(&current_branch) {
                            if !g.contains_edge(previous_node, *new_node)
                                && previous_node != *new_node
                            {
                                g.add_edge(previous_node, *new_node, "µ".to_string());
                                /* println!("Added edge"); */
                            }
                        } else {
                            // If the node was not added
                            if !node_added {
                                // Increase the index for the nodes
                                index_node.push(0);

                                // Increase the depth level
                                depth_level += 1;

                                // Add the new `step`
                                let new_node = g
                                    .add_node(extract_index_node(index_node.clone(), depth_level)?);
                                /* println!(
                                    "Adding node: {:?}",
                                    &extract_index_node(index_node.clone(), depth_level)?
                                ); */

                                // Add the corresponding edge
                                g.add_edge(previous_node, new_node, format!("& {}", &head_stack));

                                node_added = true;

                                previous_node = new_node;
                            }

                            let mut temp_branches_aready_seen = branches_aready_seen.clone();

                            for temp_current_branch in all_branches.clone() {
                                temp_branches_aready_seen
                                    .insert(temp_current_branch.0.clone(), previous_node);
                            }

                            /* println!("new session: {:?}", &session); */
                            /* println!("full session: {:?}", &full_session); */

                            g = aux_get_graph(
                                current_role,
                                session[..(session.len() - 2)].to_vec(),
                                roles,
                                index_node.clone(),
                                previous_node,
                                compare_end.clone(),
                                depth_level,
                                index_current_role,
                                g,
                                branches_receivers.clone(),
                                temp_branches_aready_seen.clone(),
                                branching_sessions.clone(),
                                group_branches.clone(),
                            )?;
                            /* println!("current graph: {:?}", Dot::new(&g)); */

                            // Insert the new node/branch in the list of the ones already seen
                            let index_group =
                                if let Some(index) = group_branches.get(&current_branch) {
                                    /* println!("index session: {:?}", &index); */
                                    index
                                } else {
                                    panic!("Missing index")
                                };

                            for (temp_current_branch, temp_index) in group_branches.clone() {
                                if temp_index == *index_group {
                                    branches_aready_seen
                                        .insert(temp_current_branch.clone(), previous_node);
                                }
                            }
                        }
                    }

                    return Ok(g);
                } else {
                    index_node[depth_level] += 1;
                    let new_node = g.add_node(extract_index_node(index_node.clone(), depth_level)?);
                    /* println!(
                        "Adding node: {:?}",
                        &extract_index_node(index_node.clone(), depth_level)?
                    ); */
                    g.add_edge(
                        previous_node,
                        new_node,
                        format!("{}?{}: {}", &current_role, &head_stack, &running_session[1]),
                    );
                    full_session[index_head - offset] = running_session[2].to_string();
                    full_session[size_full_session] = stack[1].to_string();
                    previous_node = new_node;
                }
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
                branches_receivers,
                branches_aready_seen,
                branching_sessions,
                group_branches,
            )
        } else if stack.len() == 1 && stack[0] == "RoleBroadcast" {
            let mut number_of_send = 0;

            let mut all_branches = Vec::new();

            /* println!("running RoleBroadcast"); */

            // Check all the sessions
            for (pos, session) in full_session[..(full_session.len() - 1)]
                .to_vec()
                .iter()
                .enumerate()
            {
                match (
                    get_head_payload_continuation(session.to_string())?[0].as_str(),
                    number_of_send,
                    pos,
                ) {
                    ("Send", n_send, n_pos) if n_send == n_pos => {
                        number_of_send += 1;

                        // Should be a specific `enum`
                        let payload = &get_head_payload_continuation(session.to_string())?[1];
                        /* println!("payload_either: {:?}", &payload_either); */

                        // Update all_choices
                        if let Some(choice) = branches_receivers.get(payload) {
                            for branch in choice.keys() {
                                all_branches.push(format!(
                                    "{}::{}",
                                    payload.to_string(),
                                    branch.to_string()
                                ));
                            }
                        } else {
                            panic!("Missing the enum {:?} in branches_receivers", payload)
                        }
                    }
                    _ => panic!("Wrong session heads"),
                }
            }

            let mut node_added = false;

            all_branches.sort();

            /* println!("role: {:?}", &current_role); */
            /* println!(); */
            /* println!("all_branches: {:?}", &all_branches); */
            /* println!("branches_receivers: {:?}", &branches_receivers); */
            /* println!("group_branches: {:?}", &group_branches); */
            /* println!(); */

            for current_branch in all_branches.clone() {
                /* println!("current branch: {:?}", &current_branch); */
                /* println!("branches_aready_seen: {:?}", &branches_aready_seen); */
                /* println!("group_branches: {:?}", &group_branches); */
                /* println!(); */

                if let Some(new_node) = branches_aready_seen.get(&current_branch) {
                    if !g.contains_edge(previous_node, *new_node) && previous_node != *new_node {
                        g.add_edge(previous_node, *new_node, "µ".to_string());
                        /* println!("Added edge"); */
                    }
                } else {
                    // If the node was not added
                    if !node_added {
                        // Increase the index for the nodes
                        index_node.push(0);

                        // Increase the depth level
                        depth_level += 1;

                        // Add the new `step`
                        let new_node =
                            g.add_node(extract_index_node(index_node.clone(), depth_level)?);
                        /* println!(
                            "Adding node: {:?}",
                            &extract_index_node(index_node.clone(), depth_level)?
                        ); */

                        // Add the corresponding edge
                        g.add_edge(previous_node, new_node, format!("+ {}", &current_role));

                        node_added = true;

                        previous_node = new_node;
                    }

                    let session = if let Some(session) = branching_sessions.get(&current_branch) {
                        /* println!("working session: {:?}", &session); */
                        session[..(session.len() - 1)].to_vec()
                    } else {
                        panic!("Missing session")
                    };

                    let mut temp_branches_aready_seen = branches_aready_seen.clone();

                    for temp_current_branch in all_branches.clone() {
                        temp_branches_aready_seen
                            .insert(temp_current_branch.clone(), previous_node);
                    }

                    /* println!("updated branches_aready_seen: {:?}", &branches_aready_seen); */
                    /* println!("new session: {:?}", &session); */
                    /* println!("full session: {:?}", &full_session); */

                    g = aux_get_graph(
                        current_role,
                        session,
                        roles,
                        index_node.clone(),
                        previous_node,
                        compare_end.clone(),
                        depth_level,
                        index_current_role,
                        g,
                        branches_receivers.clone(),
                        temp_branches_aready_seen.clone(),
                        branching_sessions.clone(),
                        group_branches.clone(),
                    )?;

                    // Insert the new node/branch in the list of the ones already seen
                    let index_group = if let Some(index) = group_branches.get(&current_branch) {
                        /* println!("index session: {:?}", &index); */
                        index
                    } else {
                        panic!("Missing index")
                    };

                    for (temp_current_branch, temp_index) in group_branches.clone() {
                        if temp_index == *index_group {
                            /* println!("index session: {:?}", &index_group); */
                            /* println!("added previous node: {:?}", &previous_node); */
                            /* println!("temp_current_branch: {:?}", &temp_current_branch); */
                            branches_aready_seen.insert(temp_current_branch.clone(), previous_node);
                        }
                    }

                    /* println!("current graph: {:?}", Dot::new(&g)); */
                    /* println!(); */
                }
                /* println!(); */
            }
            /* println!(); */

            Ok(g)
        } else {
            panic!("Did not found a correct stack")
        }
    }
}

/// Build the digraph from the current full_session.
#[doc(hidden)]
pub(crate) fn get_graph_session(
    current_role: &str,
    full_session: Vec<String>,
    roles: &[String],
    branches_receivers: HashMap<String, HashMap<String, Vec<String>>>,
    branching_sessions: HashMap<String, Vec<String>>,
    group_branches: HashMap<String, i32>,
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
    /* println!("Expected role: {:?}", &current_role); */
    /* println!("All role: {:?}", &roles); */
    let index_current_role = roles.iter().position(|r| r == current_role).unwrap();

    // The index of the current_role among the roles
    let start_depth_level = 0;

    // The branches already seen
    let state_branches_aready_seen = RandomState::new();
    let branches_aready_seen: HashMap<String, NodeIndex<u32>> =
        HashMap::with_hasher(state_branches_aready_seen);

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
        branches_receivers,
        branches_aready_seen,
        branching_sessions,
        group_branches,
    )
}

//////////////////////////////////

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for
    // mod tests) scope.
    use super::*;
    use std::collections::hash_map::RandomState;
    use std::collections::HashMap;

    #[test]
    fn test_clean_session() {
        let dirty_session = String::from(
            "&&mpstthree::meshedchannels::MeshedChannels<mpstthree::\
            binary::struct_trait::recv::Recv<checking_recursion::\
            Branches0AtoB, mpstthree::binary::struct_trait::end::End>, mpstthree\
            ::binary::struct_trait::recv::Recv<i32, mpstthree::binary::\
            struct_trait::send::Send<i32, mpstthree::binary::struct_trait::end::\
            End>>, mpstthree::role::c::RoleC<mpstthree::role::c::RoleC<\
            mpstthree::role::b::RoleB<mpstthree::role::end::RoleEnd>>>, mpstthree\
            ::role::a::RoleA<mpstthree::role::end::RoleEnd>>",
        );

        let clean_session_compare = vec![
            "Recv<Branches0AtoB,End>".to_string(),
            "Recv<i32,Send<i32,End>>".to_string(),
            "RoleC<RoleC<RoleB<RoleEnd>>>".to_string(),
            "RoleA<RoleEnd>".to_string(),
            "RoleA".to_string(),
        ];

        assert_eq!(clean_session(dirty_session).unwrap(), clean_session_compare);
    }

    #[test]
    fn test_clean_sessions() {
        let dirty_sessions = vec![
            "mpstthree::meshedchannels::MeshedChannels<mpstthree::binary::\
            struct_trait::recv::Recv<checking_recursion::Branches0AtoB, mpstthree\
            ::binary::struct_trait::end::End>, mpstthree::binary::\
            struct_trait::end::End, mpstthree::role::b::RoleB<mpstthree::role\
            ::end::RoleEnd>, mpstthree::role::a::RoleA<mpstthree::role::end::\
            RoleEnd>>"
                .to_string(),
            "mpstthree::meshedchannels::MeshedChannels<mpstthree::\
            binary::struct_trait::end::End, mpstthree::binary::struct_trait::\
            recv::Recv<i32, mpstthree::binary::struct_trait::send::Send<\
            i32, mpstthree::binary::struct_trait::recv::Recv<checking_recursion\
            ::Branches0CtoB, mpstthree::binary::struct_trait::end::End>>>, mpstthree\
            ::role::b::RoleB<mpstthree::role::b::RoleB<mpstthree::role::b::RoleB\
            <mpstthree::role::end::RoleEnd>>>, mpstthree::role::c::RoleC<\
            mpstthree::role::end::RoleEnd>>"
                .to_string(),
            "mpstthree::meshedchannels::\
            MeshedChannels<mpstthree::binary::struct_trait::send::Send<\
            checking_recursion::Branches0AtoB, mpstthree::binary::struct_trait\
            ::end::End>, mpstthree::binary::struct_trait::send::Send<i32, mpstthree\
            ::binary::struct_trait::recv::Recv<i32, mpstthree::binary::struct_trait\
            ::send::Send<checking_recursion::Branches0CtoB, mpstthree::binary::\
            struct_trait::end::End>>>, mpstthree::role::c::RoleC<mpstthree::\
            role::c::RoleC<mpstthree::role::broadcast::RoleBroadcast>>, mpstthree\
            ::role::b::RoleB<mpstthree::role::end::RoleEnd>>"
                .to_string(),
        ];

        // The hasher of the HashMap
        let state_clean_sessions_compare = RandomState::new();
        // The result
        let mut clean_sessions_compare: HashMap<String, Vec<String>> =
            HashMap::with_hasher(state_clean_sessions_compare);

        clean_sessions_compare.insert(
            "RoleC".to_string(),
            vec![
                "End".to_string(),
                "Recv<i32,Send<i32,Recv<Branches0CtoB,End>>>".to_string(),
                "RoleB<RoleB<RoleB<RoleEnd>>>".to_string(),
            ],
        );
        clean_sessions_compare.insert(
            "RoleA".to_string(),
            vec![
                "Recv<Branches0AtoB,End>".to_string(),
                "End".to_string(),
                "RoleB<RoleEnd>".to_string(),
            ],
        );
        clean_sessions_compare.insert(
            "RoleB".to_string(),
            vec![
                "Send<Branches0AtoB,End>".to_string(),
                "Send<i32,Recv<i32,Send<Branches0CtoB,End>>>".to_string(),
                "RoleC<RoleC<RoleBroadcast>>".to_string(),
            ],
        );

        assert_eq!(
            clean_sessions_compare,
            clean_sessions(dirty_sessions).unwrap()
        );
    }

    #[test]
    fn test_roles() {
        let dirty_roles = vec![
            "Recv<End<>>\nEnd<>\nRoleB<RoleEnd<>>\nRoleA<RoleEnd<>>".to_string(),
            "End<>\nRecv<Send<Recv<End<>>>>\nRoleB<RoleB<\
                RoleB<RoleEnd<>>>>\nRoleC<RoleEnd<>>"
                .to_string(),
            "Send<End<>>\nSend<Recv<Send<End<>>>>\nRoleC<\
                RoleC<RoleBroadcast<>>>\nRoleB<RoleEnd<>>"
                .to_string(),
        ];

        let clean_roles = vec![
            "RoleA".to_string(),
            "RoleB".to_string(),
            "RoleC".to_string(),
        ];

        assert_eq!(clean_roles, roles(dirty_roles).unwrap());
    }

    #[test]
    fn test_get_blocks() {
        let dirty_blocks = "MeshedChannels<Send<Branches0AtoB,End>,Send\
        <i32,Recv<i32,Send<Branches0CtoB,End>>>,RoleC\
        <RoleC<RoleBroadcast>>,RoleB<RoleEnd>>"
            .to_string();

        let clean_blocks = vec![
            "Send<Branches0AtoB,End>",
            "Send<i32,Recv<i32,Send<Branches0CtoB,End>>>",
            "RoleC<RoleC<RoleBroadcast>>",
            "RoleB<RoleEnd>",
        ];

        assert_eq!(clean_blocks, get_blocks(dirty_blocks).unwrap());
    }

    #[test]
    fn test_get_head_payload_continuation() {
        // End
        let dirty_end = "End".to_string();

        let clean_end = vec!["End".to_string()];

        assert_eq!(clean_end, get_head_payload_continuation(dirty_end).unwrap());

        // RoleEnd
        let dirty_role_end = "RoleEnd".to_string();

        let clean_role_end = vec!["RoleEnd".to_string()];

        assert_eq!(
            clean_role_end,
            get_head_payload_continuation(dirty_role_end).unwrap()
        );

        // Random
        let dirty_random = "Recv<i32,Send<i32,Recv<Branches0CtoB,End>>>".to_string();

        let clean_random = vec![
            "Recv".to_string(),
            "i32".to_string(),
            "Send<i32,Recv<Branches0CtoB,End>>".to_string(),
        ];

        assert_eq!(
            clean_random,
            get_head_payload_continuation(dirty_random).unwrap()
        );
    }

    #[test]
    fn test_extract_index_node() {
        let index_node = vec![0, 1, 4, 5];

        assert_eq!(
            "0.1.4.5".to_string(),
            extract_index_node(index_node.to_vec(), 3).unwrap()
        );

        assert_eq!(
            "0.1.4".to_string(),
            extract_index_node(index_node.to_vec(), 2).unwrap()
        );

        assert_eq!(
            "0".to_string(),
            extract_index_node(index_node.to_vec(), 0).unwrap()
        );
    }

    #[test]
    fn test_build_dual() {
        let session = "Recv<i32,Send<Branches0CtoB,End>>".to_string();
        let dual_session = "Send<i32,Recv<Branches0CtoB,End>>".to_string();

        assert_eq!(dual_session, build_dual(session).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_build_dual_panic() {
        let session = "Coco<i32,Banana<Branches0CtoB,End>>".to_string();

        build_dual(session).unwrap();
    }
}
