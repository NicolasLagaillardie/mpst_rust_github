use petgraph::graph::NodeIndex;
use petgraph::Graph;

use regex::Regex;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

type VecOfStr = Vec<String>;
type HashMapStrVecOfStr = HashMap<String, VecOfStr>;
type GraphOfStrStr = Graph<String, String>;
type VecOfTuple = Vec<(String, usize)>;

/// Clean the provided session, which should be stringified.
///
/// From
///     "&&mpstthree::meshedchannels::MeshedChannels<mpstthree::\
///     binary::struct_trait::recv::Recv<checking_recursion::\
///     Branches0AtoB, mpstthree::binary::struct_trait::end::End>, mpstthree\
///     ::binary::struct_trait::recv::Recv<i32, mpstthree::binary::\
///     struct_trait::send::Send<i32, mpstthree::binary::struct_trait::end::\
///     End>>, mpstthree::role::c::RoleC<mpstthree::role::c::RoleC<\
///     mpstthree::role::b::RoleB<mpstthree::role::end::RoleEnd>>>, mpstthree\
///     ::name::a::NameA>"
///
/// to
///
/// [
///     "Recv<Branches0AtoB,End>",
///     "Recv<i32,Send<i32,End>>",
///     "RoleC<RoleC<RoleB<RoleEnd>>>",
///     "RoleA<RoleEnd>",
///     "RoleA"
/// ]
///
/// /!\ Mixing former and new naming: moving from new to former
#[doc(hidden)]
pub(crate) fn clean_session(session: &str) -> Result<VecOfStr, Box<dyn Error>> {
    let mut double_colon_less = session.replace('&', "");

    // The main regex expression
    let double_colon_regex = Regex::new(r"([^<,>\s]+)::([^<,>\s]+)")?;

    // Replace with regex expression -> term1::term2::term3 by term3
    for caps in double_colon_regex.captures_iter(session) {
        double_colon_less = double_colon_less.replace(&caps[0], &caps[caps.len() - 1]);
    }

    // The name regex expression
    let name_regex = Regex::new(r"Name([[:alpha:]]+)")?;

    let mut name_to_role = double_colon_less.clone();

    // Replace with regex expression -> term1::term2::term3 by term3
    for caps in name_regex.captures_iter(&double_colon_less) {
        name_to_role =
            name_to_role.replace(&caps[0], &format!("Role{}<RoleEnd>", &caps[caps.len() - 1]));
    }

    // Remove whitespaces
    name_to_role.retain(|c| !c.is_whitespace());

    // Get each field of the MeshedChannels
    let mut full_block = get_blocks(&name_to_role)?;

    // Get the name of the role
    let name = full_block[full_block.len() - 1]
        .split(['<', '>'].as_ref())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<_>>()[0]
        .to_string();

    full_block.push(name);

    Ok(full_block)
}

// Clean the sessions received and returns a Hashmap of the cleaned sessions and their respective
// role.
//
// Remove the unnecessary terms before each :: (such as mpstthree in mpstthree::Session),
// and link each new String with its respective role.
// Uses the clean_session() function to achieve the result
#[doc(hidden)]
pub(crate) fn clean_sessions(
    sessions: VecOfStr,
) -> Result<(HashMapStrVecOfStr, VecOfStr), Box<dyn Error>> {
    // The hasher of the HashMap
    let state_branches_receivers = RandomState::new();

    // All the roles
    let mut roles = Vec::new();

    // The result
    let mut all_sessions: HashMapStrVecOfStr = HashMap::with_hasher(state_branches_receivers);

    let mut size_sessions = 0;

    for session in sessions {
        let full_block = clean_session(&session)?;

        // The number of expected roles
        size_sessions = full_block.len() - 2;

        // Collect the last field of the meshedChannels (the name field)
        let name = &full_block[full_block.len() - 1];

        // Collect the names of the roles
        roles.push(name.to_string());

        // Insert the vec of fields (minus the name's role) linked to the name of the role
        all_sessions.insert(
            name.to_string(),
            full_block[..(full_block.len() - 2)].to_vec(),
        );
    }

    // If the number of roles is different from the number of sessions
    if roles.len() != size_sessions {
        println!("roles: {:?}", roles);

        panic!("The numbers of roles and sessions are not equal")
    }

    // Sort
    roles.sort();

    // Remove duplicates
    roles.dedup();

    Ok((all_sessions, roles))
}

