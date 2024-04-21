use super::{GlobalElements, Tree};
use inflector::Inflector;
use std::io::Write;

/// Generate all imports in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_imports(
    global_elements: &mut GlobalElements,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            // Generate the imports of necessary crates
            writeln!(
                generated_file,
                "use mpstthree::binary::struct_trait::end::End;"
            )?;
            writeln!(
                generated_file,
                "use mpstthree::binary_atmp::struct_trait::{{recv::RecvTimed, send::SendTimed}};"
            )?;
            writeln!(generated_file, "use mpstthree::generate_atmp;")?;
            if global_elements.has_choice {
                writeln!(
                    generated_file,
                    "use mpstthree::role::broadcast::RoleBroadcast;"
                )?;
            }
            writeln!(generated_file, "use mpstthree::role::end::RoleEnd;")?;
            writeln!(generated_file, "use std::collections::HashMap;")?;
            writeln!(generated_file, "use std::error::Error;")?;
            writeln!(generated_file, "use std::time::Instant;\n")?;

            writeln!(
                generated_file,
                "generate_atmp!(MeshedChannels, {});\n",
                global_elements.roles.join(", ")
            )?;

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all structs in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_structs(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
    already_generated: &mut Vec<String>,
    comment_payload: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            // Generate the structs for the global_elements.payloads
            if comment_payload {
                writeln!(generated_file, "// Types of the payloads")?;
            }

            for payload in global_elements.payloads.iter() {
                if !already_generated.contains(&(payload).to_title_case()) {
                    writeln!(generated_file, "struct {};", payload.to_title_case())?;
                    already_generated.push(payload.to_title_case().to_string());
                }
            }

            for (name_message, payload) in main_tree.message_with_payloads.iter() {
                if !already_generated.contains(&(name_message).to_title_case()) {
                    if payload.is_empty() {
                        writeln!(generated_file, "struct {};", name_message.to_title_case())?;
                    } else {
                        writeln!(
                            generated_file,
                            "struct {} {{ payload: {} }}",
                            name_message.to_title_case(),
                            payload.to_title_case()
                        )?;
                    }
                    already_generated.push(name_message.to_title_case().to_string());
                }
            }

            writeln!(generated_file)?;

            for sub_tree in &main_tree.sub_trees {
                generate_structs(global_elements, sub_tree, already_generated, false)?;
            }

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all sessions in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_sessions(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            // Generate the sessions
            for (role, channels) in main_tree.messages.iter() {
                writeln!(generated_file, "// Binary sessions for {}", role)?;
                for (other_role, role_messages) in channels.iter() {
                    for message in role_messages {
                        writeln!(generated_file, "{}", message)?;
                    }
                    let message = main_tree
                        .last_message
                        .get(role)
                        .unwrap()
                        .get(other_role)
                        .unwrap();
                    if !message.is_empty() {
                        writeln!(generated_file, "type {} = End;", message)?;
                    }
                }
                writeln!(generated_file)?;
            }

            for sub_tree in &main_tree.sub_trees {
                generate_sessions(global_elements, sub_tree)?;
            }

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all sessions in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_stacks(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            // Generate the stacks
            for (role, role_stacks) in main_tree.stacks.iter() {
                writeln!(generated_file, "// Stacks for {}", role)?;
                for stack in role_stacks {
                    writeln!(generated_file, "{}", stack)?;
                }
                let stack = main_tree.last_stack.get(role).unwrap();
                if !stack.is_empty() {
                    writeln!(generated_file, "type {} = RoleEnd;", stack)?;
                }
                writeln!(generated_file)?;
            }

            for sub_tree in &main_tree.sub_trees {
                generate_stacks(global_elements, sub_tree)?;
            }

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all enums TODO
pub(crate) fn generate_enums(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            for (branch, elt) in main_tree.enums.iter() {
                for role in global_elements.roles.iter() {
                    if role != &elt.0 {
                        writeln!(generated_file, "// Enums (Branching) for {}", role)?;
                        writeln!(
                            generated_file,
                            "enum Choice_{}_From{}To{} {{",
                            branch, elt.0, role,
                        )?;

                        let endpoints_role = main_tree.endpoints.get(role).unwrap();

                        println!("{:?}", endpoints_role);

                        for i in 0..=elt.1 {
                            writeln!(
                                generated_file,
                                "\tBranching{}({}),",
                                i, endpoints_role[i as usize]
                            )?;
                        }

                        writeln!(generated_file, "}}")?;
                    }

                    writeln!(generated_file)?;
                }
            }

            for sub_tree in &main_tree.sub_trees {
                generate_enums(global_elements, sub_tree)?;
            }

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all endpoints in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_endpoints(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            // Generate the endpoints
            for role in global_elements.roles.iter() {
                writeln!(generated_file, "// Endpoint for role {}", role)?;
                write!(
                    generated_file,
                    "type {} = MeshedChannels<",
                    main_tree.endpoints.get(role).unwrap()[0]
                )?;
                for other_role in global_elements.roles.iter() {
                    if other_role != role {
                        write!(
                            generated_file,
                            "{}, ",
                            main_tree
                                .first_message
                                .get(role)
                                .unwrap()
                                .get(other_role)
                                .unwrap()
                        )?;
                    }
                }
                writeln!(
                    generated_file,
                    "{}, Name{}>;",
                    main_tree.first_stack.get(role).unwrap(),
                    role
                )?;
                writeln!(generated_file)?;
            }

            for sub_tree in &main_tree.sub_trees {
                generate_endpoints(global_elements, sub_tree)?;
            }

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate all functions for the endpoints in the global_elements.output file
/// using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_fn_endpoints(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
    comment_section: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            // Add the main function and invite dev to complete code
            if comment_section {
                writeln!(generated_file, "// Fill in the functions here.")?;
            }

            let current_index_string = main_tree
                .index
                .iter()
                .map(|&id| id.to_string())
                .collect::<Vec<_>>()
                .join("_");

            let number_branches = main_tree.index[main_tree.index.len() - 1];

            for role in global_elements.roles.iter() {
                for index_branch in 0..=number_branches {
                    writeln!(
                    generated_file,
                    "fn endpoint_{}_{}_v_{}(s: {}, all_clocks: &mut HashMap<char, Instant>) -> Result<(), Box<dyn Error>> {{",
                    role.to_lowercase(),
                    current_index_string,
                    index_branch,
                    main_tree.endpoints.get(role).unwrap()[0]
                )?;

                    if let Some(role_clock) = global_elements.clocks.get(role) {
                        for clock in role_clock {
                            writeln!(
                                generated_file,
                                "\tall_clocks.insert('{}', Instant::now());",
                                clock
                            )?;
                        }
                    }

                    writeln!(generated_file)?;
                    writeln!(generated_file, "\tOk(())")?;
                    writeln!(generated_file, "}}")?;
                    writeln!(generated_file)?;
                }
            }

            for sub_tree in &main_tree.sub_trees {
                generate_fn_endpoints(global_elements, sub_tree, false)?;
            }

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}

/// Generate the main function in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_fn_main(
    global_elements: &mut GlobalElements,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            // Add the main function and invite dev to complete code
            writeln!(generated_file, "fn main() {{")?;

            write!(generated_file, "\tlet (")?;

            for role in global_elements.roles.iter() {
                write!(generated_file, "thread_{}, ", role.to_lowercase())?;
            }

            write!(generated_file, ") = fork_mpst(")?;

            for role in global_elements.roles.iter() {
                write!(generated_file, "endpoint_{}_0_v_0, ", role.to_lowercase())?;
            }

            writeln!(generated_file, ");")?;
            writeln!(generated_file)?;

            for role in global_elements.roles.iter() {
                writeln!(
                    generated_file,
                    "\tprintln!(\"Thread {}: {{:?}}\", thread_{}.join());",
                    role,
                    role.to_lowercase()
                )?;
            }

            writeln!(generated_file, "}}")?;

            Ok(())
        }
        None => Err("Generated file was not initialised.".into()),
    }
}
