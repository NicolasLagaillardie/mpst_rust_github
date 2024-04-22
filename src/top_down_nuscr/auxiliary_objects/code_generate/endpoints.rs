use super::super::{GlobalElements, Tree};
use std::io::Write;

/// Generate all endpoints in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_endpoints(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            writeln!(
                generated_file,
                "// Endpoints in depth {}",
                main_tree
                    .index
                    .iter()
                    .map(|&id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(".")
            )?;
            // Generate the endpoints
            for role in global_elements.roles.iter() {
                writeln!(generated_file, "// Endpoint for role {}", role)?;

                for endpoint in main_tree.endpoints.get(role).unwrap() {
                    write!(generated_file, "type {} = MeshedChannels<", endpoint)?;
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
                }
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