// Separate the different _fields_ of a stringified type.
//
// From
//     "MeshedChannels<Send<Branches0AtoB,End>,Send\
//     <i32,Recv<i32,Send<Branches0CtoB,End>>>,RoleC\
//     <RoleC<RoleBroadcast>>,RoleB<RoleEnd>>"
//
// to
//
// [
//     "Send<Branches0AtoB,End>",
//     "Send<i32,Recv<i32,Send<Branches0CtoB,End>>>",
//     "RoleC<RoleC<RoleBroadcast>>",
//     "RoleB<RoleEnd>",
// ]
#[doc(hidden)]
pub(crate) fn get_blocks(full_block: &str) -> Result<VecOfStr, Box<dyn Error>> {
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

// Get the head of a Recv/Send session, its payload and its continuation.
#[doc(hidden)]
pub(crate) fn get_head_payload_continuation(full_block: &str) -> Result<VecOfStr, Box<dyn Error>> {
    if full_block == "End" {
        // If the full block is a `End` type
        Ok(vec!["End".to_string()])
    } else if full_block == "RoleEnd" {
        // If the full block is a `End` type
        Ok(vec!["RoleEnd".to_string()])
    } else {
        let mut result = vec![full_block.to_string().split('<').collect::<Vec<_>>()[0].to_string()];
        result.append(&mut get_blocks(full_block)?);

        Ok(result)
    }
}

// Extract the correct label for a node from the index_node and the depth of the current node.
//
// From [0, 1, 0, 5] and 2 to "0.1.0".
#[doc(hidden)]
pub(crate) fn extract_index_node(
    index_node: &[usize],
    depth_level: usize,
) -> Result<String, Box<dyn Error>> {
    Ok(format!(
        "{}{}",
        index_node[..depth_level]
            .iter()
            .copied()
            .map(|i| format!("{}.", i))
            .collect::<String>(),
        index_node[depth_level]
    ))
}

// Switch all Send and Recv at the head of each session
#[doc(hidden)]
pub(crate) fn build_dual(session: &str) -> Result<String, Box<dyn Error>> {
    if session == "End" {
        Ok(session.to_string())
    } else {
        let all_fields = get_head_payload_continuation(session)?;
        match all_fields[0].as_str() {
            "Recv" => Ok(format!(
                "Send<{},{}>",
                all_fields[1],
                build_dual(&all_fields[2])?
            )),
            "Send" => Ok(format!(
                "Recv<{},{}>",
                all_fields[1],
                build_dual(&all_fields[2])?
            )),
            _ => panic!("Wrong head"),
        }
    }
}

#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn aux_get_graph(
    current_role: &str,
    mut full_session: VecOfStr,
    roles: &[String],
    mut index_node: Vec<usize>,
    mut previous_node: NodeIndex<u32>,
    compare_end: VecOfStr,
    mut depth_level: usize,
    index_current_role: usize,
    mut g: GraphOfStrStr,
    branches_receivers: HashMap<String, HashMapStrVecOfStr>,
    mut branches_already_seen: HashMap<String, NodeIndex<u32>>,
    branching_sessions: HashMapStrVecOfStr,
    group_branches: HashMap<String, i32>,
    mut cfsm: VecOfTuple,
) -> Result<(GraphOfStrStr, VecOfTuple), Box<dyn Error>> {
    if compare_end == full_session {
        index_node[depth_level] += 1;
        let new_node = g.add_node(extract_index_node(&index_node, depth_level)?);
        g.add_edge(previous_node, new_node, "0".to_string());

        Ok((g, cfsm))
    } else {
        // Get the size of the full_session
        let size_full_session = full_session.len() - 1;

        // Get the head of the stack
        let stack = &get_head_payload_continuation(&full_session[size_full_session])?;

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
                    get_head_payload_continuation(session)?[0].as_str(),
                    number_of_send,
                    number_of_recv,
                    pos,
                ) {
                    ("Send", n_send, 0, n_pos) if n_send == n_pos => {
                        number_of_send += 1;

                        // Should be `Either<MC, MC>`
                        let payload_either = &get_head_payload_continuation(session)?[1];

                        // Should be `[Either, MC, MC]`
                        let choices = get_head_payload_continuation(payload_either)?;

                        // Split the new session
                        let blocks_left = get_blocks(&choices[1])?;
                        let blocks_right = get_blocks(&choices[2])?;

                        println!("aux_get_graph blocks_right: {:?}", blocks_right.clone());

                        // Get the index of the receiver
                        let receiver =
                            &get_head_payload_continuation(&blocks_left[blocks_left.len() - 1])?[0];

                        let index_receiver =
                            if let Some(elt) = roles.iter().position(|r| r == receiver) {
                                elt
                            } else {
                                panic!("Issue with roles {:?} and receiver {:?}", roles, receiver)
                            };

                        // The offset depending on the relative positions of the roles
                        let offset = (index_current_role > index_receiver) as usize;

                        // Push the choice
                        choice_left.push(build_dual(&blocks_left[index_current_role - offset])?);
                        choice_right.push(build_dual(&blocks_right[index_current_role - offset])?);
                    }
                    ("Recv", 0, 0, new_pos) => {
                        number_of_recv += 1;
                        pos_recv = new_pos;
                    }
                    ("End", 0, _, _) => {}
                    _ => panic!("Wrong session heads"),
                }
            }

            if number_of_recv == 0 && number_of_send == 0 {
                panic!("Expected choose or offer, only found End")
            }

            // Increase the index for the nodes
            index_node.push(0);

            // Increase the depth level
            depth_level += 1;

            if number_of_recv == 1 {
                // If this is a passive role

                // Should be `Either<MC, MC>`
                let payload_either = &get_head_payload_continuation(&full_session[pos_recv])?[1];

                // Should be `[Either, MC, MC]`
                let offers = get_head_payload_continuation(payload_either)?;

                // The left offer
                let offer_left = clean_session(&offers[1])?;

                let result = aux_get_graph(
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
                    branches_already_seen.clone(),
                    branching_sessions.clone(),
                    group_branches.clone(),
                    cfsm,
                )?;

                g = result.0;
                cfsm = result.1;

                let offer_right = clean_session(&offers[2])?;

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
                    branches_already_seen,
                    branching_sessions,
                    group_branches,
                    cfsm,
                )
            } else {
                // If this is the active role

                // Add the corresponding stacks
                choice_left.push(stack[1].to_string());
                choice_right.push(stack[2].to_string());

                let result = aux_get_graph(
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
                    branches_already_seen.clone(),
                    branching_sessions.clone(),
                    group_branches.clone(),
                    cfsm,
                )?;

                g = result.0;
                cfsm = result.1;

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
                    branches_already_seen,
                    branching_sessions,
                    group_branches,
                    cfsm,
                )
            }
        } else if stack.len() == 2 {
            // If it is a simple interaction
            let head_stack = &stack[0];

            // The index of the head_stack among the roles
            let index_head = if let Some(elt) = roles.iter().position(|r| r == head_stack) {
                elt
            } else {
                panic!(
                    "Issue with roles {:?} and head_stack {:?}",
                    roles, head_stack
                )
            };

            // The offset depending on the relative positions of the roles
            let offset = (index_current_role < index_head) as usize;

            // The running session
            let running_session =
                get_head_payload_continuation(&full_session[index_head - offset])?;

            // If Send/Recv, everything is good, else, panic
            if running_session[0] == *"Send" {
                // If send simple payload

                // Increase the index for the nodes
                index_node[depth_level] += 1;

                // Add the new `step`
                let new_node = g.add_node(extract_index_node(&index_node, depth_level)?);

                // Add the new edge between the previous and the new node,
                // and label it with the corresponding interaction
                g.add_edge(
                    previous_node,
                    new_node,
                    format!("{}!{}: {}", current_role, head_stack, &running_session[1]),
                );

                cfsm.push((
                    format!(
                        "{}{} {} ! {} {}",
                        current_role,
                        previous_node.index(),
                        index_head,
                        &running_session[1],
                        current_role
                    ),
                    new_node.index(),
                ));

                // Replace the old binary session with the new one
                full_session[index_head - offset] = running_session[2].to_string();

                // Replace the old stack with the new one
                full_session[size_full_session] = stack[1].to_string();

                // Update the previous node
                previous_node = new_node;
            } else if running_session[0] == *"Recv" {
                if let Some(choice) = branches_receivers.get(&running_session[1]) {
                    // If receive recursive choice
                    let mut all_branches = Vec::new();
                    let mut all_branches_vec = Vec::new();

                    for (branch, session) in choice {
                        all_branches.push((
                            format!("{}::{}", &running_session[1], &branch),
                            session.to_vec(),
                        ));

                        all_branches_vec.push(format!("{}::{}", &running_session[1], &branch));
                    }

                    all_branches_vec.sort();
                    all_branches.sort();

                    let mut node_added = false;

                    for (current_branch, session) in all_branches.clone() {
                        if let Some(new_node) = branches_already_seen.get(&current_branch) {
                            if !g.contains_edge(previous_node, *new_node)
                                && previous_node != *new_node
                            {
                                g.add_edge(previous_node, *new_node, "µ".to_string());

                                if let Some(elt) = cfsm.pop() {
                                    cfsm.push((elt.0, new_node.index()));
                                }
                            }
                        } else {
                            // If the node was not added
                            if !node_added {
                                // Increase the index for the nodes
                                index_node.push(0);

                                // Increase the depth level
                                depth_level += 1;

                                node_added = true;
                            }

                            let mut temp_branches_already_seen = branches_already_seen.clone();

                            for temp_current_branch in all_branches.clone() {
                                temp_branches_already_seen
                                    .insert(temp_current_branch.0.clone(), previous_node);
                            }

                            let result = aux_get_graph(
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
                                temp_branches_already_seen.clone(),
                                branching_sessions.clone(),
                                group_branches.clone(),
                                cfsm,
                            )?;

                            g = result.0;
                            cfsm = result.1;

                            // Insert the new node/branch in the list of the ones already seen
                            let index_group =
                                if let Some(index) = group_branches.get(&current_branch) {
                                    index
                                } else {
                                    panic!("Missing index")
                                };

                            for (temp_current_branch, temp_index) in group_branches.clone() {
                                if temp_index == *index_group {
                                    branches_already_seen
                                        .insert(temp_current_branch.clone(), previous_node);
                                }
                            }
                        }
                    }

                    return Ok((g, cfsm));
                } else {
                    // If receive simple payload

                    index_node[depth_level] += 1;

                    let new_node = g.add_node(extract_index_node(&index_node, depth_level)?);

                    g.add_edge(
                        previous_node,
                        new_node,
                        format!("{}?{}: {}", current_role, head_stack, &running_session[1]),
                    );

                    cfsm.push((
                        format!(
                            "{}{} {} ? {} {}",
                            current_role,
                            previous_node.index(),
                            index_head,
                            &running_session[1],
                            current_role
                        ),
                        new_node.index(),
                    ));

                    full_session[index_head - offset] = running_session[2].to_string();
                    full_session[size_full_session] = stack[1].to_string();
                    previous_node = new_node;
                }
            } else {
                panic!(
                    "Did not found a correct session for role {:?}. Found session: {:?}",
                    current_role, full_session
                )
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
                branches_already_seen,
                branching_sessions,
                group_branches,
                cfsm,
            )
        } else if stack.len() == 1 && stack[0] == "RoleBroadcast" {
            // If it is a broadcasting role

            let mut number_of_send = 0;

            let mut all_branches = Vec::new();

            // Check all the sessions
            for (pos, session) in full_session[..(full_session.len() - 1)]
                .to_vec()
                .iter()
                .enumerate()
            {
                match (
                    get_head_payload_continuation(session)?[0].as_str(),
                    number_of_send,
                    pos,
                ) {
                    ("Send", n_send, n_pos) if n_send == n_pos => {
                        number_of_send += 1;

                        // Should be a specific `enum`
                        let payload = &get_head_payload_continuation(session)?[1];

                        // Update all_choices
                        if let Some(choice) = branches_receivers.get(payload) {
                            for branch in choice.keys() {
                                all_branches.push(format!("{}::{}", payload, branch));
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

            for current_branch in all_branches.clone() {
                if let Some(new_node) = branches_already_seen.get(&current_branch) {
                    if !g.contains_edge(previous_node, *new_node) && previous_node != *new_node {
                        g.add_edge(previous_node, *new_node, "µ".to_string());

                        if let Some(elt) = cfsm.pop() {
                            cfsm.push((elt.0, new_node.index()));
                        }
                    }
                } else {
                    // If the node was not added
                    if !node_added {
                        // Increase the index for the nodes
                        index_node.push(0);

                        // Increase the depth level
                        depth_level += 1;

                        node_added = true;
                    }

                    let session = if let Some(session) = branching_sessions.get(&current_branch) {
                        session[..(session.len() - 1)].to_vec()
                    } else {
                        panic!("Missing session")
                    };

                    let mut temp_branches_already_seen = branches_already_seen.clone();

                    for temp_current_branch in all_branches.clone() {
                        temp_branches_already_seen
                            .insert(temp_current_branch.clone(), previous_node);
                    }

                    let result = aux_get_graph(
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
                        temp_branches_already_seen.clone(),
                        branching_sessions.clone(),
                        group_branches.clone(),
                        cfsm,
                    )?;

                    g = result.0;
                    cfsm = result.1;

                    // Insert the new node/branch in the list of the ones already seen
                    let index_group = if let Some(index) = group_branches.get(&current_branch) {
                        index
                    } else {
                        panic!("Missing index")
                    };

                    for (temp_current_branch, temp_index) in group_branches.clone() {
                        if temp_index == *index_group {
                            branches_already_seen
                                .insert(temp_current_branch.clone(), previous_node);
                        }
                    }
                }
            }

            Ok((g, cfsm))
        } else {
            panic!(
                "Did not found a correct stack for role {}. \
                Found stack and session: {:?} / {:?}",
                current_role, stack, full_session
            )
        }
    }
}

// Build the digraphs.
#[doc(hidden)]
pub(crate) fn get_graph_session(
    current_role: &str,
    full_session: VecOfStr,
    roles: &[String],
    branches_receivers: HashMap<String, HashMapStrVecOfStr>,
    branching_sessions: HashMapStrVecOfStr,
    group_branches: HashMap<String, i32>,
) -> Result<(GraphOfStrStr, VecOfStr), Box<dyn Error>> {
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

    let index_current_role = if let Some(elt) = roles.iter().position(|r| r == current_role) {
        elt
    } else {
        panic!(
            "Issue with roles {:?} and current_role {:?}",
            roles, current_role
        )
    };

    // The index of the current_role among the roles
    let start_depth_level = 0;

    // The branches already seen
    let state_branches_already_seen = RandomState::new();
    let branches_already_seen: HashMap<String, NodeIndex<u32>> =
        HashMap::with_hasher(state_branches_already_seen);

    let cfsm: VecOfTuple = Vec::new();

    let (result, cfsm) = aux_get_graph(
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
        branches_already_seen,
        branching_sessions,
        group_branches,
        cfsm,
    )?;

    // The missing strings for starting cfsm
    let mut cfsm_result = vec![".outputs".to_string(), ".state graph".to_string()];

    // Format the tuples into strings and add them to cfsm_result
    let mut clean_cfsm = cfsm
        .iter()
        .map(|(s, i)| format!("{}{}", s, i))
        .collect::<Vec<String>>();

    cfsm_result.append(&mut clean_cfsm);

    // The missing strings for ending cfsm
    cfsm_result.push(format!(".marking {}0", current_role));
    cfsm_result.push(".end".to_string());

    Ok((result, cfsm_result))
}

///////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::hash_map::RandomState;
    use std::collections::HashMap;

    #[test]
    fn test_clean_session() {
        let dirty_session = "&&mpstthree::meshedchannels::MeshedChannels<mpstthree::\
            binary::struct_trait::recv::Recv<checking_recursion::\
            Branches0AtoB, mpstthree::binary::struct_trait::end::End>, mpstthree\
            ::binary::struct_trait::recv::Recv<i32, mpstthree::binary::\
            struct_trait::send::Send<i32, mpstthree::binary::struct_trait::end::\
            End>>, mpstthree::role::c::RoleC<mpstthree::role::c::RoleC<\
            mpstthree::role::b::RoleB<mpstthree::role::end::RoleEnd>>>, mpstthree\
            ::name::a::NameA>";

        let clean_session_compare = vec![
            "Recv<Branches0AtoB,End>",
            "Recv<i32,Send<i32,End>>",
            "RoleC<RoleC<RoleB<RoleEnd>>>",
            "RoleA<RoleEnd>",
            "RoleA",
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
            ::end::RoleEnd>, mpstthree::name::a::NameA>"
                .to_string(),
            "mpstthree::meshedchannels::MeshedChannels<mpstthree::\
            binary::struct_trait::end::End, mpstthree::binary::struct_trait::\
            recv::Recv<i32, mpstthree::binary::struct_trait::send::Send<\
            i32, mpstthree::binary::struct_trait::recv::Recv<checking_recursion\
            ::Branches0CtoB, mpstthree::binary::struct_trait::end::End>>>, mpstthree\
            ::role::b::RoleB<mpstthree::role::b::RoleB<mpstthree::role::b::RoleB\
            <mpstthree::role::end::RoleEnd>>>, mpstthree::name::c::NameC>"
                .to_string(),
            "mpstthree::meshedchannels::\
            MeshedChannels<mpstthree::binary::struct_trait::send::Send<\
            checking_recursion::Branches0AtoB, mpstthree::binary::struct_trait\
            ::end::End>, mpstthree::binary::struct_trait::send::Send<i32, mpstthree\
            ::binary::struct_trait::recv::Recv<i32, mpstthree::binary::struct_trait\
            ::send::Send<checking_recursion::Branches0CtoB, mpstthree::binary::\
            struct_trait::end::End>>>, mpstthree::role::c::RoleC<mpstthree::\
            role::c::RoleC<mpstthree::role::broadcast::RoleBroadcast>>, mpstthree\
            ::name::b::NameB>"
                .to_string(),
        ];

        // The hasher of the HashMap
        let state_clean_sessions_compare = RandomState::new();

        // The result
        let mut clean_sessions_compare: HashMapStrVecOfStr =
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

        let clean_roles = vec![
            "RoleA".to_string(),
            "RoleB".to_string(),
            "RoleC".to_string(),
        ];

        assert_eq!(
            (clean_sessions_compare, clean_roles),
            clean_sessions(dirty_sessions).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_clean_sessions_panic() {
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
        ];

        clean_sessions(dirty_sessions).unwrap();
    }

    #[test]
    fn test_get_blocks() {
        let dirty_blocks = "MeshedChannels<Send<Branches0AtoB,End>,Send\
        <i32,Recv<i32,Send<Branches0CtoB,End>>>,RoleC\
        <RoleC<RoleBroadcast>>,RoleB<RoleEnd>>";

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
        let dirty_end = "End";

        let clean_end = vec!["End"];

        assert_eq!(clean_end, get_head_payload_continuation(dirty_end).unwrap());

        // RoleEnd
        let dirty_role_end = "RoleEnd";

        let clean_role_end = vec!["RoleEnd"];

        assert_eq!(
            clean_role_end,
            get_head_payload_continuation(dirty_role_end).unwrap()
        );

        // Random
        let dirty_random = "Recv<i32,Send<i32,Recv<Branches0CtoB,End>>>";

        let clean_random = vec!["Recv", "i32", "Send<i32,Recv<Branches0CtoB,End>>"];

        assert_eq!(
            clean_random,
            get_head_payload_continuation(dirty_random).unwrap()
        );
    }

    #[test]
    fn test_extract_index_node() {
        assert_eq!("0.1.4.5", extract_index_node(&[0, 1, 4, 5], 3).unwrap());

        assert_eq!("0.1.4", extract_index_node(&[0, 1, 4, 5], 2).unwrap());

        assert_eq!("0", extract_index_node(&[0, 1, 4, 5], 0).unwrap());
    }

    #[test]
    fn test_build_dual() {
        let session = "Recv<i32,Send<Branches0CtoB,End>>";

        assert_eq!(
            "Send<i32,Recv<Branches0CtoB,End>>",
            build_dual(session).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_build_dual_panic() {
        let session = "Coco<i32,Banana<Branches0CtoB,End>>";

        build_dual(session).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_aux_graph_panic_stack() {
        let state_branches = RandomState::new();
        let branches_receivers: HashMap<String, HashMapStrVecOfStr> =
            HashMap::with_hasher(state_branches);

        let state_branching_sessions = RandomState::new();
        let branching_sessions: HashMapStrVecOfStr = HashMap::with_hasher(state_branching_sessions);

        let state_group_branches = RandomState::new();
        let group_branches: HashMap<String, i32> = HashMap::with_hasher(state_group_branches);

        let current_role = "RoleA";

        let full_session = vec!["Recv<(),End>".to_string(), "RoleEnd".to_string()];

        let roles = vec!["RoleA".to_string(), "RoleB".to_string()];

        get_graph_session(
            current_role,
            full_session,
            &roles,
            branches_receivers,
            branching_sessions,
            group_branches,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_aux_graph_panic_session() {
        let state_branches = RandomState::new();
        let branches_receivers: HashMap<String, HashMapStrVecOfStr> =
            HashMap::with_hasher(state_branches);

        let state_branching_sessions = RandomState::new();
        let branching_sessions: HashMapStrVecOfStr = HashMap::with_hasher(state_branching_sessions);

        let state_group_branches = RandomState::new();
        let group_branches: HashMap<String, i32> = HashMap::with_hasher(state_group_branches);

        let current_role = "RoleB";

        let full_session = vec!["End".to_string(), "RoleA<RoleEnd>".to_string()];

        let roles = vec!["RoleA".to_string(), "RoleB".to_string()];

        get_graph_session(
            current_role,
            full_session,
            &roles,
            branches_receivers,
            branching_sessions,
            group_branches,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_aux_graph_panic_choice_end() {
        let state_branches = RandomState::new();
        let branches_receivers: HashMap<String, HashMapStrVecOfStr> =
            HashMap::with_hasher(state_branches);

        let state_branching_sessions = RandomState::new();
        let branching_sessions: HashMapStrVecOfStr = HashMap::with_hasher(state_branching_sessions);

        let state_group_branches = RandomState::new();
        let group_branches: HashMap<String, i32> = HashMap::with_hasher(state_group_branches);

        let current_role = "RoleA";

        let full_session = vec![
            "End".to_string(),
            "End".to_string(),
            "RoleAtoAll<RoleEnd,RoleEnd>".to_string(),
        ];

        let roles = vec![
            "RoleA".to_string(),
            "RoleB".to_string(),
            "RoleC".to_string(),
        ];

        get_graph_session(
            current_role,
            full_session,
            &roles,
            branches_receivers,
            branching_sessions,
            group_branches,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_aux_graph_panic_choice_end_send() {
        let state_branches = RandomState::new();
        let branches_receivers: HashMap<String, HashMapStrVecOfStr> =
            HashMap::with_hasher(state_branches);

        let state_branching_sessions = RandomState::new();
        let branching_sessions: HashMapStrVecOfStr = HashMap::with_hasher(state_branching_sessions);

        let state_group_branches = RandomState::new();
        let group_branches: HashMap<String, i32> = HashMap::with_hasher(state_group_branches);

        let current_role = "RoleA";

        let full_session = vec![
            "End".to_string(),
            "Send<(),End>".to_string(),
            "RoleAtoAll<RoleEnd,RoleEnd>".to_string(),
        ];

        let roles = vec![
            "RoleA".to_string(),
            "RoleB".to_string(),
            "RoleC".to_string(),
        ];

        get_graph_session(
            current_role,
            full_session,
            &roles,
            branches_receivers,
            branching_sessions,
            group_branches,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_aux_graph_panic_choice_recv_recv() {
        let state_branches = RandomState::new();
        let branches_receivers: HashMap<String, HashMapStrVecOfStr> =
            HashMap::with_hasher(state_branches);

        let state_branching_sessions = RandomState::new();
        let branching_sessions: HashMapStrVecOfStr = HashMap::with_hasher(state_branching_sessions);

        let state_group_branches = RandomState::new();
        let group_branches: HashMap<String, i32> = HashMap::with_hasher(state_group_branches);

        let current_role = "RoleA";

        let full_session = vec![
            "Recv<(),End>".to_string(),
            "Recv<(),End>".to_string(),
            "RoleAlltoB<RoleEnd,RoleEnd>".to_string(),
        ];

        let roles = vec![
            "RoleA".to_string(),
            "RoleB".to_string(),
            "RoleC".to_string(),
        ];

        get_graph_session(
            current_role,
            full_session,
            &roles,
            branches_receivers,
            branching_sessions,
            group_branches,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_aux_graph_panic_enum_choice_index() {
        let state_branches = RandomState::new();
        let mut branches_receivers: HashMap<String, HashMapStrVecOfStr> =
            HashMap::with_hasher(state_branches);

        let state_branches_choice_end = RandomState::new();
        let mut branches_receivers_choice_end: HashMapStrVecOfStr =
            HashMap::with_hasher(state_branches_choice_end);

        branches_receivers_choice_end.insert(
            "End".to_string(),
            vec!["End".to_string(), "End".to_string(), "RoleEnd".to_string()],
        );

        branches_receivers.insert(
            "Branching0AtoB".to_string(),
            branches_receivers_choice_end.clone(),
        );
        branches_receivers.insert(
            "Branching0AtoC".to_string(),
            branches_receivers_choice_end.clone(),
        );

        let state_branching_sessions = RandomState::new();
        let mut branching_sessions: HashMapStrVecOfStr =
            HashMap::with_hasher(state_branching_sessions);

        branching_sessions.insert(
            "Branching0AtoB::End".to_string(),
            vec!["End".to_string(), "End".to_string(), "RoleEnd".to_string()],
        );
        branching_sessions.insert(
            "Branching0AtoC::End".to_string(),
            vec!["End".to_string(), "End".to_string(), "RoleEnd".to_string()],
        );

        let state_group_branches = RandomState::new();
        let mut group_branches: HashMap<String, i32> = HashMap::with_hasher(state_group_branches);

        group_branches.insert("Branching0AtoB::End".to_string(), 0);
        group_branches.insert("Branching0AtoC::End".to_string(), 0);

        let current_role = "RoleA";

        let full_session = vec![
            "Send<Branching0AtoB,End>".to_string(),
            "Send<Branching0AtoC,End>".to_string(),
            "RoleBroadcast".to_string(),
        ];

        let roles = vec![
            "RoleA".to_string(),
            "RoleB".to_string(),
            "RoleC".to_string(),
        ];

        get_graph_session(
            current_role,
            full_session,
            &roles,
            branches_receivers,
            branching_sessions,
            group_branches,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_aux_graph_panic_enum_offer_index() {
        let state_branches = RandomState::new();
        let mut branches_receivers: HashMap<String, HashMapStrVecOfStr> =
            HashMap::with_hasher(state_branches);

        let state_branches_choice_end = RandomState::new();
        let mut branches_receivers_choice_end: HashMapStrVecOfStr =
            HashMap::with_hasher(state_branches_choice_end);

        branches_receivers_choice_end.insert(
            "End".to_string(),
            vec!["End".to_string(), "End".to_string(), "RoleEnd".to_string()],
        );

        branches_receivers.insert(
            "Branching0AtoB".to_string(),
            branches_receivers_choice_end.clone(),
        );
        branches_receivers.insert(
            "Branching0AtoC".to_string(),
            branches_receivers_choice_end.clone(),
        );

        let state_branching_sessions = RandomState::new();
        let mut branching_sessions: HashMapStrVecOfStr =
            HashMap::with_hasher(state_branching_sessions);

        branching_sessions.insert(
            "Branching0AtoB::End".to_string(),
            vec!["End".to_string(), "End".to_string(), "RoleEnd".to_string()],
        );
        branching_sessions.insert(
            "Branching0AtoC::End".to_string(),
            vec!["End".to_string(), "End".to_string(), "RoleEnd".to_string()],
        );

        let state_group_branches = RandomState::new();
        let mut group_branches: HashMap<String, i32> = HashMap::with_hasher(state_group_branches);

        group_branches.insert("Branching0AtoB::End".to_string(), 0);
        group_branches.insert("Branching0AtoC::End".to_string(), 0);

        let current_role = "RoleA";

        let full_session = vec![
            "Recv<Branching0AtoB,End>".to_string(),
            "End".to_string(),
            "RoleB<RoleEnd>".to_string(),
        ];

        let roles = vec![
            "RoleA".to_string(),
            "RoleB".to_string(),
            "RoleC".to_string(),
        ];

        get_graph_session(
            current_role,
            full_session,
            &roles,
            branches_receivers,
            branching_sessions,
            group_branches,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_aux_graph_panic_enum_missing() {
        let state_branches = RandomState::new();
        let branches_receivers: HashMap<String, HashMapStrVecOfStr> =
            HashMap::with_hasher(state_branches);

        let state_branching_sessions = RandomState::new();
        let mut branching_sessions: HashMapStrVecOfStr =
            HashMap::with_hasher(state_branching_sessions);

        branching_sessions.insert(
            "Branching0AtoB::End".to_string(),
            vec!["End".to_string(), "End".to_string(), "RoleEnd".to_string()],
        );
        branching_sessions.insert(
            "Branching0AtoC::End".to_string(),
            vec!["End".to_string(), "End".to_string(), "RoleEnd".to_string()],
        );

        let state_group_branches = RandomState::new();
        let mut group_branches: HashMap<String, i32> = HashMap::with_hasher(state_group_branches);

        group_branches.insert("Branching0AtoB::End".to_string(), 0);
        group_branches.insert("Branching0AtoC::End".to_string(), 0);

        let current_role = "RoleA";

        let full_session = vec![
            "Send<Branching0AtoB,End>".to_string(),
            "Send<Branching0AtoC,End>".to_string(),
            "RoleBroadcast".to_string(),
        ];

        let roles = vec![
            "RoleA".to_string(),
            "RoleB".to_string(),
            "RoleC".to_string(),
        ];

        get_graph_session(
            current_role,
            full_session,
            &roles,
            branches_receivers,
            branching_sessions,
            group_branches,
        )
        .unwrap();
    }
}
