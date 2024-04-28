use super::super::{GlobalElements, Tree};
use std::io::Write;

/// Generate all enums
pub(crate) fn generate_enums(
    global_elements: &mut GlobalElements,
    main_tree: &Tree,
) -> Result<(), Box<dyn std::error::Error>> {
    if !main_tree.sub_trees.is_empty() {
        match global_elements.output.as_mut() {
            Some(generated_file) => {
                if !main_tree.enums.is_empty() {
                    writeln!(
                        generated_file,
                        "// Enums (Branchings) in depth {}",
                        main_tree
                            .index
                            .iter()
                            .map(|&id| id.to_string())
                            .collect::<Vec<String>>()
                            .join(".")
                    )?;
                }
                let index = main_tree
                    .index
                    .iter()
                    .map(|&id| id.to_string())
                    .collect::<Vec<String>>()
                    .join("_");
                for (_branch, elt) in main_tree.enums.iter() {
                    for role in global_elements.roles.iter() {
                        if role != &elt.0 {
                            writeln!(generated_file, "// Enums (Branchings) for {}", role)?;
                            writeln!(
                                generated_file,
                                "enum Choice_{}_From{}To{} {{",
                                index, elt.0, role,
                            )?;

                            for i in 0..=elt.1 {
                                writeln!(
                                    generated_file,
                                    "\tBranching{}(Endpoint_{}_{}_For{}),",
                                    i, index, i, role
                                )?;
                            }

                            writeln!(generated_file, "}}")?;
                        }

                        writeln!(generated_file)?;
                    }
                }

                for sub_tree in main_tree.sub_trees.iter() {
                    generate_enums(global_elements, sub_tree)?;
                }

                Ok(())
            }
            None => Err("Generated file was not initialised.".into()),
        }
    } else {
        Ok(())
    }
}
