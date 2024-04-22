use std::fs::File;

use std::io::{BufReader, Error, Lines};

use std::collections::HashMap;
use std::iter::{Enumerate, Map};

use super::{line_is_message::update_messages, regex::*, GlobalElements, Tree};

fn init_messages(
    global_elements: &mut GlobalElements,
    tree: &mut Tree,
    current_index_string: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for (role, channels) in tree.messages.iter_mut() {
        for other_role in global_elements.roles.iter() {
            if other_role != role {
                channels.insert(other_role.to_string(), vec![]);
            }
        }
    }

    for (role, channels) in tree.first_message.iter_mut() {
        for other_role in global_elements.roles.iter() {
            if other_role != role {
                channels.insert(
                    other_role.to_string(),
                    format!(
                        "Message_{}_v_0_From{}To{}",
                        current_index_string, role, other_role
                    ),
                );
            }
        }
    }

    for (role, channels) in tree.last_message.iter_mut() {
        for other_role in global_elements.roles.iter() {
            if other_role != role {
                channels.insert(
                    other_role.to_string(),
                    format!(
                        "Message_{}_v_0_From{}To{}",
                        current_index_string, role, other_role
                    ),
                );
            }
        }
    }

    Ok(())
}

fn init_sub_tree(
    global_elements: &mut GlobalElements,
    temp_index: &[i32],
    current_index_string: &str,
    sender: &str,
) -> Result<Tree, Box<dyn std::error::Error>> {
    let mut sub_tree = Tree {
        index: temp_index.to_vec(),
        message_with_payloads: HashMap::new(),
        messages: HashMap::new(),
        first_message: HashMap::new(),
        last_message: HashMap::new(),
        stacks: HashMap::new(),
        first_stack: HashMap::new(),
        last_stack: HashMap::new(),
        enums: HashMap::new(),
        endpoints: HashMap::new(),
        sub_trees: vec![],
    };

    sub_tree
        .enums
        .insert(current_index_string.to_string(), (sender.to_string(), 0));

    for role in global_elements.roles.iter() {
        sub_tree.messages.insert(role.to_string(), HashMap::new());
        sub_tree
            .first_message
            .insert(role.to_string(), HashMap::new());
        sub_tree
            .last_message
            .insert(role.to_string(), HashMap::new());
        sub_tree.stacks.insert(role.to_string(), vec![]);
    }

    for role in global_elements.roles.iter() {
        sub_tree.first_stack.insert(
            role.to_string(),
            format!("Ordering_{}_v_0_For{}", current_index_string, role),
        );
        sub_tree.last_stack.insert(
            role.to_string(),
            format!("Ordering_{}_v_0_For{}", current_index_string, role),
        );
        sub_tree.endpoints.insert(
            role.to_string(),
            vec![format!("Endpoint_{}_For{}", current_index_string, role)],
        );
    }

    init_messages(global_elements, &mut sub_tree, current_index_string)?;

    Ok(sub_tree)
}

