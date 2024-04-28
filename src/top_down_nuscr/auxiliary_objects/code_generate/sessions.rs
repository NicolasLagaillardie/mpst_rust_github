use super::super::{GlobalElements, Tree};
use std::io::Write;

/// Generate all sessions in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_sessions(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            // Generate the sessions
            writeln!(
                generated_file,
                "// Binary sessions in depth {}",
                main_tree
                    .index
                    .iter()
                    .map(|&id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(".")
            )?;
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
