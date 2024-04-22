use super::super::{GlobalElements, Tree};
use std::io::Write;

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