pub(crate) fn process_line(
    lines_iter: &mut Map<
        Enumerate<Lines<BufReader<File>>>,
        impl FnMut((usize, Result<String, Error>)) -> (usize, String),
    >,
    global_elements: &mut GlobalElements,
    parent_tree: &mut Tree,
    main_tree: &mut Tree,
    bracket_offset: &mut usize,
) -> Result<(), Box<dyn std::error::Error>> {
    match lines_iter.next() {
        None => Ok(()),
        Some((line_number, line)) => {
            global_elements.opening_brackets += line.matches('{').count();
            global_elements.closing_brackets += line.matches('}').count();

            println!(
                "line: {:?} / {} / {} / {} / {} / {}",
                line,
                global_elements.opening_brackets,
                global_elements.closing_brackets,
                bracket_offset,
                main_tree
                    .index
                    .iter()
                    .map(|&id| id.to_string())
                    .collect::<Vec<String>>()
                    .join("."),
                parent_tree
                    .index
                    .iter()
                    .map(|&id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(".")
            );

            if global_elements.opening_brackets < global_elements.closing_brackets {
                return Err(format!(
                    "There are too many closing brackets. See line: {}",
                    line_number
                )
                .into());
            }

            if global_elements.opening_brackets
                <= global_elements.closing_brackets + *bracket_offset
            {
                if global_elements.closing_brackets > global_elements.opening_brackets {
                    *bracket_offset =
                        global_elements.closing_brackets - global_elements.opening_brackets - 1;
                }

                println!(
                    "{} / {} / {}",
                    global_elements.opening_brackets,
                    global_elements.closing_brackets,
                    bracket_offset
                );

                // let temp_basic_index_len = main_tree.index.len() - 1;
                // main_tree.index[temp_basic_index_len] += 1;

                // let basic_index_string = temp_basic_index
                //     .iter()
                //     .map(|&id| id.to_string())
                //     .collect::<Vec<_>>()
                //     .join("_");

                // let elt = parent_tree.enums.get_mut(&basic_index_string).unwrap();

                // let sender = &elt.0;
                // elt.1 += 1;

                // parent_tree.sub_trees.push(sub_tree);

                println!(
                    "{} / {} / {} / {} / {}",
                    global_elements.opening_brackets,
                    global_elements.closing_brackets,
                    bracket_offset,
                    main_tree
                        .index
                        .iter()
                        .map(|&id| id.to_string())
                        .collect::<Vec<String>>()
                        .join("."),
                    parent_tree
                        .index
                        .iter()
                        .map(|&id| id.to_string())
                        .collect::<Vec<String>>()
                        .join(".")
                );

                return process_line(
                    lines_iter,
                    global_elements,
                    parent_tree,
                    main_tree,
                    bracket_offset,
                );
            }

            if check_global(&line) && line_number == 0 {
                let captured_fields = GLOBAL_PROTOCOL.captures(&line).unwrap();

                let name = &captured_fields["name"];

                println!("{:?}", name);

                if global_elements.output.is_none() {
                    global_elements.output = Some(File::create(format!(
                        "{}{}.rs",
                        global_elements.output_path, name
                    ))?);
                }

                for (_, [role]) in ROLE.captures_iter(&line).map(|c| c.extract()) {
                    global_elements.roles.push(role.into());
                    main_tree.messages.insert(role.to_string(), HashMap::new());
                    main_tree
                        .first_message
                        .insert(role.to_string(), HashMap::new());
                    main_tree
                        .last_message
                        .insert(role.to_string(), HashMap::new());
                    main_tree.stacks.insert(role.to_string(), vec![]);
                }

                for (_, [role]) in ROLE.captures_iter(&line).map(|c| c.extract()) {
                    main_tree
                        .first_stack
                        .insert(role.to_string(), format!("Ordering_0_v_0_For{}", role));
                    main_tree
                        .last_stack
                        .insert(role.to_string(), format!("Ordering_0_v_0_For{}", role));
                    main_tree
                        .endpoints
                        .insert(role.to_string(), vec![format!("Endpoint_0_For{}", role)]);
                }

                global_elements.roles.sort();

                init_messages(global_elements, main_tree, "0")?;
            } else if !check_global(&line) && line_number > 0 {
                if check_message(&line) {
                    update_messages(
                        &line,
                        &global_elements.roles,
                        &line_number,
                        &mut global_elements.clocks,
                        &mut global_elements.payloads,
                        main_tree,
                    )?;
                } else if check_choice(&line) {
                    let captured_fields = CHOICE.captures(&line).unwrap();

                    let sender = &captured_fields["choice"];

                    let mut temp_index = main_tree.index.clone();
                    temp_index.push(0);

                    let current_index_string = temp_index
                        .iter()
                        .map(|&id| id.to_string())
                        .collect::<Vec<_>>()
                        .join("_");

                    let previous_index_string = main_tree
                        .index
                        .iter()
                        .map(|&id| id.to_string())
                        .collect::<Vec<_>>()
                        .join("_");

                    main_tree
                        .enums
                        .insert(current_index_string.to_string(), (sender.to_string(), 0));

                    let mut sub_tree =
                        init_sub_tree(global_elements, &temp_index, &current_index_string, sender)?;

                    for receiver in global_elements.roles.iter() {
                        if receiver != sender {
                            // The sender must send the choice to each other role (receiver)
                            let channels_sender = main_tree.messages.get_mut(sender).unwrap();

                            let messages_sender = channels_sender.get_mut(receiver).unwrap();

                            let last_channel_sender =
                                main_tree.last_message.get_mut(sender).unwrap();
                            let last_messages_sender =
                                last_channel_sender.get_mut(receiver).unwrap();

                            messages_sender.push(
                                format!(
                                    "type {} = SendTimed<Choice_{}_From{}To{}, ' ', -2, false, -1, false, ' ', End>;",
                                    last_messages_sender,
                                    previous_index_string,
                                    sender,
                                    receiver,
                                )
                            );

                            *last_messages_sender = "".to_string();

                            // The receiver must receive the choice from the sender
                            let channels_receiver = main_tree.messages.get_mut(receiver).unwrap();

                            let messages_receiver = channels_receiver.get_mut(sender).unwrap();

                            let last_channel_receiver =
                                main_tree.last_message.get_mut(receiver).unwrap();
                            let last_messages_receiver =
                                last_channel_receiver.get_mut(sender).unwrap();

                            messages_receiver.push(
                                format!(
                                    "type {} = RecvTimed<Choice_{}_From{}To{}, ' ', -2, false, -1, false, ' ', End>;",
                                    last_messages_receiver,
                                    previous_index_string,
                                    sender,
                                    receiver,
                                )
                            );

                            *last_messages_receiver = "".to_string();

                            // Update stack for the receiver:
                            // they must receive the choice from the sender
                            let stack_receiver = main_tree.stacks.get_mut(receiver).unwrap();

                            let last_stacks_receiver =
                                main_tree.last_stack.get_mut(receiver).unwrap();

                            stack_receiver.push(format!(
                                "type {} = Role{}<RoleEnd>;",
                                last_stacks_receiver, sender
                            ));

                            *last_stacks_receiver = "".to_string();
                        } else {
                            // Update stack for the sender:
                            // they must broadcast their choice
                            let stack_sender = main_tree.stacks.get_mut(sender).unwrap();

                            let last_stacks_sender = main_tree.last_stack.get_mut(sender).unwrap();

                            stack_sender
                                .push(format!("type {} = RoleBroadcast;", last_stacks_sender));

                            *last_stacks_sender = "".to_string();
                        }
                    }

                    *bracket_offset += 1;

                    process_line(
                        lines_iter,
                        global_elements,
                        main_tree,
                        &mut sub_tree,
                        bracket_offset,
                    )?;

                    main_tree.sub_trees.push(sub_tree);
                } else if check_or(&line) {
                    // Get the original sender of the choice
                    let mut temp_basic_index = main_tree.index.clone();
                    let temp_basic_index_len = temp_basic_index.len() - 1;
                    temp_basic_index[temp_basic_index_len] = 0;

                    let basic_index_string = temp_basic_index
                        .iter()
                        .map(|&id| id.to_string())
                        .collect::<Vec<_>>()
                        .join("_");

                    let elt = parent_tree.enums.get_mut(&basic_index_string).unwrap();

                    let sender = &elt.0;
                    elt.1 += 1;

                    // Update everything in the main_tree
                    let mut temp_index = main_tree.index.clone();
                    let temp_index_len = temp_index.len() - 1;
                    temp_index[temp_index_len] += 1;

                    let current_index_string = temp_index
                        .iter()
                        .map(|&id| id.to_string())
                        .collect::<Vec<_>>()
                        .join("_");

                    let mut sub_tree =
                        init_sub_tree(global_elements, &temp_index, &current_index_string, sender)?;

                    process_line(
                        lines_iter,
                        global_elements,
                        parent_tree,
                        &mut sub_tree,
                        bracket_offset,
                    )?;

                    parent_tree.sub_trees.push(sub_tree);
                } else if check_rec(&line) {
                    let captured_fields = REC.captures(&line).unwrap();

                    global_elements
                        .loops
                        .push(captured_fields["loop"].to_string());
                } else if check_continue(&line) {
                    let captured_fields = CONTINUE.captures(&line).unwrap();

                    if !global_elements
                        .loops
                        .contains(&(captured_fields["loop"].to_string()))
                    {
                        return Err(format!(
                            "There is a continue loop without an initialisation. See line: {}",
                            line_number
                        )
                        .into());
                    }
                }
            } else {
                return Err("This is not a timed global protocol.".into());
            }

            process_line(
                lines_iter,
                global_elements,
                parent_tree,
                main_tree,
                bracket_offset,
            )
        }
    }
}
