use super::super::{GlobalElements, Tree};
use std::io::Write;

/// Generate all sessions in the global_elements.output file using all elements gathered from
/// the nuscr protocol.
pub(crate) fn generate_stacks(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
) -> Result<(), Box<dyn std::error::Error>> {
    match global_elements.output.as_mut() {
        Some(generated_file) => {
            writeln!(
                generated_file,
                "// Stacks in depth {}",
                main_tree
                    .index
                    .iter()
                    .map(|&id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(".")
            )?;
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
