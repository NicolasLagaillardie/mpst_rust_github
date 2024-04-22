use super::super::{GlobalElements, Tree};
use inflector::Inflector;
use std::io::Write;

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
                    writeln!(
                        generated_file,
                        "struct {};",
                        payload
                            .to_title_case()
                            .chars()
                            .filter(|c| !c.is_whitespace())
                            .collect::<String>()
                    )?;
                    already_generated.push(payload.to_title_case().to_string());
                }
            }

            for (name_message, payload) in main_tree.messages_with_payloads.iter() {
                if !already_generated.contains(&(name_message).to_title_case()) {
                    if payload.is_empty() {
                        writeln!(
                            generated_file,
                            "struct {};",
                            name_message
                                .to_title_case()
                                .chars()
                                .filter(|c| !c.is_whitespace())
                                .collect::<String>()
                        )?;
                    } else {
                        writeln!(
                            generated_file,
                            "struct {} {{ payload: {} }}",
                            name_message
                                .to_title_case()
                                .chars()
                                .filter(|c| !c.is_whitespace())
                                .collect::<String>(),
                            payload
                                .to_title_case()
                                .chars()
                                .filter(|c| !c.is_whitespace())
                                .collect::<String>()
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
